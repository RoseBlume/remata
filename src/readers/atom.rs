use std::io::{Cursor, Read, Seek, SeekFrom};
use std::fmt;
use crate::ParserError;
/// Represents metadata extracted from MP4/M4A "atom"-based containers.
///
/// This struct maps common iTunes-style metadata atoms (©nam, ©ART, etc.)
/// into strongly-typed Rust fields. All fields are optional because
/// metadata presence varies between files.
#[derive(Debug, Default)]
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
}

/// Provides a human-readable display of metadata fields.
impl fmt::Display for AtomMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.title {
            writeln!(f, "Title: {}", title)?;
        }
        if let Some(year) = &self.year {
            writeln!(f, "Year: {}", year)?;
        }
        if let Some(copyright) = &self.copyright {
            writeln!(f, "Copyright: {}", copyright)?;
        }
        if let Some(artist) = &self.artist {
            writeln!(f, "Artist: {}", artist)?;
        }
        if let Some(album_artist) = &self.album_artist {
            writeln!(f, "Album Artist: {}", album_artist)?;
        }
        if let Some(author) = &self.author {
            writeln!(f, "Author: {}", author)?;
        }
        if let Some(composer) = &self.composer {
            writeln!(f, "Composer: {}", composer)?;
        }
        if let Some(album) = &self.album {
            writeln!(f, "Album: {}", album)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        if let Some(synopsis) = &self.synopsis {
            writeln!(f, "Synopsis: {}", synopsis)?;
        }
        if let Some(genre) = &self.genre {
            writeln!(f, "Genre: {}", genre)?;
        }
        if let Some(make) = &self.make {
            writeln!(f, "Make: {}", make)?;
        }
        if let Some(model) = &self.model {
            writeln!(f, "Model: {}", model)?;
        }
        if let Some(location) = &self.location {
            writeln!(f, "Location: {}", location)?;
        }
        if let Some(grouping) = &self.grouping {
            writeln!(f, "Grouping: {}", grouping)?;
        }
        if let Some(show) = &self.show {
            writeln!(f, "Show: {}", show)?;
        }
        if let Some(episode_id) = &self.episode_id {
            writeln!(f, "Episode ID: {}", episode_id)?;
        }
        if let Some(episode_sort) = self.episode_sort {
            writeln!(f, "Episode Sort: {}", episode_sort)?;
        }
        if let Some(season_number) = self.season_number {
            writeln!(f, "Season Number: {}", season_number)?;
        }
        if let Some(lyrics) = &self.lyrics {
            writeln!(f, "Lyrics: {}", lyrics)?;
        }
        if let Some(compilation) = self.compilation {
            writeln!(f, "Compilation: {}", compilation)?;
        }
        if let Some(network) = &self.network {
            writeln!(f, "Network: {}", network)?;
        }
        if let Some(media_type) = self.media_type {
            writeln!(f, "Media Type: {}", media_type)?;
        }
        if let Some(hd_video) = self.hd_video {
            writeln!(f, "HD Video: {}", hd_video)?;
        }
        if let Some(gapless_playback) = self.gapless_playback {
            writeln!(f, "Gapless Playback: {}", gapless_playback)?;
        }
        if let Some(encoder) = &self.encoder {
            writeln!(f, "Encoder: {}", encoder)?;
        }
        if let Some(encoding_tool) = &self.encoding_tool {
            writeln!(f, "Encoding Tool: {}", encoding_tool)?;
        }

        Ok(())
    }
}

impl AtomMeta {
    /// Parses MP4/M4A atom-based metadata from raw bytes.
    ///
    /// This function walks the atom hierarchy recursively and extracts
    /// known metadata fields from `ilst` entries.
    ///
    /// Supported:
    /// - Nested atoms (`moov`, `udta`, `meta`, `ilst`)
    /// - Common iTunes metadata keys
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut cursor = Cursor::new(data);
        let mut meta = AtomMeta::default();

        while (cursor.position() as usize + 8) <= data.len() {
            let size = read_u32(&mut cursor)?;
            let atom_type = read_type(&mut cursor)?;

            if size < 8 {
                break;
            }

            let payload_size = size - 8;
            let start = cursor.position();

            match &atom_type {
                // Container atoms (recursive parsing)
                b"moov" | b"udta" | b"ilst" => {
                    let mut buf = vec![0; payload_size as usize];
                    cursor.read_exact(&mut buf)?;
                    let sub = AtomMeta::parse(&buf)?;
                    meta.merge(sub);
                }

                // Metadata atom with header
                b"meta" => {
                    // Skip version + flags (4 bytes)
                    cursor.seek(SeekFrom::Current(4))?;

                    let mut buf = vec![0; (payload_size - 4) as usize];
                    cursor.read_exact(&mut buf)?;
                    let sub = AtomMeta::parse(&buf)?;
                    meta.merge(sub);
                }

                // Actual metadata entries
                b"\xa9nam" => meta.title = read_meta_entry(&mut cursor, payload_size)?,
                b"\xa9ART" => meta.artist = read_meta_entry(&mut cursor, payload_size)?,
                b"aART" => meta.album_artist = read_meta_entry(&mut cursor, payload_size)?,
                b"\xa9alb" => meta.album = read_meta_entry(&mut cursor, payload_size)?,
                b"\xa9gen" => meta.genre = read_meta_entry(&mut cursor, payload_size)?,
                b"\xa9day" => meta.year = read_meta_entry(&mut cursor, payload_size)?,
                b"\xa9too" => meta.encoding_tool = read_meta_entry(&mut cursor, payload_size)?,

                // Unknown atoms are skipped
                _ => {
                    cursor.seek(SeekFrom::Start(start + payload_size as u64))?;
                }
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
        // Extend as needed for additional fields
    }
}

/// Reads a metadata entry atom (`data` box inside a metadata key).
///
/// Returns a UTF-8 string if present.
fn read_meta_entry(
    cursor: &mut Cursor<&[u8]>,
    size: u32
) -> Result<Option<String>, ParserError> {
    let start = cursor.position();

    while (cursor.position() - start) < size as u64 {
        let atom_size = read_u32(cursor)?;
        let atom_type = read_type(cursor)?;

        if atom_size < 8 {
            break;
        }

        if &atom_type == b"data" {
            // Skip:
            // version/flags (4)
            // type indicator (4)
            cursor.seek(SeekFrom::Current(8))?;

            let data_len = atom_size - 16;
            let mut buf = vec![0u8; data_len as usize];
            cursor.read_exact(&mut buf)?;

            return Ok(Some(
                String::from_utf8_lossy(&buf)
                    .trim_matches('\0')
                    .to_string()
            ));
        } else {
            cursor.seek(SeekFrom::Current((atom_size - 8) as i64))?;
        }
    }

    Ok(None)
}

/// Reads a big-endian `u32`.
fn read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParserError> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf)
        .map_err(|e| ParserError { message: e.to_string() })?;
    Ok(u32::from_be_bytes(buf))
}

/// Reads a 4-byte atom type identifier.
fn read_type(cursor: &mut Cursor<&[u8]>) -> Result<[u8; 4], ParserError> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf)
        .map_err(|e| ParserError { message: e.to_string() })?;
    Ok(buf)
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