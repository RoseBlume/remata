use std::io::{Read, Seek, SeekFrom};
use crate::ParserError;
use super::{MetaArt, MetaExt};
use remata_macros::DisplayPretty;
/// Represents metadata extracted from MP4/M4A "atom"-based containers.
///
/// This struct maps common iTunes-style metadata atoms (©nam, ©ART, etc.)
/// into strongly-typed Rust fields. All fields are optional because
/// metadata presence varies between files.
#[derive(Default, Clone, DisplayPretty)]
pub struct AtomMeta {
    /// Track title (©nam).
    pub title: Option<String>,
    /// Year or release date (©day).
    pub year: Option<String>,
    /// Copyright information.
    pub copyright: Option<String>,
    /// Artist (©ART).
    pub artist: Option<String>,
    /// Album artist (aART).
    pub album_artist: Option<String>,
    /// Author.
    pub author: Option<String>,
    /// Composer.
    pub composer: Option<String>,
    /// Album name (©alb).
    pub album: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Synopsis (used in video media).
    pub synopsis: Option<String>,
    /// Genre (©gen).
    pub genre: Option<String>,
    /// Device make (camera/media source).
    pub make: Option<String>,
    /// Device model.
    pub model: Option<String>,
    /// Location metadata.
    pub location: Option<String>,
    /// Grouping (used by iTunes).
    pub grouping: Option<String>,
    /// TV show name.
    pub show: Option<String>,
    /// Episode ID.
    pub episode_id: Option<String>,
    /// Episode sort order.
    pub episode_sort: Option<u8>,
    /// Season number.
    pub season_number: Option<u8>,
    /// Lyrics.
    pub lyrics: Option<String>,
    /// Compilation flag.
    pub compilation: Option<u8>,
    /// Network/studio.
    pub network: Option<String>,
    /// Media type (audio, video, etc.).
    pub media_type: Option<u8>,
    /// HD video flag.
    pub hd_video: Option<u8>,
    /// Gapless playback flag.
    pub gapless_playback: Option<u8>,
    /// Encoder name.
    pub encoder: Option<String>,
    /// Encoding tool/software (©too).
    pub encoding_tool: Option<String>,
    /// Art
    pub album_art: Option<Vec<u8>>,
}

impl MetaExt for AtomMeta {
    fn title(&self) -> Option<String> {
        self.title
            .as_ref()
            .or(self.show.as_ref())
            .or(self.description.as_ref())
            .or(self.synopsis.as_ref())
            .cloned()
    }

    fn artist(&self) -> Option<String> {
        self.artist
            .as_ref()
            .or(self.album_artist.as_ref())
            .or(self.author.as_ref())
            .or(self.composer.as_ref())
            .cloned()
    }

    fn album(&self) -> Option<String> {
        self.album
            .as_ref()
            .or(self.grouping.as_ref())
            .or(self.show.as_ref())
            .cloned()
    }

    fn genre(&self) -> Option<String> {
        self.genre
            .as_ref()
            .cloned()
    }

    fn year(&self) -> Option<String> {
        self.year
            .as_ref()
            .cloned()
    }

    fn art(&self) -> Option<MetaArt> {
        self.album_art
            .as_ref()
            .map(|data| MetaArt::Bin(data.clone()))
    }
}





impl AtomMeta {
    /// Parses MP4/M4A atom-based metadata from a reader.
    ///
    /// This function walks the atom hierarchy recursively and extracts
    /// known metadata fields from `ilst` entries.
    ///
    /// Supported:
    /// - Nested atoms (`moov`, `udta`, `meta`, `ilst`)
    /// - Common iTunes metadata keys
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut meta = AtomMeta::default();

