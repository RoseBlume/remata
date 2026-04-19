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

pub enum ByteOrder {
    BigEndian,
    LittleEndian
}

pub enum MakerNoteType {
    Minolta2,
    Hp2,

}


use std::env;
use std::fs;
use std::io::{self, Write};

// use makernotes::MakerNotes;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[2] != "--dump" {
        eprintln!("Usage: {} <input_file> --dump [dump_file]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() > 3 {
        Some(&args[3])
    } else {
        None
    };

    let data = match fs::read(input_file) {
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

    let maker_notes = match MakerNotes::from_exif(exif_data) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error extracting MakerNotes: {:?}", e);
            std::process::exit(1);
        }
    };

    match output_file {
        Some(path) => {
            if let Err(e) = write_hexdump_to_file(&maker_notes.raw, path) {
                eprintln!("Failed to write dump file: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            print_hexdump(&maker_notes.raw);
        }
    }
}

/// Write hex dump to a file
fn write_hexdump_to_file(data: &[u8], path: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    write_hexdump(data, &mut file)
}

/// Print hex dump to stdout
fn print_hexdump(data: &[u8]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = write_hexdump(data, &mut handle);
}

/// Core hex dump writer
fn write_hexdump<W: Write>(data: &[u8], writer: &mut W) -> io::Result<()> {
    const BYTES_PER_LINE: usize = 16;

    for (i, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        let offset = i * BYTES_PER_LINE;

        // Offset
        write!(writer, "{:08x}  ", offset)?;

        // Hex bytes
        for j in 0..BYTES_PER_LINE {
            if j < chunk.len() {
                write!(writer, "{:02x} ", chunk[j])?;
            } else {
                write!(writer, "   ")?;
            }

            if j == 7 {
                write!(writer, " ")?;
            }
        }

        write!(writer, " |")?;

        // ASCII
        for &byte in chunk {
            let c = if byte.is_ascii_graphic() || byte == b' ' {
                byte as char
            } else {
                '.'
            };
            write!(writer, "{}", c)?;
        }

        writeln!(writer, "|")?;
    }

    Ok(())
}