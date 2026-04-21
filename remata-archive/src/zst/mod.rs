//! # Zstandard (Zstd) Format Structures
//!
//! This module defines a complete representation of the Zstandard (`.zst`) frame
//! format, including all metadata that can be extracted without performing
//! decompression.
//!
//! ## Overview
//!
//! A Zstd stream is composed of one or more **frames**, each containing:
//!
//! - A frame header (flags, sizes, dictionary info)
//! - A sequence of blocks
//! - An optional checksum
//!
//! Blocks may contain:
//!
//! - Raw (uncompressed) data
//! - RLE (run-length encoded) data
//! - Compressed data (literals + sequences)
//!
//! ## Structure hierarchy
//!
//! ```text
//! Zstd
//! └── Frames
//!     ├── Frame Header
//!     ├── Blocks
//!     │   ├── Block Header
//!     │   ├── Literals Section
//!     │   └── Sequences Section
//!     └── Optional Checksum
//! ```
//!
//! ## Notes
//!
//! - Zstd is **not an archive format** (like tar/zip), it stores a single stream.
//! - Multiple frames may be concatenated.
//! - Many fields are optional and controlled by header flags.
//! - This module does not perform decompression—only parsing/representation.

use std::io::{self, Read, Error, ErrorKind};
use remata_macros::DisplayPretty;
use std::fmt;
/// Root Zstd container (may contain multiple frames)
#[derive(DisplayPretty, Default, Clone)]
pub struct Zstd {
    /// All frames in the stream
    pub frames: Vec<ZstdFrame>,
}

/// A single Zstd frame
#[derive(DisplayPretty, Default, Clone)]
pub struct ZstdFrame {
    /// Magic number (typically 0xFD2FB528)
    pub magic: Option<u32>,

    /// Parsed frame header
    pub header: Option<ZstdFrameHeader>,

    /// All blocks contained in this frame
    pub blocks: Vec<ZstdBlock>,

    /// Optional 32-bit checksum at end of frame
    pub checksum: Option<u32>,
}

/// Frame header describing how to interpret the frame
#[derive(DisplayPretty, Default, Clone)]
pub struct ZstdFrameHeader {
    /// Descriptor byte (bit-packed flags)
    pub descriptor: u8,

    /// Window size used for decompression
    pub window_size: Option<u64>,

    /// Dictionary ID (if frame uses external dictionary)
    pub dictionary_id: Option<u32>,

    /// Optional content size (uncompressed size)
    pub content_size: Option<u64>,

    /// Whether checksum is present at end of frame
    pub has_checksum: bool,

    /// Whether this is a single-segment frame
    pub single_segment: bool,

    /// Content size field size indicator (derived from descriptor)
    pub content_size_flag: u8,

    /// Dictionary ID field size indicator
    pub dictionary_id_flag: u8,
}

/// A single block inside a frame
#[derive(DisplayPretty, Default, Clone)]
pub struct ZstdBlock {
    /// Block header (type + size + last flag)
    pub header: Option<ZstdBlockHeader>,

    /// Block content
    pub content: Option<ZstdBlockContent>,
}

/// Block header (3 bytes, bit-packed)
#[derive(DisplayPretty, Clone)]
pub struct ZstdBlockHeader {
    /// Block type (raw, RLE, compressed, reserved)
    pub block_type: ZstdBlockType,

    /// Size of the block payload in bytes
    pub block_size: u32,

    /// Whether this is the last block in the frame
    pub last_block: bool,
}

/// Block content variants
#[derive(Debug, Clone)]
pub enum ZstdBlockContent {
    /// Raw (uncompressed) data
    Raw(Vec<u8>),

    /// Run-length encoded data (single byte repeated)
    Rle {
        /// Repeated byte value
        byte: u8,
        /// Number of repetitions
        repeat_count: u32,
    },

    /// Compressed block
    Compressed(ZstdCompressedBlock),

    /// Reserved/unknown block type
    Reserved(Vec<u8>),
}


impl fmt::Display for ZstdBlockContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZstdBlockContent::Raw(data) => {
                write!(f, "Raw block ({} bytes)", data.len())
            }

            ZstdBlockContent::Rle { byte, repeat_count } => {
                write!(
                    f,
                    "RLE block (byte: 0x{:02x}, repeat_count: {})",
                    byte,
                    repeat_count
                )
            }

            ZstdBlockContent::Compressed(block) => {
                write!(f, "Compressed block: {}", block)
            }

            ZstdBlockContent::Reserved(data) => {
                write!(f, "Reserved block ({} bytes)", data.len())
            }
        }
    }
}

