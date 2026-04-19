use std::io::{self, Read, Seek, SeekFrom};
use remata_macros::DisplayPretty;

/// Represents the supported sample rates for Musepack (MPC) audio.
#[derive(Clone, Copy, DisplayPretty)]
pub enum MpcSampleRate {
    /// 44,100 Hz (CD-quality standard).
    Hz44100,

    /// 48,000 Hz (common in professional audio and video).
    Hz48000,

    /// 37,800 Hz (less common legacy rate).
    Hz37800,

    /// 32,000 Hz (low-bandwidth audio).
    Hz32000,
}

/// Represents the encoder quality presets used in Musepack encoding.
///
/// These map to historical encoder modes and quality levels.
#[derive(Clone, Copy, DisplayPretty)]
pub enum MpcQuality {
    /// Experimental or unstable encoding mode.
    Unstable,

    /// Quality level 0 (lowest standard quality).
    Q0,

    /// Quality level 1.
    Q1,

    /// Telephone-grade quality (very low bitrate).
    Telephone,

    /// Thumb quality (very small file size).
    Thumb,

    /// Radio quality (low bitrate, acceptable for streaming).
    Radio,

    /// Standard quality (default preset for most use cases).
    Standard,

    /// Xtreme quality (high quality, higher bitrate).
    Xtreme,

    /// Insane quality (very high bitrate, near-transparent).
    Insane,

    /// Maximum quality setting (extreme bitrate).
    BrainDead,

    /// Quality level 9 (fine-grained encoder setting).
    Q9,

    /// Quality level 10 (highest defined numeric quality).
    Q10,
}

/// Parsed header information from a Musepack (MPC) audio file.
///
/// This structure contains core stream properties such as duration,
/// encoding quality, replaygain values, and playback flags.
#[derive(Default, Clone, DisplayPretty)]
pub struct MpcHeader {
    /// Total number of audio frames in the stream.
    ///
    /// Used to determine playback duration.
    pub total_frames: Option<u32>,

    /// Sampling rate of the audio stream.
    pub sample_rate: Option<MpcSampleRate>,

    /// Encoder quality setting used when the file was created.
    pub quality: Option<MpcQuality>,

    /// Maximum subband used during encoding.
    ///
    /// Reflects frequency range utilization.
    pub max_band: Option<u8>,

    /// ReplayGain track peak value.
    ///
    /// Represents the maximum sample amplitude for the track.
    pub replaygain_track_peak: Option<u16>,

    /// ReplayGain track gain adjustment (in decibels, scaled).
    pub replaygain_track_gain: Option<i16>,

    /// ReplayGain album peak value.
    ///
    /// Represents the maximum sample amplitude across the album.
    pub replaygain_album_peak: Option<u16>,

    /// ReplayGain album gain adjustment (in decibels, scaled).
    pub replaygain_album_gain: Option<i16>,

    /// Indicates whether fast seeking is supported.
    pub fast_seek: Option<bool>,

    /// Indicates whether the stream is encoded for gapless playback.
    pub gapless: Option<bool>,
}

/// Represents a parsed Musepack (MPC) file.
///
/// Combines core stream header information with optional metadata
/// formats such as ID3 and APE tags.
#[derive(Default, Clone, DisplayPretty)]
pub struct Mpc {
    /// Parsed Musepack stream header.
    pub header: Option<MpcHeader>,

    /// Optional ID3 metadata (may appear in some MPC files).
    pub id3: Option<crate::Id3>,

    /// Optional APE metadata (commonly used with MPC).
    pub ape: Option<crate::Ape>,
}

impl Mpc {
    /// Parses a Musepack (MPC) audio stream from the given reader.
    ///
    /// This function attempts to:
    /// - Read and interpret the Musepack stream header
    /// - Extract core audio properties such as:
    ///   - Total frame count
    ///   - Sample rate
    ///   - Encoder quality preset
    ///   - ReplayGain information
    /// - Populate an [`MpcHeader`] with decoded values
    /// - Detect and parse optional metadata blocks (e.g., ID3 and APE tags)
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing MPC data.
    ///
    /// # Returns
    ///
    /// Returns an [`Mpc`] struct containing any successfully parsed
    /// header and metadata. Fields are optional to allow partial parsing
    /// of incomplete or non-standard files.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    /// - The underlying reader fails during I/O operations
    /// - Required header data cannot be read
    ///
    /// # Behavior
    ///
    /// - Parsing is best-effort; missing or malformed fields will not cause failure.
    /// - Unknown or unsupported data is skipped when possible.
    /// - ReplayGain values are extracted if present in the header.
    /// - Metadata parsing (ID3/APE) is attempted if such blocks are detected.
    ///
    /// # Notes
    ///
    /// - Musepack uses a frame-based compression format with a compact header.
    /// - Header layout may vary slightly between stream versions.
    /// - APE tags are commonly used for metadata in MPC files.
    /// - ID3 tags may also be present for compatibility.
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut mpc = Mpc::default();

