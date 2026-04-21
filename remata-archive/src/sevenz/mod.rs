//! 7z
use std::io::{self, Read, Seek, SeekFrom, Error, ErrorKind};
use remata_macros::DisplayPretty;
mod disp;

/// Represents a 7Z
#[derive(DisplayPretty, Default, Clone)]
pub struct SevenZ {



    /// Major Version
    pub major_version: Option<u8>,
    /// Minor Version
    pub minor_version: Option<u8>,

    /// CRC of the start header
    pub start_header_crc: Option<u32>,

    /// Offset to the next header
    pub next_header_offset: Option<u64>,

    /// Size of the next header
    pub next_header_size: Option<u64>,

    /// CRC of the next header
    pub next_header_crc: Option<u32>,

    /// Stream-related metadata
    pub streams_info: Option<SevenZStreamsInfo>,

    /// File entries
    pub files: Vec<SevenZFile>,
}

/// Represents 7Z stream info
#[derive(DisplayPretty, Default, Clone)]
pub struct SevenZStreamsInfo {
    /// Packed streams (compressed data blobs)
    pub pack_info: Option<SevenZPackInfo>,

    /// Folder structures (compression pipelines)
    pub folders: Vec<SevenZFolder>,

    /// Mapping of files to folders
    pub substreams_info: Option<SevenZSubStreamsInfo>,
}


/// Represents 7Z pack info
#[derive(DisplayPretty, Default, Clone)]
pub struct SevenZPackInfo {
    /// Offset where packed streams begin
    pub pack_pos: Option<u64>,

    /// Sizes of packed streams
    pub pack_sizes: Vec<u64>,

    /// CRCs for packed streams
    pub pack_crcs: Option<Vec<u32>>,
}
/// Represents a Folder within a 7Z
#[derive(Debug, Default, Clone)]
pub struct SevenZFolder {
    /// Coders (compression algorithms used)
    pub coders: Vec<SevenZCoder>,

    /// Bind pairs (how coder outputs feed into others)
    pub bind_pairs: Vec<(u64, u64)>,

    /// Packed stream indices
    pub packed_streams: Vec<u64>,

    /// Unpacked size after decompression
    pub unpacked_size: Option<u64>,

    /// CRC of unpacked data
    pub crc: Option<u32>,
}




#[derive(DisplayPretty, Clone)]
/// Represents a 7z Coder
pub struct SevenZCoder {
    /// Method ID (e.g., LZMA = 0x030101)
    pub method_id: Vec<u8>,

    /// Number of input streams
    pub num_in_streams: u64,

    /// Number of output streams
    pub num_out_streams: u64,

    /// Properties (algorithm-specific)
    pub properties: Option<Vec<u8>>,
}

#[derive(DisplayPretty, Default, Clone)]
/// Represents a 7zs sub streams info
pub struct SevenZSubStreamsInfo {
    /// Number of unpack streams per folder
    pub unpack_stream_counts: Vec<u64>,

    /// Unpacked sizes per file
    pub unpack_sizes: Vec<u64>,

    /// CRCs per file
    pub crcs: Option<Vec<u32>>,
}

#[derive(DisplayPretty, Default, Clone)]
/// Represents a file stored inside a 7z
pub struct SevenZFile {
    /// File name (UTF-16 in spec, usually converted to UTF-8)
    pub name: Option<String>,

    /// Is this entry a directory?
    pub is_directory: bool,

    /// Is the file empty (no data stream)?
    pub is_empty_stream: bool,

    /// Is the file empty but has metadata?
    pub is_empty_file: bool,

    /// Uncompressed size
    pub uncompressed_size: Option<u64>,

    /// CRC of file data
    pub crc: Option<u32>,

    /// Creation time (Windows FILETIME)
    pub creation_time: Option<u64>,

    /// Last access time
    pub access_time: Option<u64>,

    /// Last modification time
    pub modification_time: Option<u64>,

    /// File attributes (Windows-style)
    pub attributes: Option<u32>,

    /// Index of folder (compression stream) this file belongs to
    pub folder_index: Option<u64>,
}

