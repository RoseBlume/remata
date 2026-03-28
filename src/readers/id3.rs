use std::io::{Cursor, Read};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Represents either an ID3v1 or ID3v2 metadata block.
///
/// This enum abstracts over the two major ID3 versions:
/// - ID3v1: fixed-size footer at end of file
/// - ID3v2: flexible, frame-based metadata at the beginning
#[derive(Debug)]
pub enum Id3 {
    /// ID3v1 metadata (128-byte footer)
    V1(Id3V1),
    /// ID3v2 metadata (frame-based header)
    V2(Id3V2),
}

/// Displays the underlying metadata regardless of version.
impl Display for Id3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Id3::V1(meta) => write!(f, "{}", meta),
            Id3::V2(meta) => write!(f, "{}", meta),
        }
    }
}

/// Represents ID3v1 metadata.
///
/// ID3v1 is a simple fixed-length format located at the last 128 bytes
/// of an MP3 file.
#[derive(Debug, Default)]
pub struct Id3V1 {
    /// Track title (30 bytes).
    pub title: Option<String>,
    /// Artist name (30 bytes).
    pub artist: Option<String>,
    /// Album name (30 bytes).
    pub album: Option<String>,
    /// Year (4 bytes).
    pub year: Option<String>,
    /// Comment (30 bytes).
    pub comment: Option<String>,
    /// Genre ID (numeric index into predefined list).
    pub genre: Option<u8>,
}

/// Formats ID3v1 metadata into a readable structure.
impl Display for Id3V1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ID3v1 Metadata:\n")?;
        if let Some(title) = &self.title {
            write!(f, "  Title: {}\n", title)?;
        }
        if let Some(artist) = &self.artist {
            write!(f, "  Artist: {}\n", artist)?;
        }
        if let Some(album) = &self.album {
            write!(f, "  Album: {}\n", album)?;
        }
        if let Some(year) = &self.year {
            write!(f, "  Year: {}\n", year)?;
        }
        if let Some(comment) = &self.comment {
            write!(f, "  Comment: {}\n", comment)?;
        }
        if let Some(genre) = self.genre {
            write!(f, "  Genre ID: {}\n", genre)?;
        }
        
        Ok(())
    }
}

/// Represents ID3v2 metadata.
///
/// ID3v2 uses a flexible frame-based structure where each field is
/// identified by a 4-byte frame ID (e.g., TIT2, TPE1).
#[derive(Debug, Default)]
pub struct Id3V2 {
    // ------------------------
    // Core fields
    // ------------------------

    /// Title/song name (TIT2).
    pub title: Option<String>,
    /// Lead performer/artist (TPE1).
    pub lead_performer: Option<String>,
    /// Band/orchestra/accompaniment (TPE2).
    pub band_orchestra: Option<String>,
    /// Subtitle/description refinement (TIT3).
    pub subtitle_description_refinement: Option<String>,
    /// Composer (TCOM).
    pub composer: Option<String>,
    /// Beats per minute (TBPM).
    pub bpm: Option<String>,
    /// Content type/genre (TCON).
    pub content_type: Option<String>,
    /// Copyright message (TCOP).
    pub copyright_message: Option<String>,
    /// Date (TDAT).
    pub date: Option<String>,
    /// Publisher (TPUB).
    pub publisher: Option<String>,
    /// Track number/position (TRCK).
    pub track_number_position_in_set: Option<String>,
    /// Part of a set (TPOS).
    pub part_of_set: Option<String>,
    /// Year (TYER).
    pub year: Option<String>,
    /// User-defined text (TXXX).
    pub user_defined_text: Option<String>,

    /// Attached picture (APIC frame, raw bytes).
    pub picture: Option<Vec<u8>>,
    /// Comments (COMM frames).
    pub comments: Vec<String>,

    // ------------------------
    // Additional optional frames
    // ------------------------