/// Compressed block structure
#[derive(DisplayPretty, Default, Clone)]
pub struct ZstdCompressedBlock {
    /// Literals section (raw or compressed literals)
    pub literals: Option<ZstdLiteralsSection>,

    /// Sequences section (matches + offsets)
    pub sequences: Option<ZstdSequencesSection>,
}

/// Literals section (contains literal bytes)
#[derive(DisplayPretty, Clone)]
pub struct ZstdLiteralsSection {
    /// Encoding type of literals
    pub encoding_type: ZstdLiteralsEncodingType,

    /// Regenerated literal bytes (if decoded or raw)
    pub literals: Option<Vec<u8>>,

    /// Compressed size (if applicable)
    pub compressed_size: Option<u32>,

    /// Uncompressed size
    pub regenerated_size: Option<u32>,
}

/// Encoding types for literals
#[derive(DisplayPretty, Clone, Copy)]
pub enum ZstdLiteralsEncodingType {
    /// Raw literals (uncompressed)
    Raw,

    /// RLE literals
    Rle,

    /// Huffman-compressed literals
    Compressed,

    /// Repeat previous Huffman table
    Repeat,

    /// Unknown encoding
    Unknown(u8),
}

/// Sequences section (describes matches + offsets)
#[derive(DisplayPretty, Default, Clone)]
pub struct ZstdSequencesSection {
    /// Number of sequences in this block
    pub sequence_count: Option<u32>,

    /// Compression modes used for decoding
    pub compression_modes: Option<ZstdSequenceCompressionModes>,

    /// Decoded sequence entries (optional, requires full decoding)
    pub sequences: Vec<ZstdSequence>,
}

/// Compression modes used in sequences
#[derive(DisplayPretty, Clone)]
pub struct ZstdSequenceCompressionModes {
    /// Literal length encoding mode
    pub literal_length_mode: ZstdFseMode,

    /// Offset encoding mode
    pub offset_mode: ZstdFseMode,

    /// Match length encoding mode
    pub match_length_mode: ZstdFseMode,
}

/// Finite State Entropy (FSE) mode
#[derive(DisplayPretty, Clone, Copy)]
pub enum ZstdFseMode {
    /// Predefined distribution
    Predefined,

    /// Run-length encoded table
    Rle,

    /// Compressed FSE table
    Compressed,

    /// Repeat previous table
    Repeat,

    /// Unknown mode
    Unknown(u8),
}

/// A single sequence entry (match + literals)
#[derive(DisplayPretty, Clone)]
pub struct ZstdSequence {
    /// Length of literals before match
    pub literal_length: u32,

    /// Offset for match copy
    pub offset: u32,

    /// Match length
    pub match_length: u32,
}

/// Block type values
#[derive(DisplayPretty, Clone, Copy)]
pub enum ZstdBlockType {
    /// Raw (uncompressed) block
    Raw,

    /// Run-length encoded block
    Rle,

    /// Compressed block
    Compressed,

    /// Reserved/invalid block type
    Reserved(u8),
}



impl Zstd {
    /// Parses a Zst file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut frames = Vec::new();

        loop {
            let mut magic_buf = [0u8; 4];

            // EOF-safe read
            match reader.read_exact(&mut magic_buf) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            let magic = u32::from_le_bytes(magic_buf);

            // Zstd magic or skippable frame
            if magic & 0xFFFFFFF0 == 0x184D2A50 {
                // Skippable frame
                let mut size_buf = [0u8; 4];
                reader.read_exact(&mut size_buf)?;
                let size = u32::from_le_bytes(size_buf);

                let mut skip = vec![0u8; size as usize];
                reader.read_exact(&mut skip)?;
                continue;
            }

            if magic != 0xFD2FB528 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid Zstd magic"));
            }

            // ---- Frame Header ----
            let mut descriptor = [0u8; 1];
            reader.read_exact(&mut descriptor)?;
            let descriptor = descriptor[0];

            let single_segment = (descriptor & 0b0010_0000) != 0;
            let has_checksum = (descriptor & 0b0000_0100) != 0;

            let dictionary_id_flag = (descriptor >> 0) & 0b11;
            let content_size_flag = (descriptor >> 6) & 0b11;

