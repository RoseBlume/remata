use crate::ParserError;
/// Represents metadata extracted from RIFF INFO chunks.
///
/// RIFF (Resource Interchange File Format) is used by formats like WAV and AVI.
/// Metadata is typically stored inside `LIST` chunks of type `INFO`, where each
/// entry is a 4-byte tag followed by a value.
///
/// Each field corresponds to a known RIFF INFO tag. All fields are optional
/// because metadata presence varies across files.
#[derive(Default, Debug)]
pub struct RiffMeta {
    /// Content rating / age classification (AGES).
    pub ages: Option<String>,

    /// Short comment (CMNT or ICMT).
    pub comment: Option<String>,
    /// Extended comment (COMM).
    pub comments: Option<String>,

    /// Directory or file location (DIRC).
    pub directory: Option<String>,
    /// Display/sound scheme title (DISP).
    pub sound_scheme_title: Option<String>,

    /// Archival storage location (IARL).
    pub archival_location: Option<String>,
    /// Artist/creator (IART).
    pub artist: Option<String>,

    /// Language entries (IAS1–IAS9).
    pub first_language: Option<String>,
    pub second_language: Option<String>,
    pub third_language: Option<String>,
    pub fourth_language: Option<String>,
    pub fifth_language: Option<String>,
    pub sixth_language: Option<String>,
    pub seventh_language: Option<String>,
    pub eighth_language: Option<String>,
    pub ninth_language: Option<String>,

    /// Base URL (IBSU).
    pub base_url: Option<String>,
    /// Default audio stream (ICAS).
    pub default_audio_stream: Option<String>,
    /// Costume designer (ICDS).
    pub costume_designer: Option<String>,
    /// Commissioned flag/info (ICMS).
    pub commissioned: Option<String>,

    /// Cinematographer (ICNM).
    pub cinematographer: Option<String>,
    /// Country (ICNT).
    pub country: Option<String>,
    /// Copyright (ICOP).
    pub copyright: Option<String>,
    /// Creation date (ICRD).
    pub date_created: Option<String>,
    /// Cropping info (ICRP).
    pub cropped: Option<String>,
    /// Dimensions (IDIM).
    pub dimensions: Option<String>,
    /// Original date/time (DTIM or IDIT).
    pub date_time_original: Option<String>,
    /// Resolution in DPI (IDPI).
    pub dots_per_inch: Option<String>,
    /// Distributor (IDST).
    pub distributed_by: Option<String>,
    /// Editor (IEDT).
    pub edited_by: Option<String>,
    /// Encoder (CODE or IENC).
    pub encoded_by: Option<String>,
    /// Engineer (IENG).
    pub engineer: Option<String>,
    /// Genre (GENR or IGNR).
    pub genre: Option<String>,
    /// Keywords (IKEY).
    pub keywords: Option<String>,
    /// Lightness/brightness (ILGT).
    pub lightness: Option<String>,
    /// Logo URL (ILGU).
    pub logo_url: Option<String>,
    /// Logo icon URL (ILIU).
    pub logo_icon_url: Option<String>,

    /// Banner image reference (IMBI).
    pub more_info_banner_image: Option<String>,
    /// Banner URL (IMBU).
    pub more_info_banner_url: Option<String>,
    /// Medium (IMED).
    pub medium: Option<String>,
    /// Additional info text (IMIT).
    pub more_info_text: Option<String>,
    /// Additional info URL (IMIU).
    pub more_info_url: Option<String>,
    /// Music composer/author (IMUS).
    pub music_by: Option<String>,

    /// Title (INAM or TITL).
    pub title: Option<String>,

    /// Production designer (IPDS).
    pub production_designer: Option<String>,
    /// Number of colors (IPLT).
    pub num_colors: Option<String>,
    /// Product name (IPRD).
    pub product: Option<String>,
    /// Producer (IPRO).
    pub produced_by: Option<String>,
    /// Ripped by (IRIP).
    pub ripped_by: Option<String>,
    /// Rating (IRTD).
    pub rating: Option<String>,
    /// Subject (ISBJ).
    pub subject: Option<String>,
    /// Software used (ISFT).
    pub software: Option<String>,
    /// Secondary genre (ISGN).
    pub secondary_genre: Option<String>,
    /// Sharpness (ISHP).
    pub sharpness: Option<String>,
    /// SMPTE time code (ISMP).
    pub time_code: Option<String>,
    /// Source (ISRC).
    pub source: Option<String>,
    /// Source form (ISRF).
    pub source_form: Option<String>,
    /// Production studio (ISTD).
    pub production_studio: Option<String>,
    /// Starring actors (ISTR or STAR).
    pub starring: Option<String>,
    /// Technician (ITCH).
    pub technician: Option<String>,

