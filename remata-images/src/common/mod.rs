pub enum ResolutionUnit {
    Dpi(u32), // Dots Per Inch
    PPM(u32), // Pixels Per Module
    Unknown
}

impl ResolutionUnit {
    pub fn make_resolution(unit: u16, value: f64) -> ResolutionUnit {
        match unit {
            2 => ResolutionUnit::Dpi(value as u32),
            3 => {
                // EXIF uses pixels per centimeter, convert to per meter if you want true PPM
                let ppm = (value * 100.0) as u32;
                ResolutionUnit::PPM(ppm)
            }
            _ => ResolutionUnit::Dpi(value as u32), // fallback
        }
    }
}