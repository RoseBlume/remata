use std::io::{
    self,
    Read,
    Seek,
    SeekFrom,
    Error,
    ErrorKind
};
use std::fs::File;
use std::hash::{Hash};
use std::collections::HashMap;
use std::fmt;
use endian_reader::Endian;

use crate::exif::gps::Gps;

use crate::exif::helpers::{
    read_rational_strings,
    read_srational_strings,
    read_numeric_values,
    read_ascii
};

use crate::exif::starts::{
    find_exif_tiff_start,
    find_png_exif_start,
    find_webp_exif_start,
    find_heif_exif_start
};


use crate::exif::ParseMode;
/// TIFF/Exif field type identifiers used in an IFD entry.
///
/// These values define how the data in a field should be interpreted
/// and how many bytes each unit occupies.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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



#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Entry {
    tag_id: u16,
    name: String,
    description: Option<&'static str>,
    ifd_type: IfdType
}

#[derive(Debug)]
pub struct Ifd {
    pub index: usize,
    pub entries: HashMap<Entry, String>,
}


impl fmt::Display for Ifd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IFD {}:", self.index)?;

        for (entry, value) in &self.entries {
            let data_type = match entry.ifd_type {
                IfdType::Ascii => "Ascii",
                IfdType::Byte => "Byte",
                IfdType::Long => "Long",
                IfdType::Rational => "Rational",
                IfdType::SLong => "SLong",
                IfdType::SRational => "SRational",
                IfdType::Short => "Short",
                IfdType::Undefined => "Undefined",
                IfdType::Utf8 => "UTF-8"
            };
            let desc = if let Some(desc) = entry.description {
                format!("\nTag Description: {}", desc)
            }
            else {
                String::new()
            };
            let trimmed_value = value.trim();

            let final_value = if trimmed_value.starts_with('[') && trimmed_value.ends_with(']') && !trimmed_value.contains(',') {
                &trimmed_value[1..trimmed_value.len() - 1]
            } else {
                trimmed_value
            };
            let out = format!("\nEntry:\nTag ID: 0x{:04X}\nTag Name: {}{}\nValue Type: {}\nValue: {}",
                entry.tag_id,
                entry.name,
                desc,
                data_type,
                final_value
            );
            writeln!(f, "{}", out)?;
            // writeln!(f, "  {}: {}", entry, value)?;
        }

        Ok(())
    }
}

impl Ifd {
    pub fn from_file(path: &str, parse_mode: ParseMode) -> io::Result<Vec<Self>> {
        let mut file = std::fs::File::open(path)?;

        // Detect format
        file.seek(SeekFrom::Start(0))?;
        let mut magic = [0u8; 12];
        file.read_exact(&mut magic)?;
        let tiff_start: u64;
        let start: Option<u64> = if &magic[0..2] == b"\xFF\xD8" {
            Some(find_exif_tiff_start(&mut file)?)
        } else if &magic[0..8] == b"\x89PNG\r\n\x1a\n" {
            Some(find_png_exif_start(&mut file)?)
        } else if &magic[0..4] == b"RIFF" && &magic[8..12] == b"WEBP" {
            Some(find_webp_exif_start(&mut file)?)
        } else if &magic[4..8] == b"ftyp" {
            Some(find_heif_exif_start(&mut file)?)
        } else if &magic[0..2] == [0x49, 0x20, 0x49] {
            Some(0)
        } else {
            None
        };
        match start {
            None => return Err(Error::new(ErrorKind::Other, "Could not find exif start")),
            Some(st) => {tiff_start = st;}
        }
        file.seek(SeekFrom::Start(tiff_start))?;

        let mut header = [0u8; 8];
        file.read_exact(&mut header)?;

        let little = match &header[0..2] {
            b"II" => true,
            b"MM" => false,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid TIFF header",
                ));
            }
        };

        let magic = if little {
            u16::from_le_bytes([header[2], header[3]])
        } else {
            u16::from_be_bytes([header[2], header[3]])
        };

        if magic != 42 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid TIFF magic",
            ));
        }

        let first_ifd_offset = if little {
            u32::from_le_bytes([header[4], header[5], header[6], header[7]])
        } else {
            u32::from_be_bytes([header[4], header[5], header[6], header[7]])
        };

        let mut current_offset = first_ifd_offset;
        let mut index = 0;
        let mut ifds = Vec::new();

        while let Some((entries, next_offset)) =
            IfdEntry::parse(&mut file, tiff_start, current_offset, little, parse_mode)?
        {
            let mut map = HashMap::new();

            for e in &entries {
                let (name, desc) = crate::exif::tags::tag_info(e.tag);

                let entry_key = Entry {
                    tag_id: e.tag,
                    name: name.to_string(),
                    description: desc,
                    ifd_type: e.value_type,
                };

                let value_str = match e.value_type {
                    IfdType::Ascii => {
                        read_ascii(
                            &mut file,
                            tiff_start,
                            e.value_offset_or_inline,
                            e.count,
                        )
                        .unwrap_or_else(|_| "<error>".into())
                    }

                    IfdType::Rational => {
                        read_rational_strings(&mut file, tiff_start, e, little)
                            .map(|v| format!("{:?}", v))
                            .unwrap_or_else(|_| "<error>".into())
                    }

                    IfdType::SRational => {
                        read_srational_strings(&mut file, tiff_start, e, little)
                            .map(|v| format!("{:?}", v))
                            .unwrap_or_else(|_| "<error>".into())
                    }

                    IfdType::Long if e.tag == 0x8825 => {
                        match Gps::parse(
                            &mut file,
                            tiff_start,
                            e.value_offset_or_inline,
                            little,
                            parse_mode,
                        ) {
                            Ok(gps) => format!("{:?}", gps),
                            Err(_) => "<error>".into(),
                        }
                    }

                    IfdType::Byte
                    | IfdType::Short
                    | IfdType::Long
                    | IfdType::SLong
                    | IfdType::Undefined => {
                        read_numeric_values(&mut file, tiff_start, e, little)
                            .map(|v| format!("{:?}", v))
                            .unwrap_or_else(|_| "<error>".into())
                    }

                    _ => format!("type={:?}, count={}", e.value_type, e.count),
                };

                map.insert(entry_key, value_str);
            }

            ifds.push(Self {
                index,
                entries: map,
            });

            if next_offset == 0 {
                break;
            }

            current_offset = next_offset;
            index += 1;
        }

        Ok(ifds)
    }
}


