use std::io::{self, Read};

// -----------------------------
// Helpers
// -----------------------------

fn read_u16<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut buf = [0; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_i32<R: Read>(r: &mut R) -> io::Result<i32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

// -----------------------------
// Enums
// -----------------------------

#[derive(Debug, Clone, Copy)]
pub enum BmpVersion {
    WindowsV3,
    WindowsV4,
    WindowsV5,
    Os2V1,
    Os2V2,
    Unknown(u32),
}

impl From<u32> for BmpVersion {
    fn from(size: u32) -> Self {
        match size {
            40 => Self::WindowsV3,
            108 => Self::WindowsV4,
            124 => Self::WindowsV5,
            12 => Self::Os2V1,
            64 => Self::Os2V2,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Compression {
    None,
    Rle8,
    Rle4,
    Bitfields,
    Jpeg,
    Png,
    Unknown(u32),
}

impl From<u32> for Compression {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Rle8,
            2 => Self::Rle4,
            3 => Self::Bitfields,
            4 => Self::Jpeg,
            5 => Self::Png,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    CalibratedRgb,
    DeviceRgb,
    DeviceCmyk,
    LinkedProfile,
    EmbeddedProfile,
    Windows,
    Srgb,
    Unknown(u32),
}

impl From<u32> for ColorSpace {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::CalibratedRgb,
            1 => Self::DeviceRgb,
            2 => Self::DeviceCmyk,
            0x4C494E4B => Self::LinkedProfile, // 'LINK'
            0x4D424544 => Self::EmbeddedProfile, // 'MBED'
            0x57696E20 => Self::Windows, // 'Win '
            0x73524742 => Self::Srgb, // 'sRGB'
            other => Self::Unknown(other),
        }
    }
}

// -----------------------------
// Headers
// -----------------------------

#[derive(Debug)]
pub struct BmpFileHeader {
    pub file_size: u32,
    pub reserved1: u16,
    pub reserved2: u16,
    pub pixel_offset: u32,
}

#[derive(Debug)]
pub struct DibHeader {
    pub version: BmpVersion,
    pub header_size: u32,

    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_depth: u16,
    pub compression: Compression,
    pub image_size: u32,

    pub ppm_x: i32,
    pub ppm_y: i32,

    pub colors_used: u32,
    pub important_colors: u32,

    // Optional fields (V4+)
    pub red_mask: Option<u32>,
    pub green_mask: Option<u32>,
    pub blue_mask: Option<u32>,
    pub alpha_mask: Option<u32>,

    pub color_space: Option<ColorSpace>,
}

// -----------------------------
// Main BMP struct
// -----------------------------

#[derive(Debug)]
pub struct Bmp {
    pub file_header: BmpFileHeader,
    pub dib_header: DibHeader,
    pub pixel_data: Vec<u8>,
}

// -----------------------------
// Parsing
// -----------------------------

impl Bmp {
    pub fn parse<R: Read>(mut reader: R) -> io::Result<Self> {
        // ---- FILE HEADER ----
        let mut sig = [0; 2];
        reader.read_exact(&mut sig)?;
        if &sig != b"BM" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a BMP file"));
        }

        let file_size = read_u32(&mut reader)?;
        let reserved1 = read_u16(&mut reader)?;
        let reserved2 = read_u16(&mut reader)?;
        let pixel_offset = read_u32(&mut reader)?;

        let file_header = BmpFileHeader {
            file_size,
            reserved1,
            reserved2,
            pixel_offset,
        };

        // ---- DIB HEADER ----
        let header_size = read_u32(&mut reader)?;
        let version = BmpVersion::from(header_size);

        let width = read_i32(&mut reader)?;
        let height = read_i32(&mut reader)?;
        let planes = read_u16(&mut reader)?;
        let bit_depth = read_u16(&mut reader)?;
        let compression = Compression::from(read_u32(&mut reader)?);
        let image_size = read_u32(&mut reader)?;

        let ppm_x = read_i32(&mut reader)?;
        let ppm_y = read_i32(&mut reader)?;
        let colors_used = read_u32(&mut reader)?;
        let important_colors = read_u32(&mut reader)?;

        // Optional fields
        let mut red_mask = None;
        let mut green_mask = None;
        let mut blue_mask = None;
        let mut alpha_mask = None;
        let mut color_space = None;

        if header_size >= 52 {
            red_mask = Some(read_u32(&mut reader)?);
            green_mask = Some(read_u32(&mut reader)?);
            blue_mask = Some(read_u32(&mut reader)?);
        }

        if header_size >= 56 {
            alpha_mask = Some(read_u32(&mut reader)?);
        }

        if header_size >= 60 {
            color_space = Some(ColorSpace::from(read_u32(&mut reader)?));
        }

        let dib_header = DibHeader {
            version,
            header_size,
            width,
            height,
            planes,
            bit_depth,
            compression,
            image_size,
            ppm_x,
            ppm_y,
            colors_used,
            important_colors,
            red_mask,
            green_mask,
            blue_mask,
            alpha_mask,
            color_space,
        };

        // ---- PIXEL DATA ----
        let mut pixel_data = Vec::new();
        reader.read_to_end(&mut pixel_data)?;

        Ok(Bmp {
            file_header,
            dib_header,
            pixel_data,
        })
    }
}