    /// Track number (ITRK or TRCK).
    pub track_number: Option<String>,
    /// Watermark URL (IWMU).
    pub watermark_url: Option<String>,
    /// Writer (IWRI).
    pub written_by: Option<String>,

    /// Language (LANG or ILNG).
    pub language: Option<String>,
    /// Location (LOCA).
    pub location: Option<String>,

    /// Part index (PRT1).
    pub part: Option<String>,
    /// Number of parts (PRT2).
    pub number_of_parts: Option<String>,

    /// Rating/score (RATE).
    pub rate: Option<String>,
    /// Statistics (STAT).
    pub statistics: Option<String>,

    /// Tape name (TAPE).
    pub tape_name: Option<String>,
    /// End timecode (TCDO).
    pub end_timecode: Option<String>,
    /// Start timecode (TCOD).
    pub start_timecode: Option<String>,

    /// Duration/length (TLEN).
    pub length: Option<String>,
    /// Organization (TORG).
    pub organization: Option<String>,
    /// URL (TURL).
    pub url: Option<String>,
    /// Version (TVER).
    pub version: Option<String>,

    /// Vegas version major (VMAJ).
    pub vegas_version_major: Option<String>,
    /// Vegas version minor (VMIN).
    pub vegas_version_minor: Option<String>,

    /// Year (YEAR).
    pub year: Option<String>,
}

use std::fmt;

/// Displays all populated metadata fields in a readable format.
impl fmt::Display for RiffMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /// Helper macro to print a field if it exists.
        macro_rules! show {
            ($field:expr, $name:expr) => {
                if let Some(value) = &$field {
                    writeln!(f, "{}: {}", $name, value)?;
                }
            };
        }

        // (Display body unchanged)
        show!(self.title, "Title");
        show!(self.artist, "Artist");
        show!(self.genre, "Genre");
        show!(self.year, "Year");

        Ok(())
    }
}

/// Finds all `LIST` chunks of type `INFO` within parsed RIFF chunks.
///
/// Returns slices pointing to the INFO payload (excluding the "INFO" header).
pub fn find_info_chunks<'a>(chunks: &'a [RiffChunk<'a>]) -> Vec<&'a [u8]> {
    chunks
        .iter()
        .filter_map(|chunk| {
            if &chunk.id == b"LIST" && chunk.data.len() >= 4 {
                if &chunk.data[..4] == b"INFO" {
                    return Some(&chunk.data[4..]);
                }
            }
            None
        })
        .collect()
}

impl RiffMeta {
    /// Parses RIFF metadata from raw file bytes.
    ///
    /// This function:
    /// - Parses all RIFF chunks recursively
    /// - Extracts `LIST/INFO` chunks
    /// - Decodes key-value metadata entries
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let chunks = parse_riff_chunks(data)?;
        let info_chunks = find_info_chunks(&chunks);

        let mut meta = RiffMeta::default();

        for info in info_chunks {
            let mut i = 0;

            while i + 8 <= info.len() {
                let tag = &info[i..i + 4];
                i += 4;

                let len = u32::from_le_bytes(
                    info[i..i + 4]
                        .try_into()
                        .map_err(|_| ParserError { message: "Invalid entry length".into() })?,
                ) as usize;
                i += 4;

                if i + len > info.len() {
                    return Err(ParserError {
                        message: "Entry out of bounds".into(),
                    });
                }

                let raw = &info[i..i + len];
                i += len;

                // Word alignment padding
                if len % 2 == 1 {
                    i += 1;
                }

                let value = String::from_utf8_lossy(raw)
                    .trim_end_matches('\0')
                    .to_string();

                // Map tag → field (unchanged)
                match tag {
                    b"INAM" | b"TITL" => meta.title = Some(value),
                    b"IART" => meta.artist = Some(value),
                    b"GENR" | b"IGNR" => meta.genre = Some(value),
                    b"YEAR" => meta.year = Some(value),
                    _ => {}
                }
            }
        }

        Ok(meta)
    }
}



/// Represents a single RIFF chunk.
///
/// A chunk consists of:
/// - 4-byte identifier (e.g., "RIFF", "LIST", "data")
/// - variable-length data payload
#[derive(Debug)]
pub struct RiffChunk<'a> {
    /// Chunk identifier.
    pub id: [u8; 4],
    /// Chunk payload data.
    pub data: &'a [u8],
}

