use remata_macros::DisplayPretty;
// mod disp;
pub mod portable;
pub use portable::{
    PortableType,
    PortableVersionInfo
};
use std::io::{self, Read, Seek, SeekFrom};
/// This struct contains info extracted from the header of Windows PE (Portable Executable) EXE files and DLL libraries

#[derive(DisplayPretty)]
pub struct Windows {
    pub machine_type: Option<MachineType>,
    pub time_stamp: Option<String>,
    pub image_file_characteristics: Option<Vec<ImageFileCharacteristics>>,
    pub portable_type: Option<PortableType>,
    pub linker_version: Option<String>,
    pub code_size: Option<i64>,
    pub initialized_data_size: Option<i64>,
    pub uninitialized_data_size: Option<i64>,
    pub entry_point: Option<String>,
    pub os_version: Option<String>,
    pub image_version: Option<String>,
    pub subsystem_version: Option<String>,
    pub subsystem: Option<SubSystem>,
    pub version_info: Option<PortableVersionInfo>,
}

impl Windows {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        // ---- DOS HEADER ----
        let mut dos_header = [0u8; 64];
        reader.read_exact(&mut dos_header)?;

        let pe_offset = u32::from_le_bytes([
            dos_header[0x3C],
            dos_header[0x3D],
            dos_header[0x3E],
            dos_header[0x3F],
        ]);

        // ---- PE HEADER ----
        reader.seek(SeekFrom::Start(pe_offset as u64))?;

        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature)?;
        if &signature != b"PE\0\0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a PE file"));
        }

        // ---- COFF HEADER ----
        let machine = read_u16(reader)?;
        let num_sections = read_u16(reader)?;
        let timestamp = read_u32(reader)?;
        let _ptr_symbol_table = read_u32(reader)?;
        let _num_symbols = read_u32(reader)?;
        let opt_header_size = read_u16(reader)?;
        let characteristics_bits = read_u16(reader)?;

        let machine_type = MachineType::from_val(machine);
        let time_stamp = Some(timestamp.to_string());
        let image_file_characteristics =
            Some(parse_characteristics(characteristics_bits));

        // ---- OPTIONAL HEADER ----
        let magic = read_u16(reader)?;
        let portable_type = PortableType::from_val(magic);

        let major_linker = read_u8(reader)?;
        let minor_linker = read_u8(reader)?;
        let linker_version = Some(format!("{}.{}", major_linker, minor_linker));

        let code_size = read_u32(reader)? as i64;
        let initialized_data_size = read_u32(reader)? as i64;
        let uninitialized_data_size = read_u32(reader)? as i64;

        let entry_point = format!("0x{:X}", read_u32(reader)?);

        let _base_of_code = read_u32(reader)?;

        if magic == 0x10b {
            let _base_of_data = read_u32(reader)?;
        }

        skip(reader, 8)?;

        let os_major = read_u16(reader)?;
        let os_minor = read_u16(reader)?;
        let os_version = Some(format!("{}.{}", os_major, os_minor));

        let img_major = read_u16(reader)?;
        let img_minor = read_u16(reader)?;
        let image_version = Some(format!("{}.{}", img_major, img_minor));

        let sub_major = read_u16(reader)?;
        let sub_minor = read_u16(reader)?;
        let subsystem_version = Some(format!("{}.{}", sub_major, sub_minor));

        skip(reader, 4)?;

        let _size_of_image = read_u32(reader)?;
        let _size_of_headers = read_u32(reader)?;
        let _checksum = read_u32(reader)?;

        let subsystem_raw = read_u16(reader)?;
        let subsystem = SubSystem::from_val(subsystem_raw);

        // ---- SECTION HEADERS ----
        let section_headers_offset =
            pe_offset as u64 + 4 + 20 + opt_header_size as u64;

        reader.seek(SeekFrom::Start(section_headers_offset))?;

        let mut sections = Vec::new();
        for _ in 0..num_sections {
            sections.push(read_section(reader)?);
        }

        // ---- VERSION INFO ----
        let version_info = if let Some(rsrc) = find_rsrc(&sections) {
            parse_version_info(reader, rsrc)?
        } else {
            None
        };

        Ok(Self {
            machine_type,
            time_stamp,
            image_file_characteristics,
            portable_type,
            linker_version,
            code_size: Some(code_size),
            initialized_data_size: Some(initialized_data_size),
            uninitialized_data_size: Some(uninitialized_data_size),
            entry_point: Some(entry_point),
            os_version,
            image_version,
            subsystem_version,
            subsystem,
            version_info,
        })
    }
}

