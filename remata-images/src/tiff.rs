use crate::ParserError;

pub struct TiffMeta {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u16,
    pub compression: Option<String>,
    pub has_alpha: bool,
}

impl TiffMeta {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        if data.len() < 8 {
            return Err(ParserError::new("Data too small for TIFF"));
        }

        // Endianness
        let le = match &data[0..2] {
            b"II" => true,
            b"MM" => false,
            _ => return Err(ParserError::new("Invalid TIFF byte order")),
        };

        // Helper closures
        let read_u16 = |bytes: &[u8]| -> u16 {
            if le {
                u16::from_le_bytes(bytes.try_into().unwrap())
            } else {
                u16::from_be_bytes(bytes.try_into().unwrap())
            }
        };

        let read_u32 = |bytes: &[u8]| -> u32 {
            if le {
                u32::from_le_bytes(bytes.try_into().unwrap())
            } else {
                u32::from_be_bytes(bytes.try_into().unwrap())
            }
        };

        // Magic number (42)
        let magic = read_u16(&data[2..4]);
        if magic != 42 {
            return Err(ParserError::new("Invalid TIFF magic number"));
        }

        // Offset to first IFD
        let ifd_offset = read_u32(&data[4..8]) as usize;
        if data.len() < ifd_offset + 2 {
            return Err(ParserError::new("Invalid IFD offset"));
        }

        // Number of directory entries
        let entry_count = read_u16(&data[ifd_offset..ifd_offset + 2]) as usize;
        let mut offset = ifd_offset + 2;

        let mut width = None;
        let mut height = None;
        let mut bit_depth = None;
        let mut compression = None;
        let mut samples_per_pixel = 1;
        let mut extra_samples = 0;

        for _ in 0..entry_count {
            if data.len() < offset + 12 {
                return Err(ParserError::new("Truncated IFD entry"));
            }

            let tag = read_u16(&data[offset..offset + 2]);
            let field_type = read_u16(&data[offset + 2..offset + 4]);
            let count = read_u32(&data[offset + 4..offset + 8]);
            let value_offset = &data[offset + 8..offset + 12];

            // Helper to get value (only handles SHORT/LONG and count=1 safely)
            let value_u32 = match field_type {
                3 if count == 1 => read_u16(value_offset) as u32, // SHORT
                4 if count == 1 => read_u32(value_offset),        // LONG
                _ => 0,
            };

            match tag {
                256 => width = Some(value_u32),  // ImageWidth
                257 => height = Some(value_u32), // ImageLength
                258 => bit_depth = Some(value_u32 as u16), // BitsPerSample (simplified)
                259 => {
                    compression = Some(match value_u32 {
                        1 => "None".into(),
                        2 => "CCITT".into(),
                        5 => "LZW".into(),
                        6 => "JPEG".into(),
                        7 => "JPEG".into(),
                        8 => "Deflate".into(),
                        32773 => "PackBits".into(),
                        _ => format!("UNKNOWN({})", value_u32),
                    })
                }
                277 => samples_per_pixel = value_u32,
                338 => extra_samples = value_u32, // ExtraSamples (alpha indicator)
                _ => {}
            }

            offset += 12;
        }

        let width = width.ok_or_else(|| ParserError::new("Missing width"))?;
        let height = height.ok_or_else(|| ParserError::new("Missing height"))?;
        let bit_depth = bit_depth.unwrap_or(8);

        // Alpha detection
        let has_alpha = samples_per_pixel > 3 || extra_samples > 0;

        Ok(Self {
            width,
            height,
            bit_depth,
            compression,
            has_alpha,
        })
    }
}