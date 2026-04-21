//! Xz Module
use std::io::{self, Read, Error, ErrorKind};
use remata_macros::{FromPrimitive, DisplayPretty};

/// Holds the signature for XZ files
pub const XZ_MAGIC: &'static [u8; 6] = b"\xFD7zXZ\x00";


#[derive(DisplayPretty, Default, Clone)]
/// Represents Xz Metadata
pub struct Xz {

    /// Stream flags (version + check type)
    pub stream_flags: Option<XzStreamFlags>,

    /// Check type used (CRC32, CRC64, SHA256, etc.)
    pub check_type: Option<XzCheckType>,

    /// Optional backward size (from footer)
    pub backward_size: Option<u64>,

    /// Stream padding size (bytes of 0x00 between streams)
    pub stream_padding: Option<u64>,

    /// Total number of blocks in the stream
    pub block_count: Option<u64>,

    /// Total compressed size (sum of all blocks)
    pub total_compressed_size: Option<u64>,

    /// Total uncompressed size
    pub total_uncompressed_size: Option<u64>,

    /// Block-level metadata
    pub blocks: Vec<XzBlock>,

    /// Index records (from index field)
    pub index: Vec<XzIndexRecord>,

    /// Footer integrity check (CRC32 of footer)
    pub footer_crc32: Option<u32>,
}

/// Stream flags (2 bytes in header/footer)
#[derive(DisplayPretty, Clone)]
pub struct XzStreamFlags {
    /// Stream version (usually 0)
    pub version: u8,

    /// Check method used for integrity
    pub check_type: XzCheckType,
}

/// Integrity check algorithms supported by XZ
#[derive(DisplayPretty, Clone, Copy, FromPrimitive)]
pub enum XzCheckType {
    #[value = 0]
    /// No Integrity Check
    None,
    #[value = 1]
    /// CRC-32 Integrity Check
    Crc32,
    #[value = 4]
    /// CRC-64 Integrity Check
    Crc64,
    #[value = 10]
    /// SHA-256 Integrity Check
    Sha256,
    /// Unknown Integrity Check
    Unknown(u8),
}


/// A single block inside the XZ stream
#[derive(DisplayPretty, Default, Clone)]
pub struct XzBlock {
    /// Compressed size of this block
    pub compressed_size: Option<u64>,

    /// Uncompressed size of this block
    pub uncompressed_size: Option<u64>,

    /// Block header size
    pub header_size: Option<u32>,

    /// Filters used (LZMA2, Delta, BCJ, etc.)
    pub filters: Vec<XzFilter>,

    /// Integrity check value for this block
    pub check: Option<Vec<u8>>,
}

/// Filters applied to a block (pipeline)
#[derive(DisplayPretty, Clone)]
pub struct XzFilter {
    /// Filter ID (LZMA2 = 0x21, Delta = 0x03, etc.)
    pub id: u64,

    /// Raw properties for the filter
    pub properties: Vec<u8>,
}

/// Index record (maps blocks to sizes)
#[derive(DisplayPretty, Clone)]
pub struct XzIndexRecord {
    /// Unpadded size of the block
    pub unpadded_size: u64,

    /// Uncompressed size of the block
    pub uncompressed_size: u64,
}

impl Xz {
    /// Parses an Xz file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 6];
        reader.read_exact(&mut magic)?;

        if &magic != XZ_MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid XZ header"));
        }

        // Stream Flags
        let mut flags = [0u8; 2];
        reader.read_exact(&mut flags)?;

        let version = flags[0];
        let check_type = XzCheckType::from(flags[1]);

        // Skip header CRC32
        let mut crc_buf = [0u8; 4];
        reader.read_exact(&mut crc_buf)?;

        let mut blocks = Vec::new();
        let mut total_compressed = 0u64;
        let mut total_uncompressed = 0u64;



        loop {
            let mut first = [0u8; 1];
            reader.read_exact(&mut first)?;

            // Index indicator (0x00)
            if first[0] == 0x00 {
                break;
            }

            // Block header size (stored as (size / 4) - 1)
            let header_size = (first[0] as u32 + 1) * 4;

            let mut header_rest = vec![0u8; header_size as usize - 1];
            reader.read_exact(&mut header_rest)?;

            let mut block = XzBlock {
                header_size: Some(header_size),
                ..Default::default()
            };



            // flags byte already read earlier
            let flags = header_rest[0];

            // Bits 0–1: number of filters - 1
            let filter_count = (flags & 0x03) + 1;

            let has_compressed_size = flags & 0x40 != 0;
            let has_uncompressed_size = flags & 0x80 != 0;

            let mut cursor = &header_rest[1..];

            // Optional sizes
            if has_compressed_size {
                let size = read_vli(&mut cursor)?;
                block.compressed_size = Some(size);
                total_compressed += size;
            }

            if has_uncompressed_size {
                let size = read_vli(&mut cursor)?;
                block.uncompressed_size = Some(size);
                total_uncompressed += size;
            }

            // Parse Filters
            for _ in 0..filter_count {
                let id = read_vli(&mut cursor)?;
                let prop_size = read_vli(&mut cursor)? as usize;

                let mut props = vec![0u8; prop_size];
                if prop_size > 0 {
                    cursor.read_exact(&mut props)?;
                }

                block.filters.push(XzFilter {
                    id,
                    properties: props,
                });
            }


            blocks.push(block);

            // Skip compressed data (we don’t decode)
            if let Some(size) = blocks.last().unwrap().compressed_size {
                let mut skip = vec![0u8; size as usize];
                reader.read_exact(&mut skip)?;
            }
        }

        // ---- Index ----
        let mut index = Vec::new();

        let count = read_vli(reader)?;
        let block_count = count;

        for _ in 0..count {
            let unpadded = read_vli(reader)?;
            let uncompressed = read_vli(reader)?;

            index.push(XzIndexRecord {
                unpadded_size: unpadded,
                uncompressed_size: uncompressed,
            });
        }

        // Skip index padding to 4-byte alignment
        let mut pad = [0u8; 1];
        while let Ok(_) = reader.read_exact(&mut pad) {
            if pad[0] != 0 {
                break;
            }
        }

        // ---- Footer ----
        let mut footer = [0u8; 12];
        reader.read_exact(&mut footer)?;

        let footer_crc32 = u32::from_le_bytes([footer[0], footer[1], footer[2], footer[3]]);
        let backward_size = u32::from_le_bytes([footer[4], footer[5], footer[6], footer[7]]) as u64;

        // flags[8..10] repeat stream flags
        // footer[10..12] = "YZ"

        Ok(Self {
            stream_flags: Some(XzStreamFlags {
                version,
                check_type,
            }),
            check_type: Some(check_type),
            backward_size: Some(backward_size),
            stream_padding: None,
            block_count: Some(block_count),
            total_compressed_size: Some(total_compressed),
            total_uncompressed_size: Some(total_uncompressed),
            blocks,
            index,
            footer_crc32: Some(footer_crc32),
        })
    }
}


// ---- Helper: read variable-length integer ----
fn read_vli<R: Read>(r: &mut R) -> io::Result<u64> {
    let mut value = 0u64;
    let mut shift = 0;

    loop {
        let mut byte = [0u8; 1];
        r.read_exact(&mut byte)?;
        let b = byte[0];

        value |= ((b & 0x7F) as u64) << shift;

        if b & 0x80 == 0 {
            break;
        }

        shift += 7;
    }

    Ok(value)
}