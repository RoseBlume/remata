//! Torrent
use remata_macros::DisplayPretty;
use std::collections::HashMap;
use std::io::{self, Read, Seek};
mod disp;
/// Represents a parsed BitTorrent (.torrent) metadata file.
///
/// Torrent files use the bencode format to store structured metadata
/// such as tracker URLs, file lists, and piece hashes.
#[derive(Default, Clone, DisplayPretty)]
pub struct Torrent {
    /// Primary tracker URL (`announce`).
    pub announce: Option<String>,

    /// List of additional trackers (`announce-list`).
    pub announce_list: Option<Vec<String>>,

    /// Optional human-readable comment.
    pub comment: Option<String>,

    /// Creator of the torrent (`created by`).
    pub created_by: Option<String>,

    /// Creation date (UNIX timestamp stored as string).
    pub creation_date: Option<String>,

    /// Encoding used for strings.
    pub encoding: Option<String>,

    /// Core torrent information dictionary.
    pub info: Option<TorrentInfo>,
    /// Hash for the info dictionary
    pub infohash: Option<[u8; 20]>,

    /// Web seed URLs (`url-list`).
    pub url_list: Option<Vec<String>>,
}

/// Represents a file entry in a multi-file torrent.
#[derive(Default, Clone, DisplayPretty)]
pub struct TorrentFile {
    /// File size in bytes.
    pub length: Option<u64>,

    /// Optional MD5 checksum.
    pub md5sum: Option<String>,

    /// File path (joined).
    pub path: Option<String>,

    /// UTF-8 encoded path.
    pub path_utf8: Option<String>,
}

/// Represents media profile information.
#[derive(Default, Clone, DisplayPretty)]
pub struct TorrentProfile {
    /// Audio codec.
    pub acodec: Option<String>,

    /// Video height.
    pub height: Option<u64>,

    /// Video codec.
    pub vcodec: Option<String>,

    /// Video width.
    pub width: Option<u64>,
}



/// Represents the `info` dictionary of a torrent.
///
/// This contains the essential data used to identify and download content.
#[derive(Default, Clone)]
pub struct TorrentInfo {
    /// File duration (if media).
    pub file_duration: Option<String>,

    /// Media type.
    pub file_media: Option<String>,

    /// Files in multi-file torrents.
    pub files: Option<Vec<TorrentFile>>,

    /// File size (single-file torrents).
    pub length: Option<u64>,

    /// MD5 checksum.
    pub md5sum: Option<String>,

    /// Display name.
    pub name: Option<String>,

    /// UTF-8 name.
    pub name_utf8: Option<String>,

    /// Piece length in bytes.
    pub piece_length: Option<u64>,

    /// SHA-1 piece hashes (20 bytes each).
    pub pieces: Option<Vec<[u8; 20]>>,

    /// Private flag.
    pub private: Option<bool>,

    /// Optional media profiles.
    pub profiles: Option<Vec<TorrentProfile>>,
}




//
// -----------------------------
// BENCODE PARSER
// -----------------------------

#[derive(Debug, Clone)]
enum Bencode {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<Bencode>),
    Dict(HashMap<String, Bencode>),
}






//
// -----------------------------
// TORRENT PARSER
// -----------------------------

impl Torrent {
    /// Parses a `.torrent` file from a reader.
    ///
    /// This function:
    /// - Decodes the bencoded structure
    /// - Extracts known top-level fields
    /// - Parses the nested `info` dictionary
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The input is not valid bencode
    /// - I/O fails
    pub fn parse<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let root = parse_bencode(&mut reader)?;

        let mut torrent = Torrent::default();

