use super::DisplayPretty;
#[derive(DisplayPretty)]
pub enum PortableType {
    RomImage,
    Pe32,
    Pe32Plus,
}

impl PortableType {
    pub fn from_val(val: u16) -> Option<Self> {
        match val {
            0x107 => Some(Self::RomImage),
            0x10b => Some(Self::Pe32),
            0x20b => Some(Self::Pe32Plus),
            _ => None
        }
    }
}

// 0x107 = ROM Image
// 0x10b = PE32
// 0x20b = PE32+

#[derive(Clone, DisplayPretty)]
pub struct PortableVersionInfo {
    pub file_version: Option<String>,
    pub product_version: Option<String>,
    pub file_flags_mask: Option<u32>,
    pub file_flags: Option<Vec<FileFlags>>,
    pub file_os: Option<FileOS>,
    pub object_file_type: Option<ObjectFileType>,
    pub file_subtype: Option<u32>,
}

impl PortableVersionInfo {
    pub fn parse(data: &[u8]) -> Option<Self> {
        // Find VS_FIXEDFILEINFO signature
        let mut i = 0;
        while i + 4 < data.len() {
            let sig = u32::from_le_bytes(data[i..i + 4].try_into().ok()?);
            if sig == 0xFEEF04BD {
                break;
            }
            i += 1;
        }

        if i + 52 > data.len() {
            return None;
        }

        let ms = read_u32(data, i + 8)?;
        let ls = read_u32(data, i + 12)?;

        let flags_mask = read_u32(data, i + 24)?;
        let flags_raw = read_u32(data, i + 28)?;
        let file_os_raw = read_u32(data, i + 32)?;
        let file_type_raw = read_u32(data, i + 36)?;
        let subtype = read_u32(data, i + 40)?;

        Some(Self {
            file_version: Some(format_version(ms, ls)),
            product_version: None,
            file_flags_mask: Some(flags_mask),
            file_flags: Some(parse_file_flags(flags_raw)),
            file_os: FileOS::from_val(file_os_raw),
            object_file_type: ObjectFileType::from_val(file_type_raw),
            file_subtype: Some(subtype),
        })
    }
}


fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(offset..offset + 4)?.try_into().ok()?))
}

pub fn format_version(ms: u32, ls: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (ms >> 16) & 0xFFFF,
        ms & 0xFFFF,
        (ls >> 16) & 0xFFFF,
        ls & 0xFFFF
    )
}
// EXE PEVersion Tags
// Information extracted from the VS_VERSION_INFO structure of Windows PE files.

// Index4	Tag Name	Writable	Values / Notes
// 2	FileVersionNumber	no
// 4	ProductVersionNumber	no
// 6	FileFlagsMask	no
// 7	FileFlags	no
// 8	FileOS	no
// 0x1 = Win16
// 0x2 = PM-16
// 0x3 = PM-32
// 0x4 = Win32
// 0x10000 = DOS
// 0x10001 = Windows 16-bit
// 0x10004 = Windows 32-bit
// 0x20000 = OS/2 16-bit
// 0x20002 = OS/2 16-bit PM-16
// 0x30000 = OS/2 32-bit
// 0x30003 = OS/2 32-bit PM-32
// 0x40000 = Windows NT
// 0x40004 = Windows NT 32-bit
// 9	ObjectFileType	no
// 0 = Unknown
// 1 = Executable application
// 2 = Dynamic link library
// 3 = Driver
// 4 = Font
// 5 = VxD
// 7 = Static library
// 10	FileSubtype	no


#[derive(Clone, Copy, DisplayPretty)]
pub enum FileFlags {
    Debug,
    PreRelease,
    Patched,
    PrivateBuild,
    InfoInferred,
    SpecialBuild,
}

impl FileFlags {
    pub fn from_bit(bit: u32) -> Option<Self> {
        match bit {
            0 => Some(Self::Debug),
            1 => Some(Self::PreRelease),
            2 => Some(Self::Patched),
            3 => Some(Self::PrivateBuild),
            4 => Some(Self::InfoInferred),
            5 => Some(Self::SpecialBuild),
            _ => None,
        }
    }
}

pub fn parse_file_flags(bits: u32) -> Vec<FileFlags> {
    let mut result = Vec::new();

    for i in 0..32 {
        if (bits >> i) & 1 == 1 {
            if let Some(flag) = FileFlags::from_bit(i) {
                result.push(flag);
            }
        }
    }

    result
}
// Bit 0 = Debug
// Bit 1 = Pre-release
// Bit 2 = Patched
// Bit 3 = Private build
// Bit 4 = Info inferred
// Bit 5 = Special build


#[derive(Clone, Copy, DisplayPretty)]
pub enum FileOS {
    Win16,
    PM16,
    PM32,
    Win32,
    Dos,
    Windows16,
    Windows32,
    Os2_16,
    Os2_16Pm16,
    Os2_32,
    Os2_32Pm32,
    WindowsNT,
    WindowsNT32,
}

impl FileOS {
    pub fn from_val(val: u32) -> Option<Self> {
        match val {
            0x1 => Some(Self::Win16),
            0x2 => Some(Self::PM16),
            0x3 => Some(Self::PM32),
            0x4 => Some(Self::Win32),
            0x10000 => Some(Self::Dos),
            0x10001 => Some(Self::Windows16),
            0x10004 => Some(Self::Windows32),
            0x20000 => Some(Self::Os2_16),
            0x20002 => Some(Self::Os2_16Pm16),
            0x30000 => Some(Self::Os2_32),
            0x30003 => Some(Self::Os2_32Pm32),
            0x40000 => Some(Self::WindowsNT),
            0x40004 => Some(Self::WindowsNT32),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, DisplayPretty)]
pub enum ObjectFileType {
    Unknown,
    Executable,
    Dll,
    Driver,
    Font,
    Vxd,
    StaticLibrary,
}

impl ObjectFileType {
    pub fn from_val(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Unknown),
            1 => Some(Self::Executable),
            2 => Some(Self::Dll),
            3 => Some(Self::Driver),
            4 => Some(Self::Font),
            5 => Some(Self::Vxd),
            7 => Some(Self::StaticLibrary),
            _ => None,
        }
    }
}