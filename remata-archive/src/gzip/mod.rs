//! Gzip Module

use remata_macros::{FromPrimitive, DisplayPretty};
use std::io::{self, Read, Error, ErrorKind};

/// Holds the GZIP Magic Number
pub const GZIP_MAGIC: [u8; 2] = [0x1F, 0x8B];

/// Represents metadata extracted from GZIP archives.
///
/// Note: GZIP metadata typically applies to a single file entry.
#[derive(DisplayPretty, Default, Clone)]
pub struct Gzip {
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



impl Gzip {
    /// Parse a gzip file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut header = [0u8; 10];
        reader.read_exact(&mut header)?;

        if header[0] != GZIP_MAGIC[0] || header[1] != GZIP_MAGIC[1] {
            return Err(Error::new(ErrorKind::Other, "Not a gzip file"));
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
#[derive(DisplayPretty, Clone, Copy, FromPrimitive)]
pub enum GzipCompression {
    /// Deflate compression (standard).
    #[value = 8]
    Deflated,
    /// Unknown compression method.
    Unknown(u8),
}


/// Bit flags describing optional fields in a GZIP header.
#[derive(DisplayPretty, Default, Clone)]
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
#[derive(DisplayPretty, Clone, Copy, FromPrimitive)]
pub enum GzipExtraFlags {
    /// No Flags
    #[value = 0]
    None,
    /// Maximum Compression
    #[value = 2]
    MaximumCompression,
    /// Fastest Compression
    #[value = 4]
    Fastest,
    /// Unknown or unspecified flag.
    Unknown(u8),
}


/// Operating systems recognized in GZIP headers.
#[derive(DisplayPretty, Clone, Copy, FromPrimitive)]
pub enum GzipOs {
    /// FAT filesystem (MS-DOS, OS/2, NT).
    #[value = 0]
    Fat,
    /// Amiga.
    #[value = 1]
    Amiga,
    /// VMS (OpenVMS).
    #[value = 2]
    Vms,
    /// Unix.
    #[value = 3]
    Unix,
    /// VM/CMS.
    #[value = 4]
    VmCms,
    /// Atari TOS.
    #[value = 5]
    AtariTos,
    /// HPFS filesystem (OS/2, NT).
    #[value = 6]
    Hpfs,
    /// Macintosh.
    #[value = 7]
    Macintosh,
    /// Z-System.
    #[value = 8]
    ZSystem,
    /// CP/M.
    #[value = 9]
    CpM,
    /// TOPS-20.
    #[value = 10]
    Tops20,
    /// NTFS filesystem (NT).
    #[value = 11]
    Ntfs,
    /// QDOS.
    #[value = 12]
    Qdos,
    /// Acorn RISCOS.
    #[value = 13]
    AcornRiscos,
    /// Unknown or unsupported operating system.
    Unknown(u8),
}