        if let Bencode::Dict(dict) = root {
            for (k, v) in dict {
                match k.as_str() {
                    "announce" => torrent.announce = to_string(v),
                    "comment" => torrent.comment = to_string(v),
                    "created by" => torrent.created_by = to_string(v),
                    "creation date" => torrent.creation_date = to_string(v),
                    "encoding" => torrent.encoding = to_string(v),
                    "url-list" => torrent.url_list = to_string_list(v),
                    "announce-list" => torrent.announce_list = to_nested_string_list(v),

                    "info" => {
                        if let Bencode::Dict(_) = &v {
                            let encoded = encode_bencode(&v);
                            torrent.infohash = Some(sha1(&encoded));
                            torrent.info = parse_info(v);
                        }
                    }

                    _ => {}
                }
            }
        }

        Ok(torrent)
    }
}
fn parse_info(v: Bencode) -> Option<TorrentInfo> {
    let mut info = TorrentInfo::default();

    let dict = match v {
        Bencode::Dict(d) => d,
        _ => return None,
    };

    for (k, v) in dict {
        match k.as_str() {
            "name" => info.name = to_string(v),
            "name.utf-8" => info.name_utf8 = to_string(v),
            "length" => info.length = to_int(v),
            "piece length" => info.piece_length = to_int(v),
            "private" => info.private = to_int(v).map(|x| x != 0),
            "pieces" => info.pieces = to_pieces(v),
            "files" => info.files = parse_files(v),
            "profiles" => info.profiles = parse_profiles(v),
            _ => {}
        }
    }

    Some(info)
}

//
// -----------------------------
// FILES
// -----------------------------
//

fn parse_files(v: Bencode) -> Option<Vec<TorrentFile>> {
    let list = match v {
        Bencode::List(l) => l,
        _ => return None,
    };

    let mut out = Vec::new();

    for item in list {
        let mut file = TorrentFile::default();

        if let Bencode::Dict(d) = item {
            for (k, v) in d {
                match k.as_str() {
                    "length" => file.length = to_int(v),
                    "md5sum" => file.md5sum = to_string(v),
                    "path" => file.path = to_path(v),
                    "path.utf-8" => file.path_utf8 = to_path(v),
                    _ => {}
                }
            }
        }

        out.push(file);
    }

    Some(out)
}

//
// -----------------------------
// PROFILES
// -----------------------------
//

fn parse_profiles(v: Bencode) -> Option<Vec<TorrentProfile>> {
    let list = match v {
        Bencode::List(l) => l,
        _ => return None,
    };

    let mut out = Vec::new();

    for item in list {
        let mut p = TorrentProfile::default();

        if let Bencode::Dict(d) = item {
            for (k, v) in d {
                match k.as_str() {
                    "acodec" => p.acodec = to_string(v),
                    "vcodec" => p.vcodec = to_string(v),
                    "width" => p.width = to_int(v),
                    "height" => p.height = to_int(v),
                    _ => {}
                }
            }
        }

        out.push(p);
    }

    Some(out)
}

//
// -----------------------------
// HELPERS
// -----------------------------
//

fn to_string(v: Bencode) -> Option<String> {
    match v {
        Bencode::Bytes(b) => Some(String::from_utf8_lossy(&b).to_string()),
        _ => None,
    }
}

fn to_string_list(v: Bencode) -> Option<Vec<String>> {
    match v {
        Bencode::List(l) => Some(l.into_iter().filter_map(to_string).collect()),
        _ => None,
    }
}

fn to_nested_string_list(v: Bencode) -> Option<Vec<String>> {
    match v {
        Bencode::List(l) => {
            let mut out = Vec::new();
            for item in l {
                if let Bencode::List(inner) = item {
                    for s in inner {
                        if let Some(v) = to_string(s) {
                            out.push(v);
                        }
                    }
                }
            }
            Some(out)
        }
        _ => None,
    }
}

fn to_int(v: Bencode) -> Option<u64> {
    match v {
        Bencode::Int(i) => Some(i as u64),
        _ => None,
    }
}

fn to_path(v: Bencode) -> Option<String> {
    match v {
        Bencode::List(parts) => {
            let mut out = Vec::new();
            for p in parts {
                if let Some(s) = to_string(p) {
                    out.push(s);
                }
            }
            Some(out.join("/"))
        }
        _ => None,
    }
}