#[derive(Debug)]
struct SectionHeader {
    name: [u8; 8],
    virtual_address: u32,
    raw_ptr: u32,
}

fn read_section<R: Read + Seek>(r: &mut R) -> io::Result<SectionHeader> {
    let mut name = [0u8; 8];
    r.read_exact(&mut name)?;

    let _virtual_size = read_u32(r)?;
    let virtual_address = read_u32(r)?;
    let _raw_size = read_u32(r)?;
    let raw_ptr = read_u32(r)?;

    skip(r, 16)?;

    Ok(SectionHeader {
        name,
        virtual_address,
        raw_ptr,
    })
}

fn find_rsrc(sections: &[SectionHeader]) -> Option<&SectionHeader> {
    sections.iter().find(|s| {
        std::str::from_utf8(&s.name)
            .unwrap_or("")
            .trim_end_matches('\0') == ".rsrc"
    })
}


fn parse_version_info<R: Read + Seek>(
    reader: &mut R,
    section: &SectionHeader,
) -> io::Result<Option<PortableVersionInfo>> {
    let base = section.raw_ptr as u64;

    reader.seek(SeekFrom::Start(base))?;
    let total = read_resource_dir(reader)?;

    for _ in 0..total {
        let (id, offset) = read_resource_entry(reader)?;

        if id == 16 {
            return walk_resource_tree(reader, base, offset, section);
        }
    }

    Ok(None)
}

fn walk_resource_tree<R: Read + Seek>(
    reader: &mut R,
    base: u64,
    mut offset: u32,
    section: &SectionHeader,
) -> io::Result<Option<PortableVersionInfo>> {
    // TYPE → NAME → LANGUAGE
    for _ in 0..3 {
        let dir_offset = base + ((offset & 0x7FFFFFFF) as u64);
        reader.seek(SeekFrom::Start(dir_offset))?;

        let count = read_resource_dir(reader)?;
        if count == 0 {
            return Ok(None);
        }

        let (_, next) = read_resource_entry(reader)?;
        offset = next;
    }

    // DATA ENTRY
    let data_offset = base + ((offset & 0x7FFFFFFF) as u64);
    reader.seek(SeekFrom::Start(data_offset))?;

    let data_rva = read_u32(reader)?;
    let size = read_u32(reader)?;

    let file_offset = rva_to_file_offset(data_rva, section);

    reader.seek(SeekFrom::Start(file_offset))?;

    let mut buffer = vec![0u8; size as usize];
    reader.read_exact(&mut buffer)?;

    Ok(PortableVersionInfo::parse(&buffer))
}

fn read_resource_dir<R: Read + Seek>(r: &mut R) -> io::Result<u16> {
    skip(r, 12)?;
    let named = read_u16(r)?;
    let ids = read_u16(r)?;
    Ok(named + ids)
}

fn read_resource_entry<R: Read + Seek>(r: &mut R) -> io::Result<(u32, u32)> {
    let id = read_u32(r)? & 0xFFFF;
    let offset = read_u32(r)?;
    Ok((id, offset))
}
fn rva_to_file_offset(rva: u32, section: &SectionHeader) -> u64 {
    (rva - section.virtual_address + section.raw_ptr) as u64
}
fn read_u8<R: Read>(r: &mut R) -> io::Result<u8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_u16<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut b = [0u8; 2];
    r.read_exact(&mut b)?;
    Ok(u16::from_le_bytes(b))
}

fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

fn skip<R: Seek + Seek>(r: &mut R, n: u64) -> io::Result<()> {
    r.seek(SeekFrom::Current(n as i64))?;
    Ok(())
}


// 0	MachineType	no
// 2	TimeStamp	no
// 9	ImageFileCharacteristics	no
// 10	PEType	no
// 11	LinkerVersion	no
// 12	CodeSize	no
// 14	InitializedDataSize	no
// 16	UninitializedDataSize	no
// 18	EntryPoint	no
// 30	OSVersion	no
// 32	ImageVersion	no
// 34	SubsystemVersion	no
// 44   Subsystem


