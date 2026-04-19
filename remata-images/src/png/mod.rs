mod header;
use header::PngHeader;
use crate::common::Exif;
pub struct Png {
    header: Option<PngHeader>, // IHDR
    exif: Option<Exif>, // eXif
    srgb_rendering: Option<PngSRGBRendering>, // sRGB
    xmp: Option<Vec<u8>>, // tXMP Will output raw bytes for now
}

// Tag ID	Tag Name
// 'IHDR'	ImageHeader
// 'PLTE'	Palette
// 'acTL'	AnimationControl
// 'bKGD'	BackgroundColor 
// 'cHRM'	PrimaryChromaticities
// 'cICP'	CICodePoints
// 'caBX'	JUMBF
// 'cpIp'	OLEInfo
// 'dSIG'	DigitalSignature 
// 'eXIf'	eXIf
// 'fRAc'	FractalParameters
// 'gAMA'	Gamma (ExifTool reports the gamma for decoding the image, which is consistent with the EXIF convention, but is the inverse of the stored encoding gamma)
// 'gIFg'	GIFGraphicControlExtension 
// 'gIFt'	GIFPlainTextExtension 
// 'gIFx'	GIFApplicationExtension 
// 'gdAT'	GainMapImage
// 'hIST'	PaletteHistogram 
// 'iCCP'	ICC_Profile	-	--> ICC_Profile Tags
// (this is where ExifTool will write a new ICC_Profile. When creating a new ICC_Profile, the SRGBRendering tag should be deleted if it exists)
// 'iCCP-name'	ProfileName	yes	(not a real tag ID, this tag represents the iCCP profile name, and may only be written when the ICC_Profile is written)
// 'iDOT'	AppleDataOffsets	no	 
// 'iTXt'	InternationalText	-	--> PNG TextualData Tags
// 'meTa'	MeTa	-	--> XMP XML Tags
// 'oFFs'	ImageOffset
// 'pCAL'	PixelCalibration 
// 'pHYs'	PhysicalPixel	-	--> PNG PhysicalPixel Tags
// 'sBIT'	SignificantBits 
// 'sCAL'	SubjectScale --> PNG SubjectScale Tags
// 'sPLT'	SuggestedPalette 
// 'sRGB'	SRGBRendering	yes!	(this chunk should not be present if an iCCP chunk exists)
// 0 = Perceptual
// 1 = Relative Colorimetric
// 2 = Saturation
// 3 = Absolute Colorimetric
// 'sTER'	StereoImage	-	--> PNG StereoImage Tags
// 'seAl'	SEAL	-	--> XMP SEAL Tags
// 'tEXt'	TextualData	-	--> PNG TextualData Tags
// 'tIME'	ModifyDate	yes	 
// 'tRNS'	Transparency	no	 
// 'tXMP'	XMP	-	--> XMP Tags
// (obsolete location specified by a September 2001 XMP draft)
// 'vpAg'	VirtualPage	-	--> PNG VirtualPage Tags
// 'zTXt'	CompressedText	-	--> PNG TextualData Tags
// 'zxIf'	zxIf	-	--> EXIF Tags
// (a once-proposed chunk for compressed EXIF)


pub enum PngSRGBRendering {
    Perceptual,
    RelativeColorimetric,
    Saturation,
    AbsoluteColrimetric,
    Other(u16)
}

impl PngSRGBRendering {
    pub fn from_u16(val: u16) {
        match val {
            0 => Self::Perceptual,
            1 => Self::RelativeColorimetric,
            2 => Self::Saturation,
            3 => Self::AbsoluteColrimetric,
            _ => Self::Other(val)
        }
    }
}
// 0 = Perceptual
// 1 = Relative Colorimetric
// 2 = Saturation
// 3 = Absolute Colorimetric