/// SevenZ signature
pub const SEVENZ_MAGIC: &'static [u8; 6] = b"7z\xBC\xAF\x27\x1C";


fn decode_header_if_needed(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut cursor = data;

    let id = read_byte(&mut cursor)?;

    if id != 0x17 {
        return Ok(data.to_vec()); // plain header
    }

    // EncodedHeader
    let streams = parse_streams_info(&mut cursor)?;

    let folder = &streams.folders[0];
    let coder = &folder.coders[0];

    let compressed = cursor;

    match coder.method_id.as_slice() {
        [0x21] => decode_lzma2(coder, compressed),
        [0x00] => Ok(compressed.to_vec()),
        _ => Err(Error::new(ErrorKind::Unsupported, "Unsupported coder")),
    }
}


fn read_byte(data: &mut &[u8]) -> io::Result<u8> {
    if data.is_empty() {
        return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
    }
    let b = data[0];
    *data = &data[1..];
    Ok(b)
}

fn read_vec(data: &mut &[u8], len: usize) -> io::Result<Vec<u8>> {
    if data.len() < len {
        return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
    }
    let v = data[..len].to_vec();
    *data = &data[len..];
    Ok(v)
}

fn read_slice<'a>(data: &mut &'a [u8], len: usize) -> io::Result<&'a [u8]> {
    if data.len() < len {
        return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
    }
    let s = &data[..len];
    *data = &data[len..];
    Ok(s)
}

impl SevenZ {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        // ---- Signature ----
        let mut sig = [0u8; 6];
        reader.read_exact(&mut sig)?;
        if &sig != SEVENZ_MAGIC {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid signature"));
        }

        let mut version = [0u8; 2];
        reader.read_exact(&mut version)?;

        let start_header_crc = read_u32(reader)?;
        let next_header_offset = read_u64(reader)?;
        let next_header_size = read_u64(reader)?;
        let next_header_crc = read_u32(reader)?;

        // ---- Read Next Header ----
        let base = 32;
        reader.seek(SeekFrom::Start(base + next_header_offset))?;

        let mut header_data = vec![0u8; next_header_size as usize];
        reader.read_exact(&mut header_data)?;

        // ---- Decode header if needed ----
        let header_data = decode_header_if_needed(&header_data)?;

        // ---- Parse actual header ----
        let mut cursor: &[u8] = &header_data;

        let mut streams_info = None;
        let mut files = Vec::new();

        while !cursor.is_empty() {
            let id = read_byte(&mut cursor)?;

            match id {
                0x04 => {
                    streams_info = Some(parse_streams_info(&mut cursor)?);
                }
                0x05 => {
                    files = parse_files_info(&mut cursor)?;
                }
                0x00 => break,
                _ => break,
            }
        }

        Ok(Self {
            major_version: Some(version[0]),
            minor_version: Some(version[1]),
            start_header_crc: Some(start_header_crc),
            next_header_offset: Some(next_header_offset),
            next_header_size: Some(next_header_size),
            next_header_crc: Some(next_header_crc),
            streams_info,
            files,
        })
    }
}

// impl SevenZ {
//     /// Parse a 7Z file
//     pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
//         fn log(msg: &str) {
//             println!("[7z] {}", msg);
//         }

//         fn read_exact_dbg<R: Read>(
//             r: &mut R,
//             buf: &mut [u8],
//             label: &str,
//         ) -> io::Result<()> {
//             match r.read_exact(buf) {
//                 Ok(_) => {
//                     println!("[read] {} ({} bytes)", label, buf.len());
//                     Ok(())
//                 }
//                 Err(e) => {
//                     println!("[ERROR] read_exact failed at {}: {}", label, e);
//                     Err(e)
//                 }
//             }
//         }

//         fn ensure_len(data: &[u8], need: usize, ctx: &str) -> io::Result<()> {
//             if data.len() < need {
//                 return Err(io::Error::new(
//                     ErrorKind::UnexpectedEof,
//                     format!("{}: need {}, have {}", ctx, need, data.len()),
//                 ));
//             }
//             Ok(())
//         }

