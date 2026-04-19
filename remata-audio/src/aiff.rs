use std::io::{Read, Seek, SeekFrom};
use crate::Id3;
use crate::ParserError;
use super::{MetaArt, MetaExt};
use remata_macros::DisplayPretty;

/// Represents metadata extracted from an AIFF (Audio Interchange File Format) file.
///
/// AIFF stores metadata in various chunks such as:
/// - `NAME` (title)
/// - `AUTH` (author)
/// - `ANNO` (annotations)
/// - `COMT` (comments)
/// - `(c) ` (copyright)
/// - `ID3 ` (embedded ID3 metadata)
///
/// Not all chunks are guaranteed to be present, so all fields are optional.
#[derive(Default, Clone, DisplayPretty)]
pub struct AiffMeta {
    /// Title of the audio file (`NAME` chunk).
    pub title: Option<String>,

    /// Author or creator of the audio (`AUTH` chunk).
    pub author: Option<String>,

    /// Comment text (`COMT` chunk).
    ///
    /// AIFF comments may include timestamps and markers,
    /// but this field typically stores the extracted text only.
    pub comment: Option<String>,

    /// Annotation text (`ANNO` chunk).
    ///
    /// Often used for free-form notes or descriptions.
    pub annotation: Option<String>,

    /// Copyright notice (`(c) ` chunk).
    pub copyright: Option<String>,

    /// Parsed ID3 metadata (`ID3 ` chunk), if present and successfully decoded.
    pub id3: Option<Id3>,

    /// Raw ID3 data (`ID3 ` chunk) if parsing fails.
    ///
    /// This allows preserving metadata even when decoding is not possible.
    pub id3_raw: Option<Vec<u8>>, // store raw ID3 if parsing fails
}

impl AiffMeta {
    /// Parses AIFF metadata from the given reader.
    ///
    /// This function scans the AIFF file structure and attempts to extract:
    /// - Standard text chunks (`NAME`, `AUTH`, `ANNO`, `COMT`, `(c) `)
    /// - Embedded ID3 metadata (`ID3 ` chunk)
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing AIFF data.
    ///
    /// # Returns
    ///
    /// Returns an [`AiffMeta`] struct containing any successfully extracted metadata.
    /// Fields are optional to allow partial parsing of incomplete or non-standard files.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if:
    /// - The underlying reader fails during I/O operations
    /// - The AIFF structure is invalid or cannot be parsed reliably
    ///
    /// # Behavior
    ///
    /// - Unknown chunks are skipped safely.
    /// - If an `ID3 ` chunk is found:
    ///   - The parser attempts to decode it into [`Id3`]
    ///   - If decoding fails, the raw bytes are stored in `id3_raw`
    ///
    /// # Notes
    ///
    /// - AIFF uses big-endian encoding and chunk-based structure.
    /// - Metadata chunks may appear in any order.
    /// - Multiple comment chunks may exist; this implementation may store only one.
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut meta = AiffMeta::default();

        // Accumulator for ID3 data
        let mut id3_accumulator = Vec::new();

        // ------------------------
        // Skip FORM header (12 bytes)
        // ------------------------
        reader.seek(SeekFrom::Start(12))?;

        // ------------------------
        // Chunk loop
        // ------------------------
        loop {
            // Try reading chunk header (8 bytes)
            let mut header = [0u8; 8];
            if reader.read_exact(&mut header).is_err() {
                break;
            }

            let chunk_id = String::from_utf8_lossy(&header[0..4]).to_string();
            let chunk_size = u32::from_be_bytes(header[4..8].try_into().unwrap()) as usize;

            // ------------------------
            // Read chunk data
            // ------------------------
            let mut chunk_data = vec![0u8; chunk_size];
            if reader.read_exact(&mut chunk_data).is_err() {
                break;
            }

            // Padding byte (AIFF chunks are even-sized)
            if chunk_size % 2 != 0 {
                reader.seek(SeekFrom::Current(1))?;
            }

            // ------------------------
            // Parse known chunks
            // ------------------------
            match chunk_id.as_str() {
                "NAME" => meta.title = Some(bytes_to_string(&chunk_data)),
                "AUTH" => meta.author = Some(bytes_to_string(&chunk_data)),
                "ANNO" => meta.annotation = Some(bytes_to_string(&chunk_data)),
                "COMT" => meta.comment = Some(bytes_to_string(&chunk_data)),
                "©c  " => meta.copyright = Some(bytes_to_string(&chunk_data)),

                "ID3 " => {
                    // Accumulate ID3 chunks (AIFF can split them)
                    id3_accumulator.extend_from_slice(&chunk_data);
                }

                _ => {}
            }
        }

        // ------------------------
        // Parse accumulated ID3
        // ------------------------
        if !id3_accumulator.is_empty() {
            if id3_accumulator.starts_with(b"ID3") {
                let mut cursor = std::io::Cursor::new(&id3_accumulator);
                match Id3::parse(&mut cursor) {
                    Ok(id3) => meta.id3 = Some(id3),
                    Err(_) => meta.id3_raw = Some(id3_accumulator),
                }
            } else {
                meta.id3_raw = Some(id3_accumulator);
            }
        }

        Ok(meta)
    }
}

impl MetaExt for AiffMeta {
    fn title(&self) -> Option<String> {
        self.title
            .as_ref()
            .cloned()
            .or_else(|| self.id3.as_ref().and_then(|id3| id3.title()))
    }

    fn artist(&self) -> Option<String> {
        self.author
            .as_ref()
            .cloned()
            .or_else(|| self.id3.as_ref().and_then(|id3| id3.artist()))
    }

    fn album(&self) -> Option<String> {
        self.id3
            .as_ref()
            .and_then(|id3| id3.album())
    }

    fn genre(&self) -> Option<String> {
        self.id3
            .as_ref()
            .and_then(|id3| id3.genre())
    }

    fn year(&self) -> Option<String> {
        self.id3
            .as_ref()
            .and_then(|id3| id3.year())
    }

    fn art(&self) -> Option<MetaArt> {
        self.id3
            .as_ref()
            .and_then(|id3| id3.art())
    }
}



// ------------------------
// Helpers
// ------------------------
fn bytes_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data).trim_matches('\0').to_string()
}