        loop {
            // Try reading atom header (8 bytes)
            let _start = reader.stream_position()?;

            let size = match read_u32(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            let atom_type = match read_type(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            if size < 8 {
                break;
            }

            let payload_size = size - 8;
            let payload_start = reader.stream_position()?;

            match &atom_type {
                // Container atoms (recursive parsing)
                b"moov" | b"udta" | b"ilst" => {
                    let sub = AtomMeta::parse_limited(reader, payload_size as u64)?;
                    meta.merge(sub);
                }

                // Metadata atom with header
                b"meta" => {
                    // Skip version + flags (4 bytes)
                    reader.seek(SeekFrom::Current(4))?;

                    let sub = AtomMeta::parse_limited(reader, (payload_size - 4) as u64)?;
                    meta.merge(sub);
                }

                // Actual metadata entries
                b"\xa9nam" => meta.title = read_meta_entry(reader, payload_size)?,
                b"\xa9ART" => meta.artist = read_meta_entry(reader, payload_size)?,
                b"aART" => meta.album_artist = read_meta_entry(reader, payload_size)?,
                b"\xa9alb" => meta.album = read_meta_entry(reader, payload_size)?,
                b"\xa9gen" => meta.genre = read_meta_entry(reader, payload_size)?,
                b"\xa9day" => meta.year = read_meta_entry(reader, payload_size)?,
                b"\xa9too" => meta.encoding_tool = read_meta_entry(reader, payload_size)?,
                b"covr" | b"rqco" => meta.album_art = read_cover_entry(reader, payload_size)?,
                // Unknown atoms are skipped
                _ => {
                    reader.seek(SeekFrom::Start(payload_start + payload_size as u64))?;
                }
            }

            // Ensure we're at the end of this atom
            reader.seek(SeekFrom::Start(payload_start + payload_size as u64))?;
        }

        Ok(meta)
    }

    /// Internal helper: parses a limited region of the stream.
    fn parse_limited<R: Read + Seek>(
        reader: &mut R,
        limit: u64,
    ) -> Result<Self, ParserError> {
        let start = reader.stream_position()?;
        let mut meta = AtomMeta::default();

        while reader.stream_position()? - start < limit {
            let pos = reader.stream_position()?;

            let size = match read_u32(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            let atom_type = match read_type(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            if size < 8 {
                break;
            }

            let payload_size = size - 8;
            let payload_start = reader.stream_position()?;

            match &atom_type {
                b"moov" | b"udta" | b"ilst" => {
                    let sub = AtomMeta::parse_limited(reader, payload_size as u64)?;
                    meta.merge(sub);
                }
                b"meta" => {
                    reader.seek(SeekFrom::Current(4))?;
                    let sub = AtomMeta::parse_limited(reader, (payload_size - 4) as u64)?;
                    meta.merge(sub);
                }

                b"\xa9nam" => meta.title = read_meta_entry(reader, payload_size)?,
                b"\xa9ART" => meta.artist = read_meta_entry(reader, payload_size)?,
                b"aART" => meta.album_artist = read_meta_entry(reader, payload_size)?,
                b"\xa9alb" => meta.album = read_meta_entry(reader, payload_size)?,
                b"\xa9gen" => meta.genre = read_meta_entry(reader, payload_size)?,
                b"\xa9day" => meta.year = read_meta_entry(reader, payload_size)?,
                b"\xa9too" => meta.encoding_tool = read_meta_entry(reader, payload_size)?,
                b"covr"  => meta.album_art = read_cover_entry(reader, payload_size)?,
                _ => {
                    reader.seek(SeekFrom::Start(payload_start + payload_size as u64))?;
                }
            }

            reader.seek(SeekFrom::Start(payload_start + payload_size as u64))?;

            // Safety: prevent infinite loop
            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(meta)
    }

    /// Merges another [`AtomMeta`] into this one.
    ///
    /// Existing values are preserved; only missing fields are filled.
    fn merge(&mut self, other: AtomMeta) {
        if self.title.is_none() { self.title = other.title; }
        if self.artist.is_none() { self.artist = other.artist; }
        if self.album.is_none() { self.album = other.album; }
        if self.genre.is_none() { self.genre = other.genre; }
        if self.year.is_none() { self.year = other.year; }
    }
}

/// Reads a metadata entry atom (`data` box inside a metadata key).
///
/// Returns a UTF-8 string if present.
fn read_meta_entry<R: Read + Seek>(
    reader: &mut R,
    size: u32
) -> Result<Option<String>, ParserError> {
    let start = reader.stream_position()?;

    while reader.stream_position()? - start < size as u64 {
        let atom_size = read_u32(reader)?;
        let atom_type = read_type(reader)?;

        if atom_size < 8 {
            break;
        }

        if &atom_type == b"data" {
            // Skip:
            // version/flags (4)
            // type indicator (4)
            reader.seek(SeekFrom::Current(8))?;

            let data_len = atom_size - 16;
            let mut buf = vec![0u8; data_len as usize];
            reader.read_exact(&mut buf)?;

            return Ok(Some(
                String::from_utf8_lossy(&buf)
                    .trim_matches('\0')
                    .to_string()
            ));
        } else {
            reader.seek(SeekFrom::Current((atom_size - 8) as i64))?;
        }
    }

    Ok(None)
}

/// Reads a big-endian `u32`.
fn read_u32<R: Read>(reader: &mut R) -> Result<u32, ParserError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)
        .map_err(|e| ParserError { message: e.to_string() })?;
    Ok(u32::from_be_bytes(buf))
}

/// Reads a 4-byte atom type identifier.
fn read_type<R: Read>(reader: &mut R) -> Result<[u8; 4], ParserError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)
        .map_err(|e| ParserError { message: e.to_string() })?;
    Ok(buf)
}

