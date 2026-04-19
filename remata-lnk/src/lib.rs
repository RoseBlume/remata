// use makernote_macros::{FromPrimitive};
use remata_macros::{DisplayPretty, FromPrimitive, BitEnum};

const FLAGS: usize = 0x0014;
const FILE_ATTRIBUTES: usize = 0x0014;
const CREATE_DATE: usize = 0x001c;
const ACCESS_DATE: usize = 0x0024;
const MODIFY_DATE: usize = 0x002c;
const TARGET_FILE_SIZE: usize = 0x0034;
const ICON_INDEX: usize = 0x0038;
const RUN_WINDOW: usize = 0x003c;
const HOTKEY: usize = 0x0040;
const LINK_INFO: usize = 0x10000;
const ITEM_ID: usize = 0x20000;
const DESCRIPTION: usize = 0x30004;
const RELATIVE_PATH: usize = 0x30008;
const WORKING_DIRECTORY: usize = 0x30010;
const COMMAND_LINE_ARGUEMENTS: usize = 0x30020;
const ICON_FILE_NAME: usize = 0x30040;
const OFFSET: usize = ITEM_ID - LINK_INFO;
#[derive(Debug, Default)]
pub struct Lnk {
    pub flags: Option<FlagSet>, // BitEnum
    pub file_attributes: Option<AttributeSet>, // BitEnum
    pub create_date: Option<String>,
    pub access_date: Option<String>,
    pub modify_date: Option<String>,
    pub target_file_size: Option<u64>, // Replace with actual datatype
    pub icon_index: Option<Vec<u8>>, // Store as bytes for now
    pub run_window: Option<RunWindow>, // FromPrimitive Enum
    pub hotkey: Option<HotKey>, // FromPrimitive
    pub raw_link_info: Option<Vec<u8>>, // Raw for now
    pub raw_item_id: Option<Vec<u8>>, // Raw for now
    pub description: Option<String>,
    pub relative_path: Option<String>,
    pub working_dir: Option<String>,
    pub command_line_arguements: Option<String>,
    pub icon_file_name: Option<String>,
}



