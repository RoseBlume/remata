use std::io::{self, Read, Seek, SeekFrom};

use remata_macros::DisplayPretty;

// -----------------------------
// Helpers
// -----------------------------

/// Reads a 32-bit unsigned integer in little-endian format from the reader.
fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Reads a 16-bit unsigned integer in little-endian format from the reader.
fn read_u16<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut buf = [0; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Reads a 4-byte chunk identifier (FourCC) from the reader.
///
/// Common examples include `"wvpk"` for WavPack blocks.
fn read_chunk_id<R: Read>(r: &mut R) -> io::Result<[u8; 4]> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

// -----------------------------
// Enums
// -----------------------------

/// Describes the channel configuration of the audio stream.
#[derive(Clone, Copy, DisplayPretty)]
pub enum WvAudioType {
    /// Two-channel stereo audio.
    Stereo,

    /// Single-channel mono audio.
    Mono,
}

/// Indicates the compression mode used in the WavPack stream.
#[derive(Clone, Copy, DisplayPretty)]
pub enum WvCompression {
    /// Fully lossless compression.
    Lossless,

    /// Hybrid mode (lossy core with optional correction file).
    Hybrid,
}

/// Describes the underlying sample representation format.
#[derive(Clone, Copy, DisplayPretty)]
pub enum WvDataFormat {
    /// Integer PCM samples.
    Integer,

    /// Floating-point samples.
    Float,
}

/// Represents the sample rate of the audio stream.
#[derive(Clone, Copy, DisplayPretty)]
pub enum WvSampleRate {
    /// Standard or explicitly stored sample rate in Hz.
    Rate(u32),

    /// Non-standard or custom sample rate not directly encoded.
    Custom,
}

// -----------------------------
// Header (all Option)
// -----------------------------

/// Parsed header information from a WavPack audio stream.
///
/// This contains core properties describing how audio data is stored
/// and should be interpreted.
#[derive(Default, DisplayPretty, Clone)]
pub struct WavPackHeader {
    /// Number of bytes per individual sample.
    ///
    /// Common values are 1, 2, 3, or 4 depending on bit depth.
    pub bytes_per_sample: Option<u8>,

    /// Channel configuration (mono or stereo).
    pub audio_type: Option<WvAudioType>,

    /// Compression mode used by the encoder.
    pub compression: Option<WvCompression>,

    /// Sample data representation (integer or floating-point).
    pub data_format: Option<WvDataFormat>,

    /// Sampling rate of the audio stream.
    pub sample_rate: Option<WvSampleRate>,
}

// -----------------------------
// Main struct
// -----------------------------

/// Represents a parsed WavPack audio file.
///
/// Combines stream header information with optional metadata
/// such as ID3 and APE tags.
#[derive(Default, DisplayPretty, Clone)]
pub struct WavPack {
    /// Parsed WavPack stream header.
    pub header: Option<WavPackHeader>,

    /// Optional ID3 metadata (may appear at the beginning or end of the file).
    pub id3: Option<crate::Id3>,

    /// Optional APE metadata (commonly used with WavPack files).
    pub ape: Option<crate::Ape>,
}

// -----------------------------
// Parser
// -----------------------------

impl WavPack {
    /// Parses a WavPack audio stream from the given reader.
    ///
    /// This function attempts to:
    /// - Read and interpret the WavPack block header (`"wvpk"`)
    /// - Extract core stream properties into a [`WavPackHeader`]
    /// - Optionally detect and parse trailing metadata (e.g., ID3 or APE tags)
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing WavPack data.
    ///
    /// # Returns
    ///
    /// Returns a [`WavPack`] struct containing any successfully parsed
    /// header and metadata information. Fields are optional to allow
    /// partial parsing of incomplete or non-standard files.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    /// - The underlying reader fails during I/O operations
    /// - Required header data cannot be read
    ///
    /// # Notes
    ///
    /// - This parser is tolerant and may skip unknown or unsupported blocks.
    /// - Metadata parsing (ID3/APE) is best-effort and depends on file layout.
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut wv = WavPack::default();

        // ---- Scan blocks ----
        loop {
            let start = reader.stream_position()?;

            let id = match read_chunk_id(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            if &id != b"wvpk" {
                break;
            }

            let block_size = read_u32(reader)?;

            // Skip version, track, index
            let _version = read_u16(reader).ok();
            let _track = reader.seek(SeekFrom::Current(1)).ok();
            let _index = reader.seek(SeekFrom::Current(1)).ok();

            let _total_samples = read_u32(reader).ok();
            let _block_index = read_u32(reader).ok();
            let _block_samples = read_u32(reader).ok();

            // ---- FLAGS (important!) ----
            let flags = read_u32(reader).ok();

            if let Some(flags) = flags {
                let mut header = WavPackHeader::default();

                // 6.1 BytesPerSample
                header.bytes_per_sample = Some((flags & 0x3) as u8);

                // 6.2 AudioType
                header.audio_type = Some(if ((flags >> 2) & 0x1) == 1 {
                    WvAudioType::Mono
                } else {
                    WvAudioType::Stereo
                });

                // 6.3 Compression
                header.compression = Some(if ((flags >> 3) & 0x1) == 1 {
                    WvCompression::Hybrid
                } else {
                    WvCompression::Lossless
                });

                // 6.4 DataFormat
                header.data_format = Some(if ((flags >> 7) & 0x1) == 1 {
                    WvDataFormat::Float
                } else {
                    WvDataFormat::Integer
                });

                // 6.5 SampleRate
                let sr_code = (flags >> 23) & 0xF;
                header.sample_rate = Some(match sr_code {
                    0 => WvSampleRate::Rate(6000),
                    1 => WvSampleRate::Rate(8000),
                    2 => WvSampleRate::Rate(9600),
                    3 => WvSampleRate::Rate(11025),
                    4 => WvSampleRate::Rate(12000),
                    5 => WvSampleRate::Rate(16000),
                    6 => WvSampleRate::Rate(22050),
                    7 => WvSampleRate::Rate(24000),
                    8 => WvSampleRate::Rate(32000),
                    9 => WvSampleRate::Rate(44100),
                    10 => WvSampleRate::Rate(48000),
                    11 => WvSampleRate::Rate(64000),
                    12 => WvSampleRate::Rate(88200),
                    13 => WvSampleRate::Rate(96000),
                    14 => WvSampleRate::Rate(192000),
                    _ => WvSampleRate::Custom,
                });

                wv.header = Some(header);
            }

            // Skip rest of block
            reader.seek(SeekFrom::Start(start + 8 + block_size as u64))?;
        }

        // ---- Seek-based metadata ----

        // Try ID3 (usually at end)
        if let Ok(end) = reader.seek(SeekFrom::End(0)) {
            if reader.seek(SeekFrom::Start(end.saturating_sub(128))).is_ok() {
                wv.id3 = crate::Id3::parse(reader).ok();
            }
        }

        // Try APE (also usually footer-based)
        if let Ok(end) = reader.seek(SeekFrom::End(0)) {
            if reader.seek(SeekFrom::Start(end.saturating_sub(160))).is_ok() {
                wv.ape = crate::Ape::parse(reader).ok();
            }
        }

        Ok(wv)
    }
}