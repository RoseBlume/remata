pub enum FlashMode {
    NoFlash,
    FlashNoStrobeReturn,
    FlashStrobeReturn,
    FlashCompulsory,
    FlashCompulsoryNoStrobeReturn,
    FlashCompulsoryStrobeReturn,
    NoFlashAuto,
    FlashAuto,
    FlashAutoNoStrobeReturn,
    FlashAutoStrobeReturn,
    NoFlashFunction,
    FlashRedEye,
    FlashRedEyeNoStrobeReturn,
    FlashRedEyeStrobeReturn,
    FlashCompulsoryRedEye,
    FlashCompulsoryRedEyeNoStrobeReturn,
    FlashCompulsoryRedEyeStrobeReturn,
    FlashAutoRedEye,
    FlashAutoNoStrobeReturnRedEye,
    FlashAutoStrobeReturnRedEye,
}

impl FlashMode {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0x00 => Self::NoFlash,
            0x01 | 0x05 => Self::FlashNoStrobeReturn,
            0x07 => Self::FlashStrobeReturn,
            0x08 => Self::FlashCompulsory,
            0x09 | 0x0d => Self::FlashCompulsoryNoStrobeReturn,
            0x0f => Self::FlashCompulsoryStrobeReturn,
            0x10 | 0x14 => Self::NoFlashAuto,
            0x18 => Self::FlashAuto,
            0x19 | 0x1d => Self::FlashAutoNoStrobeReturn,
            0x1f => Self::FlashAutoStrobeReturn,
            0x20 | 0x30 => Self::NoFlashFunction,
            0x41 => Self::FlashRedEye,
            0x45 => Self::FlashRedEyeNoStrobeReturn,
            0x47 => Self::FlashRedEyeStrobeReturn,
            0x49 => Self::FlashCompulsoryRedEye,
            0x4d => Self::FlashCompulsoryRedEyeNoStrobeReturn,
            0x4f => Self::FlashCompulsoryRedEyeStrobeReturn,
            0x50 => Self::FlashAutoRedEye,
            0x58 | 0x59 | 0x5d => Self::FlashAutoNoStrobeReturnRedEye,
            0x5f => Self::FlashAutoStrobeReturnRedEye,
            _ => Self::NoFlash,
        }
    }
}

// Flash Values
// 0x0	= No Flash
// 0x1	= Fired
// 0x5	= Fired, Return not detected
// 0x7	= Fired, Return detected
// 0x8	= On, Did not fire
// 0x9	= On, Fired
// 0xd	= On, Return not detected
// 0xf	= On, Return detected
// 0x10	= Off, Did not fire
// 0x14	= Off, Did not fire, Return not detected
// 0x18	= Auto, Did not fire
// 0x19	= Auto, Fired
// 0x1d	= Auto, Fired, Return not detected
// 0x1f	= Auto, Fired, Return detected
// 0x20	= No flash function
// 0x30	= Off, No flash function
// 0x41	= Fired, Red-eye reduction
// 0x45	= Fired, Red-eye reduction, Return not detected
// 0x47	= Fired, Red-eye reduction, Return detected
// 0x49	= On, Red-eye reduction
// 0x4d	= On, Red-eye reduction, Return not detected
// 0x4f	= On, Red-eye reduction, Return detected
// 0x50	= Off, Red-eye reduction
// 0x58	= Auto, Did not fire, Red-eye reduction
// 0x59	= Auto, Fired, Red-eye reduction
// 0x5d	= Auto, Fired, Red-eye reduction, Return not detected
// 0x5f	= Auto, Fired, Red-eye reduction, Return detected


pub enum MeteringMode {
    Unknown,
    Average, // 1
    CenterWeightedAverage, // 2
    Spot, // 3
    MultiSpot, // 4
    MultiSegment, // 5
    Partial, // 6
    Other // 255
}

impl MeteringMode {
    pub fn from_u16(val: u16) -> Self {
        match val {
            1 => Self::Average,
            2 => Self::CenterWeightedAverage,
            3 => Self::Spot,
            4 => Self::MultiSpot,
            5 => Self::MultiSegment,
            6 => Self::Partial,
            255 => Self::Other,
            _ => Self::Unknown
        }

    }
}

// 0 = Unknown
// 1 = Average
// 2 = Center-weighted average
// 3 = Spot
// 4 = Multi-spot
// 5 = Multi-segment
// 6 = Partial
// 255 = Other

pub enum SubjectDistanceRange {
    Unknown,
    Macro,
    Close,
    Distant
}