        // ---- Read first 32 bytes (enough for header bits) ----
        let mut buf = [0u8; 32];
        reader.read_exact(&mut buf)?;

        // Combine into bitstream
        let bits = U256::from_be_bytes(buf);

        let mut header = MpcHeader::default();

        // ------------------------
        // Bit extraction helpers
        // ------------------------
        let get_bits = |start: usize, len: usize| -> u32 {
            bits.get_bits(start, len)
        };

        // ------------------------
        // Parse fields
        // ------------------------

        // TotalFrames (Bit 32-63)
        header.total_frames = Some(get_bits(32, 32));

        // SampleRate (Bit 80-81)
        header.sample_rate = match get_bits(80, 2) {
            0 => Some(MpcSampleRate::Hz44100),
            1 => Some(MpcSampleRate::Hz48000),
            2 => Some(MpcSampleRate::Hz37800),
            3 => Some(MpcSampleRate::Hz32000),
            _ => None,
        };

        // Quality (Bit 84-87)
        header.quality = match get_bits(84, 4) {
            1 => Some(MpcQuality::Unstable),
            5 => Some(MpcQuality::Q0),
            6 => Some(MpcQuality::Q1),
            7 => Some(MpcQuality::Telephone),
            8 => Some(MpcQuality::Thumb),
            9 => Some(MpcQuality::Radio),
            10 => Some(MpcQuality::Standard),
            11 => Some(MpcQuality::Xtreme),
            12 => Some(MpcQuality::Insane),
            13 => Some(MpcQuality::BrainDead),
            14 => Some(MpcQuality::Q9),
            15 => Some(MpcQuality::Q10),
            _ => None,
        };

        // MaxBand (Bit 88-93)
        header.max_band = Some(get_bits(88, 6) as u8);

        // ReplayGain Track Peak (96-111)
        header.replaygain_track_peak = Some(get_bits(96, 16) as u16);

        // ReplayGain Track Gain (112-127)
        header.replaygain_track_gain = Some(get_bits(112, 16) as i16);

        // ReplayGain Album Peak (128-143)
        header.replaygain_album_peak = Some(get_bits(128, 16) as u16);

        // ReplayGain Album Gain (144-159)
        header.replaygain_album_gain = Some(get_bits(144, 16) as i16);

        // FastSeek (Bit 179)
        header.fast_seek = Some(get_bits(179, 1) == 1);

        // Gapless (Bit 191)
        header.gapless = Some(get_bits(191, 1) == 1);

        mpc.header = Some(header);

        // ------------------------
        // Metadata (footer-based)
        // ------------------------

        // ID3 (end)
        if let Ok(end) = reader.seek(SeekFrom::End(0)) {
            if end >= 128 {
                reader.seek(SeekFrom::End(-128))?;
                mpc.id3 = crate::Id3::parse(&mut *reader).ok();
            }
        }

        // APE (also footer)
        if let Ok(end) = reader.seek(SeekFrom::End(0)) {
            if end >= 160 {
                reader.seek(SeekFrom::End(-160))?;
                mpc.ape = crate::Ape::parse(&mut *reader).ok();
            }
        }

        Ok(mpc)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct U256([u8; 32]);

impl U256 {
    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Extract arbitrary bit range
    /// start = bit index from MSB (0 = first bit)
    /// len = number of bits
    pub fn get_bits(&self, start: usize, len: usize) -> u32 {
        let mut result = 0u32;

        for i in 0..len {
            let bit_index = start + i;

            let byte = bit_index / 8;
            let bit = 7 - (bit_index % 8);

            let value = (self.0[byte] >> bit) & 1;
            result = (result << 1) | value as u32;
        }

        result
    }
}