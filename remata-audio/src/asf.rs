use std::io::{Read, Seek, SeekFrom};
use super::{MetaArt, MetaExt};
use crate::ParserError;
use remata_macros::DisplayPretty;
// ------------------------
// GUIDs
// ------------------------

/// ASF Header Object GUID.
/// This must be the first object in a valid ASF file.
const ASF_HEADER_OBJECT: [u8; 16] = [
    0x30,0x26,0xB2,0x75,0x8E,0x66,0xCF,0x11,
    0xA6,0xD9,0x00,0xAA,0x00,0x62,0xCE,0x6C
];

/// ASF Content Description Object GUID.
/// Contains basic metadata like title and author.
const ASF_CONTENT_DESCRIPTION_OBJECT: [u8; 16] = [
    0x33,0x26,0xB2,0x75,0x8E,0x66,0xCF,0x11,
    0xA6,0xD9,0x00,0xAA,0x00,0x62,0xCE,0x6C
];

/// ASF Extended Content Description Object GUID.
/// Contains extended metadata fields (WM/* tags).
const ASF_EXTENDED_CONTENT_DESCRIPTION_OBJECT: [u8; 16] = [
    0x40,0xA4,0xD0,0xD2,0x07,0xE3,0xD2,0x11,
    0x97,0xF0,0x00,0xA0,0xC9,0x5E,0xA8,0x50
];

// ------------------------
// Struct (FULL)
// ------------------------

/// Represents parsed metadata from an ASF (Advanced Systems Format) file.
///
/// All fields are optional because ASF metadata is sparse and varies widely
/// between files. Values are filled based on known ASF/WM metadata keys.
#[derive(Default, Clone, DisplayPretty)]
pub struct AsfMeta {
    /// Album artist name.
    pub album_artist: Option<String>,
    /// URL pointing to album cover.
    pub album_cover_url: Option<String>,
    /// Album title.
    pub album_title: Option<String>,
    /// Video aspect ratio X component.
    pub aspect_ratio_x: Option<String>,
    /// Video aspect ratio Y component.
    pub aspect_ratio_y: Option<String>,
    /// URL of the audio file.
    pub audio_file_url: Option<String>,
    /// Source URL of the audio.
    pub audio_source_url: Option<String>,
    /// Primary author/artist.
    pub author: Option<String>,
    /// Author website URL.
    pub author_url: Option<String>,
    /// Raw banner image data.
    pub banner_image_data: Option<Vec<u8>>,
    /// Banner image MIME/type.
    pub banner_image_type: Option<String>,
    /// Banner image URL.
    pub banner_image_url: Option<String>,
    /// Beats per minute.
    pub beats_per_minute: Option<String>,
    /// Bitrate of the media.
    pub bitrate: Option<String>,
    /// Broadcast flag.
    pub broadcast: Option<String>,
    /// Media category.
    pub category: Option<String>,
    /// Codec used.
    pub codec: Option<String>,
    /// Composer name.
    pub composer: Option<String>,
    /// Copyright text.
    pub copyright: Option<String>,
    /// Copyright URL.
    pub copyright_url: Option<String>,
    /// Description or comment.
    pub description: Option<String>,
    /// Director name.
    pub director: Option<String>,
    /// Duration (usually in 100-ns units).
    pub duration: Option<String>,
    /// Encoder name.
    pub encoded_by: Option<String>,
    /// Encoding settings.
    pub encoding_settings: Option<String>,
    /// Encoding timestamp.
    pub encoding_time: Option<String>,
    /// File size.
    pub file_size: Option<String>,
    /// Genre.
    pub genre: Option<String>,
    /// Genre ID.
    pub genre_id: Option<String>,
    /// Indicates arbitrary data streams exist.
    pub has_arbitrary_data_stream: Option<bool>,
    /// Indicates attached images exist.
    pub has_attached_images: Option<bool>,
    /// Indicates audio stream exists.
    pub has_audio: Option<bool>,
    /// Indicates file transfer stream exists.
    pub has_file_transfer_stream: Option<bool>,
    /// Indicates image stream exists.
    pub has_image: Option<bool>,
    /// Indicates script stream exists.
    pub has_script: Option<bool>,
    /// Indicates video stream exists.
    pub has_video: Option<bool>,
    /// International Standard Recording Code.
    pub isrc: Option<String>,
    /// Whether the file is DRM protected.
    pub is_protected: Option<bool>,
    /// Whether the file is trusted.
    pub is_trusted: Option<bool>,
    /// Language code.
    pub language: Option<String>,
    /// Lyrics.
    pub lyrics: Option<String>,
    /// Mood tag.
    pub mood: Option<String>,
    /// Number of frames.
    pub number_of_frames: Option<String>,
    /// Optimal bitrate.
    pub optimal_bitrate: Option<String>,
    /// Original album title.
    pub original_album_title: Option<String>,
    /// Original artist.
    pub original_artist: Option<String>,
    /// Original filename.
    pub original_filename: Option<String>,
    /// Original lyricist.
    pub original_lyricist: Option<String>,
    /// Original release timestamp.
    pub original_release_time: Option<String>,
    /// Original release year.
    pub original_release_year: Option<String>,
    /// Parental rating.
    pub parental_rating: Option<String>,
    /// Reason for parental rating.
    pub parental_rating_reason: Option<String>,
    /// Producer name.
    pub producer: Option<String>,
    /// Promotion URL.
    pub promotion_url: Option<String>,
    /// Content provider.
    pub provider: Option<String>,
    /// Provider copyright.
    pub provider_copyright: Option<String>,
    /// Provider rating.
    pub provider_rating: Option<String>,
    /// Publisher name.
    pub publisher: Option<String>,
    /// Rating.
    pub rating: Option<String>,
    /// Whether the stream is seekable.
    pub seekable: Option<bool>,
    /// Subtitle text.
    pub subtitle: Option<String>,
    /// Subtitle description.
    pub subtitle_description: Option<String>,
    /// Subtitle content ID.
    pub subtitle_content_id: Option<String>,
    /// Generic text field.
    pub text: Option<String>,
    /// Title.
    pub title: Option<String>,
    /// Tool name used for encoding.
    pub tool_name: Option<String>,
    /// Tool version.
    pub tool_version: Option<String>,
    /// Track name.
    pub track: Option<String>,
    /// Track number.
    pub track_number: Option<String>,
    /// User website URL.
    pub user_web_url: Option<String>,
    /// Video frame rate.
    pub video_frame_rate: Option<String>,
    /// Video height.
    pub video_height: Option<String>,
    /// Video width.
    pub video_width: Option<String>,
    /// Writer/author.
    pub writer: Option<String>,
    /// Year or date.
    pub year: Option<String>,
}

