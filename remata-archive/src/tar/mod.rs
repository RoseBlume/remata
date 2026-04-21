//! # TAR Archive Parser
//!
//! This module provides a parser for the TAR archive format, including support for:
//!
//! - USTAR (POSIX) archives
//! - GNU extensions (long names, sparse files)
//! - PAX extended headers
//!
//! ## Supported Features
//!
//! - Standard file metadata (mode, uid, gid, timestamps)
//! - GNU long filename entries (`L`)
//! - PAX key-value metadata (`x`, `g`)
//! - GNU sparse file parsing (old GNU format + extensions)
//!
//! ## Sparse Files
//!
//! GNU TAR represents sparse files by storing only non-zero regions.
//! Metadata describing these regions is stored in:
//!
//! - The main header (up to 4 entries)
//! - Optional extension headers (additional entries)
//!
//! The actual file data contains only the concatenated sparse segments.
//!
//! This parser extracts sparse metadata into [`TarSparse`] but does not
//! reconstruct the full file contents automatically.
//!
//! ## Notes
//!
//! - Block size is always 512 bytes
//! - Two consecutive zero blocks mark end of archive
//! - Numeric fields are stored as octal ASCII
use std::fmt;
use std::io::{self, Read};
use remata_macros::DisplayPretty;
/// Holds a Tar files metadata
#[derive(DisplayPretty, Default, Clone)]
pub struct Tar {
    /// All entries (files, directories, links, etc.)
    pub entries: Vec<TarEntry>,
}

#[derive(Debug, Default, Clone)]
/// Holds entry metadata
pub struct TarEntry {
    /// File name (may be combined with prefix in USTAR)
    pub file_name: Option<String>,

    /// File mode (permissions)
    pub mode: Option<u32>,

    /// Owner user ID
    pub uid: Option<u32>,

    /// Owner group ID
    pub gid: Option<u32>,

    /// File size in bytes
    pub size: Option<u64>,

    /// Last modification time (Unix timestamp)
    pub mtime: Option<u64>,

    /// Checksum of header
    pub checksum: Option<u32>,

    /// Type of entry (file, dir, symlink, etc.)
    pub entry_type: Option<TarEntryType>,

    /// Linked file name (for symlinks/hardlinks)
    pub link_name: Option<String>,

    /// USTAR indicator ("ustar\0")
    pub ustar_indicator: Option<String>,

    /// USTAR version
    pub ustar_version: Option<String>,

    /// Owner user name
    pub uname: Option<String>,

    /// Owner group name
    pub gname: Option<String>,

    /// Device major number (for special files)
    pub dev_major: Option<u32>,

    /// Device minor number
    pub dev_minor: Option<u32>,

    /// Filename prefix (used in USTAR for long paths)
    pub prefix: Option<String>,

    /// Extended attributes (PAX headers)
    pub pax_attributes: Option<Vec<(String, String)>>,

    /// GNU-specific sparse file metadata
    pub sparse: Option<TarSparse>,

    /// File data (optional; often streamed instead)
    pub data: Option<Vec<u8>>,
}


impl fmt::Display for TarEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TarEntry {{")?;

        if let Some(name) = &self.file_name {
            writeln!(f, "  file_name: {}", name)?;
        }

        if let Some(mode) = self.mode {
            writeln!(f, "  mode: {:#o}", mode)?; // octal like tar CLI
        }

        if let Some(uid) = self.uid {
            writeln!(f, "  uid: {}", uid)?;
        }

        if let Some(gid) = self.gid {
            writeln!(f, "  gid: {}", gid)?;
        }

        if let Some(size) = self.size {
            writeln!(f, "  size: {} bytes", size)?;
        }

        if let Some(mtime) = self.mtime {
            writeln!(f, "  mtime: {}", mtime)?;
        }

        if let Some(checksum) = self.checksum {
            writeln!(f, "  checksum: {}", checksum)?;
        }

        if let Some(entry_type) = &self.entry_type {
            writeln!(f, "  entry_type: {:?}", entry_type)?;
        }

        if let Some(link) = &self.link_name {
            writeln!(f, "  link_name: {}", link)?;
        }

        if let Some(indicator) = &self.ustar_indicator {
            writeln!(f, "  ustar_indicator: {:?}", indicator)?;
        }

        if let Some(version) = &self.ustar_version {
            writeln!(f, "  ustar_version: {:?}", version)?;
        }

        if let Some(uname) = &self.uname {
            writeln!(f, "  uname: {}", uname)?;
        }

        if let Some(gname) = &self.gname {
            writeln!(f, "  gname: {}", gname)?;
        }

