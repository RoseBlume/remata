use std::io::{SeekFrom, Seek, self, Read};
use super::{
    IfdEntry,
    IfdType,
    TAGS,
    Endian
};


// lazy_static::lazy_static! {
//     static ref TAG_SET: HashSet<u16> =
//         TAGS.iter().map(|(t, _)| *t).collect();
// }

fn is_known_tag(tag: u16) -> bool {
    TAGS.iter().any(|(t, _)| *t == tag)
}



pub fn rational_to_f64(n: u32, d: u32) -> Option<f64> {
    if d == 0 {
        None
    } else {
        Some(n as f64 / d as f64)
    }
}



pub fn dms_to_deg(values: &[(u32, u32)]) -> Option<f64> {
    if values.len() != 3 {
        return None;
    }

    let deg = values[0].0 as f64 / values[0].1 as f64;
    let min = values[1].0 as f64 / values[1].1 as f64;
    let sec = values[2].0 as f64 / values[2].1 as f64;

    Some(deg + min / 60.0 + sec / 3600.0)
}


/// Read a big-endian u16 (JPEG markers are always big-endian)
pub fn read_be_u16<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}


/// Finds the TIFF header inside a JPEG EXIF segment.
/// Returns absolute file offset where TIFF header begins.
pub fn find_exif_tiff_start<R: Read + Seek>(reader: &mut R) -> io::Result<u64> {
    reader.seek(SeekFrom::Start(0))?;

    let mut soi = [0u8; 2];
    reader.read_exact(&mut soi)?;

    if soi != [0xFF, 0xD8] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a JPEG file",
        ));
    }

    loop {
        let mut marker_prefix = [0u8; 1];
        reader.read_exact(&mut marker_prefix)?;

        if marker_prefix[0] != 0xFF {
            continue;
        }

        let mut marker = [0u8; 1];
        reader.read_exact(&mut marker)?;
        let marker = marker[0];

        if marker == 0xFF {
            continue;
        }

        if marker == 0xD9 || marker == 0xDA {
            break;
        }

        let length = read_be_u16(reader)? as u64;

        if length < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid segment length",
            ));
        }

        if marker == 0xE1 {
            let mut header = [0u8; 6];
            reader.read_exact(&mut header)?;

            if &header == b"Exif\0\0" {
                return reader.stream_position();
            }

            // subtract header (6) + length field (2)
            let skip = length.checked_sub(8).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid EXIF segment size")
            })?;

            reader.seek(SeekFrom::Current(skip as i64))?;
        } else {
            let skip = length.checked_sub(2).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid segment size")
            })?;

            reader.seek(SeekFrom::Current(skip as i64))?;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No EXIF segment found",
    ))
}
pub fn parse_ifd<R: Read + Seek>(
    reader: &mut R,
    base_offset: u64,
    offset: u32,
    endian: Endian,
    file_len: u64,
) -> io::Result<Option<(Vec<IfdEntry>, u32)>> {
    const MAX_ENTRIES: u16 = 4096;

    if offset == 0 {
        return Ok(None);
    }

    // ---- Compute absolute offset safely ----
    let absolute = base_offset
        .checked_add(offset as u64)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "IFD offset overflow"))?;

    if absolute > file_len {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("IFD offset out of bounds: {}", absolute),
        ));
    }

    reader.seek(SeekFrom::Start(absolute))?;

    // ---- Read entry count ----
    let mut count_buf = [0u8; 2];
    reader.read_exact(&mut count_buf)?;

    let count = match endian {
        Endian::Big => u16::from_be_bytes(count_buf),
        Endian::Little => u16::from_le_bytes(count_buf),
    };

    if count > MAX_ENTRIES {
        eprintln!("⚠️ Corrupt IFD (too many entries: {}) — stopping", count);
        return Ok(None); // 🚨 do NOT try to skip, it will desync
    }

    let mut entries = Vec::new();

    // ---- Read entries one-by-one (streamed) ----
    for _ in 0..count {
        let mut buf = [0u8; 12];
        reader.read_exact(&mut buf)?;

        let tag = endian.read_u16(&buf, 0);

        // ---- Filter unknown tags early ----
        if !is_known_tag(tag) {
            continue;
        }

        let value_type_raw = endian.read_u16(&buf, 2);

        let value_type = match IfdType::try_from(value_type_raw) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let value_count = endian.read_u32(&buf, 4);
        let value_offset_or_inline = endian.read_u32(&buf, 8);

        // ---- Optional: sanity check value offset ----
        if value_offset_or_inline != 0 {
            let abs = match base_offset.checked_add(value_offset_or_inline as u64) {
                Some(v) => v,
                None => continue,
            };

            if abs > file_len {
                eprintln!(
                    "⚠️ Skipping tag {:04X} (invalid offset {})",
                    tag, abs
                );
                continue;
            }
        }

        entries.push(IfdEntry::new(
            tag,
            value_type,
            value_count,
            value_offset_or_inline,
        ));
    }

    // ---- Read next IFD offset ----
    let mut next_buf = [0u8; 4];
    if let Err(_) = reader.read_exact(&mut next_buf) {
        return Ok(Some((entries, 0))); // EOF → stop chain safely
    }

    let next_ifd_offset = match endian {
        Endian::Big => u32::from_be_bytes(next_buf),
        Endian::Little => u32::from_le_bytes(next_buf),
    };

    // ---- Validate next IFD offset ----
    if next_ifd_offset != 0 {
        let next_abs = match base_offset.checked_add(next_ifd_offset as u64) {
            Some(v) => v,
            None => {
                eprintln!("⚠️ next IFD offset overflow");
                return Ok(Some((entries, 0)));
            }
        };

        if next_abs > file_len {
            eprintln!("⚠️ Ignoring invalid next IFD offset: {}", next_abs);
            return Ok(Some((entries, 0))); // stop chain
        }
    }

    Ok(Some((entries, next_ifd_offset)))
}

pub fn read_ascii<R: Read + Seek>(
    reader: &mut R,
    base_offset: u64,
    offset: u32,
    count: u32,
) -> io::Result<String> {
    const MAX_STRING: u32 = 3_000_000; // 1 MB

    if count > MAX_STRING {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("ASCII field too large: {}", count),
        ));
    }

    let absolute = base_offset + offset as u64;
    reader.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; count as usize];
    reader.read_exact(&mut buf)?;

    if let Some(pos) = buf.iter().position(|&b| b == 0) {
        buf.truncate(pos);
    }

    Ok(String::from_utf8_lossy(&buf).to_string())
}


pub fn tag_name(tag: u16) -> &'static str {
    TAGS
        .iter()
        .find(|(t, _)| *t == tag)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
}



pub fn type_size(t: IfdType) -> usize {
    match t {
        IfdType::Byte | IfdType::Ascii | IfdType::Undefined | IfdType::Utf8 => 1,
        IfdType::Short => 2,
        IfdType::Long | IfdType::SLong => 4,
        IfdType::Rational | IfdType::SRational => 8,
    }
}

