/// Represents metadata extracted from ZIP archives.
///
/// Contains fields from standard ZIP file headers such as compression,
/// sizes, timestamps, and file identification.
#[derive(Debug, Default, Clone)]
pub struct ZipMeta {
    /// Minimum version required to extract the file.
    pub required_version: Option<u16>,

    /// General-purpose bit flag.
    ///
    /// Contains feature flags such as encryption, compression options, etc.
    pub bit_flag: Option<u16>,

    /// Compression method used for the file.
    pub compression: Option<ZipCompression>,

    /// Last modification date/time of the file.
    pub modify_date: Option<String>,

    /// CRC-32 checksum of the uncompressed data.
    pub crc: Option<u32>,

    /// Size of the compressed data in bytes.
    pub compressed_size: Option<u64>,

    /// Size of the original uncompressed data in bytes.
    pub uncompressed_size: Option<u64>,

    /// Name of the archived file.
    pub file_name: Option<String>,

    /// Optional file comment.
    pub comment: Option<String>,
}

impl ZipMeta {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut sig = [0u8; 4];
        reader.read_exact(&mut sig)?;

        if &sig != b"PK\x03\x04" {
            return Ok(Self::default());
        }

        let mut buf = [0u8; 26];
        reader.read_exact(&mut buf)?;

        let required_version = u16::from_le_bytes([buf[0], buf[1]]);
        let bit_flag = u16::from_le_bytes([buf[2], buf[3]]);
        let compression = u16::from_le_bytes([buf[4], buf[5]]);
        let crc = u32::from_le_bytes([buf[10], buf[11], buf[12], buf[13]]);
        let compressed_size =
            u32::from_le_bytes([buf[14], buf[15], buf[16], buf[17]]) as u64;
        let uncompressed_size =
            u32::from_le_bytes([buf[18], buf[19], buf[20], buf[21]]) as u64;

        let name_len = u16::from_le_bytes([buf[22], buf[23]]) as usize;
        let extra_len = u16::from_le_bytes([buf[24], buf[25]]) as usize;

        let mut name_buf = vec![0; name_len];
        reader.read_exact(&mut name_buf)?;

        reader.seek(SeekFrom::Current(extra_len as i64))?;

        Ok(Self {
            required_version: Some(required_version),
            bit_flag: Some(bit_flag),
            compression: Some(compression.into()),
            modify_date: None,
            crc: Some(crc),
            compressed_size: Some(compressed_size),
            uncompressed_size: Some(uncompressed_size),
            file_name: Some(String::from_utf8_lossy(&name_buf).to_string()),
            comment: None,
        })
    }
}


/// Compression methods used in ZIP archives.
#[derive(Debug, Clone, Copy)]
pub enum ZipCompression {
    None,
    Shrunk,
    Reduced1,
    Reduced2,
    Reduced3,
    Reduced4,
    Imploded,
    Tokenized,
    Deflated,
    Deflate64,
    ImplodedOld,
    Bzip2,
    Lzma,
    IbmTerseNew,
    IbmLz77,
    Jpeg,
    WavPack,
    Ppmd,
    /// Unknown or unsupported compression method.
    Unknown(u16),
}

/// Represents metadata extracted from GZIP archives.
///
/// Note: GZIP metadata typically applies to a single file entry.
#[derive(Debug, Default, Clone)]
pub struct GzipMeta {
    /// Compression method used (usually Deflate).
    pub compression: Option<GzipCompression>,

    /// GZIP flags indicating optional fields and properties.
    pub flags: Option<GzipFlags>,

    /// Last modification date/time.
    pub modify_date: Option<String>,

    /// Extra compression flags indicating compression level.
    pub extra_flags: Option<GzipExtraFlags>,

    /// Operating system where the archive was created.
    pub operating_system: Option<GzipOs>,

    /// Original file name stored in the archive.
    pub file_name: Option<String>,

    /// Optional comment stored in the archive.
    pub comment: Option<String>,
}

impl GzipMeta {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut header = [0u8; 10];
        reader.read_exact(&mut header)?;

        if header[0] != 0x1F || header[1] != 0x8B {
            return Ok(Self::default());
        }

        let compression = GzipCompression::from(header[2]);
        let flags_byte = header[3];

        let flags = GzipFlags {
            text: flags_byte & 0x01 != 0,
            crc16: flags_byte & 0x02 != 0,
            extra_fields: flags_byte & 0x04 != 0,
            file_name: flags_byte & 0x08 != 0,
            comment: flags_byte & 0x10 != 0,
        };

        let extra_flags = GzipExtraFlags::from(header[8]);
        let os = GzipOs::from(header[9]);

        Ok(Self {
            compression: Some(compression),
            flags: Some(flags),
            modify_date: None,
            extra_flags: Some(extra_flags),
            operating_system: Some(os),
            file_name: None,
            comment: None,
        })
    }
}

/// Compression method used in GZIP archives.
#[derive(Debug, Clone, Copy)]
pub enum GzipCompression {
    /// Deflate compression (standard).
    Deflated,
    /// Unknown compression method.
    Unknown(u8),
}

