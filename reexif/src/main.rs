mod tags;
use tags::TAGS;
use std::env;
use std::fs::{self, File};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

/// TIFF/Exif field type identifiers used in an IFD entry.
///
/// These values define how the data in a field should be interpreted
/// and how many bytes each unit occupies.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfdType {
    /// 8-bit unsigned integer.
    Byte = 1,

    /// 8-bit ASCII character. Strings are NULL-terminated and include the NULL byte in the count.
    Ascii = 2,

    /// 16-bit unsigned integer.
    Short = 3,

    /// 32-bit unsigned integer.
    Long = 4,

    /// Two LONG values: numerator and denominator.
    Rational = 5,

    /// Arbitrary 8-bit data.
    Undefined = 7,

    /// 32-bit signed integer (two's complement).
    SLong = 9,

    /// Two signed 32-bit integers: numerator and denominator.
    SRational = 10,

    /// UTF-8 encoded string (non-standard TIFF extension).
    /// Must be NULL-terminated and must not include BOM.
    Utf8 = 129,
}

impl TryFrom<u16> for IfdType {
    type Error = ();

    /// Attempts to convert a raw 16-bit TIFF type identifier into an `IfdType`.
    ///
    /// # Errors
    /// Returns `Err(())` if the provided value does not correspond to a known type.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => IfdType::Byte,
            2 => IfdType::Ascii,
            3 => IfdType::Short,
            4 => IfdType::Long,
            5 => IfdType::Rational,
            7 => IfdType::Undefined,
            9 => IfdType::SLong,
            10 => IfdType::SRational,
            129 => IfdType::Utf8,
            _ => return Err(()),
        })
    }
}

/// Represents a rational number as defined by TIFF.
///
/// A rational is stored as two unsigned 32-bit integers:
/// numerator and denominator.
#[derive(Debug, Clone)]
pub struct Rational {
    /// Numerator of the rational value.
    pub numerator: u32,

    /// Denominator of the rational value.
    /// Must not be zero.
    pub denominator: u32,
}

/// Represents a signed rational number as defined by TIFF.
///
/// A signed rational consists of two signed 32-bit integers:
/// numerator and denominator.
#[derive(Debug, Clone)]
pub struct SRational {
    /// Numerator of the signed rational value.
    pub numerator: i32,

    /// Denominator of the signed rational value.
    /// Must not be zero.
    pub denominator: i32,
}

/// Represents the decoded value of an IFD entry.
///
/// This enum is used after parsing or before encoding to represent
/// the actual semantic value stored in the TIFF field.
#[derive(Debug, Clone)]
pub enum IfdValue {
    /// One or more raw bytes.
    Byte(Vec<u8>),

    /// ASCII string including terminating NULL byte.
    Ascii(String),

    /// One or more 16-bit unsigned integers.
    Short(Vec<u16>),

    /// One or more 32-bit unsigned integers.
    Long(Vec<u32>),

    /// One or more rational values (numerator/denominator pairs).
    Rational(Vec<Rational>),

    /// Raw uninterpreted bytes.
    Undefined(Vec<u8>),

    /// One or more 32-bit signed integers.
    SLong(Vec<i32>),

    /// One or more signed rational values.
    SRational(Vec<SRational>),

    /// UTF-8 string including terminating NULL byte.
    Utf8(String),
}

/// A single 12-byte TIFF IFD (Image File Directory) entry.
///
/// Each entry defines a tag and how to interpret its associated data.
/// The entry may either store the value directly (if it fits in 4 bytes)
/// or store an offset pointing to the actual value in the file.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IfdEntry {
    /// 2-byte tag identifier that defines the meaning of the field.
    ///
    /// Tags are unique within the IFD and follow TIFF/Exif standards.
    pub tag: u16,

    /// The data type of the value (e.g., BYTE, SHORT, ASCII, etc.).
    pub value_type: IfdType,

    /// Number of values, not number of bytes.
    ///
    /// For example, a SHORT has size 2 bytes, but count = 1 means one SHORT.
    pub count: u32,

    /// Either:
    /// - The value itself (if ≤ 4 bytes), OR
    /// - An offset from the TIFF header to the actual value data.
    pub value_offset_or_inline: u32,
}

