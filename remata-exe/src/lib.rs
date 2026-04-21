// #![deny(missing_docs)]
use std::io::{self, Read, Seek, SeekFrom};
use std::fmt;

pub mod windows;
pub mod posix;
pub mod darwin;

/// Magic number for Windows PE
const WINDOWS_PE: &'static [u8; 2] = b"MZ";

/// Magic number for Posix executables
const POSIX_MAGIC: &'static [u8; 4] = b"\x7FELF";

/// Magic U32s for Mach-O
const MACHO_MAGICS: [u32; 6] = [
    // Mach-O 32-bit
    0xFEEDFACE,
    // Mach-O 64-bit
    0xFEEDFACF,
    // Fat binary (big endian)
    0xCAFEBABE,
    // Fat binary (little endian)
    0xBEBAFECA,
    // Mach-O 32-bit (reverse endian)
    0xCEFAEDFE,
    // Mach-O 64-bit (reverse endian)
    0xCFFAEDFE
];

#[derive(Debug)]
pub enum ExecutableType {
    Windows(windows::Windows), // PE
    Posix(posix::Posix),       // ELF
    Darwin(darwin::Darwin),    // Mach-O / Fat Mach-O
}



impl fmt::Display for ExecutableType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Windows(windows) => {
                println!("{}", windows);
            },
            _ => {}
        }
        Ok(())
    }
}

impl ExecutableType {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Option<Self>> {
        // Read enough bytes for all known signatures
        let mut magic = [0u8; 4];
        let n = reader.read(&mut magic)?;

        if n < 4 {
            return Ok(None);
        }

        // Always rewind before handing off
        reader.seek(SeekFrom::Start(0))?;

        // ---- Windows PE ("MZ") ----
        if &magic[0..2] == WINDOWS_PE {
            let exe = windows::Windows::parse(reader)?;
            return Ok(Some(ExecutableType::Windows(exe)));
        }

        // ---- ELF (Linux, BSD, etc.) ----
        if &magic == POSIX_MAGIC {
            let exe = posix::Posix::parse(reader)?;
            return Ok(Some(ExecutableType::Posix(exe)));
        }

        // ---- Mach-O (macOS) ----
        // Covers:
        // 32-bit, 64-bit, and fat/universal binaries
        let magic_u32 = u32::from_be_bytes(magic);
        if MACHO_MAGICS.contains(&magic_u32) {
            let exe = darwin::Darwin::parse(reader)?;
            return Ok(Some(ExecutableType::Darwin(exe)));
        }
        // match magic_u32 {
        //     0xFEEDFACE | // Mach-O 32-bit
        //     0xFEEDFACF | // Mach-O 64-bit
        //     0xCAFEBABE | // Fat binary (big endian)
        //     0xBEBAFECA | // Fat binary (little endian)
        //     0xCEFAEDFE | // Mach-O 32-bit (reverse endian)
        //     0xCFFAEDFE   // Mach-O 64-bit (reverse endian)
        //     => {
        //         let exe = darwin::Darwin::parse(reader)?;
        //         return Ok(Some(ExecutableType::Darwin(exe)));
        //     }
        //     _ => {}
        // }

        // Unknown format
        Ok(None)
    }
}