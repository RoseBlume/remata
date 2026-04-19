pub fn read_u16(v: &[u8]) -> u16 {
    u16::from_le_bytes([v[0], v[1]])
}

pub fn read_u32(v: &[u8]) -> u32 {
    u32::from_le_bytes(v[..4].try_into().unwrap())
}

pub fn read_f64(v: &[u8]) -> f64 {
    f64::from_le_bytes(v[..8].try_into().unwrap())
}

pub fn read_string(v: &[u8]) -> String {
    let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
    String::from_utf8_lossy(&v[..end]).to_string()
}