// /// Represents the decoded value of an IFD entry.
// ///
// /// This enum is used after parsing or before encoding to represent
// /// the actual semantic value stored in the TIFF field.
// #[derive(Debug, Clone)]
// pub enum IfdValue {
//     /// One or more raw bytes.
//     Byte(Vec<u8>),

//     /// ASCII string including terminating NULL byte.
//     Ascii(String),

//     /// One or more 16-bit unsigned integers.
//     Short(Vec<u16>),

//     /// One or more 32-bit unsigned integers.
//     Long(Vec<u32>),

//     /// One or more rational values (numerator/denominator pairs).
//     Rational(Vec<Rational>),

//     /// Raw uninterpreted bytes.
//     Undefined(Vec<u8>),

//     /// One or more 32-bit signed integers.
//     SLong(Vec<i32>),

//     /// One or more signed rational values.
//     SRational(Vec<SRational>),

//     /// UTF-8 string including terminating NULL byte.
//     Utf8(String),
// }


/// A single 12-byte TIFF IFD (Image File Directory) entry.
///
/// Each entry defines a tag and how to interpret its associated data.
/// The entry may either store the value directly (if it fits in 4 bytes)
/// or store an offset pointing to the actual value in the file.
#[repr(C)]
#[derive(Debug, Clone, Hash)]
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

    pub fn parse(
        file: &mut File,
        base_offset: u64,
        offset: u32,
        little: bool,
        mode: ParseMode,
    ) -> io::Result<Option<(Vec<Self>, u32)>> {
        if offset == 0 {
            return Ok(None);
        }

        let endian = if little {
            Endian::Little
        } else {
            Endian::Big
        };

        let file_size = file.metadata()?.len();

        let absolute = match base_offset.checked_add(offset as u64) {
            Some(v) => v,
            None => {
                return handle_error(
                    mode,
                    io::Error::new(io::ErrorKind::InvalidData, "IFD offset overflow"),
                );
            }
        };

        if absolute + 2 > file_size {
            return handle_error(
                mode,
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!("IFD offset out of bounds: {}", absolute),
                ),
            );
        }

        if let Err(e) = file.seek(SeekFrom::Start(absolute)) {
            return handle_error(mode, e);
        }

        // --- Read entry count ---
        let mut count_buf = [0u8; 2];
        if let Err(e) = file.read_exact(&mut count_buf) {
            return handle_error(mode, io::Error::new(e.kind(), "Failed reading IFD count"));
        }

        let count = if little {
            u16::from_le_bytes(count_buf)
        } else {
            u16::from_be_bytes(count_buf)
        };

        // --- Compute entry table size safely ---
        let entries_len = match (count as u64).checked_mul(12) {
            Some(v) => v,
            None => {
                return handle_error(
                    mode,
                    io::Error::new(io::ErrorKind::InvalidData, "IFD entries overflow"),
                );
            }
        };

        if absolute + 2 + entries_len > file_size {
            return handle_error(
                mode,
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "IFD entries exceed file size",
                ),
            );
        }

        let mut entries_buf = vec![0u8; entries_len as usize];
        if let Err(e) = file.read_exact(&mut entries_buf) {
            return handle_error(mode, io::Error::new(e.kind(), "Failed reading IFD entries"));
        }

        let mut entries = Vec::new();

        for i in 0..count as usize {
            let base = i * 12;

            let tag = endian.read_u16(&entries_buf, base);
            let value_type_raw = endian.read_u16(&entries_buf, base + 2);

            let value_type = match IfdType::try_from(value_type_raw) {
                Ok(v) => v,
                Err(_) => {
                    if let ParseMode::Strict = mode {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Unknown IFD type: {}", value_type_raw),
                        ));
                    } else {
                        continue;
                    }
                }
            };

            let count = endian.read_u32(&entries_buf, base + 4);
            let value_offset_or_inline = endian.read_u32(&entries_buf, base + 8);

            entries.push(Self::new(
                tag,
                value_type,
                count,
                value_offset_or_inline,
            ));
        }

        // --- Read next IFD offset ---
        let mut next_buf = [0u8; 4];
        if let Err(e) = file.read_exact(&mut next_buf) {
            return handle_error(mode, io::Error::new(e.kind(), "Failed reading next IFD offset"));
        }

        let next_ifd_offset = if little {
            u32::from_le_bytes(next_buf)
        } else {
            u32::from_be_bytes(next_buf)
        };

        Ok(Some((entries, next_ifd_offset)))
    }
}