impl Lnk {
    pub fn parse(data: &[u8]) -> Self {
        // -----------------------------
        // helpers
        // -----------------------------
        let read_u16 = |offset: usize| -> Option<u16> {
            let o = offset as usize;
            data.get(o..o + 2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
        };

        let read_u32 = |offset: usize| -> Option<u32> {
            let o = offset as usize;
            data.get(o..o + 4)
                .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };

        let read_u64 = |offset: usize| -> Option<u64> {
            let o = offset as usize;
            data.get(o..o + 8)
                .map(|b| u64::from_le_bytes([
                    b[0], b[1], b[2], b[3],
                    b[4], b[5], b[6], b[7],
                ]))
        };

        let read_bytes = |offset: usize, len: usize| -> Option<Vec<u8>> {
            data.get(offset..offset + len).map(|b| b.to_vec())
        };

        let read_string = |offset: usize, len: usize| -> Option<String> {
            data.get(offset..offset + len)
                .and_then(|b| String::from_utf8(b.to_vec()).ok())
        };

        // -----------------------------
        // FIXED HEADER
        // -----------------------------

        let flags = read_u32(FLAGS).map(FlagSet::from_bits);
        let file_attributes = read_u32(FILE_ATTRIBUTES).map(AttributeSet::from_bits);

        let create_date = read_u64(CREATE_DATE).map(filetime_to_string);
        let access_date = read_u64(ACCESS_DATE).map(filetime_to_string);
        let modify_date = read_u64(MODIFY_DATE).map(filetime_to_string);

        let target_file_size = read_u32(TARGET_FILE_SIZE).map(|v| v as u64);

        let icon_index = read_u32(ICON_INDEX).map(|v| v.to_le_bytes().to_vec());

        let run_window = read_u32(RUN_WINDOW)
            .and_then(|v| RunWindow::try_from(v).ok());

        let hotkey = read_u16(HOTKEY)
            .and_then(|v| HotKey::try_from(v).ok());

        // -----------------------------
        // OPTIONAL SECTIONS (guarded by flags)
        // -----------------------------

        let raw_link_info = if flags.as_ref().map_or(false, |f| {
            f.contains(Flag::LinkInfo)
        }) {
            read_bytes(LINK_INFO as usize, OFFSET as usize) // placeholder length
        } else {
            None
        };

        let raw_item_id = if flags.as_ref().map_or(false, |f| {
            f.contains(Flag::IDList)
        }) {
            read_bytes(ITEM_ID as usize, 256) // placeholder
        } else {
            None
        };

        let description = read_string(DESCRIPTION as usize, 256);
        let relative_path = read_string(RELATIVE_PATH as usize, 256);
        let working_dir = read_string(WORKING_DIRECTORY as usize, 256);
        let command_line_arguements = read_string(COMMAND_LINE_ARGUEMENTS as usize, 512);
        let icon_file_name = read_string(ICON_FILE_NAME as usize, 256);

        // -----------------------------
        // RESULT
        // -----------------------------

        Self {
            flags,
            file_attributes,
            create_date,
            access_date,
            modify_date,
            target_file_size,
            icon_index,
            run_window,
            hotkey,
            raw_link_info,
            raw_item_id,
            description,
            relative_path,
            working_dir,
            command_line_arguements,
            icon_file_name,
        }
    }
}

fn filetime_to_string(ft: u64) -> String {
    // Windows epoch → Unix epoch difference in seconds
    const WINDOWS_TO_UNIX_EPOCH: u64 = 11644473600;

    // FILETIME is in 100-ns units
    let total_seconds = ft / 10_000_000;

    // prevent underflow if garbage input
    let unix_seconds = total_seconds.saturating_sub(WINDOWS_TO_UNIX_EPOCH);

    format_unix_time(unix_seconds)
}

use std::time::{UNIX_EPOCH, Duration};

fn format_unix_time(secs: u64) -> String {
    let t = UNIX_EPOCH + Duration::from_secs(secs);

    match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();

            let days = secs / 86400;
            let rem = secs % 86400;
            let hours = rem / 3600;
            let minutes = (rem % 3600) / 60;
            let seconds = rem % 60;

            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                1970 + days / 365,
                (days % 365) / 30 + 1,
                days % 30 + 1,
                hours,
                minutes,
                seconds
            )
        }
        Err(_) => "invalid time".to_string(),
    }
}
#[derive(BitEnum, DisplayPretty, Clone, Copy)]
pub enum Flag {
    IDList = 0,
    LinkInfo,
    Description,
    RelativePath,
    WorkingDir,
    CommandArgs,
    IconFile,
    Unicode,
    NoLinkInfo,
    ExpString,
    SeparateProc,
    DarwinID = 12,
    RunAsUser,
    ExpIcon,
    NoPidAlias,
    RunWithShim = 17,
    NoLinkTrack,
    TargetMetadata,
    NoLinkPathTracking,
    NoKnownFolderTracking,
    NoKnownFolderAlias,
    LinkToLink,
    UnaliasOnSave,
    PreferEnvPath,
    KeepLocalIDList,
}
// impl Flag {
//     pub fn parse(value: u32) -> Option<Vec<Self>> {
//         let mut flags = Vec::new();

//         if value & (1 << 0) != 0 { flags.push(Self::IDList); }
//         if value & (1 << 1) != 0 { flags.push(Self::LinkInfo); }
//         if value & (1 << 2) != 0 { flags.push(Self::Description); }
//         if value & (1 << 3) != 0 { flags.push(Self::RelativePath); }
//         if value & (1 << 4) != 0 { flags.push(Self::WorkingDir); }
//         if value & (1 << 5) != 0 { flags.push(Self::CommandArgs); }
//         if value & (1 << 6) != 0 { flags.push(Self::IconFile); }
//         if value & (1 << 7) != 0 { flags.push(Self::Unicode); }
//         if value & (1 << 8) != 0 { flags.push(Self::NoLinkInfo); }
//         if value & (1 << 9) != 0 { flags.push(Self::ExpString); }
//         if value & (1 << 10) != 0 { flags.push(Self::SeparateProc); }
//         if value & (1 << 12) != 0 { flags.push(Self::DarwinID); }
//         if value & (1 << 13) != 0 { flags.push(Self::RunAsUser); }
//         if value & (1 << 14) != 0 { flags.push(Self::ExpIcon); }
//         if value & (1 << 15) != 0 { flags.push(Self::NoPidAlias); }
//         if value & (1 << 17) != 0 { flags.push(Self::RunWithShim); }
//         if value & (1 << 18) != 0 { flags.push(Self::NoLinkTrack); }
//         if value & (1 << 19) != 0 { flags.push(Self::TargetMetadata); }
//         if value & (1 << 20) != 0 { flags.push(Self::NoLinkPathTracking); }
//         if value & (1 << 21) != 0 { flags.push(Self::NoKnownFolderTracking); }
//         if value & (1 << 22) != 0 { flags.push(Self::NoKnownFolderAlias); }
//         if value & (1 << 23) != 0 { flags.push(Self::LinkToLink); }
//         if value & (1 << 24) != 0 { flags.push(Self::UnaliasOnSave); }
//         if value & (1 << 25) != 0 { flags.push(Self::PreferEnvPath); }
//         if value & (1 << 26) != 0 { flags.push(Self::KeepLocalIDList); }