impl MetaExt for AsfMeta {
    fn title(&self) -> Option<String> {
        self.title
            .as_ref()
            .or(self.track.as_ref())
            .or(self.original_filename.as_ref())
            .cloned()
    }

    fn artist(&self) -> Option<String> {
        self.album_artist
            .as_ref()
            .or(self.original_artist.as_ref())
            .or(self.composer.as_ref())
            .cloned()
    }

    fn album(&self) -> Option<String> {
        self.album_title
            .as_ref()
            .or(self.original_album_title.as_ref())
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
            .or(self.original_release_year.as_ref())
            .cloned()
    }

    fn art(&self) -> Option<MetaArt> {
        if let Some(image) = self.banner_image_data.as_ref() {
            return Some(MetaArt::Bin(image.clone()));
        }
        if let Some(image) = self.banner_image_url.as_ref() {
            return Some(MetaArt::Url(image.clone()));
        }
        if let Some(image) = self.album_cover_url.as_ref() {
            return Some(MetaArt::Url(image.clone()));
        }
        None
    }
}


// ------------------------
// Parser
// ------------------------

impl AsfMeta {
    /// Parses ASF metadata from a byte slice.
    ///
    /// This function:
    /// - Validates the ASF header
    /// - Iterates over ASF objects
    /// - Extracts content and extended metadata
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut meta = AsfMeta::default();

        let mut guid = [0u8; 16];
        reader.read_exact(&mut guid)?;

        if guid != ASF_HEADER_OBJECT {
            return Err(ParserError::new("Not ASF"));
        }

        let _header_size = read_u64(reader)?;
        let object_count = read_u32(reader)?;
        let _r1 = read_u8(reader)?;
        let _r2 = read_u8(reader)?;

        for _ in 0..object_count {
            let mut obj_guid = [0u8; 16];
            reader.read_exact(&mut obj_guid)?;
            let size = read_u64(reader)?;

            let start = reader.stream_position()?;

            if obj_guid == ASF_CONTENT_DESCRIPTION_OBJECT {
                parse_content_description(reader, &mut meta)?;
            } else if obj_guid == ASF_EXTENDED_CONTENT_DESCRIPTION_OBJECT {
                parse_extended_description(reader, &mut meta)?;
            }

            // Skip to next object
            reader.seek(SeekFrom::Start(start + (size - 24)))?;
        }

        Ok(meta)
    }
}

// ------------------------
// Object Parsers
// ------------------------

