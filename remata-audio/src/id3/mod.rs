mod genre;
use genre::Genre;
use remata_macros::DisplayPretty;
use std::io::{Read, Seek, SeekFrom};

use crate::ParserError;

use super::{MetaArt, MetaExt};
/// Represents either an ID3v1 or ID3v2 metadata block.
///
/// This enum abstracts over the two major ID3 versions:
/// - ID3v1: fixed-size footer at end of file
/// - ID3v2: flexible, frame-based metadata at the beginning
#[derive(Clone, DisplayPretty)]
pub enum Id3 {
    /// ID3v1 metadata (128-byte footer)
    V1(Id3V1),
    /// ID3v2 metadata (frame-based header)
    V2(Id3V2),
}

// /// Displays the underlying metadata regardless of version.
// impl Display for Id3 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         match self {
//             Id3::V1(meta) => write!(f, "{}", meta),
//             Id3::V2(meta) => write!(f, "{}", meta),
//         }
//     }
// }

impl MetaExt for Id3 {
    fn title(&self) -> Option<String> {
        match self {
            Id3::V1(v1) => v1.title(),
            Id3::V2(v2) => v2.title(),
        }
    }

    fn artist(&self) -> Option<String> {
        match self {
            Id3::V1(v1) => v1.artist(),
            Id3::V2(v2) => v2.artist(),
        }
    }

    fn album(&self) -> Option<String> {
        match self {
            Id3::V1(v1) => v1.album(),
            Id3::V2(v2) => v2.album(),
        }
    }

    fn genre(&self) -> Option<String> {
        match self {
            Id3::V1(v1) => v1.genre(),
            Id3::V2(v2) => v2.genre(),
        }
    }

    fn year(&self) -> Option<String> {
        match self {
            Id3::V1(v1) => v1.year(),
            Id3::V2(v2) => v2.year(),
        }
    }

    fn art(&self) -> Option<MetaArt> {
        match self {
            Id3::V1(v1) => v1.art(),
            Id3::V2(v2) => v2.art(),
        }
    }
}



/// Represents ID3v1 metadata.
///
/// ID3v1 is a simple fixed-length format located at the last 128 bytes
/// of an MP3 file.
#[derive(Default, Clone, DisplayPretty)]
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
    pub genre: Option<String>,
}
impl Id3V1 {
    /// Parses ID3v1 metadata from the last 128 bytes of a file.
    pub fn parse<R: Read + Seek>(&mut self, reader: &mut R) -> Result<(), ParserError> {
        let file_size = reader.seek(SeekFrom::End(0))?;

        if file_size < 128 {
            return Ok(());
        }

        reader.seek(SeekFrom::End(-128))?;

        let mut tag = [0u8; 128];
        reader.read_exact(&mut tag)?;

        if &tag[0..3] != b"TAG" {
            return Ok(());
        }

        let read_str = |slice: &[u8]| {
            let s = String::from_utf8_lossy(slice)
                .trim_matches('\0')
                .trim()
                .to_string();
            if s.is_empty() { None } else { Some(s) }
        };

        self.title = read_str(&tag[3..33]);
        self.artist = read_str(&tag[33..63]);
        self.album = read_str(&tag[63..93]);
        self.year = read_str(&tag[93..97]);
        self.comment = read_str(&tag[97..127]);
        self.genre = Some(Genre::from_u8(tag[127]).as_str().to_string());

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
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut header = [0u8; 3];
        reader.read_exact(&mut header)?;

        reader.seek(SeekFrom::Start(0))?;

        if &header == b"ID3" {
            let mut meta = Id3V2::default();
            meta.parse(reader)?;
            Ok(Id3::V2(meta))
        } else {
            let mut meta = Id3V1::default();
            meta.parse(reader)?;
            Ok(Id3::V1(meta))
        }
    }
}



/// Represents ID3v2 metadata.
///
/// ID3v2 uses a flexible frame-based structure where each field is
/// identified by a 4-byte frame ID (e.g., TIT2, TPE1).
#[derive(Default, Clone, DisplayPretty)]
pub struct Id3V2 {
    /// Grouping (TIT1).
    pub grouping: Option<String>,
    /// Title/song name (TIT2).
    pub title: Option<String>,
    /// Subtitle (TIT3).
    pub subtitle: Option<String>,

