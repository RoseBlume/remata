pub fn read_string(data: &[u8], offset: usize, len: usize) -> Option<String> {
    if offset + len > data.len() {
        return None;
    }
    let slice = &data[offset..offset + len];
    let s = slice.split(|&b| b == 0).next()?;
    Some(String::from_utf8_lossy(s).to_string())
}

pub fn read_rational(data: &[u8], offset: usize) -> Option<f64> {
    if offset + 8 > data.len() {
        return None;
    }
    let num = u32::from_be_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ]);
    let den = u32::from_be_bytes([
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ]);
    if den == 0 {
        None
    } else {
        Some(num as f64 / den as f64)
    }
}