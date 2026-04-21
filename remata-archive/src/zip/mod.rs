//! Zip

use std::io::{self, Read};


use remata_macros::{DisplayPretty, FromPrimitive};
mod simd;
mod disp;


#[inline(always)]
unsafe fn read_u16(ptr: *const u8) -> u16 {
    unsafe { core::ptr::read_unaligned(ptr as *const u16) }
}

#[inline(always)]
unsafe fn read_u32(ptr: *const u8) -> u32 {
    unsafe { core::ptr::read_unaligned(ptr as *const u32) }
}

const CHUNK_SIZE: usize = 64 * 1024;
const MAX_BUFFER: usize = 4 * 1024 * 1024;

/// Holds the signature for a Zip file
pub const ZIP_SIG: &'static [u8; 4] = b"PK\x03\x04";

/// Central File Header Signature
pub const CENTRAL_FILE_HEADER_SIGNATURE: u32 = 0x02014b50;

/// Although not originally assigned a signature, the value
/// 0x08074b50 has commonly been adopted as a signature value
/// for the data descriptor record.  Implementers SHOULD be
/// aware that ZIP files MAY be encountered with or without this
/// signature marking data descriptors and SHOULD account for
/// either case when reading ZIP files to ensure compatibility.
pub const DESC_SIG: u32 = 0x08074b50;

/// Local Header Signature
pub const LOCAL_HEADER_SIG: u32 = 0x04034b50;

/// End of Central Directory Signature
pub const EOCD_SIG: &[u8; 4] = b"PK\x05\x06";

/// Zip Struct
#[derive(Debug)]
pub struct Zip {
    /// The local file headers in a zip
    pub local_file_headers: Vec<LocalFileHeader>,

    // /// Encryption Headers
    // pub crypt_header: Option<Vec<EncryptionHeader>>,

    /// Data Descriptors
    pub data_descriptors: Vec<DataDescriptor>,

    // /// Archive Decryption Headers
    // pub archive_decryption_headers: Vec<ArchiveDecryptionHeader>,

    /// Archive Extra Data Records
    pub archive_extra_data_records: Vec<ArchiveExtraDataRecord>,

    /// Central Directory Headers
    pub central_directory_headers: Vec<CentralDirectoryHeader>
}

impl std::default::Default for Zip {
    fn default() -> Self {
        Self {
            local_file_headers: Vec::with_capacity(1024),
            data_descriptors: Vec::with_capacity(1024),
            archive_extra_data_records: Vec::with_capacity(256),
            central_directory_headers: Vec::with_capacity(1024),
        }
    }
}


impl Zip {
    /// Parse a Zip file
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut zip = Zip::default();

        let mut buffer = Vec::<u8>::with_capacity(CHUNK_SIZE * 2);
        let mut temp = vec![0u8; CHUNK_SIZE];
        let mut start = 0;

        loop {
            let n = reader.read(&mut temp)?;
            if n == 0 {
                break;
            }

            buffer.extend_from_slice(&temp[..n]);

            process_buffer(&mut buffer, &mut start, &mut zip);

            // Prevent pathological growth if no valid signatures found
            if buffer.len() > MAX_BUFFER {
                let keep = 1024; // keep last 1KB (covers any partial record)
                let drain_to = buffer.len().saturating_sub(keep);
                buffer.drain(0..drain_to);
            }
        }

        // Final attempt (in case last chunk completed something)
        process_buffer(&mut buffer, &mut start, &mut zip);

        Ok(zip)
    }
}








/// Local File Header
#[derive(Default, DisplayPretty)]
pub struct LocalFileHeader {
    /// The version required to extract
    pub version_req_to_extract: Option<String>, // 2 bytes

    /// General-purpose bit flag.
    ///
    /// Contains feature flags such as encryption, compression options, etc.
    pub bit_flag: Option<u16>, // 2 bytes

    /// Compression method used for the file.
    pub compression: Option<ZipCompression>, // 2 bytes

    /// Last modification date/time of the file.
    pub modify_time: Option<String>, // 2 bytes

    /// Last modification date/time of the file.
    pub modify_date: Option<String>, // 2 bytes


    /// CRC-32 checksum of the uncompressed data.
    pub crc: Option<u32>, // 4 bytes

