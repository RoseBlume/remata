use crate::ParserError;
use super::{MetaArt, MetaExt};
use std::io::{Read, Seek, SeekFrom};
use remata_macros::DisplayPretty;
/// Represents metadata extracted from RIFF INFO chunks.
///
/// RIFF (Resource Interchange File Format) is used by formats like WAV and AVI.
/// Metadata is typically stored inside `LIST` chunks of type `INFO`, where each
/// entry is a 4-byte tag followed by a value.
///
/// Each field corresponds to a known RIFF INFO tag. All fields are optional
/// because metadata presence varies across files.
#[derive(Default, Clone, DisplayPretty)]
pub struct RiffMeta {
    /// Content rating or age classification (`AGES`).
    ///
    /// May indicate audience suitability (e.g., "PG", "18+").
    pub ages: Option<String>,

    /// Short comment field (`CMNT`, `ICMT`).
    ///
    /// Typically a brief description or note.
    pub comment: Option<String>,

    /// Extended or multi-line comment (`COMM`).
    ///
    /// May contain more detailed descriptive text.
    pub comments: Option<String>,

    /// Directory or file location reference (`DIRC`).
    ///
    /// Can indicate where the original media is stored.
    pub directory: Option<String>,

    /// Display or sound scheme title (`DISP`).
    ///
    /// Often used in UI-related contexts.
    pub sound_scheme_title: Option<String>,

    /// Archival storage location (`IARL`).
    ///
    /// Describes where the media is physically or logically archived.
    pub archival_location: Option<String>,

    /// Artist or primary creator (`IART`).
    pub artist: Option<String>,

    /// First language entry (`IAS1`).
    pub first_language: Option<String>,
    /// Second language entry (`IAS2`).
    pub second_language: Option<String>,
    /// Third language entry (`IAS3`).
    pub third_language: Option<String>,
    /// Fourth language entry (`IAS4`).
    pub fourth_language: Option<String>,
    /// Fifth language entry (`IAS5`).
    pub fifth_language: Option<String>,
    /// Sixth language entry (`IAS6`).
    pub sixth_language: Option<String>,
    /// Seventh language entry (`IAS7`).
    pub seventh_language: Option<String>,
    /// Eighth language entry (`IAS8`).
    pub eighth_language: Option<String>,
    /// Ninth language entry (`IAS9`).
    pub ninth_language: Option<String>,

    /// Base URL for related resources (`IBSU`).
    ///
    /// Can be used as a prefix for other relative URLs.
    pub base_url: Option<String>,

    /// Default audio stream identifier (`ICAS`).
    ///
    /// Indicates the primary audio stream in multi-stream content.
    pub default_audio_stream: Option<String>,

    /// Costume designer (`ICDS`).
    pub costume_designer: Option<String>,

    /// Commissioning information (`ICMS`).
    ///
    /// May indicate whether the work was commissioned.
    pub commissioned: Option<String>,

    /// Cinematographer or director of photography (`ICNM`).
    pub cinematographer: Option<String>,

    /// Country of origin (`ICNT`).
    pub country: Option<String>,

    /// Copyright information (`ICOP`).
    pub copyright: Option<String>,

    /// Creation date (`ICRD`).
    ///
    /// Format may vary (e.g., ISO date or free text).
    pub date_created: Option<String>,

    /// Cropping or framing information (`ICRP`).
    ///
    /// Describes whether the media has been cropped.
    pub cropped: Option<String>,

    /// Dimensions of the media (`IDIM`).
    ///
    /// Typically expressed as width × height.
    pub dimensions: Option<String>,

    /// Original date/time (`DTIM`, `IDIT`).
    ///
    /// Represents when the media was originally created.
    pub date_time_original: Option<String>,

    /// Resolution in dots per inch (`IDPI`).
    pub dots_per_inch: Option<String>,

    /// Distributor information (`IDST`).
    pub distributed_by: Option<String>,

    /// Editor (`IEDT`).
    pub edited_by: Option<String>,

    /// Encoder or encoding software (`CODE`, `IENC`).
    pub encoded_by: Option<String>,

    /// Audio or video engineer (`IENG`).
    pub engineer: Option<String>,

    /// Genre classification (`GENR`, `IGNR`).
    pub genre: Option<String>,

    /// Keywords or tags (`IKEY`).
    ///
    /// Often comma-separated.
    pub keywords: Option<String>,

    /// Lightness or brightness setting (`ILGT`).
    pub lightness: Option<String>,

    /// Logo URL (`ILGU`).
    pub logo_url: Option<String>,

    /// Logo icon URL (`ILIU`).
    pub logo_icon_url: Option<String>,

    /// Banner image reference (`IMBI`).
    pub more_info_banner_image: Option<String>,

