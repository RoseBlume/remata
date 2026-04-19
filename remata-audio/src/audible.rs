use remata_macros::DisplayPretty;
use std::io::{self, Read, Seek, SeekFrom};

/// Represents a parsed Audible audiobook container.
///
/// Audible files use a container format similar to MP4/M4A, where data is
/// organized into hierarchical atoms (e.g., `tags`, `meta`, `cvrx`, `tseg`).
/// These atoms store metadata such as title, author, cover art, and chapter
/// information.
///
/// This struct aggregates extracted metadata and optional fallback ID3 tags.
#[derive(Default, Clone, DisplayPretty)]
pub struct Audible {
    /// Total number of chapters in the audiobook, if available.
    pub chapter_count: Option<u32>,

    /// Raw cover artwork bytes extracted from the file.
    ///
    /// Typically represents an embedded image (e.g., JPEG or PNG).
    pub cover_art: Option<Vec<u8>>,

    /// Name of the audiobook's author.
    pub author: Option<String>,

    /// Copyright information associated with the audiobook.
    pub copyright: Option<String>,

    /// Start date of publication (may be partial or loosely formatted).
    pub publish_date_start: Option<String>,

    /// Full publication date of the audiobook.
    pub publish_date: Option<String>,

    /// Container for parsed Audible-specific tag atoms.
    pub tags: Option<AudibleTags>,

    /// Fallback ID3 metadata (typically parsed from the file trailer).
    pub id3: Option<crate::Id3>,
}

#[derive(Default, Clone, DisplayPretty)]
pub struct AudibleTags {
    /// Cover-related metadata atom (`cvrx`).
    pub cvrx: Option<AudibleCvrx>,

    /// General metadata atom (`meta`) containing textual fields.
    pub meta: Option<AudibleMeta>,

    /// Segment/chapter-related metadata atom (`tseg`).
    pub tseg: Option<AudibleTseg>,
}

#[derive(Default, Clone, DisplayPretty)]
pub struct AudibleCvrx {
    /// Raw cover image data extracted from the `cvrx` atom.
    pub cover_art: Option<Vec<u8>>,

    /// Type/format description of the cover image (e.g., "image").
    ///
    /// This implementation currently uses a generic placeholder.
    pub cover_art_type: Option<String>,
}

#[derive(Default, Clone, DisplayPretty)]
pub struct AudibleMeta {
    /// Album artist (often used for grouping audiobooks by narrator or publisher).
    pub album_artist: Option<String>,

    /// Album name (commonly the audiobook title or series name).
    pub album: Option<String>,

    /// Primary artist (may represent narrator or contributor).
    pub artist: Option<String>,

    /// Free-form comment field.
    pub comment: Option<String>,

    /// Genre classification of the audiobook.
    pub genre: Option<String>,

    /// Subtitle or secondary title information.
    pub subtitle: Option<String>,

    /// Tool or software used to create or encode the file.
    pub creator_tool: Option<String>,

    /// Main title of the audiobook or track.
    pub title: Option<String>,

    /// Year or release date string.
    pub year: Option<String>,

    /// iTunes-specific media type identifier.
    pub itunes_media_type: Option<String>,

    /// Chapter name or segment title.
    pub chapter_name: Option<String>,
}

#[derive(Default, Clone, DisplayPretty)]
pub struct AudibleTseg {
    /// Nested metadata specific to this segment/chapter.
    pub meta2: Option<AudibleMeta>,

    /// Chapter number associated with this segment.
    pub chapter_number: Option<u32>,
}

fn read_u32_be<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn read_atom_header<R: Read>(r: &mut R) -> io::Result<(u32, [u8; 4])> {
    let size = read_u32_be(r)?;
    let mut kind = [0u8; 4];
    r.read_exact(&mut kind)?;
    Ok((size, kind))
}

impl Audible {
    /// Parses an Audible audiobook container from the given reader.
    ///
    /// This function attempts to:
    /// - Iterate through top-level atoms in the file
    /// - Identify and parse known atoms (`tags`, `meta`, `cvrx`, `tseg`)
    /// - Extract structured metadata such as:
    ///   - Textual metadata (`meta`)
    ///   - Cover artwork (`cvrx`)
    ///   - Chapter/segment data (`tseg`)
    /// - Populate the [`AudibleTags`] structure accordingly
    /// - Detect and parse an optional trailing ID3 metadata block
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing Audible data.
    ///
    /// # Returns
    ///
    /// Returns an [`Audible`] struct containing any successfully parsed
    /// metadata. Fields are optional to allow partial parsing of incomplete
    /// or non-standard files.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    /// - The underlying reader fails during I/O operations
    /// - Atom headers cannot be read
    ///
    /// # Behavior
    ///
    /// - Unknown atoms are skipped safely.
    /// - Parsing is best-effort; missing or malformed atoms do not cause failure.
    /// - Nested atoms (`meta`, `cvrx`, `tseg`) may appear either inside a `tags`
    ///   container or at the top level.
    /// - ID3 metadata is typically located near the end of the file and is
    ///   parsed if present.
    ///
    /// # Notes
    ///
    /// - Audible uses a structure similar to ISO Base Media File Format (MP4).
    /// - Atom sizes include their headers and must be used to advance correctly.
    /// - Metadata layout may vary between Audible versions and encoders.
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut audible = Audible::default();

