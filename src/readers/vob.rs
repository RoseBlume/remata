use std::fmt;

/// Represents metadata extracted from FLAC or Ogg/Vorbis containers.
///
/// This struct is a normalized view over Vorbis-style comments, mapping
/// common tag names into strongly-typed Rust fields.
///
/// Many fields overlap semantically (e.g., `artist` vs `performer`)
/// because different encoders use different tag conventions.
#[derive(Default, Debug)]
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

impl fmt::Display for Vob {
    /// Formats metadata into a human-readable form.
    ///
    /// Only fields that are present (`Some`) are displayed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.title {
            writeln!(f, "Title: {}", title)?;
        }
        if let Some(artist) = &self.artist {
            writeln!(f, "Artist: {}", artist)?;
        }
        if let Some(album) = &self.album {
            writeln!(f, "Album: {}", album)?;
        }
        if let Some(album_artist) = &self.album_artist {
            writeln!(f, "Album Artist: {}", album_artist)?;
        }
        if let Some(genre) = &self.genre {
            writeln!(f, "Genre: {}", genre)?;
        }
        if let Some(track_number) = self.track_number {
            writeln!(f, "Track Number: {}", track_number)?;
        }
        if let Some(year) = self.year {
            writeln!(f, "Year: {}", year)?;
        }
        if let Some(comment) = &self.comment {
            writeln!(f, "Comment: {}", comment)?;
        }
        if let Some(cover_art) = &self.cover_art {
            writeln!(f, "Cover Art: {} bytes", cover_art.len())?;
        }
        if let Some(mime) = &self.cover_art_mime {
            writeln!(f, "Cover Art MIME: {}", mime)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "Description: {}", description)?;
        }
        if let Some(director) = &self.director {
            writeln!(f, "Director: {}", director)?;
        }
        if let Some(encoded_by) = &self.encoded_by {
            writeln!(f, "Encoded By: {}", encoded_by)?;
        }
        if let Some(encoded_using) = &self.encoded_using {
            writeln!(f, "Encoded Using: {}", encoded_using)?;
        }
        if let Some(encoder) = &self.encoder {
            writeln!(f, "Encoder: {}", encoder)?;
        }
        if let Some(encoder_options) = &self.encoder_options {
            writeln!(f, "Encoder Options: {}", encoder_options)?;
        }
        if let Some(isrc_number) = &self.isrc_number {
            writeln!(f, "ISRC Number: {}", isrc_number)?;
        }
        if let Some(license) = &self.license {
            writeln!(f, "License: {}", license)?;
        }
        if let Some(location) = &self.location {
            writeln!(f, "Location: {}", location)?;
        }
        if let Some(organization) = &self.organization {
            writeln!(f, "Organization: {}", organization)?;
        }
        if let Some(performer) = &self.performer {
            writeln!(f, "Performer: {}", performer)?;
        }
        if let Some(producer) = &self.producer {
            writeln!(f, "Producer: {}", producer)?;
        }
        if let Some(replaygain_album_gain) = &self.replaygain_album_gain {
            writeln!(f, "ReplayGain Album Gain: {}", replaygain_album_gain)?;
        }
        if let Some(replaygain_album_peak) = &self.replaygain_album_peak {
            writeln!(f, "ReplayGain Album Peak: {}", replaygain_album_peak)?;
        }
        if let Some(replaygain_track_gain) = &self.replaygain_track_gain {
            writeln!(f, "ReplayGain Track Gain: {}", replaygain_track_gain)?;
        }
        if let Some(replaygain_track_peak) = &self.replaygain_track_peak {
            writeln!(f, "ReplayGain Track Peak: {}", replaygain_track_peak)?;
        }
        if let Some(version) = &self.version {
            writeln!(f, "Version: {}", version)?;
        }
        if let Some(vendor) = &self.vendor {
            writeln!(f, "Vendor: {}", vendor)?;
        }

        Ok(())
    }
}

impl Vob {
    /// Parses metadata from raw audio data.
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
    /// let meta = Vob::parse(&bytes)?;
    /// println!("{}", meta);
    /// ```
    pub fn parse(data: &[u8]) -> Result<Self, VobError> {
        if data.starts_with(b"fLaC") {
            parse_flac(data)
        } else if data.starts_with(b"OggS") {
            parse_ogg(data)
        } else {
            Err(VobError { message: "Unsupported audio format".to_string() })
        }
    }
}

/// Parses FLAC metadata blocks and extracts Vorbis comments.
fn parse_flac(data: &[u8]) -> Result<Vob, VobError> {
    let mut offset = 4;
    let mut vob = Vob::default();

    loop {
        if offset + 4 > data.len() {
            return Err(VobError { message: "Unexpected EOF in FLAC metadata".to_string() });
        }

        let header = data[offset];
        let last = header & 0x80 != 0;
        let block_type = header & 0x7F;
        let block_size = ((data[offset+1] as u32) << 16)
            | ((data[offset+2] as u32) << 8)
            | data[offset+3] as u32;

        offset += 4;

        // Block type 4 = Vorbis comment
        if block_type == 4 {
            let block_data = &data[offset..offset + block_size as usize];
            parse_vorbis_comments(block_data, &mut vob)?;
        }

        offset += block_size as usize;
        if last { break; }
    }

    Ok(vob)
}

/// Parses Ogg container pages and extracts Vorbis comment packets.
fn parse_ogg(data: &[u8]) -> Result<Vob, VobError> {
    let mut offset = 0;
    let mut vob = Vob::default();

    while offset + 27 <= data.len() {
        if &data[offset..offset+4] != b"OggS" {
            break;
        }

        let segment_count = data[offset + 26] as usize;
        let header_size = 27 + segment_count;

        if offset + header_size > data.len() {
            break;
        }

        let segment_table = &data[offset+27..offset+header_size];
        let mut packet_data = Vec::new();

        for &seg_len in segment_table {
            let seg_len = seg_len as usize;
            offset += header_size;
            packet_data.extend_from_slice(&data[offset..offset+seg_len]);
            offset += seg_len;
        }

        // Vorbis comment packet identifier
        if packet_data.starts_with(&[0x03]) && &packet_data[1..7] == b"vorbis" {
            parse_vorbis_comments(&packet_data[7..], &mut vob)?;
            break;
        }
    }

    Ok(vob)
}

/// Parses Vorbis comment block and populates [`Vob`].
///
/// Comments are in `KEY=VALUE` format.
fn parse_vorbis_comments(block: &[u8], vob: &mut Vob) -> Result<(), VobError> {
    let mut offset = 0;

    if offset + 4 > block.len() { return Ok(()); }

    // Vendor string
    let vendor_len = u32::from_le_bytes(block[offset..offset+4].try_into().unwrap()) as usize;
    offset += 4 + vendor_len;

    if offset + 4 > block.len() { return Ok(()); }

    let count = u32::from_le_bytes(block[offset..offset+4].try_into().unwrap()) as usize;
    offset += 4;

    for _ in 0..count {
        if offset + 4 > block.len() { break; }

        let len = u32::from_le_bytes(block[offset..offset+4].try_into().unwrap()) as usize;
        offset += 4;

        if offset + len > block.len() { break; }

        let comment = std::str::from_utf8(&block[offset..offset+len])
            .map_err(|e| VobError { message: format!("UTF-8 error: {}", e) })?;
        offset += len;

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

/// Error type for Vob parsing.
#[derive(Debug)]
pub struct VobError {
    /// Human-readable error message.
    pub message: String,
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