impl SubjectDistanceRange {
    pub fn from_u16(val: u16) -> Self {
        match val {
            1 => Self::Macro,
            2 => Self::Close,
            3 => Self::Distant,
            _ => Self::Unknown
        }
    }
}

// 0 = Unknown
// 1 = Macro
// 2 = Close
// 3 = Distant

pub enum Contrast {
    Normal,
    Soft,
    Hard,
    Unknown
}

impl Contrast {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Normal,
            1 => Self::Soft,
            2 => Self::Hard,
            _ => Self::Unknown
        }
    }
}

// 0 = Normal
// 1 = Low
// 2 = High


pub enum LightSource {
    Unknown,               // 0
    Daylight,              // 1
    Fluorescent,           // 2
    Tungsten,              // 3
    Flash,                 // 4
    FineWeather,           // 9
    Cloudy,                // 10
    Shade,                 // 11
    DaylightFluorescent,   // 12
    DayWhiteFluorescent,   // 13
    CoolWhiteFluorescent,  // 14
    WhiteFluorescent,      // 15
    WarmWhiteFluorescent,  // 16
    StandardIlluminantA,   // 17
    StandardIlluminantB,   // 18
    StandardIlluminantC,   // 19
    D55,                   // 20
    D65,                   // 21
    D75,                   // 22
    D50,                   // 23
    ISOStudioTungsten,     // 24
    Other,                 // 255
}

impl LightSource {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Unknown,
            1 => Self::Daylight,
            2 => Self::Fluorescent,
            3 => Self::Tungsten,
            4 => Self::Flash,
            9 => Self::FineWeather,
            10 => Self::Cloudy,
            11 => Self::Shade,
            12 => Self::DaylightFluorescent,
            13 => Self::DayWhiteFluorescent,
            14 => Self::CoolWhiteFluorescent,
            15 => Self::WhiteFluorescent,
            16 => Self::WarmWhiteFluorescent,
            17 => Self::StandardIlluminantA,
            18 => Self::StandardIlluminantB,
            19 => Self::StandardIlluminantC,
            20 => Self::D55,
            21 => Self::D65,
            22 => Self::D75,
            23 => Self::D50,
            24 => Self::ISOStudioTungsten,
            255 => Self::Other,
            _ => Self::Unknown,
        }
    }
}

pub enum ExposureProgram {
    Unknown,
    NotDefined, // 0
    Manual, // 1
    ProgramAE, // 2
    AperturePriority, // 3
    ShutterPriority, // 4
    Creative, // 5 (Slow speed. Biased toward depth of field)
    Action, // 6 (High speed. Biased toward shutter speed)
    PortraitMode, // 7
    LandscapeMode, // 8
    Bulb // 9 (Not standard but still used by some Canon models)
}

impl ExposureProgram {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::NotDefined,
            1 => Self::Manual,
            2 => Self::ProgramAE,
            3 => Self::AperturePriority,
            4 => Self::ShutterPriority,
            5 => Self::Creative,
            6 => Self::Action,
            7 => Self::PortraitMode,
            8 => Self::LandscapeMode,
            9 => Self::Bulb,
            _ => Self::Unknown
        }
    }
}

// pub enum ExposureMode {
//     Auto, // 0
//     Manual, // 1
//     AutoBracket, // 2
//     Unknown
// }

// impl ExposureMode {
//     pub fn from_u16(val: u16) -> Self {
//         match val {
//             0 => Self::Auto,
//             1 => Self::Manual,
//             2 => Self::AutoBracket,
//             _ => Self::Unknown
//         }
//     }
// }

// // 0 = Auto
// // 1 = Manual
// // 2 = Auto bracket


pub enum Saturation {
    Normal,
    Low,
    High,
    Unknown
}

impl Saturation {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Normal,
            1 => Self::Low,
            2 => Self::High,
            _ => Self::Unknown
        }
    }
}
// 0 = Normal
// 1 = Low
// 2 = High

pub enum Sharpness {
    Normal,
    Soft,
    Hard,
    Unknown
}

impl Sharpness {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Normal,
            1 => Self::Soft,
            2 => Self::Hard,
            _ => Self::Unknown
        }
    }
}

// 0 = Normal
// 1 = Soft
// 2 = Hard

pub enum WhiteBalance {
    Auto, // 0
    Manual, // 1
}

impl WhiteBalance {
    pub fn from_u16(val: u16) -> Self {
        match val {
            1 => Self::Manual,
            _ => Self::Auto
        }
    }
}

// 0 = Auto
// 1 = Manual

pub enum SceneCaptureType {
    Standard, // 0
    Landscape, // 1
    Portrait, // 2
    Night, // 3
    Other, // 4
    Unknown
}