        if let Some(major) = self.dev_major {
            writeln!(f, "  dev_major: {}", major)?;
        }

        if let Some(minor) = self.dev_minor {
            writeln!(f, "  dev_minor: {}", minor)?;
        }

        if let Some(prefix) = &self.prefix {
            writeln!(f, "  prefix: {}", prefix)?;
        }

        if let Some(attrs) = &self.pax_attributes {
            writeln!(f, "  pax_attributes: [")?;
            for (k, v) in attrs {
                writeln!(f, "    {} = {}", k, v)?;
            }
            writeln!(f, "  ]")?;
        }

        if let Some(sparse) = &self.sparse {
            writeln!(f, "  sparse: {:?}", sparse)?;
        }

        if let Some(data) = &self.data {
            writeln!(f, "  data: {} bytes", data.len())?;
        }

        write!(f, "}}")
    }
}

/// Represents Tar Entry Type
#[derive(DisplayPretty, Clone)]
pub enum TarEntryType {
    /// Regular Tar Entry
    Regular,
    /// Hardlink Tar Entry
    HardLink,
    /// Symlink Tar Entry
    SymLink,
    /// Char Device Tar Entry
    CharDevice,
    /// Block Device Tar Entry
    BlockDevice,
    /// Directory Tar Entry
    Directory,
    /// Fifo Tar Entry
    Fifo,
    /// Contiguous Tar Entry
    Contiguous,
    /// Long Name Tar Entry
    LongName,
    /// Long Link Tar Entry
    LongLink,
    /// Sparse Tar Entry
    Sparse,
    /// Pax Header Tar Entry
    PaxHeader,
    /// Pax Global Header Tar Entry
    PaxGlobalHeader,
    /// Unknown Tar Entry
    Unknown(u8),
}

/// Repsents a Sparse Tar Entry
#[derive(Debug, Default, Clone)]
pub struct TarSparse {
    /// Sparse file segments (offset, length)
    pub entries: Vec<(u64, u64)>,

    /// Real size of sparse file
    pub real_size: Option<u64>,
}



impl fmt::Display for TarSparse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sparse Tar Entry {{")?;


        writeln!(f, "  entries: [")?;
        for (a, b) in &self.entries {
            writeln!(f, "    ({}, {})", a, b)?;
        }
        if let Some(size) = self.real_size {
            writeln!(f, "Real Size: {}", size)?;
        }

        write!(f, "}}")
    }
}



impl Tar {
    /// Parses a Tar file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut entries = Vec::new();

        let mut long_name: Option<String> = None;
        let mut pax_attrs: Option<Vec<(String, String)>> = None;