    /// Lead performer/artist (TPE1).
    pub lead_performer: Option<String>,
    /// Band/orchestra/accompaniment (TPE2).
    pub band_orchestra: Option<String>,
    /// Conductor (TPE3).
    pub conductor: Option<String>,
    /// InterpretedBy (TPE4).
    pub interpreted_by: Option<String>,
    /// Album Title (TALB)
    pub album_title: Option<String>,
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
    /// Genre ID (TCON).
    pub genre: Option<String>,

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

impl Id3V2 {
    /// Parses ID3v2 frames and fills the provided [`Id3V2`] struct.
    ///
    /// Returns the end position of the tag.
    pub fn parse<R: Read + Seek>(
        &mut self,
        reader: &mut R,
    ) -> Result<usize, ParserError> {
        // ------------------------
        // Read header (10 bytes)
        // ------------------------
        let mut header = [0u8; 10];
        reader.read_exact(&mut header)?;

        if &header[0..3] != b"ID3" {
            return Ok(0);
        }

        // version + flags already in header[3..6]

        // ------------------------
        // Syncsafe size
        // ------------------------
        let size_bytes = &header[6..10];
        let tag_size = ((size_bytes[0] as u32) << 21)
            | ((size_bytes[1] as u32) << 14)
            | ((size_bytes[2] as u32) << 7)
            | (size_bytes[3] as u32);

        let tag_end = 10 + tag_size as u64;
        let start_pos = reader.stream_position()?;

        // ------------------------
        // Frame loop
        // ------------------------
        while reader.stream_position()? < start_pos + tag_size as u64 {
            // Ensure enough bytes remain for a frame header
            if (start_pos + tag_size as u64) - reader.stream_position()? < 10 {
                break;
            }

            let mut id_bytes = [0u8; 4];
            if reader.read_exact(&mut id_bytes).is_err() {
                break;
            }

            let mut size_bytes = [0u8; 4];
            if reader.read_exact(&mut size_bytes).is_err() {
                break;
            }

            let size = u32::from_be_bytes(size_bytes) as usize;

            // Skip flags
            reader.seek(SeekFrom::Current(2))?;

            if size == 0 {
                break;
            }

            // Prevent reading past tag boundary
            let remaining = (start_pos + tag_size as u64) - reader.stream_position()?;
            if size as u64 > remaining {
                break;
            }

            // ------------------------
            // Read frame content
            // ------------------------
            let mut content = vec![0u8; size];
            if reader.read_exact(&mut content).is_err() {
                break;
            }

            match &id_bytes {
                // ------------------------
                // TEXT FRAMES
                // ------------------------
                b"TIT1" => self.grouping = decode_text(&content[1..]),
                b"TIT2" => self.title = decode_text(&content[1..]),
                b"TALB" => self.album_title = decode_text(&content[1..]),
                b"TPE1" => self.lead_performer = decode_text(&content[1..]),
                b"TPE2" => self.band_orchestra = decode_text(&content[1..]),
                b"TPE3" => self.conductor = decode_text(&content[1..]),
                b"TPE4" => self.interpreted_by = decode_text(&content[1..]),
                b"TIT3" => self.subtitle_description_refinement = decode_text(&content[1..]),
                b"TCOM" => self.composer = decode_text(&content[1..]),
                b"TBPM" => self.bpm = decode_text(&content[1..]),
                b"TCOP" => self.copyright_message = decode_text(&content[1..]),
                b"TDAT" => self.date = decode_text(&content[1..]),
                b"TPUB" => self.publisher = decode_text(&content[1..]),
                b"TRCK" => self.track_number_position_in_set = decode_text(&content[1..]),
                b"TPOS" => self.part_of_set = decode_text(&content[1..]),
                b"TYER" => self.year = decode_text(&content[1..]),
                b"TXXX" => self.user_defined_text = decode_text(&content[1..]),

                // ------------------------
                // GENRE (TCON)
                // ------------------------
                b"TCON" => {
                    if let Some(text) = decode_text(&content[1..]) {
                        let parsed = text
                            .trim_matches(|c| c == '(' || c == ')')
                            .parse::<u8>()
                            .ok();

                        if let Some(val) = parsed {
                            self.genre = Some(Genre::from_u8(val).as_str().to_string());
                        } else {
                            self.genre = Some(text);
                        }
                    }
                }

                // ------------------------
                // COMMENTS
                // ------------------------
                b"COMM" => {
                    if let Some(t) = decode_text(&content[1..]) {
                        self.comments.push(t);
                    }
                }

                // ------------------------
                // APIC (Attached Picture)
                // ------------------------
                b"APIC" => {
                    let mut i = 1; // skip encoding byte

                    // MIME type
                    while i < content.len() && content[i] != 0 {
                        i += 1;
                    }
                    i += 1;

                    if i >= content.len() { continue; }

                    // Picture type
                    i += 1;

                    if i >= content.len() { continue; }

                    // Description
                    while i < content.len() && content[i] != 0 {
                        i += 1;
                    }
                    i += 1;

                    if i >= content.len() { continue; }

                    self.picture = Some(content[i..].to_vec());
                }

                _ => {}
            }
        }

        Ok(tag_end as usize)
    }
}

impl MetaExt for Id3V2 {
    fn title(&self) -> Option<String> {
        self.title
            .as_ref()
            .or(self.original_filename.as_ref())
            .cloned()
    }

    fn artist(&self) -> Option<String> {
        self.lead_performer
            .as_ref()
            .or(self.band_orchestra.as_ref())
            .or(self.original_artist.as_ref())
            .or(self.composer.as_ref())
            .or(self.conductor.as_ref())
            .or(self.interpreted_by.as_ref())
            .cloned()
    }

    fn album(&self) -> Option<String> {
        self.album_title
            .as_ref()
            .or(self.original_album_movie_show_title.as_ref())
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
        if let Some(image) = self.picture.as_ref() {
            return Some(MetaArt::Bin(image.to_vec()));
        }
        None
    }
}

// ------------------------
// ID3v1 parser
// ------------------------


impl MetaExt for Id3V1 {
    fn title(&self) -> Option<String> {
        self.title.clone()
    }

    fn artist(&self) -> Option<String> {
        self.artist.clone()
    }

    fn album(&self) -> Option<String> {
        self.album.clone()
    }

    fn genre(&self) -> Option<String> {
        self.genre.clone()
    }

    fn year(&self) -> Option<String> {
        self.year.clone()
    }

    fn art(&self) -> Option<MetaArt> {
        None
    }
}

/// Decodes text content from an ID3 frame.
///
/// Currently assumes UTF-8/Latin-1 and strips null terminators.
fn decode_text(data: &[u8]) -> Option<String> {
    if data.is_empty() { return None; }
    Some(String::from_utf8_lossy(data).trim_matches('\0').to_string())
}





