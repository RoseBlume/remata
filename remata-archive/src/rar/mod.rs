//! Rar Module

use std::io::{self, Read, Seek, SeekFrom, Error, ErrorKind};
use remata_macros::{DisplayPretty, FromPrimitive};

/// The signature for a Rar4 file
pub const RAR4_SIG: &'static [u8; 7] = b"Rar!\x1A\x07\x00";

/// The signature for a Rar5 file
pub const RAR5_SIG: &'static [u8; 8] = b"Rar!\x1A\x07\x01\x00";

/// An enum that holds the metadata of a rar file
pub enum Rar {
    /// Holds info on a Rar4 file
    Rar4(Rar4),
    /// Holds info on a Rar5 file
    Rar5(Rar5)
}

impl Rar {
    /// Parse a Rar file
    pub fn parse<R: Read + Seek>(mut reader: &mut R) -> io::Result<Self> {
        let rar4 = Rar4::parse(&mut reader);
        match rar4 {
            Ok(rar) => {
                return Ok(Self::Rar4(rar));
            }
            Err(_) => {}
        }
        reader.seek(SeekFrom::Start(0))?;
        let rar5 = Rar5::parse(&mut reader);
        match rar5 {
            Ok(rar) => {
                return Ok(Self::Rar5(rar));
            }
            Err(_) => {}
        }

        Err(Error::new(ErrorKind::Other, "File does not match Rar signature"))
    }
}

/// Represents metadata extracted from RAR archives (v4 and earlier).
#[derive(Default, Clone, DisplayPretty)]
pub struct Rar4 {
    /// Size of the compressed file in bytes.
    pub compressed_size: Option<u64>,

    /// Size of the original uncompressed file in bytes.
    pub uncompressed_size: Option<u64>,

    /// Operating system where the archive was created.
    pub operating_system: Option<RarOs>,

    /// Last modification date/time.
    pub modify_date: Option<String>,

    /// Compression method used.
    pub packing_method: Option<RarPackingMethod>,

    /// Name of the archived file.
    pub file_name: Option<String>,
}


impl Rar4 {
    /// Parse a RAR4 file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut sig = [0u8; 7];
        reader.read_exact(&mut sig)?;

        if &sig != RAR4_SIG {
            return Err(Error::new(ErrorKind::InvalidData, "Not a RAR file"));
        }

        // Read base block header (7 bytes)
        let mut base = [0u8; 7];
        reader.read_exact(&mut base)?;

        let _crc = u16::from_le_bytes([base[0], base[1]]);
        let block_type = base[2];
        let _flags = u16::from_le_bytes([base[3], base[4]]);
        let header_size = u16::from_le_bytes([base[5], base[6]]) as usize;

        // We expect a file header (type 0x74)
        if block_type != 0x74 {
            return Err(Error::new(ErrorKind::InvalidData, "Not a file header"));
        }

        // Read the rest of the header
        let mut buf = vec![0u8; header_size - 7];
        reader.read_exact(&mut buf)?;

        // Parse fields according to RAR4 file header structure
        let compressed_size = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
        let uncompressed_size = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) as u64;

        let host_os = buf[8];
        let _file_crc = &buf[9..13];
        let _ftime = &buf[13..17];
        let _unp_ver = buf[17];
        let method = buf[18];
        let name_size = u16::from_le_bytes([buf[19], buf[20]]) as usize;
        let _attr = &buf[21..25];

        // Filename starts after fixed header (25 bytes total before name)
        let name_start = 25;
        let name_end = name_start + name_size;

        let file_name = if name_end <= buf.len() {
            Some(String::from_utf8_lossy(&buf[name_start..name_end]).to_string())
        } else {
            None
        };

        Ok(Self {
            compressed_size: Some(compressed_size),
            uncompressed_size: Some(uncompressed_size),
            operating_system: Some(RarOs::from(host_os)),
            modify_date: None, // could parse from _ftime if needed
            packing_method: Some(RarPackingMethod::from(method)),
            file_name,
        })
    }
}

/// Operating systems for RAR archives.
#[derive(Clone, Copy, FromPrimitive, DisplayPretty)]
pub enum RarOs {
    #[value = 0]
    /// MsDos
    MsDos,
    #[value = 1]
    /// Os2
    Os2,
    #[value = 2]
    /// Windows
    Win32,
    #[value = 3]
    /// Unix
    Unix,
    /// Unknown or unsupported OS.
    Unknown(u8),
}