    /// Unsynchronized lyrics (USLT).
    pub unsynchronized_lyric: Option<String>,
    /// Synchronized lyrics (SYLT).
    pub synchronized_lyric: Option<String>,
    /// Original artist (TOPE).
    pub original_artist: Option<String>,
    /// Original album/movie/show title (TOAL).
    pub original_album_movie_show_title: Option<String>,
    /// Original filename (TOFN).
    pub original_filename: Option<String>,
    /// Original lyricist (TOLY).
    pub original_lyricist: Option<String>,
    /// File owner/licensee (TOWN).
    pub file_owner_licensee: Option<String>,
    /// Internet radio station name (TRSN).
    pub internet_radio_station_name: Option<String>,
    /// Internet radio station owner (TRSO).
    pub internet_radio_station_owner: Option<String>,
    /// Official audio file webpage (WOAF).
    pub official_audio_file_webpage: Option<String>,
    /// Official artist webpage (WOAR).
    pub official_artist_webpage: Option<String>,
    /// Official audio source webpage (WOAS).
    pub official_audio_source_webpage: Option<String>,
    /// Official radio station homepage (WORS).
    pub official_internet_radio_station_homepage: Option<String>,
    /// Payment URL (WPAY).
    pub payment: Option<String>,
    /// Publisher's official webpage (WPUB).
    pub publishers_official_webpage: Option<String>,
    /// User-defined URL (WXXX).
    pub user_defined_url_link: Option<String>,
}

/// Displays ID3v2 metadata in a readable format.
impl Display for Id3V2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(title) = &self.title {
            writeln!(f, "Title: {}", title)?;
        }
        if let Some(lead) = &self.lead_performer {
            writeln!(f, "Lead Performer: {}", lead)?;
        }
        if let Some(band) = &self.band_orchestra {
            writeln!(f, "Band/Orchestra: {}", band)?;
        }
        if let Some(subtitle) = &self.subtitle_description_refinement {
            writeln!(f, "Subtitle/Refinement: {}", subtitle)?;
        }
        if let Some(composer) = &self.composer {
            writeln!(f, "Composer: {}", composer)?;
        }
        if let Some(bpm) = &self.bpm {
            writeln!(f, "BPM: {}", bpm)?;
        }
        if let Some(content_type) = &self.content_type {
            writeln!(f, "Content Type: {}", content_type)?;
        }
        if let Some(copyright) = &self.copyright_message {
            writeln!(f, "Copyright: {}", copyright)?;
        }
        if let Some(date) = &self.date {
            writeln!(f, "Date: {}", date)?;
        }
        if let Some(publisher) = &self.publisher {
            writeln!(f, "Publisher: {}", publisher)?;
        }
        if let Some(track) = &self.track_number_position_in_set {
            writeln!(f, "Track Number/Position: {}", track)?;
        }
        if let Some(part_of_set) = &self.part_of_set {
            writeln!(f, "Part of Set: {}", part_of_set)?;
        }
        if let Some(year) = &self.year {
            writeln!(f, "Year: {}", year)?;
        }
        if let Some(user_text) = &self.user_defined_text {
            writeln!(f, "User Defined Text: {}", user_text)?;
        }

        if let Some(picture) = &self.picture {
            writeln!(f, "Picture: {} bytes", picture.len())?;
        }

        for comment in &self.comments {
            writeln!(f, "Comment: {}", comment)?;
        }

        Ok(())
    }
}

// ------------------------
// Parser
// ------------------------

impl Id3 {
    /// Parses ID3 metadata from raw file bytes.
    ///
    /// Automatically detects:
    /// - ID3v2 (header starts with "ID3")
    /// - ID3v1 (footer at end of file)
    pub fn parse(data: &[u8]) -> Result<Self, Id3Error> {
        if data.starts_with(b"ID3") {
            let mut meta = Id3V2::default();
            parse_id3v2(data, &mut meta)?;
            Ok(Id3::V2(meta))
        } else {
            let mut meta = Id3V1::default();
            parse_id3v1(data, &mut meta);
            Ok(Id3::V1(meta))
        }
    }
}

