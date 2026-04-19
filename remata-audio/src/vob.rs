use crate::ParserError;
use std::io::{Read, Seek, SeekFrom};
use super::{MetaArt, MetaExt};
use remata_macros::DisplayPretty;
/// Represents metadata extracted from FLAC or Ogg/Vorbis containers.
///
/// This struct is a normalized view over Vorbis-style comments, mapping
/// common tag names into strongly-typed Rust fields.
///
/// Many fields overlap semantically (e.g., `artist` vs `performer`)
/// because different encoders use different tag conventions.
#[derive(Default, Clone, DisplayPretty)]
pub struct Vob {
    /// Track title.
    pub title: Option<String>,

    /// Primary artist or performer.
    pub artist: Option<String>,

    /// Album name.
    pub album: Option<String>,

    /// Album artist or producer.
    pub album_artist: Option<String>,

    /// Genre classification.
    pub genre: Option<String>,

    /// Track number within the album.
    pub track_number: Option<u32>,

    /// Release year or recording date.
    pub year: Option<u32>,

    /// Free-form comment.
    pub comment: Option<String>,

    /// Embedded cover art (decoded from base64).
    pub cover_art: Option<Vec<u8>>,

    /// MIME type of the cover art.
    pub cover_art_mime: Option<String>,

    /// Description or license text.
    pub description: Option<String>,

    /// Director (used in video/audio hybrids).
    pub director: Option<String>,

    /// Encoder author.
    pub encoded_by: Option<String>,

    /// Encoding method/toolchain.
    pub encoded_using: Option<String>,

    /// Encoder name.
    pub encoder: Option<String>,

    /// Encoder configuration/options.
    pub encoder_options: Option<String>,

    /// ISRC identifier.
    pub isrc_number: Option<String>,

    /// License information.
    pub license: Option<String>,

    /// Recording location.
    pub location: Option<String>,

    /// Organization or label.
    pub organization: Option<String>,

    /// Performer (alternate artist field).
    pub performer: Option<String>,

    /// Producer.
    pub producer: Option<String>,

    /// ReplayGain album gain value.
    pub replaygain_album_gain: Option<String>,

    /// ReplayGain album peak value.
    pub replaygain_album_peak: Option<String>,

    /// ReplayGain track gain value.
    pub replaygain_track_gain: Option<String>,

    /// ReplayGain track peak value.
    pub replaygain_track_peak: Option<String>,

    /// Metadata/version tag.
    pub version: Option<String>,

    /// Vendor string from encoder.
    pub vendor: Option<String>,
}

impl MetaExt for Vob {
    fn title(&self) -> Option<String> {
        self.title
            .as_ref()
            .cloned()
    }

    fn artist(&self) -> Option<String> {
        self.artist
            .as_ref()
            .or(self.performer.as_ref())
            .or(self.album_artist.as_ref())
            .or(self.producer.as_ref())
            .cloned()
    }

    fn album(&self) -> Option<String> {
        self.album
            .as_ref()
            .or(self.organization.as_ref())
            .cloned()
    }

    fn genre(&self) -> Option<String> {
        self.genre
            .as_ref()
            .cloned()
    }

    fn year(&self) -> Option<String> {
        self.year
            .map(|y| y.to_string())
    }

    fn art(&self) -> Option<MetaArt> {
        if let Some(data) = self.cover_art.as_ref() {
            return Some(MetaArt::Bin(data.clone()));
        }
        None
    }
}



impl Vob {
    /// Parses metadata from a reader.
    ///
    /// Automatically detects:
    /// - FLAC (`fLaC`)
    /// - Ogg/Vorbis (`OggS`)
    ///
    /// # Errors
    /// Returns an error if the format is unsupported or malformed.
    ///
    /// # Example
    /// ```
    /// let mut file = std::fs::File::open("audio.flac")?;
    /// let meta = Vob::parse(&mut file)?;
    /// println!("{}", meta);
    /// ```
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;
        reader.seek(SeekFrom::Start(0))?;

        if &header == b"fLaC" {
            parse_flac(reader)
        } else if &header == b"OggS" {
            parse_ogg(reader)
        } else {
            Err(ParserError { message: "Unsupported audio format".to_string() })
        }
    }
}

/// Parses FLAC metadata blocks and extracts Vorbis comments.
fn parse_flac<R: Read + Seek>(reader: &mut R) -> Result<Vob, ParserError> {
    let mut vob = Vob::default();

    // Skip "fLaC"
    reader.seek(SeekFrom::Start(4))?;

    loop {
        let mut header = [0u8; 4];
        if reader.read_exact(&mut header).is_err() {
            return Err(ParserError { message: "Unexpected EOF in FLAC metadata".to_string() });
        }

        let last = header[0] & 0x80 != 0;
        let block_type = header[0] & 0x7F;
        let block_size =
            ((header[1] as u32) << 16) |
            ((header[2] as u32) << 8) |
            (header[3] as u32);

        let data_start = reader.stream_position()?;

        // Block type 4 = Vorbis comment
        if block_type == 4 {
            let mut limited = reader.take(block_size as u64);
            parse_vorbis_comments(&mut limited, &mut vob)?;
        }

        // Move to next block
        reader.seek(SeekFrom::Start(data_start + block_size as u64))?;

        if last {
            break;
        }
    }

    Ok(vob)
}

