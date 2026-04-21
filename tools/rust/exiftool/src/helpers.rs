
use std::io::{Read, Seek, SeekFrom, self};
use std::fs::File;
use crate::ifd::{
    IfdEntry,
    IfdType
};

pub fn read_rational_strings(
    file: &mut File,
    base_offset: u64,
    entry: &IfdEntry,
    little: bool,
) -> io::Result<Vec<String>> {
    let absolute = base_offset + entry.value_offset_or_inline as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; (entry.count * 8) as usize];
    file.read_exact(&mut buf)?;

    let mut result = Vec::new();

    for i in 0..entry.count as usize {
        let o = i * 8;

        let num = if little {
            u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        } else {
            u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        };

        let den = if little {
            u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        } else {
            u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        };

        if den == 0 {
            result.push(format!("{}/{} (NaN)", num, den));
        } else {
            let value = num as f64 / den as f64;
            result.push(format!("{}/{} ({:.6})", num, den, value));
        }
    }

    Ok(result)
}


const MAX_SRATIONAL_BYTES: u64 = 10 * 1024 * 1024;
pub fn read_srational_strings(
    file: &mut File,
    base_offset: u64,
    entry: &IfdEntry,
    little: bool,
) -> io::Result<Vec<String>> {
    let absolute = base_offset + entry.value_offset_or_inline as u64;

    let byte_len = (entry.count as u64)
        .checked_mul(8)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "SRATIONAL size overflow"))?;


    if byte_len > MAX_SRATIONAL_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "SRATIONAL data too large",
        ));
    }

    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; byte_len as usize];
    file.read_exact(&mut buf)?;

    let mut result = Vec::new();

    for i in 0..entry.count as usize {
        let o = i * 8;

        let num = if little {
            i32::from_le_bytes([buf[o], buf[o + 1], buf[o + 2], buf[o + 3]])
        } else {
            i32::from_be_bytes([buf[o], buf[o + 1], buf[o + 2], buf[o + 3]])
        };

        let den = if little {
            i32::from_le_bytes([buf[o + 4], buf[o + 5], buf[o + 6], buf[o + 7]])
        } else {
            i32::from_be_bytes([buf[o + 4], buf[o + 5], buf[o + 6], buf[o + 7]])
        };

        if den == 0 {
            result.push(format!("{}/{} (NaN)", num, den));
        } else {
            let value = num as f64 / den as f64;
            result.push(format!("{}/{} ({:.6})", num, den, value));
        }
    }

    Ok(result)
}

pub fn read_rational(
    file: &mut File,
    base_offset: u64,
    offset: u32,
    count: u32,
    little: bool,
) -> io::Result<Vec<(u32, u32)>> {
    let absolute = base_offset + offset as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; (count * 8) as usize];
    file.read_exact(&mut buf)?;

    let mut result = Vec::new();

    for i in 0..count as usize {
        let o = i * 8;

        let num = if little {
            u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        } else {
            u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        };

        let den = if little {
            u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        } else {
            u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        };

        result.push((num, den));
    }

    Ok(result)
}



pub fn read_numeric_values(
    file: &mut File,
    base_offset: u64,
    entry: &IfdEntry,
    little: bool,
) -> io::Result<Vec<u64>> {
    let elem_size = type_size(entry.value_type);
    let total_size = elem_size * entry.count as usize;

    let mut buf = vec![0u8; total_size];

    if total_size <= 4 {
        // INLINE
        let raw = if little {
            entry.value_offset_or_inline.to_le_bytes()
        } else {
            entry.value_offset_or_inline.to_be_bytes()
        };

        buf[..total_size].copy_from_slice(&raw[..total_size]);
    } else {
        // OFFSET
        let absolute = base_offset + entry.value_offset_or_inline as u64;
        file.seek(SeekFrom::Start(absolute))?;
        file.read_exact(&mut buf)?;
    }

    let mut values = Vec::new();

    for i in 0..entry.count as usize {
        let offset = i * elem_size;

        let v = match entry.value_type {
            IfdType::Byte | IfdType::Undefined | IfdType::Utf8 => {
                buf[offset] as u64
            }

            IfdType::Short => {
                let bytes = [buf[offset], buf[offset + 1]];
                if little {
                    u16::from_le_bytes(bytes) as u64
                } else {
                    u16::from_be_bytes(bytes) as u64
                }
            }

            IfdType::Long => {
                let bytes = [
                    buf[offset],
                    buf[offset + 1],
                    buf[offset + 2],
                    buf[offset + 3],
                ];
                if little {
                    u32::from_le_bytes(bytes) as u64
                } else {
                    u32::from_be_bytes(bytes) as u64
                }
            }

            IfdType::SLong => {
                let bytes = [
                    buf[offset],
                    buf[offset + 1],
                    buf[offset + 2],
                    buf[offset + 3],
                ];
                if little {
                    i32::from_le_bytes(bytes) as i64 as u64
                } else {
                    i32::from_be_bytes(bytes) as i64 as u64
                }
            }

            // skip rationals here (handled separately if needed)
            _ => continue,
        };

        values.push(v);
    }

    Ok(values)
}


fn type_size(t: IfdType) -> usize {
    match t {
        IfdType::Byte | IfdType::Ascii | IfdType::Undefined | IfdType::Utf8 => 1,
        IfdType::Short => 2,
        IfdType::Long | IfdType::SLong => 4,
        IfdType::Rational | IfdType::SRational => 8,
    }
}


pub fn read_ascii(
    file: &mut File,
    base_offset: u64,
    offset: u32,
    count: u32,
) -> io::Result<String> {
    let absolute = base_offset + offset as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; count as usize];
    file.read_exact(&mut buf)?;

    // Remove trailing NULL if present
    if let Some(pos) = buf.iter().position(|&b| b == 0) {
        buf.truncate(pos);
    }

    Ok(String::from_utf8_lossy(&buf).to_string())
}