impl IfdEntry {
    /// Creates a new IFD entry.
    ///
    /// This constructor does not enforce whether the value fits inline or requires an offset.
    /// That logic must be handled during serialization.
    ///
    /// # Parameters
    /// - `tag`: TIFF/Exif tag identifier
    /// - `value_type`: Data type of the field
    /// - `count`: Number of values
    /// - `value_offset_or_inline`: Either inline value or offset to value data
    pub fn new(
        tag: u16,
        value_type: IfdType,
        count: u32,
        value_offset_or_inline: u32,
    ) -> Self {
        Self {
            tag,
            value_type,
            count,
            value_offset_or_inline,
        }
    }
}

/// Represents a TIFF Image File Directory (IFD).
///
/// An IFD is a collection of entries describing image metadata.
/// It begins with a count of entries, followed by the entries themselves,
/// and ends with a pointer to the next IFD.
#[derive(Debug, Clone)]
pub struct Ifd {
    /// Number of entries in this directory.
    pub count: u16,

    /// Collection of IFD entries sorted by tag (ascending order).
    ///
    /// Sorting is required by the TIFF specification.
    pub entries: Vec<IfdEntry>,

    /// Offset to the next IFD in the file.
    ///
    /// A value of 0 indicates that there are no further IFDs.
    pub next_ifd_offset: u32,
}

impl Ifd {
    /// Creates a new IFD from a list of entries and a next-IFD offset.
    ///
    /// Entries are automatically sorted by tag as required by the TIFF specification.
    ///
    /// # Parameters
    /// - `entries`: List of IFD entries (unsorted allowed)
    /// - `next_ifd_offset`: Offset to the next IFD (0 if none)
    pub fn new(mut entries: Vec<IfdEntry>, next_ifd_offset: u32) -> Self {
        // Ensure compliance with TIFF requirement: entries must be sorted by tag.
        entries.sort_by_key(|e| e.tag);

        let count = entries
            .len()
            .try_into()
            .unwrap_or(u16::MAX);

        Self {
            count,
            entries,
            next_ifd_offset,
        }
    }
}

/// Read a big-endian u16 (JPEG markers are always big-endian)
fn read_be_u16(file: &mut File) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

fn read_u16(buf: &[u8], offset: usize, little: bool) -> u16 {
    let bytes = [buf[offset], buf[offset + 1]];
    if little {
        u16::from_le_bytes(bytes)
    } else {
        u16::from_be_bytes(bytes)
    }
}

fn read_u32(buf: &[u8], offset: usize, little: bool) -> u32 {
    let bytes = [
        buf[offset],
        buf[offset + 1],
        buf[offset + 2],
        buf[offset + 3],
    ];
    if little {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    }
}

/// Finds the TIFF header inside a JPEG EXIF segment.
/// Returns absolute file offset where TIFF header begins.
fn find_exif_tiff_start(file: &mut File) -> io::Result<u64> {
    file.seek(SeekFrom::Start(0))?;

    let mut soi = [0u8; 2];
    file.read_exact(&mut soi)?;

    if soi != [0xFF, 0xD8] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a JPEG file",
        ));
    }

    loop {
        let mut marker_prefix = [0u8; 1];
        file.read_exact(&mut marker_prefix)?;

        if marker_prefix[0] != 0xFF {
            continue;
        }

        let mut marker = [0u8; 1];
        file.read_exact(&mut marker)?;

        let marker = marker[0];

        // Skip padding FFs
        if marker == 0xFF {
            continue;
        }

        // Standalone markers
        if marker == 0xD9 || marker == 0xDA {
            break;
        }

        let length = read_be_u16(file)? as u64;

        if marker == 0xE1 {
            // APP1 segment
            let mut header = [0u8; 6];
            file.read_exact(&mut header)?;

            if &header == b"Exif\0\0" {
                let pos = file.stream_position()?;
                return Ok(pos);
            } else {
                file.seek(SeekFrom::Current((length - 8) as i64))?;
            }
        } else {
            file.seek(SeekFrom::Current((length - 2) as i64))?;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No EXIF segment found",
    ))
}