/// Compression methods used in RAR archives.
#[derive(Clone, Copy, FromPrimitive, DisplayPretty)]
pub enum RarPackingMethod {
    /// Stored Rar Packing Method
    #[value = 0x30]
    Stored,
    /// Fastest Rar Packing Method
    #[value = 0x31]
    Fastest,
    /// Fast Rar Packing Method
    #[value = 0x32]
    Fast,
    /// Normal Rar Packing Method
    #[value = 0x33]
    Normal,
    /// Good Rar Packing Method
    #[value = 0x34]
    Good,
    /// Best Rar Packing Method
    #[value = 0x35]
    Best,
    /// Unknown method.
    Unknown(u8),
}



/// Represents metadata extracted from RAR5 and 7z archives.
///
/// These formats share similar metadata structures.
#[derive(Default, Clone, DisplayPretty)]
pub struct Rar5 {
    /// Name of the archived file.
    pub file_name: Option<String>,

    /// Size of the compressed data in bytes.
    pub compressed_size: Option<u64>,

    /// File version (if present).
    pub file_version: Option<String>,

    /// Last modification date/time.
    pub modify_date: Option<String>,

    /// Operating system where the archive was created.
    pub operating_system: Option<Rar5Os>,

    /// Size of the uncompressed data in bytes.
    pub uncompressed_size: Option<u64>,
}

impl Rar5 {
    /// Parse a Rar5 file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut sig = [0u8; 8];
        reader.read_exact(&mut sig)?;

        if &sig != RAR5_SIG {
            return Err(Error::new(ErrorKind::InvalidData, "Not a RAR5 file"));
        }

        // Helper to read RAR5 variable-length integer
        fn read_vint<R: Read>(r: &mut R) -> io::Result<u64> {
            let mut value = 0u64;
            let mut shift = 0;

            loop {
                let mut byte = [0u8; 1];
                r.read_exact(&mut byte)?;
                let b = byte[0];

                value |= ((b & 0x7F) as u64) << shift;

                if (b & 0x80) == 0 {
                    break;
                }

                shift += 7;
            }

            Ok(value)
        }

        // ---- Read first block header ----
        let _crc = read_vint(reader)?;
        let _block_size = read_vint(reader)?;
        let block_type = read_vint(reader)?;
        let flags = read_vint(reader)?;

        // We only handle file header (type = 2)
        if block_type != 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Not a file block"));
        }

        let extra_size = if flags & 0x01 != 0 {
            read_vint(reader)?
        } else {
            0
        };

        let data_size = if flags & 0x02 != 0 {
            read_vint(reader)?
        } else {
            0
        };

        // ---- File header fields ----
        let file_flags = read_vint(reader)?;
        let uncompressed_size = read_vint(reader)?;

        let attributes = read_vint(reader)?;
        let _ = attributes; // unused for now

        let mut modify_date = None;

        // Optional timestamp
        if file_flags & 0x02 != 0 {
            let ts = read_vint(reader)?;
            modify_date = Some(format!("{}", ts)); // raw timestamp for now
        }

        let data_crc = read_vint(reader)?;
        let _ = data_crc;

        let compression = read_vint(reader)?;
        let _ = compression;

        let host_os = read_vint(reader)?;

        let name_len = read_vint(reader)? as usize;

        let mut name_buf = vec![0u8; name_len];
        reader.read_exact(&mut name_buf)?;

        let file_name = Some(String::from_utf8_lossy(&name_buf).to_string());

        // Skip extra area if present
        if extra_size > 0 {
            let mut skip = vec![0u8; extra_size as usize];
            reader.read_exact(&mut skip)?;
        }

        Ok(Self {
            file_name,
            compressed_size: Some(data_size),
            file_version: None,
            modify_date,
            operating_system: Some(Rar5Os::from(host_os as u8)),
            uncompressed_size: Some(uncompressed_size),
        })
    }
}
/// Operating systems for RAR5 / 7z archives.
#[derive(Clone, Copy, FromPrimitive, DisplayPretty)]
pub enum Rar5Os {
    #[value = 0]
    /// Windows
    Win32,
    #[value = 1]
    /// Unix
    Unix,
    /// Unknown or unspecified OS.
    Unknown(u8),
}








