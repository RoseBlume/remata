use std::io::{Read, Seek, SeekFrom, self};
use super::{IfdEntry, IfdType};
use std::fs::File;
#[derive(Clone)]
pub enum Endian {
    Little,
    Big
}

impl Endian {
    pub fn read_u16(&self, buf: &[u8], offset: usize) -> u16 {
        let bytes = [buf[offset], buf[offset + 1]];
        match self {
            Self::Little => u16::from_le_bytes(bytes),
            Self::Big => u16::from_be_bytes(bytes)
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
            Self::Little => u32::from_le_bytes(bytes),
            Self::Big => u32::from_be_bytes(bytes)
        }
    }
    pub fn read_rational_strings(
        &self,
        file: &mut File,
        base_offset: u64,
        entry: &IfdEntry,
    ) -> io::Result<Vec<String>> {
        let absolute = base_offset + entry.value_offset_or_inline as u64;
        file.seek(SeekFrom::Start(absolute))?;

        let mut buf = vec![0u8; (entry.count * 8) as usize];
        file.read_exact(&mut buf)?;

        let mut result = Vec::new();

        for i in 0..entry.count as usize {
            let o = i * 8;

            let num = match self {
                Self::Little => u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]]),
                Self::Big => u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
            };

            let den = match self {
                Self::Little => u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
                Self::Big => u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
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

    pub fn read_srational_strings(
        &self,
        file: &mut File,
        base_offset: u64,
        entry: &IfdEntry,
    ) -> io::Result<Vec<String>> {
        let absolute = base_offset + entry.value_offset_or_inline as u64;
        file.seek(SeekFrom::Start(absolute))?;

        let mut buf = vec![0u8; (entry.count * 8) as usize];
        file.read_exact(&mut buf)?;

        let mut result = Vec::new();

        for i in 0..entry.count as usize {
            let o = i * 8;

            let num = match self {
                Self::Little => u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]]),
                Self::Big => u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
            };

            let den = match self {
                Self::Little => u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
                Self::Big => u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
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

    pub fn parse_ifd(
        &self,
        file: &mut File,
        base_offset: u64,
        offset: u32,
    ) -> io::Result<Option<(Vec<IfdEntry>, u32)>> {
        if offset == 0 {
            return Ok(None);
        }

        let absolute = base_offset + offset as u64;
        file.seek(SeekFrom::Start(absolute))?;

        let mut count_buf = [0u8; 2];
        file.read_exact(&mut count_buf)?;
        let count = match self {
            Self::Little => u16::from_le_bytes(count_buf),
            Self::Big => u16::from_be_bytes(count_buf)
        };

        let mut entries_buf = vec![0u8; count as usize * 12];
        file.read_exact(&mut entries_buf)?;

        let mut entries = Vec::new();

        for i in 0..count as usize {
            let base = i * 12;

            let tag = self.read_u16(&entries_buf, base);
            let value_type_raw = self.read_u16(&entries_buf, base + 2);

            let value_type = match IfdType::try_from(value_type_raw) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let count = self.read_u32(&entries_buf, base + 4);
            let value_offset_or_inline = self.read_u32(&entries_buf, base + 8);

            entries.push(IfdEntry::new(
                tag,
                value_type,
                count,
                value_offset_or_inline,
            ));
        }

        let mut next_buf = [0u8; 4];
        file.read_exact(&mut next_buf)?;
        let next_ifd_offset = match self {
            Self::Little => u32::from_le_bytes(next_buf),
            Self::Big => u32::from_be_bytes(next_buf)
        };

        Ok(Some((entries, next_ifd_offset)))
    }
    pub fn read_numeric_values(
        &self,
        file: &mut File,
        base_offset: u64,
        entry: &IfdEntry,
    ) -> io::Result<Vec<u64>> {
        let elem_size = type_size(entry.value_type);
        let total_size = elem_size * entry.count as usize;

        let mut buf = vec![0u8; total_size];

        if total_size <= 4 {
            // INLINE
            let raw = match self {
                Endian::Little => entry.value_offset_or_inline.to_le_bytes(),
                Endian::Big => entry.value_offset_or_inline.to_be_bytes()
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
                    match self {
                        Self::Little => u16::from_le_bytes(bytes) as u64,
                        Self::Big => u16::from_be_bytes(bytes) as u64
                    }
                }

                IfdType::Long => {
                    let bytes = [
                        buf[offset],
                        buf[offset + 1],
                        buf[offset + 2],
                        buf[offset + 3],
                    ];
                    match self {
                        Self::Little => u32::from_le_bytes(bytes) as u64,
                        Self::Big => u32::from_be_bytes(bytes) as u64
                    }
                }

                IfdType::SLong => {
                    let bytes = [
                        buf[offset],
                        buf[offset + 1],
                        buf[offset + 2],
                        buf[offset + 3],
                    ];
                    match self {
                        Self::Little => i32::from_le_bytes(bytes) as i64 as u64,
                        Self::Big => i32::from_be_bytes(bytes) as i64 as u64
                    }
                }

                // skip rationals here (handled separately if needed)
                _ => continue,
            };

            values.push(v);
        }

        Ok(values)
    }
    pub fn read_rational(
        &self,
        file: &mut File,
        base_offset: u64,
        offset: u32,
        count: u32,
    ) -> io::Result<Vec<(u32, u32)>> {
        let absolute = base_offset + offset as u64;
        file.seek(SeekFrom::Start(absolute))?;

        let mut buf = vec![0u8; (count * 8) as usize];
        file.read_exact(&mut buf)?;

        let mut result = Vec::new();

        for i in 0..count as usize {
            let o = i * 8;

            let num =  match self {
                Self::Little => u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]]),
                Self::Big => u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
            };

            let den = match self {
                Self::Little => u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]]),
                Self::Big => u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
            };

            result.push((num, den));
        }

        Ok(result)
    }

}

fn type_size(t: IfdType) -> usize {
    match t {
        IfdType::Byte | IfdType::Ascii | IfdType::Undefined | IfdType::Utf8 => 1,
        IfdType::Short => 2,
        IfdType::Long | IfdType::SLong => 4,
        IfdType::Rational | IfdType::SRational => 8,
    }
}