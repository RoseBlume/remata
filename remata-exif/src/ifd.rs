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

// /// Represents a TIFF Image File Directory (IFD).
// ///
// /// An IFD is a collection of entries describing image metadata.
// /// It begins with a count of entries, followed by the entries themselves,
// /// and ends with a pointer to the next IFD.
// #[derive(Debug, Clone)]
// pub struct Ifd {
//     /// Number of entries in this directory.
//     pub count: usize,

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
//             .unwrap_or(usize::MAX);

//         Self {
//             count,
//             entries,
//             next_ifd_offset,
//         }
//     }
// }
