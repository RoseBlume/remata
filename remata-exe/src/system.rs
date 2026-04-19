
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn from_val(val: u16) -> Self {
        match val {
            0x0 => Self::Unknown,
            0x1 => Self::TargetHost,
            0x14c => Self::Intel386,
            0x14d => Self::IntelI860,
            0x162 => Self::MipsR3000,
            0x166 => Self::MipsR4000,
            0x168 => Self::MipsR10000,
            0x169 => Self::MipsWciV2,
            0x183 => Self::AlphaAxpOld,
            0x184 => Self::AlphaAxp,
            0x1a2 => Self::HitachiSh3,
            0x1a3 => Self::HitachiSh3Dsp,
            0x1a4 => Self::HitachiSh3E,
            0x1a6 => Self::HitachiSh4,
            0x1a8 => Self::HitachiSh5,
            0x1c0 => Self::ArmLE,
            0x1c2 => Self::Thumb,
            0x1c4 => Self::Thumb2LE,
            0x1d3 => Self::MatsushitaAm33,
            0x1f0 => Self::PowerPCLE,
            0x1f1 => Self::PowerPCFP,
            0x200 => Self::IntelIA64,
            0x266 => Self::Mips16,
            0x268 => Self::Motorola68000,
            0x284 => Self::AlphaAxp64,
            0x366 => Self::MipsFpu,
            0x466 => Self::Mips16Fpu,
            0x520 => Self::InfineonTricore,
            0xcef => Self::Cef,
            0xebc => Self::EfiBytecode,
            0x3a64 => Self::ChpE,
            0x5032 => Self::RiscV32,
            0x5064 => Self::RiscV64,
            0x5128 => Self::RiscV128,
            0x6232 => Self::LoongArch32,
            0x6264 => Self::LoongArch64,
            0x8664 => Self::Amd64,
            0x9041 => Self::MitsubishiM32R,
            0xaa64 => Self::Arm64LE,
            0xc0ee => Self::ClrPureMsil,
            0xec20 => Self::Dotnet,
            _ => Self::Unknown,
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
    pub fn from_val(val: u16) -> Self {
        match val {
            1 => Self::Native,
            2 => Self::WindowsGui,
            3 => Self::WindowsCli,
            5 => Self::Os2Cli,
            7 => Self::PosixCli,
            9 => Self::WindowsCeGui
            10 => Self::EfiApplication,
            11 => Self::EfiBootService,
            12 => Self::EfiRuntimeDriver,
            13 => Self::EfiRom,
            14 => Self::Xbox,
            _ => Self::Unknown

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
    pub fn from_bit(bit: u16) -> Self {
        match bit {
            0 => Self::NoRelocs,
            1 => Self::Executable,
            2 => Self::NoLineNumbers,
            3 => Self::NoSymbols,
            4 => Self::AggressiveWorkingSetTrim,
            5 => Self::LargeAddressAware,
            7 => Self::BytesReversedLo,
            8 => Self::ThirtyTwoBit,
            9 => Self::NoDebug,
            10 => Self::RemovableRunFromSwap,
            11 => Self::NetRunFromSwap,
            12 => Self::SystemFile,
            13 => Self::Dll,
            14 => Self::UniprocessorOnly,
            15 => Self::BytesReversedHi,
            _ => Self::Unknown,
        }
    }
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