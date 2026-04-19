pub enum Thresholding {
    NoDitheringOrHalftoning,
    OrderedDitherOrHalftone,
    RandomizedDither,
    InvalidOrUnknown
}

impl Thresholding {
    pub fn from_u16(val: u16) -> Self {
        match val {
            1 => Self::NoDitheringOrHalftoning,
            2 => Self::OrderedDitherOrHalftone,
            3 => Self::RandomizedDither,
            _ => Self::InvalidOrUnknown
        }
    }
}
// 1 = No dithering or halftoning
// 2 = Ordered dither or halftone
// 3 = Randomized dither