//         fn read_byte(data: &mut &[u8], ctx: &str) -> io::Result<u8> {
//             ensure_len(data, 1, ctx)?;
//             let b = data[0];
//             *data = &data[1..];
//             Ok(b)
//         }

//         fn read_slice<'a>(
//             data: &mut &'a [u8],
//             len: usize,
//             ctx: &str,
//         ) -> io::Result<&'a [u8]> {
//             ensure_len(data, len, ctx)?;
//             let out = &data[..len];
//             *data = &data[len..];
//             Ok(out)
//         }

//         fn read_vint_dbg(data: &mut &[u8], ctx: &str) -> io::Result<u64> {
//             ensure_len(data, 1, ctx)?;
//             let first = data[0];
//             *data = &data[1..];

//             let mut mask = 0x80;
//             let mut additional = 0;

//             while first & mask != 0 {
//                 additional += 1;
//                 mask >>= 1;
//             }

//             let mut value = (first & (mask - 1)) as u64;

//             ensure_len(data, additional, ctx)?;

//             for _ in 0..additional {
//                 value = (value << 8) | data[0] as u64;
//                 *data = &data[1..];
//             }

//             println!("[vint:{}] {}", ctx, value);
//             Ok(value)
//         }

//         // ---- FILE SIZE ----
//         let file_len = reader.seek(SeekFrom::End(0))?;
//         reader.seek(SeekFrom::Start(0))?;
//         log(&format!("file_len = {}", file_len));

//         // ---- SIGNATURE ----
//         let mut sig = [0u8; 6];
//         read_exact_dbg(reader, &mut sig, "signature")?;

//         if &sig != SEVENZ_MAGIC {
//             return Err(Error::new(ErrorKind::InvalidData, "Invalid 7z signature"));
//         }

//         // ---- VERSION ----
//         let mut version = [0u8; 2];
//         read_exact_dbg(reader, &mut version, "version")?;

//         // ---- CRC ----
//         let mut crc_buf = [0u8; 4];
//         read_exact_dbg(reader, &mut crc_buf, "start_header_crc")?;
//         let start_header_crc = u32::from_le_bytes(crc_buf);

//         // ---- OFFSET ----
//         let mut buf8 = [0u8; 8];
//         read_exact_dbg(reader, &mut buf8, "next_header_offset")?;
//         let next_header_offset = u64::from_le_bytes(buf8);

//         read_exact_dbg(reader, &mut buf8, "next_header_size")?;
//         let next_header_size = u64::from_le_bytes(buf8);

//         let mut crc_buf2 = [0u8; 4];
//         read_exact_dbg(reader, &mut crc_buf2, "next_header_crc")?;
//         let next_header_crc = u32::from_le_bytes(crc_buf2);

//         log(&format!(
//             "offset={} size={} crc={}",
//             next_header_offset, next_header_size, next_header_crc
//         ));

//         // ---- SEEK ----
//         let base = 32;
//         let header_start = base + next_header_offset;

//         if header_start + next_header_size > file_len {
//             return Err(io::Error::new(
//                 ErrorKind::InvalidData,
//                 format!(
//                     "Header out of bounds: {} + {} > {}",
//                     header_start, next_header_size, file_len
//                 ),
//             ));
//         }

//         reader.seek(SeekFrom::Start(header_start))?;
//         log(&format!("seek -> {}", header_start));

//         // ---- READ HEADER ----
//         let mut header_data = vec![0u8; next_header_size as usize];
//         read_exact_dbg(reader, &mut header_data, "next_header")?;

//         let mut cursor: &[u8] = &header_data;
//         let mut files = Vec::new();
//         let mut streams_info = None;

//         // ---- PARSE LOOP ----
//         while !cursor.is_empty() {
//             let id = read_byte(&mut cursor, "header_id")?;
//             log(&format!("section id = 0x{:02x}", id));

//             match id {
//                 0x05 => {
//                     let num_files = read_vint_dbg(&mut cursor, "num_files")? as usize;
//                     log(&format!("files = {}", num_files));

//                     files = vec![SevenZFile::default(); num_files];

//                     loop {
//                         let prop_id = read_byte(&mut cursor, "file_prop_id")?;
//                         if prop_id == 0x00 {
//                             break;
//                         }

