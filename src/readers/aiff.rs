use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::Id3;
use crate::ParserError;
#[derive(Debug, Default)]
pub struct AiffMeta {
    pub title: Option<String>,
    pub author: Option<String>,
    pub comment: Option<String>,
    pub annotation: Option<String>,
    pub copyright: Option<String>,
    pub id3: Option<Id3>,
    pub id3_raw: Option<Vec<u8>>, // store raw ID3 if parsing fails
}

use std::fmt::{Display, Formatter, Result as FmtResult};
impl Display for AiffMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "AIFF Metadata:")?;
        if let Some(title) = &self.title {
            writeln!(f, "  Title: {}", title)?;
        }
        if let Some(author) = &self.author {
            writeln!(f, "  Author: {}", author)?;
        }
        if let Some(comment) = &self.comment {
            writeln!(f, "  Comment: {}", comment)?;
        }
        if let Some(annotation) = &self.annotation {
            writeln!(f, "  Annotation: {}", annotation)?;
        }
        if let Some(copyright) = &self.copyright {
            writeln!(f, "  Copyright: {}", copyright)?;
        }
        if let Some(id3) = &self.id3 {
            writeln!(f, "  ID3 Metadata:\n{}", id3)?;
        } else if let Some(raw) = &self.id3_raw {
            writeln!(f, "  Raw ID3 Data ({} bytes)", raw.len())?;
        }
        Ok(())
    }
}

impl AiffMeta {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut meta = AiffMeta::default();
        let mut cursor = Cursor::new(data);

        // Accumulator for ID3 data
        let mut id3_accumulator = Vec::new();

        // Skip FORM header (12 bytes: 'FORM' + size + 'AIFF'/'AIFC')
        cursor.seek(SeekFrom::Start(12)).ok();

        while (cursor.position() as usize + 8) <= data.len() {
            // Read chunk ID
            let mut chunk_id_buf = [0u8; 4];
            if cursor.read_exact(&mut chunk_id_buf).is_err() { break; }
            let chunk_id = String::from_utf8_lossy(&chunk_id_buf).to_string();

            // Read chunk size (4 bytes, big-endian)
            let mut size_buf = [0u8; 4];
            if cursor.read_exact(&mut size_buf).is_err() { break; }
            let chunk_size = u32::from_be_bytes(size_buf) as usize;

            // Read chunk data
            let mut chunk_data = vec![0u8; chunk_size];
            if cursor.read_exact(&mut chunk_data).is_err() { break; }

            // If odd size, skip padding
            if chunk_size % 2 != 0 {
                cursor.seek(SeekFrom::Current(1)).ok();
            }

            // Parse known chunks
            match chunk_id.as_str() {
                "NAME" => meta.title = Some(bytes_to_string(&chunk_data)),
                "AUTH" => meta.author = Some(bytes_to_string(&chunk_data)),
                "ANNO" => meta.annotation = Some(bytes_to_string(&chunk_data)),
                "COMT" => meta.comment = Some(bytes_to_string(&chunk_data)),
                "©c  " => meta.copyright = Some(bytes_to_string(&chunk_data)),
                "ID3 " => {
                    // Accumulate all ID3 chunks
                    id3_accumulator.extend_from_slice(&chunk_data);
                }
                _ => {} // ignore unknown chunks
            }
        }

        // Attempt to parse accumulated ID3
        if !id3_accumulator.is_empty() {
            if id3_accumulator.starts_with(b"ID3") {
                match Id3::parse(&id3_accumulator) {
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

// ------------------------
// Helpers
// ------------------------
fn bytes_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data).trim_matches('\0').to_string()
}