// /// Represents a TIFF Image File Directory (IFD).
// ///
// /// An IFD is a collection of entries describing image metadata.
// /// It begins with a count of entries, followed by the entries themselves,
// /// and ends with a pointer to the next IFD.
// #[derive(Debug, Clone)]
// pub struct Ifd {
//     /// Number of entries in this directory.
//     pub count: u16,

//     /// Collection of IFD entries sorted by tag (ascending order).
//     ///
//     /// Sorting is required by the TIFF specification.
//     pub entries: Vec<IfdEntry>,

//     /// Offset to the next IFD in the file.
//     ///
//     /// A value of 0 indicates that there are no further IFDs.
//     pub next_ifd_offset: u32,
// }

// impl Ifd {
//     /// Creates a new IFD from a list of entries and a next-IFD offset.
//     ///
//     /// Entries are automatically sorted by tag as required by the TIFF specification.
//     ///
//     /// # Parameters
//     /// - `entries`: List of IFD entries (unsorted allowed)
//     /// - `next_ifd_offset`: Offset to the next IFD (0 if none)
//     pub fn new(mut entries: Vec<IfdEntry>, next_ifd_offset: u32) -> Self {
//         // Ensure compliance with TIFF requirement: entries must be sorted by tag.
//         entries.sort_by_key(|e| e.tag);

//         let count = entries
//             .len()
//             .try_into()
//             .unwrap_or(u16::MAX);

//         Self {
//             count,
//             entries,
//             next_ifd_offset,
//         }
//     }
// }

// /// Represents a rational number as defined by TIFF.
// ///
// /// A rational is stored as two unsigned 32-bit integers:
// /// numerator and denominator.
// #[derive(Debug, Clone)]
// pub struct Rational {
//     /// Numerator of the rational value.
//     pub numerator: u32,

//     /// Denominator of the rational value.
//     /// Must not be zero.
//     pub denominator: u32,
// }

// /// Represents a signed rational number as defined by TIFF.
// ///
// /// A signed rational consists of two signed 32-bit integers:
// /// numerator and denominator.
// #[derive(Debug, Clone)]
// pub struct SRational {
//     /// Numerator of the signed rational value.
//     pub numerator: i32,

//     /// Denominator of the signed rational value.
//     /// Must not be zero.
//     pub denominator: i32,
// }



pub fn handle_error<T>(
    mode: ParseMode,
    err: io::Error,
) -> io::Result<Option<T>> {
    match mode {
        ParseMode::Strict => Err(err),
        ParseMode::Lenient => Ok(None),
    }
}