//                         let size = read_vint_dbg(&mut cursor, "prop_size")? as usize;
//                         let mut prop_data = read_slice(&mut cursor, size, "prop_data")?;

//                         log(&format!("prop 0x{:02x}, size={}", prop_id, size));

//                         match prop_id {
//                             0x11 => {
//                                 let external = read_byte(&mut prop_data, "names.external")?;
//                                 if external != 0 {
//                                     log("external names unsupported");
//                                     continue;
//                                 }

//                                 for file in &mut files {
//                                     let mut name_u16 = Vec::new();

//                                     loop {
//                                         if prop_data.len() < 2 {
//                                             break;
//                                         }

//                                         let ch = u16::from_le_bytes([prop_data[0], prop_data[1]]);
//                                         prop_data = &prop_data[2..];

//                                         if ch == 0 {
//                                             break;
//                                         }

//                                         name_u16.push(ch);
//                                     }

//                                     file.name =
//                                         Some(String::from_utf16_lossy(&name_u16));
//                                 }
//                             }

//                             _ => {
//                                 log("skipping unknown file property");
//                             }
//                         }
//                     }
//                 }

//                 0x04 => {
//                     log("StreamsInfo encountered");
//                     streams_info = Some(parse_streams_info(&mut cursor)?);
//                 }

//                 0x00 => {
//                     log("end marker");
//                     break;
//                 }
//                 0x17 => {
//                     log("EncodedHeader detected (compressed header)");

//                     // TEMP: just dump bytes so you can inspect
//                     println!("Encoded header raw: {:02x?}", cursor);

//                     break;
//                 }

//                 _ => {
//                     log(&format!("unknown section: 0x{:02x}", id));
//                     break;
//                 }
//             }
//         }

