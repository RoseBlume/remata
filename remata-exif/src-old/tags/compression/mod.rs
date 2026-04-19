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
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::Uncompressed,
            2 => Self::CCITT1D,
            3 => Self::T4Group3Fax,
            4 => Self::T6Group4Fax,
            5 => Self::LZW,
            6 => Self::JpegOld,
            7 => Self::Jpeg,
            8 => Self::AdobeDeflate,
            9 => Self::JBIGBW,
            10 => Self::JBIGColor,
            99 => Self::JPEG,
            262 => Self::Kodak262,
            32766 => Self::NeXtOrSonyARWCompressed2,
            32767 => Self::SonyARWCompressed,
            32769 => Self::PackedRAW,
            32770 => Self::SamsungSRWCompressed,
            32771 => Self::CCIRLEW,
            32772 => Self::SamsungSRWCompressed2,
            32773 => Self::PackBits,
            32809 => Self::Thunderscan,
            32867 => Self::KodakKDCCompressed,
            32895 => Self::IT8CTPAD,
            32896 => Self::IT8LW,
            32897 => Self::IT8MP,
            32898 => Self::IT8BL,
            32908 => Self::PixarFilm,
            32909 => Self::PixarLog,
            32946 => Self::Deflate,
            32947 => Self::DCS,
            33003 => Self::AperioJPEG2000YCbCr,
            33005 => Self::AperioJPEG2000RGB,
            34661 => Self::JBIG,
            34676 => Self::SGILog,
            34677 => Self::SGILog24,
            34712 => Self::JPEG2000,
            34713 => Self::NikonNEFCompressed,
            34715 => Self::JBIG2TIFFFX,
            34718 => Self::MicrosoftDocumentImagingBinaryLevelCodec,
            34719 => Self::MicrosoftDocumentImagingProgressiveTransformCodec,
            34720 => Self::MicrosoftDocumentImagingVector,
            34887 => Self::ESRILerc,
            34892 => Self::LossyJPEG,
            34925 => Self::LZMA2,
            34926 => Self::ZstdOld,
            34927 => Self::WebPOld,
            34933 => Self::PNG,
            34934 => Self::JPEGXR,
            50000 => Self::Zstd,
            50001 => Self::WebP,
            50002 => Self::JPEGXLOld,
            52546 => Self::JPEGXL,
            65000 => Self::KodakDCRCompressed,
            65535 => Self::PentaxPEFCompressed,
            _ => Self::Unknown,
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