#[derive(DisplayPretty, Clone, Copy, PartialEq, Eq)]
pub enum MachineType {
    Unknown,
    TargetHost,
    Intel386,
    IntelI860,
    MipsR3000,
    MipsR4000,
    MipsR10000,
    MipsWciV2,
    AlphaAxpOld,
    AlphaAxp,
    HitachiSh3,
    HitachiSh3Dsp,
    HitachiSh3E,
    HitachiSh4,
    HitachiSh5,
    ArmLE,
    Thumb,
    Thumb2LE,
    MatsushitaAm33,
    PowerPCLE,
    PowerPCFP,
    IntelIA64,
    Mips16,
    Motorola68000,
    AlphaAxp64,
    MipsFpu,
    Mips16Fpu,
    InfineonTricore,
    Cef,
    EfiBytecode,
    ChpE,
    RiscV32,
    RiscV64,
    RiscV128,
    LoongArch32,
    LoongArch64,
    Amd64,
    MitsubishiM32R,
    Arm64LE,
    ClrPureMsil,
    Dotnet,
}

impl MachineType {
    pub fn from_val(val: u16) -> Option<Self> {
        match val {
            0x0 => Some(Self::Unknown),
            0x1 => Some(Self::TargetHost),
            0x14c => Some(Self::Intel386),
            0x14d => Some(Self::IntelI860),
            0x162 => Some(Self::MipsR3000),
            0x166 => Some(Self::MipsR4000),
            0x168 => Some(Self::MipsR10000),
            0x169 => Some(Self::MipsWciV2),
            0x183 => Some(Self::AlphaAxpOld),
            0x184 => Some(Self::AlphaAxp),
            0x1a2 => Some(Self::HitachiSh3),
            0x1a3 => Some(Self::HitachiSh3Dsp),
            0x1a4 => Some(Self::HitachiSh3E),
            0x1a6 => Some(Self::HitachiSh4),
            0x1a8 => Some(Self::HitachiSh5),
            0x1c0 => Some(Self::ArmLE),
            0x1c2 => Some(Self::Thumb),
            0x1c4 => Some(Self::Thumb2LE),
            0x1d3 => Some(Self::MatsushitaAm33),
            0x1f0 => Some(Self::PowerPCLE),
            0x1f1 => Some(Self::PowerPCFP),
            0x200 => Some(Self::IntelIA64),
            0x266 => Some(Self::Mips16),
            0x268 => Some(Self::Motorola68000),
            0x284 => Some(Self::AlphaAxp64),
            0x366 => Some(Self::MipsFpu),
            0x466 => Some(Self::Mips16Fpu),
            0x520 => Some(Self::InfineonTricore),
            0xcef => Some(Self::Cef),
            0xebc => Some(Self::EfiBytecode),
            0x3a64 => Some(Self::ChpE),
            0x5032 => Some(Self::RiscV32),
            0x5064 => Some(Self::RiscV64),
            0x5128 => Some(Self::RiscV128),
            0x6232 => Some(Self::LoongArch32),
            0x6264 => Some(Self::LoongArch64),
            0x8664 => Some(Self::Amd64),
            0x9041 => Some(Self::MitsubishiM32R),
            0xaa64 => Some(Self::Arm64LE),
            0xc0ee => Some(Self::ClrPureMsil),
            0xec20 => Some(Self::Dotnet),
            _ => None,
        }
    }
}

// 0x0 = Unknown
// 0x1 = Target host
// 0x14c = Intel 386 or later, and compatibles
// 0x14d = Intel i860
// 0x162 = MIPS R3000
// 0x166 = MIPS little endian (R4000)
// 0x168 = MIPS R10000
// 0x169 = MIPS little endian WCI v2
// 0x183 = Alpha AXP (old)
// 0x184 = Alpha AXP
// 0x1a2 = Hitachi SH3
// 0x1a3 = Hitachi SH3 DSP
// 0x1a4 = Hitachi SH3E
// 0x1a6 = Hitachi SH4
// 0x1a8 = Hitachi SH5
// 0x1c0 = ARM little endian
// 0x1c2 = Thumb
// 0x1c4 = Thumb 2 little endian
// 0x1d3 = Matsushita AM33
// 0x1f0 = PowerPC little endian
// 0x1f1 = PowerPC with floating point support
// 0x200 = Intel IA64
// 0x266 = MIPS16
// 0x268 = Motorola 68000 series
// 0x284 = Alpha AXP 64-bit
// 0x366 = MIPS with FPU
// 0x466 = MIPS16 with FPU
// 0x520 = Infineon Tricore
// 0xcef = CEF
// 0xebc = EFI Byte Code
// 0x3a64 = Compiled Hybrid PE
// 0x5032 = RISC-V 32-bit
// 0x5064 = RISC-V 64-bit
// 0x5128 = RISC-V 128-bit
// 0x6232 = LoongArch 32-bit
// 0x6264 = LoongArch 64-bit
// 0x8664 = AMD AMD64
// 0x9041 = Mitsubishi M32R little endian
// 0xaa64 = ARM64 little endian
// 0xc0ee = clr pure MSIL
// 0xec20 = Dotnet 0xEC20
#[derive(DisplayPretty)]
pub enum SubSystem {
    Native,
    WindowsGui,
    WindowsCli,
    Os2Cli,
    PosixCli,
    WindowsCeGui,
    EfiApplication,
    EfiBootService,
    EfiRuntimeDriver,
    EfiRom,
    Xbox,
    Unknown
}