fn read_cover_entry<R: Read + Seek>(
    reader: &mut R,
    size: u32,
) -> Result<Option<Vec<u8>>, ParserError> {
    let start = reader.stream_position()?;
    let mut best: Option<Vec<u8>> = None;

    while reader.stream_position()? - start < size as u64 {
        let atom_size = read_u32(reader)?;
        let atom_type = read_type(reader)?;

        if atom_size < 16 {
            break;
        }

        if &atom_type == b"data" {
            let mut header = [0u8; 8];
            reader.read_exact(&mut header)?;

            let _data_type = u32::from_be_bytes(header[4..8].try_into().unwrap());

            let data_len = atom_size - 16;
            let mut buf = vec![0u8; data_len as usize];
            reader.read_exact(&mut buf)?;

            // Prefer first valid image
            if best.is_none() {
                best = Some(buf);
            }

            // (optional) you could match on data_type:
            // 13 = JPEG, 14 = PNG
        } else {
            reader.seek(SeekFrom::Current((atom_size - 8) as i64))?;
        }
    }

    Ok(best)
}



/*
| Element | FFmpeg Metadata Key | Description |
|---------|-------------------|-------------|
| Title | title | The title of this video. (String) |
| Year | date | The date of production. Please note that the ffmpeg documentation is totally wrong here, there is no key named year, but only date. (String) |
| Copyright | copyright | The copyright of your video. (String) |
| Artist | artist | The name of the (video) artist. Please don't use this element for the composer, as there is a dedicated element especially for the composer, see below. (String) |
| Album Artist | album_artist | The name of the album artist: this may be a guest artist or a featured artist. This element can also be left out or be the same name as the artist. (String) |
| Author | author | The author of the video. (String) |
| Composer | composer | The name of the composer. (String) |
| Album | album | The title or the name of this album. (String) |
| Description | comment | A (content) description of this video. For a synopsis, please see the separate element instead. (String) |
| Synopsis | synopsis | A synopsis, a longer description of this video. (String) |
| Genre | genre | The genre this video belongs to. (String) |
| Make | make | (String) |
| Model | model | (String) |
| Location | location | (String) |
| Grouping | grouping | The name of a group of videos somehow belonging together. In contrast to the album elment, grouping happens inside (that is, below) the album level. (String) |
| Show | show | The name of the TV show, if applicable. (String) |
| Episode | episode_id | Either the episode name or episode number, for display. If necessary, use the separate, yet optional episode number element for correct sorting. (String) |
| Eposide (Sorting) | episode_sort | This element is for sorting only, but never displayed. It allows numerical sorting of episode names that are strings, but not (necessarily) numbers. The valid range is limited to 0 to 255 only, so this doesn't support all those endless telenovas, it seems… (Int8) |
| Season | season_number | The season number, in the range of 0 to 255 only. (Int8) |
| Lyrics | lyrics | Optional lyrics for badly sung sing-along... (String) |
| Compilation | compilation | If 1, then this video file is part of a compilation. 0 otherwise. (Int8) |
| Network | network | (String) |
| Media Type | media_type | (Int8) |
| HD Video | hd_video | (Int8) |
| Gapless Playback | gapless_playback | (Int8) |
| Encoding Tool | encoder | Not available to us users, as it gets automatically set by ffmpeg itself; this is set to the libavformat version string. |
| Encoding Tool | encoding_tool | Not available to us users, as it gets automatically set by ffmpeg itself; this is set to the libavformat version string. |
*/