    /// Size of the compressed data in bytes.
    pub compressed_size: Option<u32>, // 4 Bytes

    /// Size of the original uncompressed data in bytes.
    pub uncompressed_size: Option<u32>, // 4 Bytes

    /// File Name Length
    pub file_name_length: Option<u16>, // 2 bytes

    /// Extra Field Length
    pub extra_field_length: Option<u16>, // 2 bytes

    /// File Name
    pub file_name: Option<String>, // Variable length

    /// Extra Field
    pub extra_field: Option<String> // Variable length
}

impl LocalFileHeader {
    /// Parse a Local File Header
    pub fn parse(input: &[u8]) -> io::Result<(Self, usize)> {
        const FIXED: usize = 26;

        if input.len() < FIXED {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let buf = &input[..FIXED];
        let ptr = buf.as_ptr();

        let file_name_length =
            unsafe { read_u16(ptr.add(22)) }.to_le() as usize;
        let extra_length =
            unsafe { read_u16(ptr.add(24)) }.to_le() as usize;

        let total = FIXED + file_name_length + extra_length;

        if input.len() < total {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let name = &input[FIXED..FIXED + file_name_length];
        let extra = &input[FIXED + file_name_length..total];

        let header = Self {
            version_req_to_extract: Some(format!("{}", unsafe { read_u16(ptr.add(0)) }.to_le())),
            bit_flag: Some(unsafe { read_u16(ptr.add(2)) }.to_le()),
            compression: Some(ZipCompression::from(unsafe { read_u16(ptr.add(4)) }.to_le())),
            modify_time: Some(format!("{}", unsafe { read_u16(ptr.add(6)) }.to_le())),
            modify_date: Some(format!("{}", unsafe { read_u16(ptr.add(8)) }.to_le())),
            crc: Some(unsafe { read_u32(ptr.add(10)) }.to_le()),
            compressed_size: Some(unsafe { read_u32(ptr.add(14)) }.to_le()),
            uncompressed_size: Some(unsafe { read_u32(ptr.add(18)) }.to_le()),
            file_name_length: Some(file_name_length as u16),
            extra_field_length: Some(extra_length as u16),
            file_name: Some(String::from_utf8_lossy(name).to_string()),
            extra_field: Some(format!("{:x?}", extra)),
        };

        Ok((header, total))
    }
    /// Merge a DataDescriptor with a Local Filer Header
    pub fn merge(&mut self, desc: &DataDescriptor) {
        if self.crc.unwrap_or(0) == 0 {
            self.crc = desc.crc;
        }
        if self.compressed_size.unwrap_or(0) == 0 {
            self.compressed_size = desc.compressed_size;
        }
        if self.uncompressed_size.unwrap_or(0) == 0 {
            self.uncompressed_size = desc.uncompressed_size;
        }
    }
}



/// When the Central Directory Encryption method is used,
/// the data descriptor record is not required, but MAY be used.
/// If present, and bit 3 of the general purpose bit field is set to
/// indicate its presence, the values in fields of the data descriptor
/// record MUST be set to binary zeros.  See the section on the Strong
/// Encryption Specification for information. Refer to the section in
/// this document entitled "Incorporating PKWARE Proprietary Technology
/// into Your Product" for more information.
#[derive(Default, DisplayPretty)]
pub struct DataDescriptor {
    /// CRC-32 checksum of the data.
    pub crc: Option<u32>, // 4 bytes

    /// Size of the compressed data in bytes.
    pub compressed_size: Option<u32>, // 4 Bytes

    /// Size of the original uncompressed data in bytes.
    pub uncompressed_size: Option<u32>, // 4 Bytes
}

impl DataDescriptor {
    /// Parse a Data Descriptor
    pub fn parse(input: &[u8]) -> io::Result<(Self, usize)> {
        const SIZE: usize = 12;

        if input.len() < SIZE {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let ptr = input.as_ptr();

        let desc = Self {
            crc: Some(unsafe { read_u32(ptr.add(0)) }.to_le()),
            compressed_size: Some(unsafe { read_u32(ptr.add(4)) }.to_le()),
            uncompressed_size: Some(unsafe { read_u32(ptr.add(8)) }.to_le()),
        };

        Ok((desc, SIZE))
    }
}

/// Compression methods used in ZIP archives.
#[derive(DisplayPretty, Clone, Copy, FromPrimitive)]
pub enum ZipCompression {
    /// No compression applied.
    #[value = 0]
    None,
    /// LZW compression (Shrunk).
    #[value = 1]
    Shrunk,
    /// Reduced compression with factor 1.
    #[value = 2]
    Reduced1,
    /// Reduced compression with factor 2.
    #[value = 3]
    Reduced2,
    /// Reduced compression with factor 3.
    #[value = 4]
    Reduced3,
    /// Reduced compression with factor 4.
    #[value = 5]
    Reduced4,
    /// PKWARE's implode method.
    #[value = 6]
    Imploded,
    /// Tokenized compression (reserved).
    #[value = 7]
    Tokenized,
    /// DEFLATE compression (most common).
    #[value = 8]
    Deflated,
    /// Enhanced DEFLATE (Deflate64).
    #[value = 9]
    Deflate64,
    /// Older implode method.
    #[value = 10]
    ImplodedOld,
    /// BZIP2 compression.
    #[value = 12]
    Bzip2,
    /// LZMA compression.
    #[value = 14]
    Lzma,
    /// IBM TERSE (new).
    #[value = 18]
    IbmTerseNew,
    /// IBM LZ77 compression.
    #[value = 19]
    IbmLz77,
    /// JPEG compression for images.
    #[value = 96]
    Jpeg,
    /// WavPack audio compression.
    #[value = 97]
    WavPack,
    /// PPMd compression.
    #[value = 98]
    Ppmd,
    /// Unknown or unsupported compression method.
    Unknown(u16),
}

/// Signature for Extra Data Record
pub const ARCHIVE_EXTRA_DATA_REC_SIG: u32 = 0x08064b50;

/// Archive Extra Data Record
#[derive(Default, DisplayPretty)]
pub struct ArchiveExtraDataRecord {
    /// The length of the extra field
    pub extra_field_length: Option<u32>, // 4 bytes
    /// The extra fields data
    pub extra_field_data: Vec<u8> // Variable Size
}

impl ArchiveExtraDataRecord {
    /// Parse an Archive Data Record
    pub fn parse(input: &[u8]) -> io::Result<(Self, usize)> {
        if input.len() < 4 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let ptr = input.as_ptr();

        let len = unsafe { read_u32(ptr) }.to_le() as usize;
        let total = 4 + len;

        if input.len() < total {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let data = input[4..total].to_vec();

        let record = Self {
            extra_field_length: Some(len as u32),
            extra_field_data: data,
        };

        Ok((record, total))
    }
}


/// Central Directory header
#[derive(Default, DisplayPretty)]
pub struct CentralDirectoryHeader {
    /// The version required to extract
    pub version_made_by: Option<String>, // 2 bytes
    /// The version required to extract
    pub version_req_to_extract: Option<String>, // 2 bytes

    /// General-purpose bit flag.
    ///
    /// Contains feature flags such as encryption, compression options, etc.
    pub bit_flag: Option<u16>, // 2 bytes

    /// Compression method used for the file.
    pub compression: Option<ZipCompression>, // 2 bytes

    /// Last modification date/time of the file.
    pub modify_time: Option<String>, // 2 bytes

    /// Last modification date/time of the file.
    pub modify_date: Option<String>, // 2 bytes


    /// CRC-32 checksum of the uncompressed data.
    pub crc: Option<u32>, // 4 bytes

    /// Size of the compressed data in bytes.
    pub compressed_size: Option<u32>, // 4 Bytes

    /// Size of the original uncompressed data in bytes.
    pub uncompressed_size: Option<u32>, // 4 Bytes

    /// File Name Length
    pub file_name_length: Option<u16>, // 2 bytes

    /// Extra Field Length
    pub extra_field_length: Option<u16>, // 2 bytes

    /// File Comment Length
    pub file_comment_length: Option<u16>, // 2 bytes

    /// Disk Number Start
    pub disk_number_start: Option<u16>, // 2 bytes

    /// Internal File Attributes
    pub internal_file_attributes: Option<u16>, // 2 bytes

    /// External File Attributes
    pub external_file_attributes: Option<u32>, // 2 bytes

    /// Relative Offset of Local Header
    pub relative_offset_of_local_header: Option<u32>,
    // relative offset of local header 4 bytes

    /// File Name
    pub file_name: Option<String>, // Variable length

    /// Extra Field
    pub extra_field: Option<String>, // Variable length

    /// File Comment
    pub file_comment: Option<String>, // Variable length
}

impl CentralDirectoryHeader {
    /// Parse a Central Directory Header
    pub fn parse(input: &[u8]) -> io::Result<(Self, usize)> {
        const FIXED: usize = 42;

        if input.len() < FIXED {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let buf = &input[..FIXED];
        let ptr = buf.as_ptr();

        let file_name_length =
            unsafe { read_u16(ptr.add(24)) }.to_le() as usize;
        let extra_length =
            unsafe { read_u16(ptr.add(26)) }.to_le() as usize;
        let comment_length =
            unsafe { read_u16(ptr.add(28)) }.to_le() as usize;

        let total = FIXED + file_name_length + extra_length + comment_length;

        if input.len() < total {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        let name_start = FIXED;
        let extra_start = name_start + file_name_length;
        let comment_start = extra_start + extra_length;

        let name = &input[name_start..extra_start];
        let extra = &input[extra_start..comment_start];
        let comment = &input[comment_start..total];

        let header = Self {
            version_made_by: Some(format!("{}", unsafe { read_u16(ptr.add(0)) }.to_le())),
            version_req_to_extract: Some(format!("{}", unsafe { read_u16(ptr.add(2)) }.to_le())),
            bit_flag: Some(unsafe { read_u16(ptr.add(4)) }.to_le()),
            compression: Some(ZipCompression::from(unsafe { read_u16(ptr.add(6)) }.to_le())),
            modify_time: Some(format!("{}", unsafe { read_u16(ptr.add(8)) }.to_le())),
            modify_date: Some(format!("{}", unsafe { read_u16(ptr.add(10)) }.to_le())),
            crc: Some(unsafe { read_u32(ptr.add(12)) }.to_le()),
            compressed_size: Some(unsafe { read_u32(ptr.add(16)) }.to_le()),
            uncompressed_size: Some(unsafe { read_u32(ptr.add(20)) }.to_le()),
            file_name_length: Some(file_name_length as u16),
            extra_field_length: Some(extra_length as u16),
            file_comment_length: Some(comment_length as u16),
            disk_number_start: Some(unsafe { read_u16(ptr.add(30)) }.to_le()),
            internal_file_attributes: Some(unsafe { read_u16(ptr.add(32)) }.to_le()),
            external_file_attributes: Some(unsafe { read_u32(ptr.add(34)) }.to_le()),
            relative_offset_of_local_header: Some(unsafe { read_u32(ptr.add(38)) }.to_le()),
            file_name: Some(String::from_utf8_lossy(name).to_string()),
            extra_field: Some(format!("{:x?}", extra)),
            file_comment: Some(String::from_utf8_lossy(comment).to_string()),
        };

        Ok((header, total))
    }
}

fn process_buffer(buffer: &mut Vec<u8>, start: &mut usize, zip: &mut Zip) {
    let mut i = *start;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        if is_x86_feature_detected!("avx2") {
            i = simd::process_avx2(buffer, i, zip);
        } else if is_x86_feature_detected!("sse2") {
            i = simd::process_sse2(buffer, i, zip);
        }
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        if is_aarch64_feature_detected!("neon") {
            i = process_neon(buffer, i, zip);
        }
    }

    while i + 4 <= buffer.len() {
        if buffer[i] == b'P' && buffer[i + 1] == b'K' {
            let sig = u32::from_le_bytes([
                buffer[i],
                buffer[i + 1],
                buffer[i + 2],
                buffer[i + 3],
            ]);

            let slice = &buffer[i + 4..];

            let result = if sig == LOCAL_HEADER_SIG {
                LocalFileHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.local_file_headers.push(h);
                        c
                    })
            } else {
                Err(io::ErrorKind::InvalidData.into())
            };

            if let Ok(consumed) = result {
                i += 4 + consumed;
                continue;
            }
        }

        i += 1;
    }

    *start = i;

    if *start > (1 << 20) {
        let drain_to = (*start).min(buffer.len());
        buffer.drain(0..drain_to);
        *start -= drain_to; // keep relative position correct
        // *start = 0;
    }
}