            // Window descriptor (if not single segment)
            let window_size = if !single_segment {
                let mut win = [0u8; 1];
                reader.read_exact(&mut win)?;
                let exponent = (win[0] >> 3) as u64;
                let mantissa = (win[0] & 0b111) as u64;
                Some((1 << (exponent + 10)) + (mantissa << (exponent + 7)))
            } else {
                None
            };

            // Dictionary ID
            let dictionary_id = match dictionary_id_flag {
                0 => None,
                1 => {
                    let mut b = [0u8; 1];
                    reader.read_exact(&mut b)?;
                    Some(b[0] as u32)
                }
                2 => {
                    let mut b = [0u8; 2];
                    reader.read_exact(&mut b)?;
                    Some(u16::from_le_bytes(b) as u32)
                }
                3 => {
                    let mut b = [0u8; 4];
                    reader.read_exact(&mut b)?;
                    Some(u32::from_le_bytes(b))
                }
                _ => None,
            };

            // Content size
            let content_size = match content_size_flag {
                0 => {
                    if single_segment {
                        let mut b = [0u8; 1];
                        reader.read_exact(&mut b)?;
                        Some(b[0] as u64)
                    } else {
                        None
                    }
                }
                1 => {
                    let mut b = [0u8; 2];
                    reader.read_exact(&mut b)?;
                    Some(u16::from_le_bytes(b) as u64 + 256)
                }
                2 => {
                    let mut b = [0u8; 4];
                    reader.read_exact(&mut b)?;
                    Some(u32::from_le_bytes(b) as u64)
                }
                3 => {
                    let mut b = [0u8; 8];
                    reader.read_exact(&mut b)?;
                    Some(u64::from_le_bytes(b))
                }
                _ => None,
            };

            let header = ZstdFrameHeader {
                descriptor,
                window_size,
                dictionary_id,
                content_size,
                has_checksum,
                single_segment,
                content_size_flag,
                dictionary_id_flag,
            };

            // ---- Blocks ----
            let mut blocks = Vec::new();

            loop {
                let mut bh = [0u8; 3];
                reader.read_exact(&mut bh)?;

                let header_raw =
                    (bh[0] as u32) |
                    ((bh[1] as u32) << 8) |
                    ((bh[2] as u32) << 16);

                let last_block = (header_raw & 1) != 0;
                let block_type_bits = (header_raw >> 1) & 0b11;
                let block_size = header_raw >> 3;

                let block_type = match block_type_bits {
                    0 => ZstdBlockType::Raw,
                    1 => ZstdBlockType::Rle,
                    2 => ZstdBlockType::Compressed,
                    x => ZstdBlockType::Reserved(x as u8),
                };

                let mut block = ZstdBlock {
                    header: Some(ZstdBlockHeader {
                        block_type,
                        block_size,
                        last_block,
                    }),
                    content: None,
                };

                match block_type {
                    ZstdBlockType::Raw => {
                        let mut data = vec![0u8; block_size as usize];
                        reader.read_exact(&mut data)?;
                        block.content = Some(ZstdBlockContent::Raw(data));
                    }

                    ZstdBlockType::Rle => {
                        let mut b = [0u8; 1];
                        reader.read_exact(&mut b)?;
                        block.content = Some(ZstdBlockContent::Rle {
                            byte: b[0],
                            repeat_count: block_size,
                        });
                    }

                    ZstdBlockType::Compressed => {
                        let mut data = vec![0u8; block_size as usize];
                        reader.read_exact(&mut data)?;

                        block.content = Some(ZstdBlockContent::Compressed(
                            ZstdCompressedBlock {
                                literals: None,
                                sequences: None,
                            },
                        ));
                    }

                    ZstdBlockType::Reserved(_) => {
                        let mut data = vec![0u8; block_size as usize];
                        reader.read_exact(&mut data)?;
                        block.content = Some(ZstdBlockContent::Reserved(data));
                    }
                }

                blocks.push(block);

                if last_block {
                    break;
                }
            }

            // ---- Checksum ----
            let checksum = if header.has_checksum {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Some(u32::from_le_bytes(buf))
            } else {
                None
            };

            frames.push(ZstdFrame {
                magic: Some(magic),
                header: Some(header),
                blocks,
                checksum,
            });
        }

        Ok(Self { frames })
    }
}