impl SceneCaptureType {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Standard,
            1 => Self::Landscape,
            2 => Self::Portrait,
            3 => Self::Night,
            4 => Self::Other,
            _ => Self::Unknown
        }
    }
}

// 0 = Standard
// 1 = Landscape
// 2 = Portrait
// 3 = Night
// 4 = Other (the value of 4 is non-standard, and used by some Samsung models)


pub enum PhotoMetricInterpretation {
    WhiteIsZero = 0,
    BlackIsZero = 1,
    Rgb = 2,
    RgbPalette = 3,
    TransparencyMask = 4,
    CMYK = 5,
    YCbCr = 6,
    CIELab = 8,
    ICCLab = 9,
    ITULab = 10,
    ColorFilterArray = 32803,
    PixarLogL = 32844,
    PixarLogLuv = 32845,
    SequentialColorFilter = 32892,
    LinearRaw = 34892,
    DepthMap = 51177,
    SemanticMask = 52527
}



impl PhotoMetricInterpretation {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::WhiteIsZero,
            1 => Self::BlackIsZero,
            2 => Self::Rgb,
            3 => Self::RgbPalette,
            4 => Self::TransparencyMask,
            5 => Self::CMYK,
            6 => Self::YCbCr,
            8 => Self::CIELab,
            9 => Self::ICCLab,
            10 => Self::ITULab,
            32803 => Self::ColorFilterArray,
            32844 => Self::PixarLogL,
            32845 => Self::PixarLogLuv,
            32892 => Self::SequentialColorFilter,
            34892 => Self::LinearRaw,
            51177 => Self::DepthMap,
            52527 => Self::SemanticMask,
            _ => Self::WhiteIsZero,
        }
    }
}



// 0 = WhiteIsZero
// 1 = BlackIsZero
// 2 = RGB
// 3 = RGB Palette
// 4 = Transparency Mask
// 5 = CMYK
// 6 = YCbCr
// 8 = CIELab
// 9 = ICCLab
// 10 = ITULab
// 32803 = Color Filter Array
// 32844 = Pixar LogL
// 32845 = Pixar LogLuv
// 32892 = Sequential Color Filter
// 34892 = Linear Raw


pub enum Opcode {
    WarpRectilinear = 1,
    WarpFisheye = 2,
    FixVignetteRadial = 3,
    FixBadPixelsConstant = 4,
    FixBadPixelsList = 5,
    TrimBounds = 6,
    MapTable = 7,
    MapPolynomial = 8,
    GainMap = 9,
    DeltaPerRow = 10,
    DeltaPerColumn = 11,
    ScalePerRow = 12,
    ScalePerColumn = 13,
    WarpRectilinear2 = 14,
    Unknown
}

// 1 = WarpRectilinear
// 2 = WarpFisheye
// 3 = FixVignetteRadial
// 4 = FixBadPixelsConstant
// 5 = FixBadPixelsList
// 6 = TrimBounds
// 7 = MapTable
// 8 = MapPolynomial
// 9 = GainMap
// 10 = DeltaPerRow
// 11 = DeltaPerColumn
// 12 = ScalePerRow
// 13 = ScalePerColumn
// 14 = WarpRectilinear2

pub enum ProfileHueSatMapEncoding {
    Linear = 0,
    SRgb = 1,
    Unknown
}

// 0 = Linear
// 1 = sRGB

pub enum ProfileLookTableEncoding {
    Linear = 0,
    SRgb = 1,
    Unknown
}

// 0 = Linear
// 1 = sRGB

pub enum DefaultBlackRender {
    Auto = 0,
    None = 1,
    Unknown
}

// 0 = Auto
// 1 = None

pub enum DepthFormat {
    Linear = 1,
    Inverse = 2,
    Unknown
}

// 0 = Unknown
// 1 = Linear
// 2 = Inverse

pub enum DepthUnits {
    Meters = 1,
    Unknown
}

// 0 = Unknown
// 1 = Meters

pub enum DepthMeasureType {
    OpticalAxis = 1,
    OpticalRay = 2,
    Unknown
}

// 0 = Unknown
// 1 = Optical Axis
// 2 = Optical Ray


