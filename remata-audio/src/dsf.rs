use std::io::{self, Read, Seek, SeekFrom};
use remata_macros::DisplayPretty;


// -----------------------------
// Helpers
// -----------------------------

fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64<R: Read>(r: &mut R) -> io::Result<u64> {
    let mut buf = [0; 8];
    r.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_chunk_id<R: Read>(r: &mut R) -> io::Result<[u8; 4]> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

// -----------------------------
// Enums
// -----------------------------

#[derive(Clone, Copy, DisplayPretty)]
pub enum DsfFormatId {
    DsdRaw,
    Unknown(u32),
}

impl From<u32> for DsfFormatId {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::DsdRaw,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Clone, Copy, DisplayPretty)]
pub enum DsfChannelType {
    Mono,
    Stereo,
    Multi(u32),
}

impl From<u32> for DsfChannelType {
    fn from(v: u32) -> Self {
        match v {
            1 => Self::Mono,
            2 => Self::Stereo,
            other => Self::Multi(other),
        }
    }
}

// -----------------------------
// fmt chunk
// -----------------------------

#[derive(Clone, Default, DisplayPretty)]
pub struct DsfFmt {
    /// Version of the DSF format specification used by the file.
    pub format_version: Option<u32>,

    /// Format identifier describing how audio data is encoded.
    ///
    /// Typically `DsdRaw` for standard DSF files.
    pub format_id: Option<DsfFormatId>,

    /// Channel layout type (e.g., mono, stereo, or multi-channel).
    pub channel_type: Option<DsfChannelType>,

    /// Total number of audio channels.
    pub channel_count: Option<u32>,

    /// Sampling rate in Hz (e.g., 2822400 for DSD64).
    pub sample_rate: Option<u32>,

    /// Bits per sample (usually 1 for DSD audio).
    pub bits_per_sample: Option<u32>,

    /// Total number of samples per channel in the stream.
    pub sample_count: Option<u64>,

    /// Block size per channel in bytes.
    ///
    /// Used for organizing DSD data into fixed-size chunks.
    pub block_size: Option<u32>,
}

// -----------------------------
// data chunk
// -----------------------------

#[derive(Clone, Default, DisplayPretty)]
pub struct DsfData {
    /// Absolute file offset where the audio data begins.
    pub data_offset: Option<u64>,

    /// Total size of the audio data section in bytes.
    pub data_size: Option<u64>,
}


/// Represents a parsed DSF (DSD Stream File) audio file.
///
/// DSF is a container format used for storing DSD (Direct Stream Digital)
/// audio data. It consists of structured chunks such as `fmt` (format)
/// and `data`, along with optional metadata like ID3 tags.
///
/// This struct aggregates the parsed components of a DSF file, including
/// audio format information and metadata.
#[derive(Clone, Default, DisplayPretty)]
pub struct Dsf {
    /// Parsed `fmt` chunk containing audio format metadata.
    pub fmt: Option<DsfFmt>,

    /// Parsed `data` chunk describing audio data location and size.
    pub data: Option<DsfData>,

    /// Optional ID3 metadata block (typically located at the end of the file).
    pub id3: Option<crate::Id3>,
}
// -----------------------------
// Parser
// -----------------------------

impl Dsf {
    /// Parses a DSF (DSD Stream File) from the given reader.
    ///
    /// This function attempts to:
    /// - Read and validate the DSF file structure
    /// - Parse the `fmt` chunk for audio format information
    /// - Parse the `data` chunk for audio data location and size
    /// - Detect and parse an optional trailing ID3 metadata block
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing DSF data.
    ///
    /// # Returns
    ///
    /// Returns a [`Dsf`] struct containing any successfully parsed
    /// components. Fields are optional to allow partial parsing of
    /// incomplete or non-standard files.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    /// - The underlying reader fails during I/O operations
    /// - Required DSF header or chunk data cannot be read
    ///
    /// # Behavior
    ///
    /// - Unknown or unsupported chunks are skipped safely.
    /// - Parsing is best-effort; missing chunks will not cause failure.
    /// - ID3 metadata is typically located at the end of the file and
    ///   is parsed if present.
    ///
    /// # Notes
    ///
    /// - DSF uses little-endian encoding.
    /// - Audio data is stored separately from metadata in distinct chunks.
    /// - The `fmt` chunk is required for proper interpretation of the stream.
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut dsf = Dsf::default();

        // ---- DSD header ----
        let id = read_chunk_id(reader)?;
        if &id != b"DSD " {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not DSF"));
        }

        let _chunk_size = read_u64(reader)?;
        let _file_size = read_u64(reader)?;
        let metadata_offset = read_u64(reader).ok();

        // ---- Scan chunks ----
        loop {
            let pos = reader.stream_position()?;

            let id = match read_chunk_id(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            let size = read_u64(reader)?;

            match &id {
                b"fmt " => {
                    let mut fmt = DsfFmt::default();

                    fmt.format_version = read_u32(reader).ok();
                    fmt.format_id = read_u32(reader).ok().map(Into::into);
                    fmt.channel_type = read_u32(reader).ok().map(Into::into);
                    fmt.channel_count = read_u32(reader).ok();
                    fmt.sample_rate = read_u32(reader).ok();
                    fmt.bits_per_sample = read_u32(reader).ok();
                    fmt.sample_count = read_u64(reader).ok();

                    let _reserved = read_u32(reader).ok();
                    fmt.block_size = read_u32(reader).ok();

                    dsf.fmt = Some(fmt);
                }

                b"data" => {
                    let data_offset = reader.stream_position().ok();
                    dsf.data = Some(DsfData {
                        data_offset,
                        data_size: Some(size),
                    });

                    // skip actual audio data
                    reader.seek(SeekFrom::Current(size as i64))?;
                }

                _ => {
                    // skip unknown chunk
                    reader.seek(SeekFrom::Start(pos + 12 + size))?;
                }
            }
        }

        // ---- ID3 (seek-based) ----
        if let Some(offset) = metadata_offset {
            if reader.seek(SeekFrom::Start(offset)).is_ok() {
                dsf.id3 = crate::Id3::parse(reader).ok();
            }
        }

        Ok(dsf)
    }
}