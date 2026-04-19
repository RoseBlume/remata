use std::io::{self, Read, Seek};
use remata_macros::DisplayPretty;


/// Reads a 16-bit unsigned integer in little-endian format from the reader.
fn read_u16<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut buf = [0; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Reads a 32-bit unsigned integer in little-endian format from the reader.
fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Reads a fixed-length string from the reader.
///
/// The string is interpreted as UTF-8 (lossy), which allows recovery of
/// partially invalid text commonly found in metadata fields.
fn read_string<R: Read>(r: &mut R, len: usize) -> io::Result<String> {
    let mut buf = vec![0; len];
    r.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

/// Represents the header structure used in newer APE (Monkey's Audio) files.
///
/// This format is used in more recent versions and includes detailed
/// information about frame layout and audio properties.
#[derive(Clone, Default, DisplayPretty)]
pub struct ApeHeaderNew {
    /// Compression level used during encoding.
    pub compression_level: Option<u16>,

    /// Number of audio blocks per frame.
    pub blocks_per_frame: Option<u32>,

    /// Number of blocks in the final frame.
    ///
    /// This is often smaller than `blocks_per_frame`.
    pub final_frame_blocks: Option<u32>,

    /// Total number of frames in the audio stream.
    pub total_frames: Option<u32>,

    /// Bits per sample (e.g., 16, 24).
    pub bits_per_sample: Option<u16>,

    /// Number of audio channels.
    pub channels: Option<u16>,

    /// Sampling rate in Hz.
    pub sample_rate: Option<u32>,
}

/// Represents the header structure used in older APE file versions.
///
/// This format is simpler and lacks some of the extended fields
/// present in newer headers.
#[derive(Default, Clone, DisplayPretty)]
pub struct ApeHeaderOld {
    /// APE file format version.
    pub version: Option<u16>,

    /// Compression level used during encoding.
    pub compression_level: Option<u16>,

    /// Number of audio channels.
    pub channels: Option<u16>,

    /// Sampling rate in Hz.
    pub sample_rate: Option<u32>,

    /// Total number of frames in the stream.
    pub total_frames: Option<u32>,

    /// Number of blocks in the final frame.
    pub final_frame_blocks: Option<u32>,
}

/// Represents either a new or old APE header format.
///
/// This abstraction allows handling multiple APE versions transparently.
#[derive(Clone, DisplayPretty)]
pub enum ApeHeader {
    /// Newer APE header format.
    New(ApeHeaderNew),

    /// Legacy APE header format.
    Old(ApeHeaderOld),
}


/// Represents parsed APE metadata tags.
///
/// APE tags are flexible key-value pairs commonly used for storing
/// audio metadata such as title, artist, and album information.
#[derive(Default, Clone, DisplayPretty)]
pub struct ApeTags {
    /// Album name.
    pub album: Option<String>,

    /// Artist or performer name.
    pub artist: Option<String>,

    /// Duration of the track (typically as a string).
    pub duration: Option<String>,

    /// Genre classification.
    pub genre: Option<String>,

    /// Track title.
    pub title: Option<String>,

    /// Name of the encoding tool or software.
    pub tool_name: Option<String>,

    /// Version of the encoding tool.
    pub tool_version: Option<String>,

    /// Track number (may include total tracks, e.g., "1/10").
    pub track: Option<String>,

    /// Year of release.
    pub year: Option<String>,
}


/// Represents a parsed APE (Monkey's Audio) file.
///
/// Contains audio header information, metadata tags, and optional
/// fallback ID3 metadata if present.
#[derive(Default, Clone, DisplayPretty)]
pub struct Ape {
    /// Parsed APE header (new or old format).
    pub header: Option<ApeHeader>,

    /// Parsed APE metadata tags.
    pub tags: Option<ApeTags>,

    /// Optional ID3 metadata (may be present alongside APE tags).
    pub id3: Option<crate::Id3>,
}

// -----------------------------
// Parsing
// -----------------------------

impl Ape {
    /// Parses an APE (Monkey's Audio) file from the given reader.
    ///
    /// This function attempts to:
    /// - Detect and parse either the old or new APE header format
    /// - Extract core audio stream properties (channels, sample rate, etc.)
    /// - Read APE tag metadata if present
    /// - Optionally parse fallback ID3 metadata
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing APE data.
    ///
    /// # Returns
    ///
    /// Returns an [`Ape`] struct containing any successfully parsed
    /// header and metadata. Fields are optional to allow partial parsing
    /// of incomplete or non-standard files.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    /// - The reader fails during I/O operations
    /// - Required header data cannot be read
    ///
    /// # Notes
    ///
    /// - This parser is tolerant and may skip unknown or unsupported sections.
    /// - APE files may contain both APE tags and ID3 tags; both are attempted.
    /// - Header format (old vs new) is typically determined by version fields.
    pub fn parse<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let mut ape = Ape::default();

        // ---- Signature ----
        let mut sig = [0; 4];
        if reader.read_exact(&mut sig).is_err() || &sig != b"MAC " {
            return Ok(ape); // return empty instead of failing
        }

        // ---- Version ----
        let version = read_u16(&mut reader).ok();

        // ---- Header ----
        let header = match version {
            Some(v) if v >= 3980 => {
                let mut h = ApeHeaderNew::default();

                h.compression_level = read_u16(&mut reader).ok();
                h.blocks_per_frame = read_u32(&mut reader).ok();
                h.final_frame_blocks = read_u32(&mut reader).ok();
                h.total_frames = read_u32(&mut reader).ok();
                h.bits_per_sample = read_u16(&mut reader).ok();
                h.channels = read_u16(&mut reader).ok();
                h.sample_rate = read_u32(&mut reader).ok();

                Some(ApeHeader::New(h))
            }
            Some(v) => {
                let mut h = ApeHeaderOld::default();

                h.version = Some(v);
                h.compression_level = read_u16(&mut reader).ok();
                h.channels = read_u16(&mut reader).ok();
                h.sample_rate = read_u32(&mut reader).ok();
                h.total_frames = read_u32(&mut reader).ok();
                h.final_frame_blocks = read_u32(&mut reader).ok();

                Some(ApeHeader::Old(h))
            }
            None => None,
        };

        ape.header = header;

        // ---- ID3 ----
        ape.id3 = crate::Id3::parse(&mut reader).ok();

        // ---- Tags ----
        let mut tags = ApeTags::default();
        let mut found_any_tag = false;

        loop {
            let value_len = match read_u32(&mut reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            let _flags = read_u32(&mut reader).ok();

            // Read key
            let mut key_bytes = Vec::new();
            loop {
                let mut b = [0; 1];
                if reader.read_exact(&mut b).is_err() {
                    break;
                }
                if b[0] == 0 {
                    break;
                }
                key_bytes.push(b[0]);
            }

            let key = String::from_utf8_lossy(&key_bytes);
            let value = match read_string(&mut reader, value_len as usize) {
                Ok(v) => v,
                Err(_) => break,
            };

            match key.as_ref() {
                "Album" => { tags.album = Some(value); found_any_tag = true; }
                "Artist" => { tags.artist = Some(value); found_any_tag = true; }
                "DURATION" => { tags.duration = Some(value); found_any_tag = true; }
                "Genre" => { tags.genre = Some(value); found_any_tag = true; }
                "Title" => { tags.title = Some(value); found_any_tag = true; }
                "Tool Name" => { tags.tool_name = Some(value); found_any_tag = true; }
                "Tool Version" => { tags.tool_version = Some(value); found_any_tag = true; }
                "Track" => { tags.track = Some(value); found_any_tag = true; }
                "Year" => { tags.year = Some(value); found_any_tag = true; }
                _ => {}
            }
        }

        if found_any_tag {
            ape.tags = Some(tags);
        }

        Ok(ape)
    }
}