    /// Banner URL (`IMBU`).
    pub more_info_banner_url: Option<String>,

    /// Medium type (`IMED`).
    ///
    /// Example: "CD", "DVD", "Digital".
    pub medium: Option<String>,

    /// Additional descriptive text (`IMIT`).
    pub more_info_text: Option<String>,

    /// Additional information URL (`IMIU`).
    pub more_info_url: Option<String>,

    /// Music composer or author (`IMUS`).
    pub music_by: Option<String>,

    /// Title of the media (`INAM`, `TITL`).
    pub title: Option<String>,

    /// Production designer (`IPDS`).
    pub production_designer: Option<String>,

    /// Number of colors (`IPLT`).
    ///
    /// Relevant for indexed or palette-based media.
    pub num_colors: Option<String>,

    /// Product or collection name (`IPRD`).
    pub product: Option<String>,

    /// Producer (`IPRO`).
    pub produced_by: Option<String>,

    /// Ripping source or user (`IRIP`).
    pub ripped_by: Option<String>,

    /// Rating or classification (`IRTD`).
    pub rating: Option<String>,

    /// Subject or topic (`ISBJ`).
    pub subject: Option<String>,

    /// Software used (`ISFT`).
    pub software: Option<String>,

    /// Secondary genre (`ISGN`).
    pub secondary_genre: Option<String>,

    /// Sharpness setting (`ISHP`).
    pub sharpness: Option<String>,

    /// SMPTE timecode (`ISMP`).
    ///
    /// Used for synchronization in video/audio production.
    pub time_code: Option<String>,

    /// Source identifier (`ISRC`).
    pub source: Option<String>,

    /// Source form (`ISRF`).
    ///
    /// Example: "analog", "digital".
    pub source_form: Option<String>,

    /// Production studio (`ISTD`).
    pub production_studio: Option<String>,

    /// Starring actors or featured performers (`ISTR`, `STAR`).
    pub starring: Option<String>,

    /// Technician or technical contributor (`ITCH`).
    pub technician: Option<String>,

    /// Track number (`ITRK`, `TRCK`).
    ///
    /// May include total tracks (e.g., "3/12").
    pub track_number: Option<String>,

    /// Watermark URL (`IWMU`).
    pub watermark_url: Option<String>,

    /// Writer or author (`IWRI`).
    pub written_by: Option<String>,

    /// Language (`LANG`, `ILNG`).
    pub language: Option<String>,

    /// Location information (`LOCA`).
    pub location: Option<String>,

    /// Part index (`PRT1`).
    ///
    /// Indicates the current part in a multi-part work.
    pub part: Option<String>,

    /// Total number of parts (`PRT2`).
    pub number_of_parts: Option<String>,

    /// Rating or score (`RATE`).
    ///
    /// Often numeric or star-based.
    pub rate: Option<String>,

    /// Statistical information (`STAT`).
    ///
    /// May include play counts or other metrics.
    pub statistics: Option<String>,

    /// Tape or media name (`TAPE`).
    pub tape_name: Option<String>,

    /// End timecode (`TCDO`).
    pub end_timecode: Option<String>,

    /// Start timecode (`TCOD`).
    pub start_timecode: Option<String>,

    /// Duration or length (`TLEN`).
    ///
    /// Often expressed in milliseconds.
    pub length: Option<String>,

    /// Organization or publisher (`TORG`).
    pub organization: Option<String>,

    /// URL (`TURL`).
    pub url: Option<String>,

    /// Version identifier (`TVER`).
    pub version: Option<String>,

    /// Vegas software major version (`VMAJ`).
    pub vegas_version_major: Option<String>,

    /// Vegas software minor version (`VMIN`).
    pub vegas_version_minor: Option<String>,

    /// Year of release or creation (`YEAR`).
    pub year: Option<String>,
}





/// Parses a LIST/INFO chunk payload.
///
/// Extracts key-value metadata entries.
fn parse_info_chunk<R: Read + Seek>(
    reader: &mut R,
    start: u64,
    size: u64,
    meta: &mut RiffMeta,
) -> Result<(), ParserError> {
    let mut pos = start;

    while pos + 8 <= start + size {
        reader.seek(SeekFrom::Start(pos))?;

        let tag = read_fourcc(reader)?;
        let len = read_u32_le(reader)? as u64;

        if pos + 8 + len > start + size {
            return Err(ParserError {
                message: "Entry out of bounds".into(),
            });
        }

        let mut buf = vec![0u8; len as usize];
        reader.read_exact(&mut buf)?;

        let value = String::from_utf8_lossy(&buf)
            .trim_end_matches('\0')
            .to_string();

        match &tag {
            b"INAM" | b"TITL" => meta.title = Some(value),
            b"IART" => meta.artist = Some(value),
            b"GENR" | b"IGNR" => meta.genre = Some(value),
            b"YEAR" => meta.year = Some(value),
            _ => {}
        }

        pos += 8 + len;

        // Word alignment padding
        if len % 2 == 1 {
            pos += 1;
        }
    }

    Ok(())
}



