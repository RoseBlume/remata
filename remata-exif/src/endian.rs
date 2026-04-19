use std::io::{self, Seek, SeekFrom, Read};
#[derive(Copy, Clone, Debug)]
pub enum Endian {
    Big,
    Little
}
use super::helpers::type_size;
use super::{
    IfdEntry,
    IfdType
};

// const MAX_EXIF_BLOCK: u64 = 1024 * 1024 * 1024 * 4; // 4 MB (very generous)


impl Endian {
    pub fn read_u16(&self, buf: &[u8], offset: usize) -> u16 {
        let bytes = [buf[offset], buf[offset + 1]];
        match self {
            Self::Big => u16::from_be_bytes(bytes),
            Self::Little => u16::from_le_bytes(bytes),
        }
    }
    pub fn read_u32(&self, buf: &[u8], offset: usize) -> u32 {
        let bytes = [
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ];
        match self {
            Self::Big => u32::from_be_bytes(bytes),
            Self::Little => u32::from_le_bytes(bytes),
        }
    }

    pub fn read_rational<R: Read + Seek>(
        &self,
        reader: &mut R,
        base_offset: u64,
        offset: u32,
        count: u32,
    ) -> io::Result<Vec<(u32, u32)>> {
        const MAX_COUNT: u32 = 100_000;

        if count > MAX_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Too many rationals: {}", count),
            ));
        }

        let absolute = base_offset + offset as u64;
        reader.seek(SeekFrom::Start(absolute))?;

        let mut result = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;

            let num = match self {
                Endian::Big => u32::from_be_bytes(buf[0..4].try_into().unwrap()),
                Endian::Little => u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            };

            let den = match self {
                Endian::Big => u32::from_be_bytes(buf[4..8].try_into().unwrap()),
                Endian::Little => u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            };

            result.push((num, den));
        }

        Ok(result)
    }
    pub fn read_rational_strings<R: Read + Seek>(
        &self,
        reader: &mut R,
        base_offset: u64,
        entry: &IfdEntry,
    ) -> io::Result<Vec<String>> {
        const MAX_COUNT: u32 = 100_000;

        if entry.count > MAX_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Too many rationals: {}", entry.count),
            ));
        }

        let absolute = base_offset + entry.value_offset_or_inline as u64;
        reader.seek(SeekFrom::Start(absolute))?;

        let mut result = Vec::with_capacity(entry.count as usize);

        for _ in 0..entry.count {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;

            let num = match self {
                Endian::Big => u32::from_be_bytes(buf[0..4].try_into().unwrap()),
                Endian::Little => u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            };

            let den = match self {
                Endian::Big => u32::from_be_bytes(buf[4..8].try_into().unwrap()),
                Endian::Little => u32::from_le_bytes(buf[4..8].try_into().unwrap()),
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
    pub fn read_srational_strings<R: Read + Seek>(
        &self,
        reader: &mut R,
        base_offset: u64,
        entry: &IfdEntry,
    ) -> io::Result<Vec<String>> {
        const MAX_COUNT: u32 = 100_000;

        if entry.count > MAX_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Too many srationals: {}", entry.count),
            ));
        }

        let absolute = base_offset + entry.value_offset_or_inline as u64;
        reader.seek(SeekFrom::Start(absolute))?;

        let mut result = Vec::with_capacity(entry.count as usize);

        for _ in 0..entry.count {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;

            let num = match self {
                Endian::Big => i32::from_be_bytes(buf[0..4].try_into().unwrap()),
                Endian::Little => i32::from_le_bytes(buf[0..4].try_into().unwrap()),
            };

            let den = match self {
                Endian::Big => i32::from_be_bytes(buf[4..8].try_into().unwrap()),
                Endian::Little => i32::from_le_bytes(buf[4..8].try_into().unwrap()),
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
    pub fn read_numeric_values<R: Read + Seek>(
        &self,
        reader: &mut R,
        base_offset: u64,
        entry: &IfdEntry,
    ) -> io::Result<Vec<u64>> {
        const MAX_COUNT: u32 = 100_000;

        if entry.count > MAX_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Too many numeric values: {}", entry.count),
            ));
        }

        let elem_size = type_size(entry.value_type);

        if elem_size == 0 || elem_size > 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid element size: {}", elem_size),
            ));
        }

        let mut values = Vec::with_capacity(entry.count as usize);

        // safe size calc
        let total_size = (elem_size as u64)
            .checked_mul(entry.count as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Size overflow"))?;

        if total_size <= 4 {
            // ---- INLINE ----
            let raw = match self {
                Endian::Big => entry.value_offset_or_inline.to_be_bytes(),
                Endian::Little => entry.value_offset_or_inline.to_le_bytes(),
            };

            for i in 0..entry.count as usize {
                let offset = i * elem_size;

                let v = match entry.value_type {
                    IfdType::Byte | IfdType::Undefined | IfdType::Utf8 => {
                        raw[offset] as u64
                    }

                    IfdType::Short => {
                        let bytes = [raw[offset], raw[offset + 1]];
                        match self {
                            Endian::Big => u16::from_be_bytes(bytes) as u64,
                            Endian::Little => u16::from_le_bytes(bytes) as u64,
                        }
                    }

                    IfdType::Long => {
                        let bytes = [
                            raw[offset],
                            raw[offset + 1],
                            raw[offset + 2],
                            raw[offset + 3],
                        ];
                        match self {
                            Endian::Big => u32::from_be_bytes(bytes) as u64,
                            Endian::Little => u32::from_le_bytes(bytes) as u64,
                        }
                    }

                    IfdType::SLong => {
                        let bytes = [
                            raw[offset],
                            raw[offset + 1],
                            raw[offset + 2],
                            raw[offset + 3],
                        ];
                        match self {
                            Endian::Big => i32::from_be_bytes(bytes) as i64 as u64,
                            Endian::Little => i32::from_le_bytes(bytes) as i64 as u64,
                        }
                    }

                    _ => continue,
                };

                values.push(v);
            }
        } else {
            // ---- OFFSET (streamed) ----
            let absolute = base_offset + entry.value_offset_or_inline as u64;
            reader.seek(SeekFrom::Start(absolute))?;

            let mut buf = [0u8; 8]; // stack buffer

            for _ in 0..entry.count {
                let slice = &mut buf[..elem_size];
                reader.read_exact(slice)?;

                let v = match entry.value_type {
                    IfdType::Byte | IfdType::Undefined | IfdType::Utf8 => {
                        slice[0] as u64
                    }

                    IfdType::Short => {
                        let bytes = [slice[0], slice[1]];
                        match self {
                            Endian::Big => u16::from_be_bytes(bytes) as u64,
                            Endian::Little => u16::from_le_bytes(bytes) as u64,
                        }
                    }

                    IfdType::Long => {
                        let bytes = [slice[0], slice[1], slice[2], slice[3]];
                        match self {
                            Endian::Big => u32::from_be_bytes(bytes) as u64,
                            Endian::Little => u32::from_le_bytes(bytes) as u64,
                        }
                    }

                    IfdType::SLong => {
                        let bytes = [slice[0], slice[1], slice[2], slice[3]];
                        match self {
                            Endian::Big => i32::from_be_bytes(bytes) as i64 as u64,
                            Endian::Little => i32::from_le_bytes(bytes) as i64 as u64,
                        }
                    }

                    _ => continue,
                };

                values.push(v);
            }
        }

        Ok(values)
    }

}


