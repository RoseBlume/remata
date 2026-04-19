use std::io::{Read, Seek, SeekFrom};
use remata_macros::DisplayPretty;

/// Represents a parsed ISO9660 image.
///
/// This struct only contains high-level descriptors parsed from the
/// Volume Descriptor Set (starting at sector 16).
///
/// It does NOT load the entire ISO into memory — parsing is done via
/// streaming reads.
///
/// ISO9660 layout:
/// - Sector size: typically 2048 bytes
/// - Volume descriptors start at sector 16
#[derive(DisplayPretty)]
pub struct Iso {
    /// Boot Record Descriptor (Type 0)
    ///
    /// Located in the Volume Descriptor Set.
    /// Contains boot system and identifier information.
    pub boot_record: Option<BootRecord>,

    /// Primary Volume Descriptor (Type 1)
    ///
    /// The main descriptor containing filesystem metadata.
    pub primary_volume: Option<PrimaryVolume>,
}

/// Boot Record Descriptor (Type 0).
///
/// Located at:
/// - Sector: 16+
/// - Byte 0: Descriptor Type = 0
///
/// This descriptor is primarily used for bootable ISOs.
#[derive(DisplayPretty)]
pub struct BootRecord {
    /// Boot System Identifier
    ///
    /// Offset: bytes 7–38 (32 bytes)
    ///
    /// Identifies the system used for booting (e.g., "EL TORITO SPECIFICATION").
    pub boot_system: String,

    /// Boot Identifier
    ///
    /// Offset: bytes 39–70 (32 bytes)
    ///
    /// Provides additional boot-specific identification.
    pub boot_identifier: String,
}

/// Primary Volume Descriptor (Type 1).
///
/// Located at:
/// - Sector: typically 16
/// - Byte 0: Descriptor Type = 1
///
/// This is the main metadata block for an ISO9660 filesystem.
#[derive(DisplayPretty)]
pub struct PrimaryVolume {
    /// System Identifier
    ///
    /// Offset: bytes 8–39 (32 bytes)
    ///
    /// Identifies the system that can act upon this volume.
    pub system_identifier: Option<String>,

    /// Volume Identifier (Volume Name)
    ///
    /// Offset: bytes 40–71 (32 bytes)
    ///
    /// The human-readable name of the ISO volume.
    pub volume_identifier: Option<String>,

    /// Total number of logical blocks in the volume
    ///
    /// Offset: bytes 80–83 (LSB), 84–87 (MSB)
    ///
    /// Multiply by `logical_block_size` to get total size.
    pub volume_block_count: Option<u32>,

    /// Number of disks in the volume set
    ///
    /// Offset: bytes 120–121
    pub volume_set_size: Option<u16>,

    /// Index of this disk within the volume set
    ///
    /// Offset: bytes 124–125
    pub volume_sequence_number: Option<u16>,

    /// Logical block size (usually 2048)
    ///
    /// Offset: bytes 128–129
    pub logical_block_size: Option<u16>,

    /// Size of the path table (bytes)
    ///
    /// Offset: bytes 132–135
    pub path_table_size: Option<u32>,

    /// Location of the Type-L path table (little-endian)
    ///
    /// Offset: bytes 140–143
    pub path_table_location_l: Option<u32>,

    /// Location of the Type-M path table (big-endian)
    ///
    /// Offset: bytes 148–151
    pub path_table_location_m: Option<u32>,

    /// Volume Set Identifier
    ///
    /// Offset: bytes 190–317
    pub volume_set_identifier: Option<String>,

    /// Publisher Identifier
    ///
    /// Offset: bytes 318–445
    pub publisher_identifier: Option<String>,

    /// Data Preparer Identifier
    ///
    /// Offset: bytes 446–573
    pub data_preparer_identifier: Option<String>,

    /// Application Identifier
    ///
    /// Offset: bytes 574–701
    pub application_identifier: Option<String>,

    /// Copyright File Identifier
    ///
    /// Offset: bytes 702–739
    pub copyright_file_identifier: Option<String>,

    /// Abstract File Identifier
    ///
    /// Offset: bytes 740–775
    pub abstract_file_identifier: Option<String>,

    /// Bibliographic File Identifier
    ///
    /// Offset: bytes 776–812
    pub bibliographic_file_identifier: Option<String>,

    /// Volume Creation Date
    ///
    /// Offset: bytes 813–829 (17 bytes ASCII)
    ///
    /// Format: YYYYMMDDHHMMSSccTZ
    pub creation_date: Option<String>,

    /// Volume Modification Date
    ///
    /// Offset: bytes 830–846
    pub modification_date: Option<String>,

    /// Volume Expiration Date
    ///
    /// Offset: bytes 847–863
    pub expiration_date: Option<String>,

    /// Volume Effective Date
    ///
    /// Offset: bytes 864–880
    pub effective_date: Option<String>,
}
fn read_str(buf: &[u8]) -> String {
    let s = String::from_utf8_lossy(buf);
    s.trim_matches(char::from(0)).trim().to_string()
}

fn read_u16_le_be(buf: &[u8]) -> u16 {
    u16::from_le_bytes([buf[0], buf[1]])
}

fn read_u32_le_be(buf: &[u8]) -> u32 {
    u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])
}