impl RiffMeta {
    /// Parses RIFF metadata from a reader.
    ///
    /// This function:
    /// - Parses all RIFF chunks recursively
    /// - Extracts `LIST/INFO` chunks
    /// - Decodes key-value metadata entries
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut meta = RiffMeta::default();

        // Skip RIFF header (12 bytes: "RIFF" + size + type)
        reader.seek(SeekFrom::Start(12))?;

        parse_riff_chunks(reader, None, &mut |id, data_start, size, reader| {
            // Only care about LIST/INFO
            if &id == b"LIST" {
                let mut kind = [0u8; 4];
                reader.seek(SeekFrom::Start(data_start))?;
                reader.read_exact(&mut kind)?;

                if &kind == b"INFO" {
                    parse_info_chunk(reader, data_start + 4, size - 4, &mut meta)?;
                }
            }

            Ok(())
        })?;

        Ok(meta)
    }
}





/// Parses all RIFF chunks from a reader.
///
/// This function:
/// - Iterates through chunks
/// - Validates bounds
/// - Recursively parses nested `RIFF` and `LIST` chunks
fn parse_riff_chunks<R: Read + Seek, F>(
    reader: &mut R,
    limit: Option<u64>,
    callback: &mut F,
) -> Result<(), ParserError>
where
    F: FnMut([u8; 4], u64, u64, &mut R) -> Result<(), ParserError>,
{
    let start = reader.stream_position()?;
    let end = match limit {
        Some(l) => start + l,
        None => reader.seek(SeekFrom::End(0))?,
    };

    reader.seek(SeekFrom::Start(start))?;

    while reader.stream_position()? + 8 <= end {
        let chunk_start = reader.stream_position()?;

        let id = read_fourcc(reader)?;
        let size = read_u32_le(reader)? as u64;

        let data_start = reader.stream_position()?;

        if data_start + size > end {
            return Err(ParserError {
                message: "Chunk out of bounds".into(),
            });
        }

        // Let caller inspect chunk
        callback(id, data_start, size, reader)?;

        // Recurse into LIST/RIFF
        if &id == b"RIFF" || &id == b"LIST" {
            if size >= 4 {
                reader.seek(SeekFrom::Start(data_start + 4))?;
                parse_riff_chunks(reader, Some(size - 4), callback)?;
            }
        }

        // Move to next chunk
        reader.seek(SeekFrom::Start(data_start + size))?;

        // Word alignment padding
        if size % 2 == 1 {
            reader.seek(SeekFrom::Current(1))?;
        }

        // Safety guard
        if reader.stream_position()? <= chunk_start {
            break;
        }
    }

    Ok(())
}

/// Reads a 4-byte chunk identifier.
fn read_fourcc<R: Read>(reader: &mut R) -> Result<[u8; 4], ParserError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

/// Reads a little-endian `u32`.
fn read_u32_le<R: Read>(reader: &mut R) -> Result<u32, ParserError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

impl MetaExt for RiffMeta {
    fn title(&self) -> Option<String> {
        self.title
            .as_ref()
            .or(self.product.as_ref())
            .or(self.sound_scheme_title.as_ref())
            .cloned()
    }

    fn artist(&self) -> Option<String> {
        self.artist
            .as_ref()
            .or(self.music_by.as_ref())
            .or(self.produced_by.as_ref())
            .or(self.engineer.as_ref())
            .cloned()
    }

    fn album(&self) -> Option<String> {
        self.product
            .as_ref()
            .or(self.organization.as_ref())
            .cloned()
    }

    fn genre(&self) -> Option<String> {
        self.genre
            .as_ref()
            .or(self.secondary_genre.as_ref())
            .cloned()
    }

    fn year(&self) -> Option<String> {
        self.year
            .as_ref()
            .or(self.date_created.as_ref())
            .or(self.date_time_original.as_ref())
            .cloned()
    }

    fn art(&self) -> Option<MetaArt> {
        if let Some(url) = self.more_info_banner_image.as_ref() {
            return Some(MetaArt::Url(url.clone()));
        }
        if let Some(url) = self.more_info_banner_url.as_ref() {
            return Some(MetaArt::Url(url.clone()));
        }
        if let Some(url) = self.logo_url.as_ref() {
            return Some(MetaArt::Url(url.clone()));
        }
        if let Some(url) = self.logo_icon_url.as_ref() {
            return Some(MetaArt::Url(url.clone()));
        }
        None
    }
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