/// Bit flags describing optional fields in a GZIP header.
#[derive(Debug, Default, Clone)]
pub struct GzipFlags {
    /// Indicates text data.
    pub text: bool,
    /// Indicates presence of CRC16.
    pub crc16: bool,
    /// Indicates extra fields exist.
    pub extra_fields: bool,
    /// Indicates file name is present.
    pub file_name: bool,
    /// Indicates comment is present.
    pub comment: bool,
}

/// Extra compression flags for GZIP.
#[derive(Debug, Clone, Copy)]
pub enum GzipExtraFlags {
    None,
    MaximumCompression,
    Fastest,
    /// Unknown or unspecified flag.
    Unknown(u8),
}

/// Operating systems recognized in GZIP headers.
#[derive(Debug, Clone, Copy)]
pub enum GzipOs {
    Fat,
    Amiga,
    Vms,
    Unix,
    VmCms,
    AtariTos,
    Hpfs,
    Macintosh,
    ZSystem,
    CpM,
    Tops20,
    Ntfs,
    Qdos,
    AcornRiscos,
    Unknown,
}

/// Represents metadata extracted from RAR archives (v4 and earlier).
#[derive(Debug, Default, Clone)]
pub struct RarMeta {
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

impl RarMeta {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut sig = [0u8; 7];
        reader.read_exact(&mut sig)?;

        if &sig != b"Rar!\x1A\x07\x00" {
            return Ok(Self::default());
        }

        let mut buf = [0u8; 32];
        reader.read_exact(&mut buf)?;

        Ok(Self {
            compressed_size: None,
            uncompressed_size: None,
            operating_system: Some(RarOs::from(buf[15])),
            modify_date: None,
            packing_method: Some(RarPackingMethod::from(buf[25])),
            file_name: None,
        })
    }
}

/// Operating systems for RAR archives.
#[derive(Debug, Clone, Copy)]
pub enum RarOs {
    MsDos,
    Os2,
    Win32,
    Unix,
    /// Unknown or unsupported OS.
    Unknown(u8),
}

/// Compression methods used in RAR archives.
#[derive(Debug, Clone, Copy)]
pub enum RarPackingMethod {
    Stored,
    Fastest,
    Fast,
    Normal,
    Good,
    Best,
    /// Unknown method.
    Unknown(u8),
}

/// Represents metadata extracted from RAR5 and 7z archives.
///
/// These formats share similar metadata structures.
#[derive(Debug, Default, Clone)]
pub struct Rar5Meta {
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

impl Rar5Meta {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut sig = [0u8; 8];
        reader.read_exact(&mut sig)?;

        if &sig != b"Rar!\x1A\x07\x01\x00" {
            return Ok(Self::default());
        }

        Ok(Self {
            file_name: None,
            compressed_size: None,
            file_version: None,
            modify_date: None,
            operating_system: None,
            uncompressed_size: None,
        })
    }
}

/// Operating systems for RAR5 / 7z archives.
#[derive(Debug, Clone, Copy)]
pub enum Rar5Os {
    Win32,
    Unix,
    /// Unknown or unspecified OS.
    Unknown(u8),
}

impl From<u16> for ZipCompression {
    fn from(v: u16) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Shrunk,
            2 => Self::Reduced1,
            3 => Self::Reduced2,
            4 => Self::Reduced3,
            5 => Self::Reduced4,
            6 => Self::Imploded,
            7 => Self::Tokenized,
            8 => Self::Deflated,
            9 => Self::Deflate64,
            10 => Self::ImplodedOld,
            12 => Self::Bzip2,
            14 => Self::Lzma,
            18 => Self::IbmTerseNew,
            19 => Self::IbmLz77,
            96 => Self::Jpeg,
            97 => Self::WavPack,
            98 => Self::Ppmd,
            other => Self::Unknown(other),
        }
    }
}

impl From<u8> for GzipCompression {
    fn from(v: u8) -> Self {
        match v {
            8 => Self::Deflated,
            other => Self::Unknown(other),
        }
    }
}

impl From<u8> for GzipExtraFlags {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::None,
            2 => Self::MaximumCompression,
            4 => Self::Fastest,
            other => Self::Unknown(other),
        }
    }
}

impl From<u8> for GzipOs {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Fat,
            1 => Self::Amiga,
            2 => Self::Vms,
            3 => Self::Unix,
            4 => Self::VmCms,
            5 => Self::AtariTos,
            6 => Self::Hpfs,
            7 => Self::Macintosh,
            8 => Self::ZSystem,
            9 => Self::CpM,
            10 => Self::Tops20,
            11 => Self::Ntfs,
            12 => Self::Qdos,
            13 => Self::AcornRiscos,
            _ => Self::Unknown,
        }
    }
}

impl From<u8> for RarOs {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::MsDos,
            1 => Self::Os2,
            2 => Self::Win32,
            3 => Self::Unix,
            other => Self::Unknown(other),
        }
    }
}

impl From<u8> for RarPackingMethod {
    fn from(v: u8) -> Self {
        match v {
            0x30 => Self::Stored,
            0x31 => Self::Fastest,
            0x32 => Self::Fast,
            0x33 => Self::Normal,
            0x34 => Self::Good,
            0x35 => Self::Best,
            other => Self::Unknown(other),
        }
    }
}

impl From<u8> for Rar5Os {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Win32,
            1 => Self::Unix,
            other => Self::Unknown(other),
        }
    }
}