impl SubSystem {
    pub fn from_val(val: u16) -> Option<Self> {
        match val {
            0 => Some(Self::Unknown),
            1 => Some(Self::Native),
            2 => Some(Self::WindowsGui),
            3 => Some(Self::WindowsCli),
            5 => Some(Self::Os2Cli),
            7 => Some(Self::PosixCli),
            9 => Some(Self::WindowsCeGui),
            10 => Some(Self::EfiApplication),
            11 => Some(Self::EfiBootService),
            12 => Some(Self::EfiRuntimeDriver),
            13 => Some(Self::EfiRom),
            14 => Some(Self::Xbox),
            _ => None,
        }
    }
}


// 0 = Unknown
// 1 = Native
// 2 = Windows GUI
// 3 = Windows command line
// 5 = OS/2 command line
// 7 = POSIX command line
// 9 = Windows CE GUI
// 10 = EFI application
// 11 = EFI boot service
// 12 = EFI runtime driver
// 13 = EFI ROM
// 14 = XBOX
#[derive(DisplayPretty)]
pub enum ImageFileCharacteristics {
    NoRelocs,
    Executable,
    NoLineNumbers,
    NoSymbols,
    AggressiveWorkingSetTrim,
    LargeAddressAware,
    BytesReversedLo,
    ThirtyTwoBit,
    NoDebug,
    RemovableRunFromSwap,
    NetRunFromSwap,
    SystemFile,
    Dll,
    UniprocessorOnly,
    BytesReversedHi,
    Unknown
}

impl ImageFileCharacteristics {
    pub fn from_bit(bit: u16) -> Option<Self> {
        match bit {
            0 => Some(Self::NoRelocs),
            1 => Some(Self::Executable),
            2 => Some(Self::NoLineNumbers),
            3 => Some(Self::NoSymbols),
            4 => Some(Self::AggressiveWorkingSetTrim),
            5 => Some(Self::LargeAddressAware),
            7 => Some(Self::BytesReversedLo),
            8 => Some(Self::ThirtyTwoBit),
            9 => Some(Self::NoDebug),
            10 => Some(Self::RemovableRunFromSwap),
            11 => Some(Self::NetRunFromSwap),
            12 => Some(Self::SystemFile),
            13 => Some(Self::Dll),
            14 => Some(Self::UniprocessorOnly),
            15 => Some(Self::BytesReversedHi),
            _ => None,
        }
    }
}

fn parse_characteristics(bits: u16) -> Vec<ImageFileCharacteristics> {
    let mut result = Vec::new();

    for i in 0..16 {
        if (bits >> i) & 1 == 1 {
            if let Some(flag) = ImageFileCharacteristics::from_bit(i) {
                result.push(flag);
            }
        }
    }

    result
}
// Bit 0 = No relocs
// Bit 1 = Executable
// Bit 2 = No line numbers
// Bit 3 = No symbols
// Bit 4 = Aggressive working-set trim
// Bit 5 = Large address aware
// Bit 7 = Bytes reversed lo
// Bit 8 = 32-bit
// Bit 9 = No debug
// Bit 10 = Removable run from swap
// Bit 11 = Net run from swap
// Bit 12 = System file
// Bit 13 = DLL
// Bit 14 = Uniprocessor only
// Bit 15 = Bytes reversed hi