        loop {
            let start = reader.stream_position()?;

            let (size, kind) = match read_atom_header(reader) {
                Ok(v) => v,
                Err(_) => break,
            };

            match &kind {
                b"tags" => audible.tags = parse_tags(reader, size - 8).ok(),
                b"meta" => {
                    audible
                        .tags
                        .get_or_insert_with(Default::default)
                        .meta = parse_meta(reader, size - 8).ok();
                }
                b"cvrx" => {
                    audible
                        .tags
                        .get_or_insert_with(Default::default)
                        .cvrx = parse_cvrx(reader, size - 8).ok();
                }
                b"tseg" => {
                    audible
                        .tags
                        .get_or_insert_with(Default::default)
                        .tseg = parse_tseg(reader, size - 8).ok();
                }
                _ => {}
            }

            reader.seek(SeekFrom::Start(start + size as u64))?;
        }

        // ID3 fallback
        if let Ok(end) = reader.seek(SeekFrom::End(0)) {
            if end >= 128 {
                reader.seek(SeekFrom::End(-128))?;
                audible.id3 = crate::Id3::parse(&mut *reader).ok();
            }
        }

        Ok(audible)
    }
}

fn parse_tags<R: Read + Seek>(reader: &mut R, size: u32) -> io::Result<AudibleTags> {
    let mut tags = AudibleTags::default();
    let start = reader.stream_position()?;

    while reader.stream_position()? < start + size as u64 {
        let (atom_size, kind) = read_atom_header(reader)?;

        match &kind {
            b"meta" => tags.meta = parse_meta(reader, atom_size - 8).ok(),
            b"cvrx" => tags.cvrx = parse_cvrx(reader, atom_size - 8).ok(),
            b"tseg" => tags.tseg = parse_tseg(reader, atom_size - 8).ok(),
            _ => {}
        }

        reader.seek(SeekFrom::Current((atom_size - 8) as i64))?;
    }

    Ok(tags)
}

fn read_data_string<R: Read + Seek>(reader: &mut R, size: u32) -> io::Result<String> {
    let mut remaining = size;

    while remaining > 8 {
        let (atom_size, kind) = read_atom_header(reader)?;

        if &kind == b"data" {
            // skip version/flags (8 bytes)
            let mut skip = [0u8; 8];
            reader.read_exact(&mut skip)?;

            let data_size = atom_size - 16;
            let mut buf = vec![0; data_size as usize];
            reader.read_exact(&mut buf)?;

            return Ok(String::from_utf8_lossy(&buf).to_string());
        } else {
            reader.seek(SeekFrom::Current((atom_size - 8) as i64))?;
        }

        remaining -= atom_size;
    }

    Ok(String::new())
}

fn parse_meta<R: Read + Seek>(reader: &mut R, size: u32) -> io::Result<AudibleMeta> {
    let mut meta = AudibleMeta::default();
    let start = reader.stream_position()?;

    while reader.stream_position()? < start + size as u64 {
        let (atom_size, kind) = read_atom_header(reader)?;

        let value = read_data_string(reader, atom_size - 8)?;

        match &kind {
            b"aART" => meta.album_artist = Some(value),
            b"\xA9alb" => meta.album = Some(value),
            b"\xA9ART" => meta.artist = Some(value),
            b"\xA9cmt" => meta.comment = Some(value),
            b"\xA9gen" => meta.genre = Some(value),
            b"titl" => meta.title = Some(value),
            b"\xA9day" => meta.year = Some(value),
            b"tool" => meta.creator_tool = Some(value),
            b"trak" => meta.chapter_name = Some(value),
            _ => {}
        }
    }

    Ok(meta)
}

fn parse_cvrx<R: Read + Seek>(reader: &mut R, size: u32) -> io::Result<AudibleCvrx> {
    let mut cvrx = AudibleCvrx::default();

    let mut buf = vec![0; size as usize];
    reader.read_exact(&mut buf)?;

    cvrx.cover_art = Some(buf);
    cvrx.cover_art_type = Some("image".into());

    Ok(cvrx)
}

fn parse_tseg<R: Read + Seek>(reader: &mut R, size: u32) -> io::Result<AudibleTseg> {
    let mut tseg = AudibleTseg::default();
    let start = reader.stream_position()?;

    while reader.stream_position()? < start + size as u64 {
        let (atom_size, kind) = read_atom_header(reader)?;

        match &kind {
            b"meta" => tseg.meta2 = parse_meta(reader, atom_size - 8).ok(),
            b"tshd" => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                tseg.chapter_number = Some(u32::from_be_bytes(buf));
            }
            _ => {}
        }

        reader.seek(SeekFrom::Current((atom_size - 8) as i64))?;
    }

    Ok(tseg)
}