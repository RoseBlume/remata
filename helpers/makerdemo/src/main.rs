use std::env;
use std::fs::{File, self};
use std::io::{self, Write};

use remata_makernotes::nikon::parse;

pub struct MakerNotes {
    pub raw: Vec<u8>,
}

#[derive(Debug)]
pub enum ExifError {
    InvalidFormat,
    MakerNotesNotFound,
    UnexpectedEOF,
}
pub const MAKERNOTES: u16 = 0x927c;
pub const EXIF_SUBIFD: u16 = 0x8769;

impl MakerNotes {
    /// Extract MakerNotes from raw EXIF segment (APP1 payload)
    pub fn from_exif(exif_data: &[u8]) -> Result<Self, ExifError> {
        // EXIF header should start with "Exif\0\0"
        if exif_data.len() < 6 || &exif_data[0..6] != b"Exif\0\0" {
            return Err(ExifError::InvalidFormat);
        }

        let tiff = &exif_data[6..];

        // Byte order: "II" (little endian) or "MM" (big endian)
        let is_le = match &tiff[0..2] {
            b"II" => true,
            b"MM" => false,
            _ => return Err(ExifError::InvalidFormat),
        };

        let read_u16 = |buf: &[u8]| -> u16 {
            if is_le {
                u16::from_le_bytes([buf[0], buf[1]])
            } else {
                u16::from_be_bytes([buf[0], buf[1]])
            }
        };

        let read_u32 = |buf: &[u8]| -> u32 {
            if is_le {
                u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
            } else {
                u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]])
            }
        };

        // Offset to first IFD
        let ifd_offset = read_u32(&tiff[4..8]) as usize;

        Self::find_makernotes(tiff, ifd_offset, &read_u16, &read_u32)
    }

    fn find_makernotes(
        tiff: &[u8],
        offset: usize,
        read_u16: &dyn Fn(&[u8]) -> u16,
        read_u32: &dyn Fn(&[u8]) -> u32,
    ) -> Result<Self, ExifError> {
        if offset + 2 > tiff.len() {
            return Err(ExifError::UnexpectedEOF);
        }

        let entry_count = read_u16(&tiff[offset..offset + 2]) as usize;
        let mut pos = offset + 2;

        for _ in 0..entry_count {
            if pos + 12 > tiff.len() {
                return Err(ExifError::UnexpectedEOF);
            }

            let tag = read_u16(&tiff[pos..pos + 2]);
            let field_type = read_u16(&tiff[pos + 2..pos + 4]);
            let count = read_u32(&tiff[pos + 4..pos + 8]) as usize;
            let value_offset = read_u32(&tiff[pos + 8..pos + 12]) as usize;

            // MakerNotes tag
            if tag == MAKERNOTES {
                let byte_len = Self::type_size(field_type) * count;

                let data = if byte_len <= 4 {
                    // stored inline
                    tiff[pos + 8..pos + 8 + byte_len].to_vec()
                } else {
                    if value_offset + byte_len > tiff.len() {
                        return Err(ExifError::UnexpectedEOF);
                    }
                    tiff[value_offset..value_offset + byte_len].to_vec()
                };

                return Ok(MakerNotes { raw: data });
            }

            pos += 12;
        }

        // Look for Exif SubIFD (tag 0x8769)
        pos = offset + 2;
        for _ in 0..entry_count {
            let tag = read_u16(&tiff[pos..pos + 2]);
            let value_offset = read_u32(&tiff[pos + 8..pos + 12]) as usize;

            if tag == EXIF_SUBIFD {
                return Self::find_makernotes(tiff, value_offset, read_u16, read_u32);
            }

            pos += 12;
        }

        Err(ExifError::MakerNotesNotFound)
    }

    fn type_size(field_type: u16) -> usize {
        match field_type {
            1 | 2 | 7 => 1, // BYTE, ASCII, UNDEFINED
            3 => 2,        // SHORT
            4 | 9 => 4,    // LONG, SLONG
            5 | 10 => 8,   // RATIONAL, SRATIONAL
            _ => 1,
        }
    }
}



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[2] != "--dump" {
        eprintln!("Usage: {} <input_file> --dump [dump_file]", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let output_path = &args[3];
        // .nth(1)

    let data = match fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            std::process::exit(1);
        }
    };

    // Find EXIF header
    let exif_start = match data.windows(6).position(|w| w == b"Exif\0\0") {
        Some(pos) => pos,
        None => {
            eprintln!("No EXIF data found");
            std::process::exit(1);
        }
    };

    let exif_data = &data[exif_start..];

    let _maker_notes = match MakerNotes::from_exif(exif_data) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error extracting MakerNotes: {:?}", e);
            std::process::exit(1);
        }
    };
    let notes = parse(&data)?;
    let mut file = File::create(output_path)?;
    let note_str = format!("{}", notes);
    file.write_all(note_str.as_bytes())?;
    // Avoid expensive formatting when debugging speed
    // println!("{notes}");

    Ok(())
}