// The actual PixelFormat values are 16-byte GUID's but the leading 15 bytes, '6fddc324-4e03-4bfe-b1853-d77768dc9'
pub enum PixelFormat {
    BlackAndWhite = 0x5,
    EightBitGray = 0x8,
    SixteenBitBgr555 = 0x9,
    SixteenBitBgr565 = 0xa,
    SixteenBitGray = 0xb,
    TwentyFourBitBgr = 0xc,
    TwentyFourBitRgb = 0xd,
    ThirtyTwoBitBgr = 0xe,
    ThirtyTwoBitBgra = 0xf,
    ThirtyTwoBitPbgra = 0x10,
    ThirtyTwoBitGrayFloat = 0x11,
    FortyEightBitRgbFixedPoint = 0x12,
    ThirtyTwoBitBgr101010 = 0x13,
    FortyEightBitRgb = 0x15,
    SixtyFourBitRgba = 0x16,
    SixtyFourBitPrgba = 0x17,
    NinetySixBitRgbFixedPoint = 0x18,
    OneTwentyEightBitRgbaFloat = 0x19,
    OneTwentyEightBitPrgbaFloat = 0x1a,
    OneTwentyEightBitRgbFloat = 0x1b,
    ThirtyTwoBitCmyk = 0x1c,
    SixtyFourBitRgbaFixedPoint = 0x1d,
    OneTwentyEightBitRgbaFixedPoint = 0x1e,
    SixtyFourBitCmyk = 0x1f,
    TwentyFourBitThreeChannels = 0x20,
    ThirtyTwoBitFourChannels = 0x21,
    FortyBitFiveChannels = 0x22,
    FortyEightBitSixChannels = 0x23,
    FiftySixBitSevenChannels = 0x24,
    SixtyFourBitEightChannels = 0x25,
    FortyEightBitThreeChannels = 0x26,
    SixtyFourBitFourChannels = 0x27,
    EightyBitFiveChannels = 0x28,
    NinetySixBitSixChannels = 0x29,
    OneTwelvebitSevenChannels = 0x2a,
    OneTwentyEightBitEightChannels = 0x2b,
    FortyBitCmykAlpha = 0x2c,
    EightyBitCmykAlpha = 0x2d,
    ThirtyTwoBitThreeChannelsAlpha = 0x2e,
    FortyBitFourChannelsAlpha = 0x2f,
    FortyEightBitFiveChannelsAlpha = 0x30,
    FiftySixBitSixChannelsAlpha = 0x31,
    SixtyFourBitSevenChannelsAlpha = 0x32,
    SeventyTwoBitEightChannelsAlpha = 0x33,
    SixtyFourBitThreeChannelsAlpha = 0x34,
    EightyBitFourChannelsAlpha = 0x35,
    NinetySixBitFiveChannelsAlpha = 0x36,
    OneTwelvebitSixChannelsAlpha = 0x37,
    OneTwentyEightBitSevenChannelsAlpha = 0x38,
    OneFortyfourBitEightChannelsAlpha = 0x39,
    SixtyFourBitRgbaHalf = 0x3a,
    FortyEightBitRgbHalf = 0x3b,
    ThirtyTwoBitRgbe = 0x3d,
    SixteenBitGrayHalf = 0x3e,
    ThirtyTwoBitGrayFixedPoint = 0x3f,
    Unknown
}

// 0x5 = Black & White
// 0x8 = 8-bit Gray
// 0x9 = 16-bit BGR555
// 0xa = 16-bit BGR565
// 0xb = 16-bit Gray
// 0xc = 24-bit BGR
// 0xd = 24-bit RGB
// 0xe = 32-bit BGR
// 0xf = 32-bit BGRA
// 0x10 = 32-bit PBGRA
// 0x11 = 32-bit Gray Float
// 0x12 = 48-bit RGB Fixed Point
// 0x13 = 32-bit BGR101010
// 0x15 = 48-bit RGB
// 0x16 = 64-bit RGBA
// 0x17 = 64-bit PRGBA
// 0x18 = 96-bit RGB Fixed Point
// 0x19 = 128-bit RGBA Float
// 0x1a = 128-bit PRGBA Float
// 0x1b = 128-bit RGB Float
// 0x1c = 32-bit CMYK
// 0x1d = 64-bit RGBA Fixed Point
// 0x1e = 128-bit RGBA Fixed Point
// 0x1f = 64-bit CMYK
// 0x20 = 24-bit 3 Channels
// 0x21 = 32-bit 4 Channels
// 0x22 = 40-bit 5 Channels
// 0x23 = 48-bit 6 Channels
// 0x24 = 56-bit 7 Channels
// 0x25 = 64-bit 8 Channels
// 0x26 = 48-bit 3 Channels
// 0x27 = 64-bit 4 Channels
// 0x28 = 80-bit 5 Channels
// 0x29 = 96-bit 6 Channels
// 0x2a = 112-bit 7 Channels
// 0x2b = 128-bit 8 Channels
// 0x2c = 40-bit CMYK Alpha
// 0x2d = 80-bit CMYK Alpha
// 0x2e = 32-bit 3 Channels Alpha
// 0x2f = 40-bit 4 Channels Alpha
// 0x30 = 48-bit 5 Channels Alpha
// 0x31 = 56-bit 6 Channels Alpha
// 0x32 = 64-bit 7 Channels Alpha
// 0x33 = 72-bit 8 Channels Alpha
// 0x34 = 64-bit 3 Channels Alpha
// 0x35 = 80-bit 4 Channels Alpha
// 0x36 = 96-bit 5 Channels Alpha
// 0x37 = 112-bit 6 Channels Alpha
// 0x38 = 128-bit 7 Channels Alpha
// 0x39 = 144-bit 8 Channels Alpha
// 0x3a = 64-bit RGBA Half
// 0x3b = 48-bit RGB Half
// 0x3d = 32-bit RGBE
// 0x3e = 16-bit Gray Half
// 0x3f = 32-bit Gray Fixed Point