// pub fn read_rational_strings(
//     file: &mut File,
//     base_offset: u64,
//     entry: &IfdEntry,
//     endian: Endian,
// ) -> io::Result<Vec<String>> {
//     let absolute = base_offset + entry.value_offset_or_inline as u64;
//     file.seek(SeekFrom::Start(absolute))?;

//     let mut buf = vec![0u8; (entry.count * 8) as usize];
//     file.read_exact(&mut buf)?;

//     let mut result = Vec::new();

//     for i in 0..entry.count as usize {
//         let o = i * 8;
//         let num = match self {
//             Endian::Big => u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]]),
//             Endian::Little => u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
//         };

//         let den = match self {
//             Endian::Big => u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
//             Endian::Little => u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
//         };


//         if den == 0 {
//             result.push(format!("{}/{} (NaN)", num, den));
//         } else {
//             let value = num as f64 / den as f64;
//             result.push(format!("{}/{} ({:.6})", num, den, value));
//         }
//     }

//     Ok(result)

// }

// pub fn read_srational_strings(
//     file: &mut File,
//     base_offset: u64,
//     entry: &IfdEntry,
//     endian: Endian
// ) -> io::Result<Vec<String>> {
//     let absolute = base_offset + entry.value_offset_or_inline as u64;
//     file.seek(SeekFrom::Start(absolute))?;