        loop {
            let mut header = [0u8; 512];
            reader.read_exact(&mut header)?;

            // End of archive = two consecutive zero blocks
            if header.iter().all(|&b| b == 0) {
                break;
            }

            let name = parse_string(&header[0..100]);
            let mode = parse_octal(&header[100..108]).map(|v| v as u32);
            let uid = parse_octal(&header[108..116]).map(|v| v as u32);
            let gid = parse_octal(&header[116..124]).map(|v| v as u32);
            let size = parse_octal(&header[124..136]).unwrap_or(0);
            let mtime = parse_octal(&header[136..148]);
            let checksum = parse_octal(&header[148..156]).map(|v| v as u32);

            let typeflag = header[156];
            let link_name = parse_string(&header[157..257]);

            let ustar_indicator = parse_string(&header[257..263]);
            let ustar_version = parse_string(&header[263..265]);

            let uname = parse_string(&header[265..297]);
            let gname = parse_string(&header[297..329]);

            let dev_major = parse_octal(&header[329..337]).map(|v| v as u32);
            let dev_minor = parse_octal(&header[337..345]).map(|v| v as u32);

            let prefix = parse_string(&header[345..500]);

            // Combine prefix + name (USTAR)
            let mut file_name = match (prefix.clone(), name.clone()) {
                (Some(p), Some(n)) => Some(format!("{}/{}", p, n)),
                (_, n) => n,
            };

            // Override with GNU long name if present
            if let Some(ln) = long_name.take() {
                file_name = Some(ln);
            }

            let entry_type = match typeflag {
                b'0' | 0 => TarEntryType::Regular,
                b'1' => TarEntryType::HardLink,
                b'2' => TarEntryType::SymLink,
                b'3' => TarEntryType::CharDevice,
                b'4' => TarEntryType::BlockDevice,
                b'5' => TarEntryType::Directory,
                b'6' => TarEntryType::Fifo,
                b'7' => TarEntryType::Contiguous,
                b'L' => TarEntryType::LongName,
                b'K' => TarEntryType::LongLink,
                b'S' => TarEntryType::Sparse,
                b'x' => TarEntryType::PaxHeader,
                b'g' => TarEntryType::PaxGlobalHeader,
                x => TarEntryType::Unknown(x),
            };

            // ---- read file data ----
            let mut data = vec![0u8; size as usize];
            if size > 0 {
                reader.read_exact(&mut data)?;
            }

            // Align to 512-byte boundary
            let padding = (512 - (size % 512)) % 512;
            if padding > 0 {
                let mut skip = vec![0u8; padding as usize];
                reader.read_exact(&mut skip)?;
            }

            let mut sparse: Option<TarSparse> = None;
            // ---- special handling ----

            match entry_type {
                TarEntryType::LongName => {
                    // GNU long filename
                    if let Ok(s) = String::from_utf8(data.clone()) {
                        long_name = Some(s.trim_end_matches('\0').to_string());
                    }
                    continue;
                }

                TarEntryType::PaxHeader | TarEntryType::PaxGlobalHeader => {
                    // Parse PAX key-value pairs
                    let mut attrs = Vec::new();
                    let mut i = 0;

                    while i < data.len() {
                        // format: "<len> key=value\n"
                        let mut j = i;
                        while j < data.len() && data[j] != b' ' {
                            j += 1;
                        }

                        if j >= data.len() {
                            break;
                        }

                        let len_str = String::from_utf8_lossy(&data[i..j]);
                        let len: usize = len_str.trim().parse().unwrap_or(0);

                        if len == 0 || i + len > data.len() {
                            break;
                        }

                        let record = &data[j + 1..i + len];
                        if let Some(eq) = record.iter().position(|&b| b == b'=') {
                            let key = String::from_utf8_lossy(&record[..eq]).to_string();
                            let val = String::from_utf8_lossy(&record[eq + 1..])
                                .trim_end_matches('\n')
                                .to_string();
                            attrs.push((key, val));
                        }

                        i += len;
                    }

                    pax_attrs = Some(attrs);
                    continue;
                },
                TarEntryType::Sparse => {
                    let mut sparse_entries = Vec::new();

                    // First 4 entries are in main header
                    let header_sparse = parse_sparse_entries(&header[386..482], 4);
                    sparse_entries.extend(header_sparse);

                    // Real size stored at 483..495
                    let real_size = parse_octal(&header[483..495]);

                    // Extension flag at 482
                    let mut is_extended = header[482] != 0;

                    // ---- Read extension headers ----
                    while is_extended {
                        let mut ext = [0u8; 512];
                        reader.read_exact(&mut ext)?;

                        let ext_entries = parse_sparse_entries(&ext[0..504], 21);
                        sparse_entries.extend(ext_entries);

                        is_extended = ext[504] != 0;
                    }

                    sparse = Some(TarSparse {
                        entries: sparse_entries,
                        real_size,
                    });
                }

                _ => {}
            }

            entries.push(TarEntry {
                file_name,
                mode,
                uid,
                gid,
                size: Some(size),
                mtime,
                checksum,
                entry_type: Some(entry_type),
                link_name,
                ustar_indicator,
                ustar_version,
                uname,
                gname,
                dev_major,
                dev_minor,
                prefix,
                pax_attributes: pax_attrs.take(),
                sparse,
                data: Some(data),
            });
        }

        Ok(Self { entries })
    }
}

fn parse_sparse_entries(buf: &[u8], count: usize) -> Vec<(u64, u64)> {
    let mut entries = Vec::new();

    for i in 0..count {
        let base = i * 24;

        let offset = {
            let s = String::from_utf8_lossy(&buf[base..base + 12])
                .trim_matches(char::from(0))
                .trim()
                .to_string();
            u64::from_str_radix(&s, 8).unwrap_or(0)
        };

        let size = {
            let s = String::from_utf8_lossy(&buf[base + 12..base + 24])
                .trim_matches(char::from(0))
                .trim()
                .to_string();
            u64::from_str_radix(&s, 8).unwrap_or(0)
        };

        if size > 0 {
            entries.push((offset, size));
        }
    }

    entries
}

fn parse_string(field: &[u8]) -> Option<String> {
                let s = field.split(|&b| b == 0).next().unwrap_or(&[]);
                if s.is_empty() {
                    None
                } else {
                    Some(String::from_utf8_lossy(s).trim().to_string())
                }
            }

            fn parse_octal(field: &[u8]) -> Option<u64> {
                let binding = String::from_utf8_lossy(field);
                let s = binding
                    .trim_matches(char::from(0))
                    .trim();
                if s.is_empty() {
                    None
                } else {
                    u64::from_str_radix(s, 8).ok()
                }
            }