//         if flags.is_empty() { None } else { Some(flags) }
//     }
// }



// Bit 0 = IDList
// Bit 1 = LinkInfo
// Bit 2 = Description
// Bit 3 = RelativePath
// Bit 4 = WorkingDir
// Bit 5 = CommandArgs
// Bit 6 = IconFile
// Bit 7 = Unicode
// Bit 8 = NoLinkInfo
// Bit 9 = ExpString
// Bit 10 = SeparateProc
// Bit 12 = DarwinID
// Bit 13 = RunAsUser
// Bit 14 = ExpIcon
// Bit 15 = NoPidAlias
// Bit 17 = RunWithShim
// Bit 18 = NoLinkTrack
// Bit 19 = TargetMetadata
// Bit 20 = NoLinkPathTracking
// Bit 21 = NoKnownFolderTracking
// Bit 22 = NoKnownFolderAlias
// Bit 23 = LinkToLink
// Bit 24 = UnaliasOnSave
// Bit 25 = PreferEnvPath
// Bit 26 = KeepLocalIDList

#[derive(BitEnum, DisplayPretty, Clone, Copy)]
pub enum Attribute {
    ReadOnly = 0,
    Hidden,
    System,
    Reserved1,
    Directory,
    Archive,
    Reserved2,
    Normal,
    Temporary,
    Sparse,
    ReparsePoint,
    Compressed,
    Offline,
    NotContentIndexed,
    Encrypted,
}
// Value	FileAttributes	Value	FileAttributes	Value	FileAttributes
// 'Bit 0'	= Read-only	'Bit 5'	= Archive	'Bit 10'	= Reparse point
// 'Bit 1'	= Hidden	'Bit 6'	= Encrypted?	'Bit 11'	= Compressed
// 'Bit 2'	= System	'Bit 7'	= Normal	'Bit 12'	= Offline
// 'Bit 3'	= Volume	'Bit 8'	= Temporary	'Bit 13'	= Not indexed
// 'Bit 4'	= Directory	'Bit 9'	= Sparse	'Bit 14'	= Encrypted

// impl Attribute {
//     pub fn parse(value: u32) -> Option<Vec<Self>> {
//         let mut attributes = Vec::new();

//         if value & (1 << 0) != 0 { attributes.push(Self::ReadOnly); }
//         if value & (1 << 1) != 0 { attributes.push(Self::Hidden); }
//         if value & (1 << 2) != 0 { attributes.push(Self::System); }
//         if value & (1 << 3) != 0 { attributes.push(Self::Volume); }
//         if value & (1 << 4) != 0 { attributes.push(Self::Directory); }
//         if value & (1 << 5) != 0 { attributes.push(Self::Archive); }
//         if value & (1 << 6) != 0 { attributes.push(Self::EncryptedQ); }
//         if value & (1 << 7) != 0 { attributes.push(Self::Normal); }
//         if value & (1 << 8) != 0 { attributes.push(Self::Sparse); }
//         if value & (1 << 9) != 0 { attributes.push(Self::ReparsePoint); }
//         if value & (1 << 10) != 0 { attributes.push(Self::Compressed); }
//         if value & (1 << 11) != 0 { attributes.push(Self::Offline); }
//         if value & (1 << 12) != 0 { attributes.push(Self::NotIndexed); }
//         if value & (1 << 13) != 0 { attributes.push(Self::Encrypted); }