// ------------------------
// ID3v2 parser
// ------------------------

/// Parses ID3v2 frames and fills the provided [`Id3V2`] struct.
///
/// Returns the end position of the tag.
fn parse_id3v2(data: &[u8], meta: &mut Id3V2) -> Result<usize, Id3Error> {
    let mut cur = Cursor::new(data);
    cur.set_position(3);

    // Skip version + flags
    let _version_minor = cur.get_ref()[cur.position() as usize];
    cur.set_position(cur.position() + 2);

    // Syncsafe integer (tag size)
    let size_bytes = &data[6..10];
    let tag_size = ((size_bytes[0] as u32) << 21)
        | ((size_bytes[1] as u32) << 14)
        | ((size_bytes[2] as u32) << 7)
        | (size_bytes[3] as u32);

    let tag_end = 10 + tag_size as usize;
    if tag_end > data.len() { return Ok(0); }

    let tag_data = &data[10..tag_end];
    let mut tag_cur = Cursor::new(tag_data);

    while (tag_cur.position() as usize + 10) <= tag_data.len() {
        let mut id_bytes = [0u8; 4];
        if tag_cur.read_exact(&mut id_bytes).is_err() { break; }

        let mut size_bytes = [0u8; 4];
        if tag_cur.read_exact(&mut size_bytes).is_err() { break; }
        let size = u32::from_be_bytes(size_bytes) as usize;

        // Skip flags
        tag_cur.set_position(tag_cur.position() + 2);

        if size == 0 || (tag_cur.position() as usize + size) > tag_data.len() { break; }

        let mut content = vec![0u8; size];
        if tag_cur.read_exact(&mut content).is_err() { break; }

        let text = decode_text(&content[1..]);

        match &id_bytes {
            b"TIT2" => meta.title = text,
            b"TPE1" => meta.lead_performer = text,
            b"TPE2" => meta.band_orchestra = text,
            b"TIT3" => meta.subtitle_description_refinement = text,
            b"TCOM" => meta.composer = text,
            b"TBPM" => meta.bpm = text,
            b"TCON" => meta.content_type = text,
            b"TCOP" => meta.copyright_message = text,
            b"TDAT" => meta.date = text,
            b"TPUB" => meta.publisher = text,
            b"TRCK" => meta.track_number_position_in_set = text,
            b"TPOS" => meta.part_of_set = text,
            b"TYER" => meta.year = text,
            b"TXXX" => meta.user_defined_text = text,
            b"COMM" => { if let Some(t) = text { meta.comments.push(t); } }
            _ => {}
        }
    }

    Ok(tag_end)
}

// ------------------------
// ID3v1 parser
// ------------------------

/// Parses ID3v1 metadata from the last 128 bytes of a file.
fn parse_id3v1(data: &[u8], meta: &mut Id3V1) {
    if data.len() < 128 { return; }

    let tag = &data[data.len() - 128..];
    if &tag[0..3] != b"TAG" { return; }

    let read_str = |slice: &[u8]| {
        let s = String::from_utf8_lossy(slice)
            .trim_matches('\0')
            .trim()
            .to_string();
        if s.is_empty() { None } else { Some(s) }
    };

    meta.title = read_str(&tag[3..33]);
    meta.artist = read_str(&tag[33..63]);
    meta.album = read_str(&tag[63..93]);
    meta.year = read_str(&tag[93..97]);
    meta.comment = read_str(&tag[97..127]);
    meta.genre = Some(tag[127]);
}

/// Decodes text content from an ID3 frame.
///
/// Currently assumes UTF-8/Latin-1 and strips null terminators.
fn decode_text(data: &[u8]) -> Option<String> {
    if data.is_empty() { return None; }
    Some(String::from_utf8_lossy(data).trim_matches('\0').to_string())
}

/// Error type for ID3 parsing failures.
#[derive(Debug)]
pub struct Id3Error {
    /// Human-readable error message.
    pub message: String,
}