/// Parses the Content Description object (basic metadata).
fn parse_content_description<R: Read>(
    reader: &mut R,
    meta: &mut AsfMeta,
) -> Result<(), ParserError> {
    let title_len = read_u16(reader)? as usize;
    let author_len = read_u16(reader)? as usize;
    let _ = read_u16(reader)?;
    let _ = read_u16(reader)?;
    let _ = read_u16(reader)?;

    if title_len > 0 {
        meta.title = Some(read_utf16(reader, title_len)?);
    }
    if author_len > 0 {
        meta.author = Some(read_utf16(reader, author_len)?);
    }

    Ok(())
}

/// Parses the Extended Content Description object (WM/* metadata).
fn parse_extended_description<R: Read + Seek>(
    reader: &mut R,
    meta: &mut AsfMeta,
) -> Result<(), ParserError> {
    let count = read_u16(reader)?;

    for _ in 0..count {
        let name_len = read_u16(reader)? as usize;
        let raw_name = read_utf16(reader, name_len)?;
        let key = normalize_key(&raw_name);

        let value_type = read_u16(reader)?;
        let value_len = read_u16(reader)? as usize;

        match value_type {
            0 => {
                let val = read_utf16(reader, value_len)?;
                set_string(meta, &key, val);
            }
            1 => {
                let mut buf = vec![0u8; value_len];
                reader.read_exact(&mut buf)?;
                set_binary(meta, &key, buf);
            }
            2 => {
                let val = read_u16(reader)? != 0;
                set_bool(meta, &key, val);
            }
            3 => {
                let val = read_u32(reader)?.to_string();
                set_string(meta, &key, val);
            }
            4 => {
                let val = read_u64(reader)?.to_string();
                set_string(meta, &key, val);
            }
            _ => skip_bytes(reader, value_len)?,
        }
    }

    Ok(())
}




// ------------------------
// Mapping
// ------------------------

/// Normalizes ASF metadata keys by removing the "WM/" prefix.
fn normalize_key(k: &str) -> String {
    k.trim_start_matches("WM/").to_string()
}

/// Maps string values into the appropriate [`AsfMeta`] field.
fn set_string(m: &mut AsfMeta, k: &str, v: String) {
    match k {
        "AlbumArtist" => m.album_artist = Some(v),
        "AlbumCoverURL" => m.album_cover_url = Some(v),
        "AlbumTitle" => m.album_title = Some(v),
        "AspectRatioX" => m.aspect_ratio_x = Some(v),
        "AspectRatioY" => m.aspect_ratio_y = Some(v),
        "AudioFileURL" => m.audio_file_url = Some(v),
        "AudioSourceURL" => m.audio_source_url = Some(v),
        "Author" => m.author = Some(v),
        "AuthorURL" => m.author_url = Some(v),
        "BannerImageType" => m.banner_image_type = Some(v),
        "BannerImageURL" => m.banner_image_url = Some(v),
        "BeatsPerMinute" => m.beats_per_minute = Some(v),
        "Bitrate" => m.bitrate = Some(v),
        "Broadcast" => m.broadcast = Some(v),
        "Category" => m.category = Some(v),
        "Codec" => m.codec = Some(v),
        "Composer" => m.composer = Some(v),
        "Copyright" => m.copyright = Some(v),
        "CopyrightURL" => m.copyright_url = Some(v),
        "Description" => m.description = Some(v),
        "Director" => m.director = Some(v),
        "Duration" => m.duration = Some(v),
        "EncodedBy" => m.encoded_by = Some(v),
        "EncodingSettings" => m.encoding_settings = Some(v),
        "EncodingTime" => m.encoding_time = Some(v),
        "FileSize" => m.file_size = Some(v),
        "Genre" => m.genre = Some(v),
        "GenreID" => m.genre_id = Some(v),
        "ISRC" => m.isrc = Some(v),
        "Language" => m.language = Some(v),
        "Lyrics" => m.lyrics = Some(v),
        "Mood" => m.mood = Some(v),
        "NumberOfFrames" => m.number_of_frames = Some(v),
        "OptimalBitrate" => m.optimal_bitrate = Some(v),
        "OriginalAlbumTitle" => m.original_album_title = Some(v),
        "OriginalArtist" => m.original_artist = Some(v),
        "OriginalFileName" => m.original_filename = Some(v),
        "OriginalLyricist" => m.original_lyricist = Some(v),
        "OriginalReleaseTime" => m.original_release_time = Some(v),
        "OriginalReleaseYear" => m.original_release_year = Some(v),
        "ParentalRating" => m.parental_rating = Some(v),
        "ParentalRatingReason" => m.parental_rating_reason = Some(v),
        "Producer" => m.producer = Some(v),
        "PromotionURL" => m.promotion_url = Some(v),
        "Provider" => m.provider = Some(v),
        "ProviderCopyright" => m.provider_copyright = Some(v),
        "ProviderRating" => m.provider_rating = Some(v),
        "Publisher" => m.publisher = Some(v),
        "Rating" => m.rating = Some(v),
        "Subtitle" => m.subtitle = Some(v),
        "SubtitleDescription" => m.subtitle_description = Some(v),
        "SubscriptionContentID" => m.subtitle_content_id = Some(v),
        "Text" => m.text = Some(v),
        "Title" => m.title = Some(v),
        "ToolName" => m.tool_name = Some(v),
        "ToolVersion" => m.tool_version = Some(v),
        "Track" => m.track = Some(v),
        "TrackNumber" => m.track_number = Some(v),
        "UserWebURL" => m.user_web_url = Some(v),
        "VideoFrameRate" => m.video_frame_rate = Some(v),
        "VideoHeight" => m.video_height = Some(v),
        "VideoWidth" => m.video_width = Some(v),
        "Writer" => m.writer = Some(v),
        "Year" => m.year = Some(v),
        _ => {}
    }
}