//         if attributes.is_empty() { None } else { Some(attributes) }
//     }
// }

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum RunWindow {
    #[value = 0] Hide,
    #[value = 1] Normal,
    #[value = 2] ShowMinimized,
    #[value = 3] ShowMaxmimized,
    #[value = 4] ShowNoActivate,
    #[value = 5] Show,
    #[value = 6] Minimized,
    #[value = 7] ShowMinimizedNoActivate,
    #[value = 8] ShowNA,
    #[value = 9] Restore,
    #[value = 10] ShowDefault,
}


// 0 = Hide
// 1 = Normal
// 2 = Show Minimized
// 3 = Show Maximized
// 4 = Show No Activate
// 5 = Show
// 6 = Minimized
// 7 = Show Minimized No Activate
// 8 = Show NA
// 9 = Restore
// 10 = Show Default

// impl RunWindow {
//     pub fn from_val(val: u16) -> Option<Self> {
//         match val {
//             0 => Some(Self::Hide),
//             1 => Some(Self::Normal),
//             2 => Some(Self::ShowMinimized),
//             3 => Some(Self::ShowMaxmimized),
//             4 => Some(Self::ShowNoActivate),
//             5 => Some(Self::Show),
//             6 => Some(Self::Minimized),
//             7 => Some(Self::ShowMinimizedNoActivate),
//             8 => Some(Self::ShowNA),
//             9 => Some(Self::Restore),
//             _ => None
//         }
//     }
// }


// #[derive(Debug, Clone, Copy, DisplayPretty, FromPrimitive)]
// pub enum DaylightSavings {
//     #[value = 0] No,
//     #[value = 0] Yes,
//     Unknown(u8),
// }


#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum HotKey  {
    #[value = 0x0] None,
    #[value = 0x90] NumLock,
    #[value = 0x91] ScrollLock,
    #[value = 0x100] Shift,
    #[value = 0x200] Control,
    #[value = 0x400] Alt,
    #[value = 0x30] Num0,
    #[value = 0x31] Num1,
    #[value = 0x32] Num2,
    #[value = 0x33] Num3,
    #[value = 0x34] Num4,
    #[value = 0x35] Num5,
    #[value = 0x36] Num6,
    #[value = 0x37] Num7,
    #[value = 0x38] Num8,
    #[value = 0x39] Num9,
    #[value = 0x41] A,
    #[value = 0x42] B,
    #[value = 0x43] C,
    #[value = 0x44] D,
    #[value = 0x45] E,
    #[value = 0x46] F,
    #[value = 0x47] G,
    #[value = 0x48] H,
    #[value = 0x49] I,
    #[value = 0x4A] J,
    #[value = 0x4B] K,
    #[value = 0x4C] L,
    #[value = 0x4D] M,
    #[value = 0x4E] N,
    #[value = 0x4F] O,
    #[value = 0x50] P,
    #[value = 0x51] Q,
    #[value = 0x52] R,
    #[value = 0x53] S,
    #[value = 0x54] T,
    #[value = 0x55] U,
    #[value = 0x56] V,
    #[value = 0x57] W,
    #[value = 0x58] X,
    #[value = 0x59] Y,
    #[value = 0x5A] Z,
    #[value = 0x70] F1,
    #[value = 0x71] F2,
    #[value = 0x72] F3,
    #[value = 0x73] F4,
    #[value = 0x74] F5,
    #[value = 0x75] F6,
    #[value = 0x76] F7,
    #[value = 0x77] F8,
    #[value = 0x78] F9,
    #[value = 0x79] F10,
    #[value = 0x7A] F11,
    #[value = 0x7B] F12,
    #[value = 0x7C] F13,
    #[value = 0x7D] F14,
    #[value = 0x7E] F15,
    #[value = 0x7F] F16,
    #[value = 0x80] F17,
    #[value = 0x81] F18,
    #[value = 0x82] F19,
    #[value = 0x83] F20,
    #[value = 0x84] F21,
    #[value = 0x85] F22,
    #[value = 0x86] F23,
    #[value = 0x87] F24,
}



// 0x0 = (none)
// 0x90 = Num Lock
// 0x91 = Scroll Lock
// 0x100 = Shift
// 0x200 = Control	  	0x400 = Alt
// '0x30'-'0x39' = 0-9
// '0x41'-'0x5a' = A-Z
// '0x70'-'0x87' = F1-F24