fn to_pieces(v: Bencode) -> Option<Vec<[u8; 20]>> {
    match v {
        Bencode::Bytes(b) => {
            let mut out = Vec::new();
            for chunk in b.chunks(20) {
                if chunk.len() == 20 {
                    let mut arr = [0u8; 20];
                    arr.copy_from_slice(chunk);
                    out.push(arr);
                }
            }
            Some(out)
        }
        _ => None,
    }
}

//
// -----------------------------
// BENCODE PARSER (minimal safe version)
// -----------------------------
//

fn parse_bencode<R: Read>(reader: &mut R) -> io::Result<Bencode> {
    let mut byte = [0u8; 1];
    reader.read_exact(&mut byte)?;

    match byte[0] {
        b'i' => {
            let mut num = Vec::new();
            loop {
                reader.read_exact(&mut byte)?;
                if byte[0] == b'e' {
                    break;
                }
                num.push(byte[0]);
            }
            Ok(Bencode::Int(
                String::from_utf8_lossy(&num).parse().unwrap_or(0),
            ))
        }

        b'l' => {
            let mut list = Vec::new();
            loop {
                let peek = parse_peek(reader)?;
                if peek == b'e' {
                    break;
                }
                list.push(parse_bencode(reader)?);
            }
            Ok(Bencode::List(list))
        }

        b'd' => {
            let mut dict = HashMap::new();
            loop {
                let peek = parse_peek(reader)?;
                if peek == b'e' {
                    break;
                }

                let key = parse_string(reader, peek)?;
                let value = parse_bencode(reader)?;
                dict.insert(key, value);
            }
            Ok(Bencode::Dict(dict))
        }

        b'0'..=b'9' => {
            let s = parse_string(reader, byte[0])?;
            Ok(Bencode::Bytes(s.into_bytes()))
        }

        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "bad bencode")),
    }
}

fn parse_peek<R: Read>(reader: &mut R) -> io::Result<u8> {
    let mut b = [0u8; 1];
    reader.read_exact(&mut b)?;
    Ok(b[0])
}

fn parse_string<R: Read>(reader: &mut R, first: u8) -> io::Result<String> {
    let mut len_buf = vec![first];
    let mut byte = [0u8; 1];

    loop {
        reader.read_exact(&mut byte)?;
        if byte[0] == b':' {
            break;
        }
        len_buf.push(byte[0]);
    }

    let len: usize = String::from_utf8_lossy(&len_buf)
        .parse()
        .unwrap_or(0);

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).to_string())
}

//
// -----------------------------
// BENCODE ENCODER (CRITICAL FOR INFOHASH)
// -----------------------------
//

fn encode_bencode(v: &Bencode) -> Vec<u8> {
    match v {
        Bencode::Int(i) => format!("i{}e", i).into_bytes(),

        Bencode::Bytes(b) => {
            let mut out = b.len().to_string().into_bytes();
            out.push(b':');
            out.extend_from_slice(b);
            out
        }

        Bencode::List(l) => {
            let mut out = vec![b'l'];
            for i in l {
                out.extend(encode_bencode(i));
            }
            out.push(b'e');
            out
        }

        Bencode::Dict(d) => {
            let mut out = vec![b'd'];
            let mut keys: Vec<_> = d.keys().collect();
            keys.sort();

            for k in keys {
                out.extend(encode_bencode(&Bencode::Bytes(k.as_bytes().to_vec())));
                out.extend(encode_bencode(&d[k]));
            }

            out.push(b'e');
            out
        }
    }
}

//
// -----------------------------
// SHA-1 (STANDARD IMPLEMENTATION)
// -----------------------------
//

fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h0 = 0x67452301u32;
    let mut h1 = 0xEFCDAB89u32;
    let mut h2 = 0x98BADCFEu32;
    let mut h3 = 0x10325476u32;
    let mut h4 = 0xC3D2E1F0u32;

    let mut msg = data.to_vec();
    let bit_len = (msg.len() as u64) * 8;

    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];

        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }

        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut out = [0u8; 20];
    out[..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
}