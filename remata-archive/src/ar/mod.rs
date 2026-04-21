//! # Unix Archive (`ar`) Parser
//!
//! This module provides structures for representing and parsing the
//! Unix **`ar` archive format**, commonly used for:
//!
//! - Static libraries (`.a`) on Unix-like systems
//! - Debian packages (`.deb`, which internally use `ar`)
//!
//! ## Format Overview
//!
//! An `ar` archive consists of:
//!
//! ```text
//! Global Header ("!<arch>\n")
//! ├── File Header (60 bytes)
//! ├── File Data
//! ├── Padding (if needed)
//! ├── File Header
//! ├── ...
//! ```
//!
//! Each file entry contains a fixed-width ASCII header followed by raw data.
//!
//! ## Key Characteristics
//!
//! - No compression (pure container format)
//! - Flat structure (no directories)
//! - Fixed-size headers (60 bytes per entry)
//! - Even-byte alignment (entries padded if needed)
//!
//! ## Extensions
//!
//! Several extensions exist to overcome format limitations:
//!
//! ### GNU Extensions
//! - `"//"` → String table for long filenames
//! - `"/123"` → Offset into string table
//! - `"/"` → Symbol table (used by linkers)
//!
//! ### BSD Extensions
//! - `"#1/<len>"` → Filename stored inline before file data
//!
//! ## Notes
//!
//! - Numeric fields are stored as ASCII (not binary)
//! - Filenames may require resolution through extension mechanisms
//! - This module stores file data eagerly, but streaming is recommended for large archives

use std::io::{self, Read, Error, ErrorKind};
use remata_macros::DisplayPretty;

/// Holds the AR Magic Number
pub const AR_MAGIC: &'static [u8; 8] = b"!<arch>\n";

/// Represents a complete `ar` archive.
///
/// This is the root container holding all entries parsed from the archive.
#[derive(Default, Clone, DisplayPretty)]
pub struct Ar {

    /// All entries contained in the archive.
    ///
    /// Entries are stored in the order they appear in the file.
    pub entries: Vec<ArEntry>,
}

/// Represents a single entry (file) inside an `ar` archive.
///
/// Each entry corresponds to one file or special record (such as symbol tables).
#[derive(DisplayPretty, Default, Clone)]
pub struct ArEntry {
    /// File identifier (name).
    ///
    /// This may contain:
    /// - A normal filename
    /// - Special values like `"/"` (symbol table)
    /// - `"//"` (string table for long filenames)
    /// - References like `"/123"` (GNU extension)
    pub file_name: Option<String>,

    /// File modification timestamp (Unix time).
    ///
    /// Stored as ASCII in the archive header and parsed into a numeric value.
    pub timestamp: Option<u64>,

    /// Owner user ID.
    ///
    /// Parsed from ASCII field in the header.
    pub owner_id: Option<u32>,

    /// Owner group ID.
    ///
    /// Parsed from ASCII field in the header.
    pub group_id: Option<u32>,

    /// File mode (Unix permissions).
    ///
    /// Typically stored as an octal value in ASCII form.
    pub file_mode: Option<u32>,

    /// Size of the file data in bytes.
    ///
    /// This represents the size of the stored data, not including padding.
    pub file_size: Option<u64>,

    /// File data contents.
    ///
    /// This contains the raw bytes of the file.
    ///
    /// ⚠️ Note:
    /// - For large archives, storing this in memory may be inefficient.
    /// - A streaming approach is often preferable.
    pub data: Option<Vec<u8>>,

    /// Entry type (normal file or special extension).
    ///
    /// Used to distinguish between standard files and special entries
    /// such as symbol tables or string tables.
    pub entry_type: Option<ArEntryType>,
}

/// Describes the type of an `ar` archive entry.
///
/// The format includes several special entries used by linkers and extensions.
#[derive(DisplayPretty, Clone)]
pub enum ArEntryType {
    /// A regular file entry.
    ///
    /// This is the most common type and represents actual file contents.
    Normal,

    /// Symbol table entry (`"/"`).
    ///
    /// Used by linkers to map symbols to object files in static libraries.
    SymbolTable,

