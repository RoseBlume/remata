use remata_macros::{DisplayPretty, FromPrimitive};

#[derive(DisplayPretty, FromPrimitive)]
pub enum Orientation {
    #[value = 1]
    Horizontal,
    #[value = 2]
    MirrorHorizontal,
    #[value = 3]
    Rotate180,
    #[value = 4]
    MirrorVertical,
    #[value = 5]
    MirrorHorizontalAndRotate270Cw,
    #[value = 6]
    Rotate90Cw,
    #[value = 7]
    MirrorHorizontalAndRotate90Cw,
    #[value = 8]
    Rotate270Cw
}



// 1 = Horizontal (normal)
// 2 = Mirror horizontal
// 3 = Rotate 180
// 4 = Mirror vertical
// 5 = Mirror horizontal and rotate 270 CW
// 6 = Rotate 90 CW
// 7 = Mirror horizontal and rotate 90 CW
// 8 = Rotate 270 CW

#[derive(DisplayPretty)]
pub enum Compression {
    Uncompressed,
    CCITT1D,
    T4Group3Fax,
    T6Group4Fax,
    LZW,
    JpegOld,
    Jpeg,
    AdobeDeflate,
    JBIGBW,
    JBIGColor,
    JPEG,
    Kodak262,
    NeXtOrSonyARWCompressed2,
    SonyARWCompressed,
    PackedRAW,
    SamsungSRWCompressed,
    CCIRLEW,
    SamsungSRWCompressed2,
    PackBits,
    Thunderscan,
    KodakKDCCompressed,
    IT8CTPAD,
    IT8LW,
    IT8MP,
    IT8BL,
    PixarFilm,
    PixarLog,
    Deflate,
    DCS,
    AperioJPEG2000YCbCr,
    AperioJPEG2000RGB,
    JBIG,
    SGILog,
    SGILog24,
    JPEG2000,
    NikonNEFCompressed,
    JBIG2TIFFFX,
    MicrosoftDocumentImagingBinaryLevelCodec,
    MicrosoftDocumentImagingProgressiveTransformCodec,
    MicrosoftDocumentImagingVector,
    ESRILerc,
    LossyJPEG,
    LZMA2,
    ZstdOld,
    WebPOld,
    PNG,
    JPEGXR,
    Zstd,
    WebP,
    JPEGXLOld,
    JPEGXL,
    KodakDCRCompressed,
    PentaxPEFCompressed,
    Unknown,
}

impl Compression {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::Uncompressed),
            2 => Some(Self::CCITT1D),
            3 => Some(Self::T4Group3Fax),
            4 => Some(Self::T6Group4Fax),
            5 => Some(Self::LZW),
            6 => Some(Self::JpegOld),
            7 => Some(Self::Jpeg),
            8 => Some(Self::AdobeDeflate),
            9 => Some(Self::JBIGBW),
            10 => Some(Self::JBIGColor),
            99 => Some(Self::JPEG),
            262 => Some(Self::Kodak262),
            32766 => Some(Self::NeXtOrSonyARWCompressed2),
            32767 => Some(Self::SonyARWCompressed),
            32769 => Some(Self::PackedRAW),
            32770 => Some(Self::SamsungSRWCompressed),
            32771 => Some(Self::CCIRLEW),
            32772 => Some(Self::SamsungSRWCompressed2),
            32773 => Some(Self::PackBits),
            32809 => Some(Self::Thunderscan),
            32867 => Some(Self::KodakKDCCompressed),
            32895 => Some(Self::IT8CTPAD),
            32896 => Some(Self::IT8LW),
            32897 => Some(Self::IT8MP),
            32898 => Some(Self::IT8BL),
            32908 => Some(Self::PixarFilm),
            32909 => Some(Self::PixarLog),
            32946 => Some(Self::Deflate),
            32947 => Some(Self::DCS),
            33003 => Some(Self::AperioJPEG2000YCbCr),
            33005 => Some(Self::AperioJPEG2000RGB),
            34661 => Some(Self::JBIG),
            34676 => Some(Self::SGILog),
            34677 => Some(Self::SGILog24),
            34712 => Some(Self::JPEG2000),
            34713 => Some(Self::NikonNEFCompressed),
            34715 => Some(Self::JBIG2TIFFFX),
            34718 => Some(Self::MicrosoftDocumentImagingBinaryLevelCodec),
            34719 => Some(Self::MicrosoftDocumentImagingProgressiveTransformCodec),
            34720 => Some(Self::MicrosoftDocumentImagingVector),
            34887 => Some(Self::ESRILerc),
            34892 => Some(Self::LossyJPEG),
            34925 => Some(Self::LZMA2),
            34926 => Some(Self::ZstdOld),
            34927 => Some(Self::WebPOld),
            34933 => Some(Self::PNG),
            34934 => Some(Self::JPEGXR),
            50000 => Some(Self::Zstd),
            50001 => Some(Self::WebP),
            50002 => Some(Self::JPEGXLOld),
            52546 => Some(Self::JPEGXL),
            65000 => Some(Self::KodakDCRCompressed),
            65535 => Some(Self::PentaxPEFCompressed),
            _ => None,
        }
    }
}


