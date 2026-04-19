

pub struct PngHeader {
    width: Option<u32>,
    height: Option<u32>,
    bit_depth: Option<i32>,
    color_type: Option<PngColorType>,
    compression: Option<PngCompression>,
    filter: Option<PngFilter>,
    interlace: Option<PngInterlace>
}

// Index1	TagName
// 0	ImageWidth	 
// 4	ImageHeight	 
// 8	BitDepth	 
// 9	ColorType
// 10	Compression	
// 11	Filter
// 12	Interlace	

pub enum PngColorType {
    Grayscale, // 0
    Rgb, // 2
    Palette, // 3
    GrayscaleWithAlpha, // 4
    RgbWithAlpha, // 6
    Unknown
}

impl PngColorType {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Grayscale,
            2 => Self::Rgb, // 2
            3 => Self::Palette, // 3
            4 => Self::GrayscaleWithAlpha, // 4
            6 => Self::RgbWithAlpha, // 6
            _ => Self::Unknown
        }
    }
    pub fn from_u32(val: u32) -> Self {
        match val {
            0 => Self::Grayscale,
            2 => Self::Rgb, // 2
            3 => Self::Palette, // 3
            4 => Self::GrayscaleWithAlpha, // 4
            6 => Self::RgbWithAlpha, // 6
            _ => Self::Unknown
        }
    }
}

// 0 = Grayscale
// 2 = RGB
// 3 = Palette
// 4 = Grayscale with Alpha
// 6 = RGB with Alpha



pub enum PngFilter {
    Adaptive, // 0
    Other(u16), 
}

impl PngFilter {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Adaptive,
            _ => Self::Other(val)
        }
    }
}
// 0 = Adaptive


pub enum PngCompression {
    DeflateInflate, // 0
    Other(u16), 
}

impl PngCompression {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::DeflateInflate,
            _ => Self::Other(val)
        }
    }
}

// 0 = Deflate/Inflate

pub enum PngInterlace {
    NonInterlaced,
    Adam7Interlace,
    UnknownInterlace,
}

impl PngInterlace {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::NonInterlaced,
            1 => Self::Adam7Interlace,
            _ => Self::UnknownInterlace
        }
    }
}
// 0 = Noninterlaced
// 1 = Adam7 Interlace