    /// String table entry (`"//"`).
    ///
    /// Contains long filenames used by GNU `ar`.
    /// Other entries may reference this table via offsets.
    StringTable,

    /// GNU extended filename reference (`"/<offset>"`).
    ///
    /// Indicates that the filename is stored in the string table
    /// at the specified byte offset.
    GnuFilename,

    /// BSD extended filename (`"#1/<length>"`).
    ///
    /// The filename is stored inline at the beginning of the file data.
    /// The actual file data follows immediately after the name.
    BsdFilename,

    /// Unknown or non-standard entry type.
    ///
    /// Stores the raw identifier string for further inspection.
    Unknown(String),
}

impl Ar {
    /// Parse an AR file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut global = [0u8; 8];
        reader.read_exact(&mut global)?;

        if &global != b"!<arch>\n" {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid ar header"));
        }

        let mut entries = Vec::new();
        let mut gnu_string_table: Option<Vec<u8>> = None;

        loop {
            let mut header = [0u8; 60];

            // Stop cleanly at EOF
            match reader.read_exact(&mut header) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            // Validate header trailer
            if &header[58..60] != b"`\n" {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid file header"));
            }

            // Helper: parse ASCII field
            fn parse_str(field: &[u8]) -> String {
                String::from_utf8_lossy(field).trim().to_string()
            }

            fn parse_u64(field: &[u8]) -> Option<u64> {
                parse_str(field).parse().ok()
            }

            fn parse_u32(field: &[u8]) -> Option<u32> {
                parse_str(field).parse().ok()
            }

            let raw_name = parse_str(&header[0..16]);
            let timestamp = parse_u64(&header[16..28]);
            let owner_id = parse_u32(&header[28..34]);
            let group_id = parse_u32(&header[34..40]);
            let file_mode = parse_u32(&header[40..48]);
            let file_size = parse_u64(&header[48..58]).unwrap_or(0);

            let mut data = vec![0u8; file_size as usize];
            reader.read_exact(&mut data)?;

            // Handle padding (2-byte alignment)
            if file_size % 2 != 0 {
                let mut pad = [0u8; 1];
                reader.read_exact(&mut pad)?;
            }

            let mut file_name = Some(raw_name.clone());
            let mut entry_type = Some(ArEntryType::Normal);

            // ---- Special cases ----

            if raw_name == "/" {
                entry_type = Some(ArEntryType::SymbolTable);
            } else if raw_name == "//" {
                entry_type = Some(ArEntryType::StringTable);
                gnu_string_table = Some(data.clone());
            } else if raw_name.starts_with('/') && raw_name.len() > 1 {
                // GNU filename reference: "/123"
                if let Ok(offset) = raw_name[1..].parse::<usize>() {
                    if let Some(table) = &gnu_string_table {
                        if offset < table.len() {
                            let name = table[offset..]
                                .split(|&b| b == b'\n')
                                .next()
                                .unwrap_or(&[]);
                            file_name = Some(String::from_utf8_lossy(name).to_string());
                            entry_type = Some(ArEntryType::GnuFilename);
                        }
                    }
                }
            } else if raw_name.starts_with("#1/") {
                // BSD long filename: "#1/<len>"
                if let Ok(len) = raw_name[3..].parse::<usize>() {
                    if len <= data.len() {
                        let name_bytes = &data[..len];
                        let real_data = data[len..].to_vec();

                        file_name = Some(String::from_utf8_lossy(name_bytes).to_string());
                        entry_type = Some(ArEntryType::BsdFilename);

                        entries.push(ArEntry {
                            file_name,
                            timestamp,
                            owner_id,
                            group_id,
                            file_mode,
                            file_size: Some((file_size as usize - len) as u64),
                            data: Some(real_data),
                            entry_type,
                        });

                        continue;
                    }
                }
            }

            entries.push(ArEntry {
                file_name,
                timestamp,
                owner_id,
                group_id,
                file_mode,
                file_size: Some(file_size),
                data: Some(data),
                entry_type,
            });
        }

        Ok(Self {
            entries,
        })
    }
}