fn parse_ifd(
    file: &mut File,
    base_offset: u64,
    offset: u32,
    little: bool,
) -> io::Result<Option<(Vec<IfdEntry>, u32)>> {
    if offset == 0 {
        return Ok(None);
    }

    let absolute = base_offset + offset as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut count_buf = [0u8; 2];
    file.read_exact(&mut count_buf)?;

    let count = if little {
        u16::from_le_bytes(count_buf)
    } else {
        u16::from_be_bytes(count_buf)
    };

    let mut entries_buf = vec![0u8; count as usize * 12];
    file.read_exact(&mut entries_buf)?;

    let mut entries = Vec::new();

    for i in 0..count as usize {
        let base = i * 12;

        let tag = read_u16(&entries_buf, base, little);
        let value_type_raw = read_u16(&entries_buf, base + 2, little);

        let value_type = match IfdType::try_from(value_type_raw) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let count = read_u32(&entries_buf, base + 4, little);
        let value_offset_or_inline = read_u32(&entries_buf, base + 8, little);

        entries.push(IfdEntry::new(
            tag,
            value_type,
            count,
            value_offset_or_inline,
        ));
    }

    let mut next_buf = [0u8; 4];
    file.read_exact(&mut next_buf)?;

    let next_ifd_offset = if little {
        u32::from_le_bytes(next_buf)
    } else {
        u32::from_be_bytes(next_buf)
    };

    Ok(Some((entries, next_ifd_offset)))
}

fn read_rational_strings(
    file: &mut File,
    base_offset: u64,
    entry: &IfdEntry,
    little: bool,
) -> io::Result<Vec<String>> {
    let absolute = base_offset + entry.value_offset_or_inline as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; (entry.count * 8) as usize];
    file.read_exact(&mut buf)?;

    let mut result = Vec::new();

    for i in 0..entry.count as usize {
        let o = i * 8;

        let num = if little {
            u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        } else {
            u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        };

        let den = if little {
            u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        } else {
            u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        };

        if den == 0 {
            result.push(format!("{}/{} (NaN)", num, den));
        } else {
            let value = num as f64 / den as f64;
            result.push(format!("{}/{} ({:.6})", num, den, value));
        }
    }

    Ok(result)
}

fn read_srational_strings(
    file: &mut File,
    base_offset: u64,
    entry: &IfdEntry,
    little: bool,
) -> io::Result<Vec<String>> {
    let absolute = base_offset + entry.value_offset_or_inline as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; (entry.count * 8) as usize];
    file.read_exact(&mut buf)?;

    let mut result = Vec::new();

    for i in 0..entry.count as usize {
        let o = i * 8;

        let num = if little {
            i32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        } else {
            i32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        };

        let den = if little {
            i32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        } else {
            i32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        };

        if den == 0 {
            result.push(format!("{}/{} (NaN)", num, den));
        } else {
            let value = num as f64 / den as f64;
            result.push(format!("{}/{} ({:.6})", num, den, value));
        }
    }

    Ok(result)
}

fn tag_name(tag: u16) -> &'static str {
    TAGS
        .iter()
        .find(|(t, _)| *t == tag)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
}