// 1	= Uncompressed
// 2	= CCITT 1D
// 3	= T4/Group 3 Fax
// 4	= T6/Group 4 Fax
// 5	= LZW
// 6	= JPEG (old-style)
// 7	= JPEG
// 8	= Adobe Deflate
// 9	= JBIG B&W
// 10	= JBIG Color
// 99	= JPEG
// 262	= Kodak 262
// 32766	= NeXt or Sony ARW Compressed 2
// 32767	= Sony ARW Compressed
// 32769	= Packed RAW
// 32770	= Samsung SRW Compressed
// 32771	= CCIRLEW
// 32772	= Samsung SRW Compressed 2
// 32773	= PackBits
// 32809	= Thunderscan
// 32867	= Kodak KDC Compressed
// 32895	= IT8CTPAD
// 32896	= IT8LW
// 32897	= IT8MP
// 32898	= IT8BL
// 32908	= PixarFilm
// 32909	= PixarLog
// 32946	= Deflate
// 32947	= DCS
// 33003	= Aperio JPEG 2000 YCbCr
// 33005	= Aperio JPEG 2000 RGB
// 34661	= JBIG
// 34676	= SGILog
// 34677	= SGILog24
// 34712	= JPEG 2000
// 34713	= Nikon NEF Compressed
// 34715	= JBIG2 TIFF FX
// 34718	= Microsoft Document Imaging (MDI) Binary Level Codec
// 34719	= Microsoft Document Imaging (MDI) Progressive Transform Codec
// 34720	= Microsoft Document Imaging (MDI) Vector
// 34887	= ESRI Lerc
// 34892	= Lossy JPEG
// 34925	= LZMA2
// 34926	= Zstd (old)
// 34927	= WebP (old)
// 34933	= PNG
// 34934	= JPEG XR
// 50000	= Zstd
// 50001	= WebP
// 50002	= JPEG XL (old)
// 52546	= JPEG XL
// 65000	= Kodak DCR Compressed
// 65535	= Pentax PEF Compressed


#[derive(DisplayPretty, FromPrimitive)]
pub enum ResolutionUnit {
    #[value = 1]
    None,
    #[value = 2]
    Inches,
    #[value = 3]
    Centimeters
}

// 1 = None
// 2 = inches
// 3 = cm

#[derive(DisplayPretty, FromPrimitive)]
pub enum YCbCrPositioning {
    #[value = 1]
    Centered,
    #[value = 2]
    CoSited
}

// 1 = Centered
// 2 = Co-sited
#[derive(DisplayPretty)]
pub enum YCbCrSubSampling {
    YCbCr4_4_4,
    YCbCr4_4_0,
    YCbCr4_4_1,
    YCbCr4_2_2,
    YCbCr4_2_0,
    YCbCr4_2_1,
    YCbCr4_1_1,
    YCbCr4_1_0,
}

impl YCbCrSubSampling {
    pub fn from_seq(val: [u16; 2]) -> Option<Self> {
        match val {
            [1, 1] => Some(Self::YCbCr4_4_4),
            [1, 2] => Some(Self::YCbCr4_4_0),
            [1, 4] => Some(Self::YCbCr4_4_1),
            [2, 1] => Some(Self::YCbCr4_2_2),
            [2, 2] => Some(Self::YCbCr4_2_0),
            [2, 4] => Some(Self::YCbCr4_2_1),
            [4, 1] => Some(Self::YCbCr4_1_1),
            [4, 2] => Some(Self::YCbCr4_1_0),
            _ => None
        }
    }
}


// '1 1' = YCbCr4:4:4 (1 1)
// '1 2' = YCbCr4:4:0 (1 2)
// '1 4' = YCbCr4:4:1 (1 4)
// '2 1' = YCbCr4:2:2 (2 1)
// '2 2' = YCbCr4:2:0 (2 2)
// '2 4' = YCbCr4:2:1 (2 4)
// '4 1' = YCbCr4:1:1 (4 1)
// '4 2' = YCbCr4:1:0 (4 2)




#[derive(DisplayPretty, FromPrimitive)]
pub enum Flash {
    #[value = 0x0]
    NoFlash,
    #[value = 0x1]
    Fired,
    #[value = 0x5]
    FiredReturnNotDetected,
    #[value = 0x7]
    FiredReturnDetected,
    #[value = 0x8]
    OnDidNotFire,
    #[value = 0x9]
    OnFired,
    #[value = 0xd]
    OnReturnNotDetected,
    #[value = 0xf]
    OnReturnDetected,
    #[value = 0x10]
    OffDidNotFire,
    #[value = 0x14]
    OffDidNotFireReturnNotDetected,
    #[value = 0x18]
    AutoDidNotFire,
    #[value = 0x19]
    AutoFired,
    #[value = 0x1d]
    AutoFiredReturnNotDetected,
    #[value = 0x1f]
    AutoFiredReturnDetected,
    #[value = 0x20]
    NoFlashFunction,
    #[value = 0x30]
    OffNoFlashFunction,
    #[value = 0x41]
    FiredRedEyeReduction,
    #[value = 0x45]
    FiredRedEyeReductionReturnNotDetected,
    #[value = 0x47]
    FiredRedEyeReductionReturnDetected,
    #[value = 0x49]
    OnRedEyeReduction,
    #[value = 0x4d]
    OnRedEyeReductionReturnNotDetected,
    #[value = 0x4f]
    OnRedEyeReductionReturnDetected,
    #[value = 0x50]
    OffRedEyeReduction,
    #[value = 0x58]
    AutoDidNotFireRedEyeReduction,
    #[value = 0x59]
    AutoFiredRedEyeReduction,
    #[value = 0x5d]
    AutoFiredRedEyeReductionReturnNotDetected,
    #[value = 0x5f]
    AutoFiredRedEyeReductionReturnDetected,
}


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