//         Ok(Self {
//             major_version: Some(version[0]),
//             minor_version: Some(version[1]),
//             start_header_crc: Some(start_header_crc),
//             next_header_offset: Some(next_header_offset),
//             next_header_size: Some(next_header_size),
//             next_header_crc: Some(next_header_crc),
//             streams_info,
//             files,
//         })
//     }
// }
// Read a variable length integer
fn read_vint(data: &mut &[u8]) -> io::Result<u64> {
    if data.is_empty() {
        return Err(io::Error::new(ErrorKind::UnexpectedEof, "EOF in vint"));
    }

    let first = data[0];
    *data = &data[1..];

    let mut mask = 0x80;
    let mut additional = 0;

    while first & mask != 0 {
        additional += 1;
        mask >>= 1;
    }

    let mut value = (first & (mask - 1)) as u64;

    if data.len() < additional {
        return Err(io::Error::new(ErrorKind::UnexpectedEof, "EOF in vint payload"));
    }

    for _ in 0..additional {
        value = (value << 8) | data[0] as u64;
        *data = &data[1..];
    }

    Ok(value)
}
fn parse_streams_info(data: &mut &[u8]) -> io::Result<SevenZStreamsInfo> {
    let mut streams = SevenZStreamsInfo::default();

    fn read_vint(data: &mut &[u8]) -> io::Result<u64> {
        let first = data[0];
        *data = &data[1..];

        let mut mask = 0x80;
        let mut value;
        let mut additional = 0;

        while first & mask != 0 {
            additional += 1;
            mask >>= 1;
        }

        value = (first & (mask - 1)) as u64;

        for _ in 0..additional {
            value = (value << 8) | data[0] as u64;
            *data = &data[1..];
        }

        Ok(value)
    }

    while !data.is_empty() {
        let id = data[0];
        *data = &data[1..];

        match id {
            0x06 => {
                // PackInfo
                let pack_pos = read_vint(data)?;
                let num_streams = read_vint(data)? as usize;

                let mut pack_sizes = Vec::new();
                let mut pack_crcs = None;

                loop {
                    let pid = data[0];
                    *data = &data[1..];

                    if pid == 0x00 {
                        break;
                    }

                    match pid {
                        0x09 => {
                            for _ in 0..num_streams {
                                pack_sizes.push(read_vint(data)?);
                            }
                        }
                        0x0A => {
                            let mut crcs = Vec::new();
                            for _ in 0..num_streams {
                                let mut buf = [0u8; 4];
                                buf.copy_from_slice(&data[..4]);
                                *data = &data[4..];
                                crcs.push(u32::from_le_bytes(buf));
                            }
                            pack_crcs = Some(crcs);
                        }
                        _ => {}
                    }
                }

                streams.pack_info = Some(SevenZPackInfo {
                    pack_pos: Some(pack_pos),
                    pack_sizes,
                    pack_crcs,
                });
            }

            0x07 => {
                // UnpackInfo → folders
                loop {
                    let pid = data[0];
                    *data = &data[1..];

                    if pid == 0x00 {
                        break;
                    }

                    match pid {
                        0x0B => {
                            // Folder
                            let num_folders = read_vint(data)? as usize;

                            let external = data[0];
                            *data = &data[1..];

                            if external != 0 {
                                return Err(Error::new(ErrorKind::InvalidData, "External folders unsupported"));
                            }

                            for _ in 0..num_folders {
                                let num_coders = read_vint(data)? as usize;

                                let mut coders = Vec::new();

                                for _ in 0..num_coders {
                                    let flags = data[0];
                                    *data = &data[1..];

                                    let id_size = (flags & 0x0F) as usize;
                                    let mut method_id = vec![0u8; id_size];
                                    method_id.copy_from_slice(&data[..id_size]);
                                    *data = &data[id_size..];

                                    let mut num_in = 1;
                                    let mut num_out = 1;

                                    if flags & 0x10 != 0 {
                                        num_in = read_vint(data)?;
                                        num_out = read_vint(data)?;
                                    }

                                    let properties = if flags & 0x20 != 0 {
                                        let size = read_vint(data)? as usize;
                                        let mut props = vec![0u8; size];
                                        if data.len() < size {
                                            return Err(io::Error::new(ErrorKind::UnexpectedEof, "truncated data"));
                                        }
                                        props.copy_from_slice(&data[..size]);
                                        *data = &data[size..];
                                        Some(props)
                                    } else {
                                        None
                                    };

                                    coders.push(SevenZCoder {
                                        method_id,
                                        num_in_streams: num_in,
                                        num_out_streams: num_out,
                                        properties,
                                    });
                                }

                                streams.folders.push(SevenZFolder {
                                    coders,
                                    ..Default::default()
                                });
                            }
                        }

                        0x0C => {
                            // CodersUnpackSize
                            for folder in &mut streams.folders {
                                folder.unpacked_size = Some(read_vint(data)?);
                            }
                        }

                        0x0A => {
                            // CRCs
                            for folder in &mut streams.folders {
                                let mut buf = [0u8; 4];
                                buf.copy_from_slice(&data[..4]);
                                *data = &data[4..];
                                folder.crc = Some(u32::from_le_bytes(buf));
                            }
                        }

                        _ => {}
                    }
                }
            }

            0x08 => {
                // SubStreamsInfo
                let mut sub = SevenZSubStreamsInfo::default();

                loop {
                    let pid = data[0];
                    *data = &data[1..];

                    if pid == 0x00 {
                        break;
                    }

                    match pid {
                        0x0D => {
                            for _ in 0..streams.folders.len() {
                                sub.unpack_stream_counts.push(read_vint(data)?);
                            }
                        }
                        0x09 => {
                            let total: usize = sub.unpack_stream_counts.iter().sum::<u64>() as usize;
                            for _ in 0..total {
                                sub.unpack_sizes.push(read_vint(data)?);
                            }
                        }
                        0x0A => {
                            let total: usize = sub.unpack_stream_counts.iter().sum::<u64>() as usize;
                            let mut crcs = Vec::new();

                            for _ in 0..total {
                                let mut buf = [0u8; 4];
                                buf.copy_from_slice(&data[..4]);
                                *data = &data[4..];
                                crcs.push(u32::from_le_bytes(buf));
                            }

                            sub.crcs = Some(crcs);
                        }
                        _ => {}
                    }
                }

                streams.substreams_info = Some(sub);
            }

            0x00 => break,
            _ => break,
        }
    }

    Ok(streams)
}