//     let mut buf = vec![0u8; (entry.count * 8) as usize];
//     file.read_exact(&mut buf)?;

//     let mut result = Vec::new();

//     for i in 0..entry.count as usize {
//         let o = i * 8;

//         let num = match self {
//             Endian::Big => u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]]),
//             Endian::Little => u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
//         };

//         let den = match self {
//             Endian::Big => u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
//             Endian::Little => u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
//         };

//         if den == 0 {
//             result.push(format!("{}/{} (NaN)", num, den));
//         } else {
//             let value = num as f64 / den as f64;
//             result.push(format!("{}/{} ({:.6})", num, den, value));
//         }
//     }

//     Ok(result)
// }

// pub fn read_ascii(
//     file: &mut File,
//     base_offset: u64,
//     offset: u32,
//     count: u32,
// ) -> io::Result<String> {
//     let absolute = base_offset + offset as u64;
//     file.seek(SeekFrom::Start(absolute))?;

//     let mut buf = vec![0u8; count as usize];
//     file.read_exact(&mut buf)?;

//     // Remove trailing NULL if present
//     if let Some(pos) = buf.iter().position(|&b| b == 0) {
//         buf.truncate(pos);
//     }

//     Ok(String::from_utf8_lossy(&buf).to_string())
// }

// pub fn read_numeric_values(
//     file: &mut File,
//     base_offset: u64,
//     entry: &IfdEntry,
//     endian: Endian
// ) -> io::Result<Vec<u64>> {
//     let elem_size = type_size(entry.value_type);
//     let total_size = elem_size * entry.count as usize;

//     let mut buf = vec![0u8; total_size];

//     if total_size <= 4 {
//         // INLINE
//         let raw = match self {
//             Endian::Big => entry.value_offset_or_inline.to_be_bytes(),
//             Endian::Little => entry.value_offset_or_inline.to_le_bytes()
//         };

//         buf[..total_size].copy_from_slice(&raw[..total_size]);
//     } else {
//         // OFFSET
//         let absolute = base_offset + entry.value_offset_or_inline as u64;
//         file.seek(SeekFrom::Start(absolute))?;
//         file.read_exact(&mut buf)?;
//     }

//     let mut values = Vec::new();

//     for i in 0..entry.count as usize {
//         let offset = i * elem_size;

//         let v = match entry.value_type {
//             IfdType::Byte | IfdType::Undefined | IfdType::Utf8 => {
//                 buf[offset] as u64
//             }

//             IfdType::Short => {
//                 let bytes = [buf[offset], buf[offset + 1]];
//                 match self {
//                     Endian::Big => u16::from_be_bytes(bytes) as u64,
//                     Endian::Little => u16::from_le_bytes(bytes) as u64
//                 }

//             }

//             IfdType::Long => {
//                 let bytes = [
//                     buf[offset],
//                     buf[offset + 1],
//                     buf[offset + 2],
//                     buf[offset + 3],
//                 ];
//                 match self {
//                     Endian::Big => u32::from_be_bytes(bytes) as u64,
//                     Endian::Little => u32::from_le_bytes(bytes) as u64
//                 }
//             }

//             IfdType::SLong => {
//                 let bytes = [
//                     buf[offset],
//                     buf[offset + 1],
//                     buf[offset + 2],
//                     buf[offset + 3],
//                 ];
//                 match self {
//                     Endian::Big => i32::from_be_bytes(bytes) as i64 as u64,
//                     Endian::Little => i32::from_le_bytes(bytes) as i64 as u64
//                 }
//             }

//             // skip rationals here (handled separately if needed)
//             _ => continue,
//         };

//         values.push(v);
//     }

//     Ok(values)
// }