impl Iso {
    /// Parses an ISO 9660 filesystem image from the given reader.
    ///
    /// This function scans the Volume Descriptor Set starting at sector 16,
    /// as defined by the ISO 9660 specification. Each descriptor is read
    /// sequentially until a terminator descriptor is encountered.
    ///
    /// The parser attempts to:
    /// - Locate and parse the Boot Record (type `0`)
    /// - Locate and parse the Primary Volume Descriptor (type `1`)
    /// - Stop when the Volume Descriptor Set Terminator (type `255`) is reached
    ///
    /// # Parameters
    ///
    /// - `reader`: A readable and seekable input source containing ISO data.
    ///
    /// # Returns
    ///
    /// Returns an [`Iso`] struct containing any successfully parsed descriptors.
    /// Fields are optional to allow partial parsing of incomplete or non-standard images.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if:
    /// - The underlying reader fails during I/O operations
    /// - A sector cannot be read
    ///
    /// # Behavior
    ///
    /// - The parser begins at sector 16 (standard ISO 9660 starting point).
    /// - Each sector is assumed to be 2048 bytes.
    /// - Unknown descriptor types are skipped.
    /// - Parsing stops when a terminator descriptor (`255`) is found.
    ///
    /// # Notes
    ///
    /// - ISO 9660 uses fixed-size sectors and descriptor-based metadata.
    /// - Not all descriptors are parsed; only commonly used ones are extracted.
    /// - Additional descriptors (e.g., Supplementary Volume Descriptor) are ignored.
    pub fn parse<R: Read + Seek>(mut reader: R) -> std::io::Result<Self> {
        const SECTOR_SIZE: u64 = 2048;
        const START_SECTOR: u64 = 16;

        let mut iso = Iso {
            boot_record: None,
            primary_volume: None,
        };

        let mut sector = START_SECTOR;

        loop {
            reader.seek(SeekFrom::Start(sector * SECTOR_SIZE))?;

            let mut buf = [0u8; 2048];
            reader.read_exact(&mut buf)?;

            let descriptor_type = buf[0];

            match descriptor_type {
                0 => {
                    // Boot Record
                    iso.boot_record = Some(BootRecord::parse(&buf));
                }
                1 => {
                    // Primary Volume Descriptor
                    iso.primary_volume = Some(PrimaryVolume::parse(&buf));
                }
                255 => break, // Volume Descriptor Set Terminator
                _ => {}
            }

            sector += 1;
        }

        Ok(iso)
    }
}

impl BootRecord {
    /// Parses a Boot Record descriptor from a raw 2048-byte sector buffer.
    ///
    /// The Boot Record (descriptor type `0`) contains information about
    /// bootable systems and is typically used for El Torito bootable CDs.
    ///
    /// # Parameters
    ///
    /// - `buf`: A 2048-byte buffer representing a single ISO sector.
    ///
    /// # Returns
    ///
    /// Returns a [`BootRecord`] populated with extracted fields.
    ///
    /// # Notes
    ///
    /// - String fields are fixed-width and may contain padded spaces.
    /// - The boot system identifier typically indicates the boot specification used.
    pub fn parse(buf: &[u8]) -> Self {
        Self {
            boot_system: read_str(&buf[7..39]),
            boot_identifier: read_str(&buf[39..71]),
        }
    }
}

impl PrimaryVolume {
    /// Parses a Primary Volume Descriptor from a raw 2048-byte sector buffer.
    ///
    /// The Primary Volume Descriptor (descriptor type `1`) contains core
    /// filesystem metadata such as volume identifiers, sizes, and directory
    /// structure information.
    ///
    /// # Parameters
    ///
    /// - `buf`: A 2048-byte buffer representing a single ISO sector.
    ///
    /// # Returns
    ///
    /// Returns a [`PrimaryVolume`] populated with extracted metadata fields.
    ///
    /// # Notes
    ///
    /// - Many numeric fields are stored in both little-endian and big-endian
    ///   formats; helper functions (`read_u16_le_be`, `read_u32_le_be`) handle this.
    /// - String fields are fixed-length and may include padding.
    /// - Not all fields defined in the ISO 9660 specification are parsed here.
    pub fn parse(buf: &[u8]) -> Self {
        Self {
            system_identifier: Some(read_str(&buf[8..40])),
            volume_identifier: Some(read_str(&buf[40..72])),

            volume_block_count: Some(read_u32_le_be(&buf[80..84])),
            volume_set_size: Some(read_u16_le_be(&buf[120..122])),
            volume_sequence_number: Some(read_u16_le_be(&buf[124..126])),
            logical_block_size: Some(read_u16_le_be(&buf[128..130])),

            path_table_size: Some(read_u32_le_be(&buf[132..136])),
            path_table_location_l: Some(read_u32_le_be(&buf[140..144])),
            path_table_location_m: Some(read_u32_le_be(&buf[148..152])),

            volume_set_identifier: Some(read_str(&buf[190..318])),
            publisher_identifier: Some(read_str(&buf[318..446])),
            data_preparer_identifier: Some(read_str(&buf[446..574])),
            application_identifier: Some(read_str(&buf[574..702])),

            copyright_file_identifier: Some(read_str(&buf[702..740])),
            abstract_file_identifier: Some(read_str(&buf[740..776])),
            bibliographic_file_identifier: Some(read_str(&buf[776..813])),

            creation_date: Some(read_str(&buf[813..830])),
            modification_date: Some(read_str(&buf[830..847])),
            expiration_date: Some(read_str(&buf[847..864])),
            effective_date: Some(read_str(&buf[864..881])),
        }
    }
}