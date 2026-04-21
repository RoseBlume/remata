
use std::io::{Read, Seek, SeekFrom, Write, self};
use std::fs;
use super::ParseMode;

use crate::helpers::{
    read_rational_strings,
    read_srational_strings,
    read_numeric_values,
    read_ascii
};

use crate::starts::{
    find_exif_tiff_start,
    find_png_exif_start,
    find_webp_exif_start,
    find_heif_exif_start
};
use crate::gps::Gps;
use crate::ifd::{
    IfdEntry,
    IfdType
};

pub fn process_path(path: &str, output: Option<&str>, parse_mode: ParseMode) -> io::Result<()> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        process_file(path, output, parse_mode)?;
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();

            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext == "jpg" || ext == "jpeg" || ext == "tif" || ext == "tiff" {
                        let path_str = p.to_string_lossy();
                        process_file(&path_str, output, parse_mode)?;
                    }
                }
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not file or directory"));
    }

    Ok(())
}


pub fn process_file(path: &str, output: Option<&str>, parse_mode: ParseMode) -> io::Result<()> {
    let mut file = std::fs::File::open(path)?;

    let mut out: Box<dyn Write> = match output {
        Some(o) => Box::new(std::fs::File::create(o)?),
        None => Box::new(io::stdout()),
    };

    writeln!(out, "File: {}\n", path)?;

    // Detect format
    file.seek(SeekFrom::Start(0))?;
    let mut magic = [0u8; 12];
    file.read_exact(&mut magic)?;

    let tiff_start = if &magic[0..2] == b"\xFF\xD8" {
        writeln!(out, "Detected JPEG")?;
        find_exif_tiff_start(&mut file)?
    } else if &magic[0..8] == b"\x89PNG\r\n\x1a\n" {
        writeln!(out, "Detected PNG")?;
        find_png_exif_start(&mut file)?
    } else if &magic[0..4] == b"RIFF" && &magic[8..12] == b"WEBP" {
        writeln!(out, "Detected WebP")?;
        find_webp_exif_start(&mut file)?
    } else if &magic[4..8] == b"ftyp" {
        writeln!(out, "Detected HEIF/HEIC")?;
        find_heif_exif_start(&mut file)?
    } else {
        writeln!(out, "Assuming TIFF")?;
        0
    };

    file.seek(SeekFrom::Start(tiff_start))?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;

    let little = match &header[0..2] {
        b"II" => true,
        b"MM" => false,
        _ => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid TIFF header"));
        }
    };

    let magic = if little {
        u16::from_le_bytes([header[2], header[3]])
    } else {
        u16::from_be_bytes([header[2], header[3]])
    };

    if magic != 42 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid TIFF magic"));
    }

    let first_ifd_offset = if little {
        u32::from_le_bytes([header[4], header[5], header[6], header[7]])
    } else {
        u32::from_be_bytes([header[4], header[5], header[6], header[7]])
    };

    let mut current_offset = first_ifd_offset;
    let mut index = 0;

    while let Some((entries, next_offset)) =
        IfdEntry::parse(&mut file, tiff_start, current_offset, little, parse_mode)?
    {
        writeln!(out, "IFD {}:", index)?;

        for e in &entries {
            let (name, _desc) = crate::tags::tag_info(e.tag);

            match e.value_type {
                IfdType::Ascii => {
                    match read_ascii(&mut file, tiff_start, e.value_offset_or_inline, e.count) {
                        Ok(s) => writeln!(out, "  {:04X} ({}): {}", e.tag, name, s)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                IfdType::Rational => {
                    match read_rational_strings(&mut file, tiff_start, e, little) {
                        Ok(values) => writeln!(out, "  {:04X} ({}): {:?}", e.tag, name, values)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                IfdType::SRational => {
                    match read_srational_strings(&mut file, tiff_start, e, little) {
                        Ok(values) => writeln!(out, "  {:04X} ({}): {:?}", e.tag, name, values)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                IfdType::Long if e.tag == 0x8825 => {
                    writeln!(out, "  8825 (GPS IFD)")?;
                    match Gps::parse(&mut file, tiff_start, e.value_offset_or_inline, little, parse_mode) {
                        Ok(gps) => writeln!(out, "    GPS: {:?}", gps)?,
                        Err(_) => writeln!(out, "    GPS: <error>")?,
                    }
                }

                IfdType::Byte
                | IfdType::Short
                | IfdType::Long
                | IfdType::SLong
                | IfdType::Undefined => {
                    match read_numeric_values(&mut file, tiff_start, e, little) {
                        Ok(values) => writeln!(out, "  {:04X} ({}): {:?}", e.tag, name, values)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                _ => {
                    writeln!(
                        out,
                        "  {:04X} ({}): type={:?}, count={}",
                        e.tag, name, e.value_type, e.count
                    )?;
                }
            }
        }

        writeln!(out)?;

        if next_offset == 0 {
            break;
        }

        current_offset = next_offset;
        index += 1;
    }

    Ok(())
}