fn process_file(path: &str, output: Option<&str>) -> io::Result<()> {
    let mut file = std::fs::File::open(path)?;

    let mut out: Box<dyn Write> = match output {
        Some(o) => Box::new(std::fs::File::create(o)?),
        None => Box::new(io::stdout()),
    };

    writeln!(out, "File: {}\n", path)?;

    // Detect format
    let mut magic = [0u8; 2];
    file.read_exact(&mut magic)?;

    let tiff_start = if magic == [0xFF, 0xD8] {
        writeln!(out, "Detected JPEG, scanning for EXIF...")?;
        find_exif_tiff_start(&mut file)?
    } else {
        writeln!(out, "Assuming TIFF file...")?;
        0
    };

    file.seek(SeekFrom::Start(tiff_start))?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;

    let little = match &header[0..2] {
        b"II" => true,
        b"MM" => false,
        _ => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid TIFF header"));
        }
    };

    let magic = if little {
        u16::from_le_bytes([header[2], header[3]])
    } else {
        u16::from_be_bytes([header[2], header[3]])
    };

    if magic != 42 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid TIFF magic"));
    }

    let first_ifd_offset = if little {
        u32::from_le_bytes([header[4], header[5], header[6], header[7]])
    } else {
        u32::from_be_bytes([header[4], header[5], header[6], header[7]])
    };

    let mut current_offset = first_ifd_offset;
    let mut index = 0;

    while let Some((entries, next_offset)) =
        parse_ifd(&mut file, tiff_start, current_offset, little)?
    {
        writeln!(out, "IFD {}:", index)?;

        for e in &entries {
            let name = tag_name(e.tag);

            match e.value_type {
                IfdType::Ascii => {
                    match read_ascii(&mut file, tiff_start, e.value_offset_or_inline, e.count) {
                        Ok(s) => writeln!(out, "  {:04X} ({}): {}", e.tag, name, s)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                IfdType::Rational => {
                    match read_rational_strings(&mut file, tiff_start, e, little) {
                        Ok(values) => writeln!(out, "  {:04X} ({}): {:?}", e.tag, name, values)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                IfdType::SRational => {
                    match read_srational_strings(&mut file, tiff_start, e, little) {
                        Ok(values) => writeln!(out, "  {:04X} ({}): {:?}", e.tag, name, values)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                IfdType::Long if e.tag == 0x8825 => {
                    writeln!(out, "  8825 (GPS IFD)")?;
                    match parse_gps_ifd(&mut file, tiff_start, e.value_offset_or_inline, little) {
                        Ok(gps) => writeln!(out, "    GPS: {:?}", gps)?,
                        Err(_) => writeln!(out, "    GPS: <error>")?,
                    }
                }

                IfdType::Byte
                | IfdType::Short
                | IfdType::Long
                | IfdType::SLong
                | IfdType::Undefined => {
                    match read_numeric_values(&mut file, tiff_start, e, little) {
                        Ok(values) => writeln!(out, "  {:04X} ({}): {:?}", e.tag, name, values)?,
                        Err(_) => writeln!(out, "  {:04X} ({}): <error>", e.tag, name)?,
                    }
                }

                _ => {
                    writeln!(
                        out,
                        "  {:04X} ({}): type={:?}, count={}",
                        e.tag, name, e.value_type, e.count
                    )?;
                }
            }
        }

        writeln!(out)?;

        if next_offset == 0 {
            break;
        }

        current_offset = next_offset;
        index += 1;
    }

    Ok(())
}

fn process_path(path: &str, output: Option<&str>) -> io::Result<()> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        process_file(path, output)?;
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();

            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext == "jpg" || ext == "jpeg" || ext == "tif" || ext == "tiff" {
                        let path_str = p.to_string_lossy();
                        process_file(&path_str, output)?;
                    }
                }
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not file or directory"));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file|directory> [--output file.txt]", args[0]);
        return Ok(());
    }

    let input = &args[1];
    let mut output: Option<&str> = None;

    for i in 2..args.len() {
        if args[i] == "--output" && i + 1 < args.len() {
            output = Some(&args[i + 1]);
        }
    }

    process_path(input, output)
}
fn read_ascii(
    file: &mut File,
    base_offset: u64,
    offset: u32,
    count: u32,
) -> io::Result<String> {
    let absolute = base_offset + offset as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; count as usize];
    file.read_exact(&mut buf)?;

    // Remove trailing NULL if present
    if let Some(pos) = buf.iter().position(|&b| b == 0) {
        buf.truncate(pos);
    }

    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn type_size(t: IfdType) -> usize {
    match t {
        IfdType::Byte | IfdType::Ascii | IfdType::Undefined | IfdType::Utf8 => 1,
        IfdType::Short => 2,
        IfdType::Long | IfdType::SLong => 4,
        IfdType::Rational | IfdType::SRational => 8,
    }
}

fn read_numeric_values(
    file: &mut File,
    base_offset: u64,
    entry: &IfdEntry,
    little: bool,
) -> io::Result<Vec<u64>> {
    let elem_size = type_size(entry.value_type);
    let total_size = elem_size * entry.count as usize;

    let mut buf = vec![0u8; total_size];

    if total_size <= 4 {
        // INLINE
        let raw = if little {
            entry.value_offset_or_inline.to_le_bytes()
        } else {
            entry.value_offset_or_inline.to_be_bytes()
        };

        buf[..total_size].copy_from_slice(&raw[..total_size]);
    } else {
        // OFFSET
        let absolute = base_offset + entry.value_offset_or_inline as u64;
        file.seek(SeekFrom::Start(absolute))?;
        file.read_exact(&mut buf)?;
    }

    let mut values = Vec::new();

    for i in 0..entry.count as usize {
        let offset = i * elem_size;

        let v = match entry.value_type {
            IfdType::Byte | IfdType::Undefined | IfdType::Utf8 => {
                buf[offset] as u64
            }

            IfdType::Short => {
                let bytes = [buf[offset], buf[offset + 1]];
                if little {
                    u16::from_le_bytes(bytes) as u64
                } else {
                    u16::from_be_bytes(bytes) as u64
                }
            }

            IfdType::Long => {
                let bytes = [
                    buf[offset],
                    buf[offset + 1],
                    buf[offset + 2],
                    buf[offset + 3],
                ];
                if little {
                    u32::from_le_bytes(bytes) as u64
                } else {
                    u32::from_be_bytes(bytes) as u64
                }
            }

            IfdType::SLong => {
                let bytes = [
                    buf[offset],
                    buf[offset + 1],
                    buf[offset + 2],
                    buf[offset + 3],
                ];
                if little {
                    i32::from_le_bytes(bytes) as i64 as u64
                } else {
                    i32::from_be_bytes(bytes) as i64 as u64
                }
            }

            // skip rationals here (handled separately if needed)
            _ => continue,
        };

        values.push(v);
    }

    Ok(values)
}

#[derive(Debug, Default)]
pub struct GpsInfo {
    pub version_id: Option<[u8; 4]>,

    pub latitude_ref: Option<char>,
    pub latitude: Option<f64>,

    pub longitude_ref: Option<char>,
    pub longitude: Option<f64>,

    pub altitude_ref: Option<u8>,
    pub altitude: Option<f64>,

    pub timestamp: Option<String>,

    pub satellites: Option<String>,
    pub status: Option<char>,
    pub measure_mode: Option<String>,

    pub dop: Option<f64>,

    pub speed_ref: Option<char>,
    pub speed: Option<f64>,

    pub track_ref: Option<char>,
    pub track: Option<f64>,

    pub img_direction_ref: Option<char>,
    pub img_direction: Option<f64>,

    pub map_datum: Option<String>,

    pub dest_latitude_ref: Option<char>,
    pub dest_latitude: Option<f64>,

    pub dest_longitude_ref: Option<char>,
    pub dest_longitude: Option<f64>,

    pub dest_bearing_ref: Option<char>,
    pub dest_bearing: Option<f64>,

    pub dest_distance_ref: Option<char>,
    pub dest_distance: Option<f64>,

    pub processing_method: Option<Vec<u8>>,
    pub area_information: Option<Vec<u8>>,

    pub date_stamp: Option<String>,

    pub differential: Option<u16>,
    pub h_positioning_error: Option<f64>,
}

fn rational_to_f64(n: u32, d: u32) -> Option<f64> {
    if d == 0 {
        None
    } else {
        Some(n as f64 / d as f64)
    }
}


fn read_rational(
    file: &mut File,
    base_offset: u64,
    offset: u32,
    count: u32,
    little: bool,
) -> io::Result<Vec<(u32, u32)>> {
    let absolute = base_offset + offset as u64;
    file.seek(SeekFrom::Start(absolute))?;

    let mut buf = vec![0u8; (count * 8) as usize];
    file.read_exact(&mut buf)?;

    let mut result = Vec::new();

    for i in 0..count as usize {
        let o = i * 8;

        let num = if little {
            u32::from_le_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        } else {
            u32::from_be_bytes([buf[o], buf[o+1], buf[o+2], buf[o+3]])
        };

        let den = if little {
            u32::from_le_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        } else {
            u32::from_be_bytes([buf[o+4], buf[o+5], buf[o+6], buf[o+7]])
        };

        result.push((num, den));
    }

    Ok(result)
}

fn dms_to_deg(values: &[(u32, u32)]) -> Option<f64> {
    if values.len() != 3 {
        return None;
    }

    let deg = values[0].0 as f64 / values[0].1 as f64;
    let min = values[1].0 as f64 / values[1].1 as f64;
    let sec = values[2].0 as f64 / values[2].1 as f64;

    Some(deg + min / 60.0 + sec / 3600.0)
}


fn parse_gps_ifd(
    file: &mut File,
    base_offset: u64,
    offset: u32,
    little: bool,
) -> io::Result<GpsInfo> {
    let mut gps = GpsInfo::default();

    if let Some((entries, _)) = parse_ifd(file, base_offset, offset, little)? {
        let mut lat_raw = None;
        let mut lon_raw = None;
        let mut dest_lat_raw = None;
        let mut dest_lon_raw = None;

        for e in entries {
            match e.tag {
                0x0000 => {
                    let vals = read_numeric_values(file, base_offset, &e, little)?;
                    if vals.len() == 4 {
                        gps.version_id = Some([
                            vals[0] as u8,
                            vals[1] as u8,
                            vals[2] as u8,
                            vals[3] as u8,
                        ]);
                    }
                }

                0x0001 => {
                    gps.latitude_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x0002 => {
                    lat_raw = Some(read_rational(file, base_offset, e.value_offset_or_inline, 3, little)?);
                }

                0x0003 => {
                    gps.longitude_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x0004 => {
                    lon_raw = Some(read_rational(file, base_offset, e.value_offset_or_inline, 3, little)?);
                }

                0x0005 => {
                    gps.altitude_ref = read_numeric_values(file, base_offset, &e, little)
                        .ok()
                        .and_then(|v| v.first().copied())
                        .map(|v| v as u8);
                }

                0x0006 => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.altitude = rational_to_f64(*n, *d);
                    }
                }

                0x0007 => {
                    let t = read_rational(file, base_offset, e.value_offset_or_inline, 3, little)?;
                    if t.len() == 3 {
                        gps.timestamp = Some(format!(
                            "{:02}:{:02}:{:02} UTC",
                            t[0].0 / t[0].1,
                            t[1].0 / t[1].1,
                            t[2].0 / t[2].1
                        ));
                    }
                }

                0x0008 => {
                    gps.satellites = read_ascii(file, base_offset, e.value_offset_or_inline, e.count).ok();
                }

                0x0009 => {
                    gps.status = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x000A => {
                    gps.measure_mode = read_ascii(file, base_offset, e.value_offset_or_inline, e.count).ok();
                }

                0x000B => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.dop = rational_to_f64(*n, *d);
                    }
                }

                0x000C => {
                    gps.speed_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x000D => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.speed = rational_to_f64(*n, *d);
                    }
                }

                0x000E => {
                    gps.track_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x000F => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.track = rational_to_f64(*n, *d);
                    }
                }

                0x0010 => {
                    gps.img_direction_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x0011 => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.img_direction = rational_to_f64(*n, *d);
                    }
                }

                0x0012 => {
                    gps.map_datum = read_ascii(file, base_offset, e.value_offset_or_inline, e.count).ok();
                }

                0x0013 => {
                    gps.dest_latitude_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x0014 => {
                    dest_lat_raw = Some(read_rational(file, base_offset, e.value_offset_or_inline, 3, little)?);
                }

                0x0015 => {
                    gps.dest_longitude_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x0016 => {
                    dest_lon_raw = Some(read_rational(file, base_offset, e.value_offset_or_inline, 3, little)?);
                }

                0x0017 => {
                    gps.dest_bearing_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x0018 => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.dest_bearing = rational_to_f64(*n, *d);
                    }
                }

                0x0019 => {
                    gps.dest_distance_ref = read_ascii(file, base_offset, e.value_offset_or_inline, e.count)
                        .ok()
                        .and_then(|s| s.chars().next());
                }

                0x001A => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.dest_distance = rational_to_f64(*n, *d);
                    }
                }

                0x001B => {
                    let size = (e.count as usize).min(1024);
                    let mut buf = vec![0u8; size];
                    let absolute = base_offset + e.value_offset_or_inline as u64;
                    file.seek(SeekFrom::Start(absolute))?;
                    file.read_exact(&mut buf)?;
                    gps.processing_method = Some(buf);
                }

                0x001C => {
                    let size = (e.count as usize).min(1024);
                    let mut buf = vec![0u8; size];
                    let absolute = base_offset + e.value_offset_or_inline as u64;
                    file.seek(SeekFrom::Start(absolute))?;
                    file.read_exact(&mut buf)?;
                    gps.area_information = Some(buf);
                }

                0x001D => {
                    gps.date_stamp = read_ascii(file, base_offset, e.value_offset_or_inline, e.count).ok();
                }

                0x001E => {
                    gps.differential = read_numeric_values(file, base_offset, &e, little)
                        .ok()
                        .and_then(|v| v.first().copied())
                        .map(|v| v as u16);
                }

                0x001F => {
                    let r = read_rational(file, base_offset, e.value_offset_or_inline, 1, little)?;
                    if let Some((n, d)) = r.first() {
                        gps.h_positioning_error = rational_to_f64(*n, *d);
                    }
                }

                _ => {}
            }
        }

        // Convert lat/lon
        if let (Some(v), Some(r)) = (lat_raw, gps.latitude_ref) {
            if let Some(mut d) = dms_to_deg(&v) {
                if r == 'S' {
                    d = -d;
                }
                gps.latitude = Some(d);
            }
        }

        if let (Some(v), Some(r)) = (lon_raw, gps.longitude_ref) {
            if let Some(mut d) = dms_to_deg(&v) {
                if r == 'W' {
                    d = -d;
                }
                gps.longitude = Some(d);
            }
        }

        if let (Some(v), Some(r)) = (dest_lat_raw, gps.dest_latitude_ref) {
            if let Some(mut d) = dms_to_deg(&v) {
                if r == 'S' {
                    d = -d;
                }
                gps.dest_latitude = Some(d);
            }
        }

        if let (Some(v), Some(r)) = (dest_lon_raw, gps.dest_longitude_ref) {
            if let Some(mut d) = dms_to_deg(&v) {
                if r == 'W' {
                    d = -d;
                }
                gps.dest_longitude = Some(d);
            }
        }
    }

    Ok(gps)
}