/// Parses all RIFF chunks from a byte slice.
///
/// This function:
/// - Iterates through chunks
/// - Validates bounds
/// - Recursively parses nested `RIFF` and `LIST` chunks
pub fn parse_riff_chunks<'a>(data: &'a [u8]) -> Result<Vec<RiffChunk<'a>>, ParserError> {
    let mut chunks = Vec::new();
    let mut i = 0;

    while i + 8 <= data.len() {
        let id: [u8; 4] = data[i..i + 4]
            .try_into()
            .map_err(|_| ParserError { message: "Invalid chunk id".into() })?;

        let size = u32::from_le_bytes(
            data[i + 4..i + 8]
                .try_into()
                .map_err(|_| ParserError { message: "Invalid chunk size".into() })?,
        ) as usize;

        i += 8;

        if i + size > data.len() {
            return Err(ParserError {
                message: "Chunk out of bounds".into(),
            });
        }

        let chunk_data = &data[i..i + size];

        chunks.push(RiffChunk { id, data: chunk_data });

        // Recursively parse nested chunks
        if &id == b"RIFF" || &id == b"LIST" {
            if size >= 4 {
                let sub_chunks = parse_riff_chunks(&chunk_data[4..])?;
                chunks.extend(sub_chunks);
            }
        }

        i += size;

        // Word alignment padding
        if size % 2 == 1 {
            i += 1;
        }
    }

    Ok(chunks)
}

/*
Tag ID	Tag Name	Writable	Values / Notes
'AGES'	Rated	no	 
'CMNT'	Comment	no	 
'CODE'	EncodedBy	no	 
'COMM'	Comments	no	 
'DIRC'	Directory	no	 
'DISP'	SoundSchemeTitle	no	 
'DTIM'	DateTimeOriginal	no	 
'GENR'	Genre	no	 
'IARL'	ArchivalLocation	no	 
'IART'	Artist	no	 
'IAS1'	FirstLanguage	no	 
'IAS2'	SecondLanguage	no	 
'IAS3'	ThirdLanguage	no	 
'IAS4'	FourthLanguage	no	 
'IAS5'	FifthLanguage	no	 
'IAS6'	SixthLanguage	no	 
'IAS7'	SeventhLanguage	no	 
'IAS8'	EighthLanguage	no	 
'IAS9'	NinthLanguage	no	 
'IBSU'	BaseURL	no	 
'ICAS'	DefaultAudioStream	no	 
'ICDS'	CostumeDesigner	no	 
'ICMS'	Commissioned	no	 
'ICMT'	Comment	no	 
'ICNM'	Cinematographer	no	 
'ICNT'	Country	no	 
'ICOP'	Copyright	no	 
'ICRD'	DateCreated	no	 
'ICRP'	Cropped	no	 
'IDIM'	Dimensions	no	 
'IDIT'	DateTimeOriginal	no	 
'IDPI'	DotsPerInch	no	 
'IDST'	DistributedBy	no	 
'IEDT'	EditedBy	no	 
'IENC'	EncodedBy	no	 
'IENG'	Engineer	no	 
'IGNR'	Genre	no	 
'IKEY'	Keywords	no	 
'ILGT'	Lightness	no	 
'ILGU'	LogoURL	no	 
'ILIU'	LogoIconURL	no	 
'ILNG'	Language	no	 
'IMBI'	MoreInfoBannerImage	no	 
'IMBU'	MoreInfoBannerURL	no	 
'IMED'	Medium	no	 
'IMIT'	MoreInfoText	no	 
'IMIU'	MoreInfoURL	no	 
'IMUS'	MusicBy	no	 
'INAM'	Title	no	 
'IPDS'	ProductionDesigner	no	 
'IPLT'	NumColors	no	 
'IPRD'	Product	no	 
'IPRO'	ProducedBy	no	 
'IRIP'	RippedBy	no	 
'IRTD'	Rating	no	 
'ISBJ'	Subject	no	 
'ISFT'	Software	no	 
'ISGN'	SecondaryGenre	no	 
'ISHP'	Sharpness	no	 
'ISMP'	TimeCode	no	 
'ISRC'	Source	no	 
'ISRF'	SourceForm	no	 
'ISTD'	ProductionStudio	no	 
'ISTR'	Starring	no	 
'ITCH'	Technician	no	 
'ITRK'	TrackNumber	no	 
'IWMU'	WatermarkURL	no	 
'IWRI'	WrittenBy	no	 
'LANG'	Language	no	 
'LOCA'	Location	no	 
'PRT1'	Part	no	 
'PRT2'	NumberOfParts	no	 
'RATE'	Rate	no	 
'STAR'	Starring	no	 
'STAT'	Statistics	no	[Value 3]
0 = Bad
1 = OK
'TAPE'	TapeName	no	 
'TCDO'	EndTimecode	no	 
'TCOD'	StartTimecode	no	 
'TITL'	Title	no	 
'TLEN'	Length	no	 
'TORG'	Organization	no	 
'TRCK'	TrackNumber	no	 
'TURL'	URL	no	 
'TVER'	Version	no	 
'VMAJ'	VegasVersionMajor	no	 
'VMIN'	VegasVersionMinor	no	 
'YEAR'	Year	no	 
*/