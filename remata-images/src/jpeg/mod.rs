use crate::{ParserError};
// use remata_exif::{
//     Exif,
// };
use crate::common::{
    ResolutionUnit
};
pub struct Jpeg {
    pub width: u32,
    pub height: u32,
    pub horizontal_resolution: ResolutionUnit, // DPI (Dots Per Inch), PPM (Pixels Per Modules)
    pub vertical_resolution: ResolutionUnit,
    pub bit_depth: u32,
    pub compressed_bits_per_pixel: f64,
    // pub exif: Option<Exif>
}


impl Jpeg {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
            return Err(ParserError::new("Invalid Format"));
        }

        let mut i = 2;
        let mut width = 0;
        let mut height = 0;
        // let mut exif = None;

        while i + 4 < data.len() {
            if data[i] != 0xFF {
                return Err(ParserError::new("InvalidFormat"));
            }

            let marker = data[i + 1];
            let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;

            match marker {
                // SOF0 / SOF2 (image size)
                0xC0 | 0xC2 => {
                    height = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u32;
                    width = u16::from_be_bytes([data[i + 7], data[i + 8]]) as u32;
                }

                // APP1 (EXIF)
                0xE1 => {
                    let segment = &data[i + 4..i + 2 + len];
                    if segment.starts_with(b"Exif\0\0") {
                        // exif = Some(Exif::parse(&segment[6..])?);
                    }
                }

                _ => {}
            }

            i += 2 + len;
        }

        Ok(Jpeg {
            width,
            height,
            horizontal_resolution: ResolutionUnit::Unknown,
            vertical_resolution: ResolutionUnit::Unknown,
            bit_depth: 8,
            compressed_bits_per_pixel: 0.0,
            // exif,
        })
    }
}