/// Maps boolean values into the appropriate [`AsfMeta`] field.
fn set_bool(m: &mut AsfMeta, k: &str, v: bool) {
    match k {
        "HasArbitraryDataStream" => m.has_arbitrary_data_stream = Some(v),
        "HasAttachedImages" => m.has_attached_images = Some(v),
        "HasAudio" => m.has_audio = Some(v),
        "HasFileTransferStream" => m.has_file_transfer_stream = Some(v),
        "HasImage" => m.has_image = Some(v),
        "HasScript" => m.has_script = Some(v),
        "HasVideo" => m.has_video = Some(v),
        "Is_Protected" => m.is_protected = Some(v),
        "Is_Trusted" => m.is_trusted = Some(v),
        "Seekable" => m.seekable = Some(v),
        _ => {}
    }
}

/// Maps binary values into the appropriate [`AsfMeta`] field.
fn set_binary(m: &mut AsfMeta, k: &str, v: Vec<u8>) {
    match k {
        "BannerImageData" => m.banner_image_data = Some(v),
        _ => {}
    }
}

// ------------------------
// Helpers
// ------------------------
/// Reads a single byte.
fn read_u8<R: Read>(r: &mut R) -> Result<u8, ParserError> {
    let mut b = [0; 1];
    r.read_exact(&mut b)?;
    Ok(b[0])
}

/// Reads a little-endian `u16`.
fn read_u16<R: Read>(r: &mut R) -> Result<u16, ParserError> {
    let mut b = [0; 2];
    r.read_exact(&mut b)?;
    Ok(u16::from_le_bytes(b))
}

/// Reads a little-endian `u32`.
fn read_u32<R: Read>(r: &mut R) -> Result<u32, ParserError> {
    let mut b = [0; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

/// Reads a little-endian `u64`.
fn read_u64<R: Read>(r: &mut R) -> Result<u64, ParserError> {
    let mut b = [0; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}

/// Reads a UTF-16LE string of a given byte length.
fn read_utf16<R: Read>(r: &mut R, len: usize) -> Result<String, ParserError> {
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;

    let mut out = Vec::new();
    for ch in buf.chunks_exact(2) {
        let v = u16::from_le_bytes([ch[0], ch[1]]);
        if v == 0 { break; }
        out.push(v);
    }

    Ok(String::from_utf16_lossy(&out))
}
/// Skips a number of bytes in the cursor.
fn skip_bytes<R: Read + Seek>(r: &mut R, len: usize) -> Result<(), ParserError> {
    r.seek(SeekFrom::Current(len as i64))?;
    Ok(())
}




/*
ASF ExtendedDescr Tags

AlbumArtist
AlbumCoverUrl
AlbumTitle
AspectRatioX
AspectRatioY
AudioFileUrl
AudioSourceUrl
Author
AuthorUrl
BannerImageData
BannerImageType
BannerImageUrl
BeatsPerMinute
Bitrate
Broadcast
Category
Codec
Composer
Copyright
CopyrightUrl
Description
Director
Duration
EncodedBy
EncodingSettings
EncodingTime
FileSize
Genre
GenreId
HasArbitraryDataStream
HasAttachedImages
HasAudio
HasFileTransferStream
HasImage
HasScript
HasVideo
ISRC
Is_Protected
Is_Trusted
Language
Lyrics
Mood
NumberOfFrames
OptimalBitrate
OriginalAlbumTitle
OriginalArtist
OriginalFilename
OriginalLyricist
OriginalReleaseTime
OriginalReleaseYear
ParentalRating
ParentalRatingReason
Producer
PromotionURL
Provider
ProviderCopyright
ProviderRating
Publisher
Rating
Seekable
Subtitle
SubtitleDescription
SubtitleContentId
Text
Title
ToolName
ToolVersion
Track
TrackNumber
UserWebUrl
VideoFrameRate
VideoHeight
VideoWidth
Writer
Year
 */