pub enum CompositeImage {
    NotCompositeImage = 1,
    GeneralCompositeImage = 2,
    CompositeImageCapturedWhileShooting = 3,
    Unknown
}

// 0 = Unknown
// 1 = Not a Composite Image
// 2 = General Composite Image
// 3 = Composite Image Captured While Shooting


pub enum CustomRendered {
    Normal = 0,
    Custom = 1,
    HdrNoOriginalSaved = 2,
    HdrOriginalSaved = 3,
    Original = 4,
    Panorama = 6,
    PortraitHdr = 7,
    Portrait = 8
}

// 0 = Normal
// 1 = Custom
// 2 = HDR (no original saved)
// 3 = HDR (original saved)
// 4 = Original (for HDR)
// 6 = Panorama
// 7 = Portrait HDR
// 8 = Portrait

pub enum FileSource {
    FilmScanner,
    ReflectionPrintScanner,
    DigitalCamera,
    SigmaCamera,
    Unknown
}

// 1 = Film Scanner
// 2 = Reflection Print Scanner
// 3 = Digital Camera
// "\x03\x00\x00\x00" = Sigma Digital Camera

pub enum SensingMethod {
    NotDefined = 1,
    OneChipColorArea = 2,
    TwoChipColorArea = 3,
    ThreeChipColorArea = 4,
    ColorSequentialArea = 5,
    Trilinear = 7,
    ColorSequentialLinear = 8,
    Unknown
}

// 1 = Not defined
// 2 = One-chip color area
// 3 = Two-chip color area
// 4 = Three-chip color area
// 5 = Color sequential area
// 7 = Trilinear
// 8 = Color sequential linear

pub enum FocalPlaneResolutionUnit {
    None = 1,
    Inches = 2,
    Cm = 3,
    Mm = 4,
    Um = 5
}

// 1 = None
// 2 = inches
// 3 = cm
// 4 = mm
// 5 = um

pub enum ColorSpace {
    SRgb = 0x1,
    AdobeRgb = 0x2,
    WideGamutRgb = 0xfffd,
    IccProfile = 0xfffe,
    Uncalibrated = 0xffff,
    Unknown
}

// 0x1 = sRGB
// 0x2 = Adobe RGB
// 0xfffd = Wide Gamut RGB
// 0xfffe = ICC Profile
// 0xffff = Uncalibrated

pub enum SecurityClassification {
    Confidential,
    Restricted,
    Secret,
    TopSecret,
    Unclassified,
    Unknown
}

// 'C' = Confidential
// 'R' = Restricted
// 'S' = Secret
// 'T' = Top Secret
// 'U' = Unclassified

pub enum SensitivityType {
    StandardOutputSensitivity = 1,
    RecommendedExposureIndex = 2,
    ISOSpeed = 3,
    StandardOutputSensitivityAndRecommendedExposureIndex = 4,
    StandardOutputSensitivityAndISOSpeed = 5,
    RecommendedExposureIndexAndISOSpeed = 6,
    StandardOutputSensitivityRecommendedExposureIndexAndISOSpeed = 7,
    Unknown
}

// 0 = Unknown
// 1 = Standard Output Sensitivity
// 2 = Recommended Exposure Index
// 3 = ISO Speed
// 4 = Standard Output Sensitivity and Recommended Exposure Index
// 5 = Standard Output Sensitivity and ISO Speed
// 6 = Recommended Exposure Index and ISO Speed
// 7 = Standard Output Sensitivity, Recommended Exposure Index and ISO Speed