/// Parses Ogg container pages and extracts Vorbis comment packets.
fn parse_ogg<R: Read + Seek>(reader: &mut R) -> Result<Vob, ParserError> {
    let mut vob = Vob::default();

    loop {
        let mut header = [0u8; 27];
        if reader.read_exact(&mut header).is_err() {
            break;
        }

        if &header[0..4] != b"OggS" {
            break;
        }

        let segment_count = header[26] as usize;

        let mut segment_table = vec![0u8; segment_count];
        reader.read_exact(&mut segment_table)?;

        let mut packet_data = Vec::new();

        for seg_len in segment_table {
            let mut buf = vec![0u8; seg_len as usize];
            reader.read_exact(&mut buf)?;
            packet_data.extend_from_slice(&buf);
        }

        // Vorbis comment packet identifier
        if packet_data.starts_with(&[0x03]) && &packet_data[1..7] == b"vorbis" {
            let mut cursor = std::io::Cursor::new(&packet_data[7..]);
            parse_vorbis_comments(&mut cursor, &mut vob)?;
            break;
        }
    }

    Ok(vob)
}

/// Reads a little-endian `u32`.
fn read_u32_le<R: Read>(reader: &mut R) -> Result<u32, ParserError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Parses Vorbis comment block and populates [`Vob`].
///
/// Comments are in `KEY=VALUE` format.
fn parse_vorbis_comments<R: Read>(
    reader: &mut R,
    vob: &mut Vob
) -> Result<(), ParserError> {
    // Vendor string
    let vendor_len = read_u32_le(reader)? as usize;

    let mut vendor_buf = vec![0u8; vendor_len];
    reader.read_exact(&mut vendor_buf)?;
    vob.vendor = Some(String::from_utf8_lossy(&vendor_buf).to_string());

    let count = read_u32_le(reader)? as usize;

    for _ in 0..count {
        let len = read_u32_le(reader)? as usize;

        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;

        let comment = std::str::from_utf8(&buf)
            .map_err(|e| ParserError { message: format!("UTF-8 error: {}", e) })?;

        if let Some((tag, value)) = comment.split_once('=') {
            match tag.to_uppercase().as_str() {
                "TITLE" => vob.title = Some(value.to_string()),
                "ARTIST" => vob.artist = Some(value.to_string()),
                "ALBUM" => vob.album = Some(value.to_string()),
                "ALBUM_ARTIST" => vob.album_artist = Some(value.to_string()),
                "GENRE" => vob.genre = Some(value.to_string()),
                "TRACKNUMBER" => vob.track_number = value.parse().ok(),
                "DATE" => vob.year = value.parse().ok(),
                "COMMENT" => vob.comment = Some(value.to_string()),
                "COVERART" => vob.cover_art = decode_base64(value),
                "COVERARTMIME" => vob.cover_art_mime = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Ok(())
}

/// Minimal base64 decoder (standard alphabet).
///
/// Returns `None` if invalid characters are encountered.
fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;

    for c in input.chars() {
        let val = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' => 62,
            '/' => 63,
            '=' => continue,
            _ => return None,
        };

        buffer = (buffer << 6) | val;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            result.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }

    Some(result)
}


/*
Tag ID	Tag Name	Writable	Values / Notes
'ACTOR'	Actor	no
'ALBUM'	Album	no
'ARTIST'	Artist	no+
'COMMENT'	Comment	no
'COMPOSER'	Composer	no
'CONTACT'	Contact	no+
'COPYRIGHT'	Copyright	no
'COVERART'	CoverArt	no	(base64-encoded image)
'COVERARTMIME'	CoverArtMIMEType	no
'DATE'	Date	no
'DESCRIPTION'	Description	no
'DIRECTOR'	Director	no
'ENCODED_BY'	EncodedBy	no
'ENCODED_USING'	EncodedUsing	no
'ENCODER'	Encoder	no
'ENCODER_OPTIONS'	EncoderOptions	no
'GENRE'	Genre	no
'ISRC'	ISRCNumber	no
'LICENSE'	License	no
'LOCATION'	Location	no
'METADATA_BLOCK_PICTURE'	Picture	-	--> FLAC Picture Tags
'ORGANIZATION'	Organization	no
'PERFORMER'	Performer	no+
'PRODUCER'	Producer	no
'REPLAYGAIN_ALBUM_GAIN'	ReplayGainAlbumGain	no
'REPLAYGAIN_ALBUM_PEAK'	ReplayGainAlbumPeak	no
'REPLAYGAIN_TRACK_GAIN'	ReplayGainTrackGain	no
'REPLAYGAIN_TRACK_PEAK'	ReplayGainTrackPeak	no
'TITLE'	Title	no
'TRACKNUMBER'	TrackNumber	no
'VERSION'	Version	no
'vendor'	Vendor	no	(from comment header)

*/
