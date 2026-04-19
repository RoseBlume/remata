use crate::Exif;
use super::consts::*;
use super::enums::*;
#[macro_export]
macro_rules! exif_registry {
    (
        $(
            $field:ident : $const:ident => $kind:tt
        ),* $(,)?
    ) => {
        pub fn registry() -> &'static [(u16, fn(&mut Exif, &[u8]))] {
            &[
                $(
                    (
                        $const,
                        |exif: &mut Exif, v: &[u8]| {
                            exif_registry!(@dispatch exif, $field, v, $kind);
                        }
                    ),
                )*
            ]
        }
    };

    /* =========================
       STRING
       ========================= */
    (@dispatch $exif:ident, $name:ident, $v:ident, string) => {{
        let end = $v.iter().position(|&b| b == 0).unwrap_or($v.len());
        $exif.$name = Some(String::from_utf8_lossy(&$v[..end]).to_string());
    }};

    /* =========================
       ARRAY
       ========================= */
    (@dispatch $exif:ident, $name:ident, $v:ident, [$ty:ident]) => {{
        let mut out = Vec::new();
        let size = std::mem::size_of::<$ty>();

        for chunk in $v.chunks(size) {
            let bytes: [u8; std::mem::size_of::<$ty>()] =
                chunk.try_into().unwrap();
            out.push(<$ty>::from_le_bytes(bytes));
        }

        $exif.$name = Some(out);
    }};

    /* =========================
       ENUM / CUSTOM MAPPING (NEW)
       ========================= */
    (@dispatch $exif:ident, $name:ident, $v:ident, enum::from($ty:ty)) => {{
        let bytes: [u8; std::mem::size_of::<$ty>()] =
            $v[..std::mem::size_of::<$ty>()].try_into().unwrap();

        let raw = <$ty>::from_le_bytes(bytes);

        $exif.$name = Some(raw.into());
    }};

    /* =========================
       SCALAR
       ========================= */
    (@dispatch $exif:ident, $name:ident, $v:ident, $ty:ty) => {{
        let bytes: [u8; std::mem::size_of::<$ty>()] =
            $v[..std::mem::size_of::<$ty>()].try_into().unwrap();

        $exif.$name = Some(<$ty>::from_le_bytes(bytes));
    }};
}

exif_registry! {
    offsetschema: OFFSETSCHEMA => i32,
    seal: SEAL => string,
    jxldecodespeed: JXLDECODESPEED => u32,
    profilehuesatmapencoding: PROFILEHUESATMAPENCODING => u32,
    orientation: ORIENTATION => u16,
    focallengthin35mmformat: FOCALLENGTHIN35MMFORMAT => u16,
    exposure: EXPOSURE => string,
    converter: CONVERTER => string,
    rawfile: RAWFILE => string,
    lens: LENS => string,
    lensinfo: LENSINFO => [f64],
    lensmake: LENSMAKE => string,
    lensmodel: LENSMODEL => string,
    lensserialnumber: LENSSERIALNUMBER => string,

    exposuretime: EXPOSURETIME => f64,
    shadows: SHADOWS => string,
    smoothness: SMOOTHNESS => string,
    brightness: BRIGHTNESS => string,
    framerate: FRAMERATE => f64,
    interopindex: INTEROPINDEX => string,
    processingsoftware: PROCESSINGSOFTWARE => string,
    moirefilter: MOIREFILTER => string,
    subfiletype: SUBFILETYPE => u32,
    oldsubfiletype: OLDSUBFILETYPE => u16,
    imagewidth: IMAGEWIDTH => u32,
    imageheight: IMAGEHEIGHT => u32,
    bitspersample: BITSPERSAMPLE => [u16],
    compression: COMPRESSION => u16,
    photometricinterpretation: PHOTOMETRICINTERPRETATION => u16,
    thresholding: THRESHOLDING => u16,
    cellwidth: CELLWIDTH => u16,
    celllength: CELLLENGTH => u16,
    fillorder: FILLORDER => u16,
    documentname: DOCUMENTNAME => string,
    imagedescription: IMAGEDESCRIPTION => string,
    make: MAKE => string,
    model: MODEL => string,
    tstop: TSTOP => [f64],
    orientation: ORIENTATION => u16,
    samplesperpixel: SAMPLESPERPIXEL => u16,
    rowsperstrip: ROWSPERSTRIP => u32,
    minsamplevalue: MINSAMPLEVALUE => u16,
    maxsamplevalue: MAXSAMPLEVALUE => u16,
    xresolution: XRESOLUTION => f64,
    yresolution: YRESOLUTION => f64,
    planarconfiguration: PLANARCONFIGURATION => u16,
    pagename: PAGENAME => string,
    xposition: XPOSITION => f64,
    yposition: YPOSITION => f64,
    grayresponseunit: GRAYRESPONSEUNIT => u16,
    resolutionunit: RESOLUTIONUNIT => u16,
    pagenumber: PAGENUMBER => [u16],
    transferfunction: TRANSFERFUNCTION => [u16],
    software: SOFTWARE => string,
    modifydate: MODIFYDATE => string,
    artist: ARTIST => string,
    hostcomputer: HOSTCOMPUTER => string,
    predictor: PREDICTOR => u16,
    whitepoint: WHITEPOINT => [f64],
    primarychromaticities: PRIMARYCHROMATICITIES => [f64],
    halftonehints: HALFTONEHINTS => [u16],
    tilewidth: TILEWIDTH => u32,
    tilelength: TILELENGTH => u32,
    inkset: INKSET => u16,
    targetprinter: TARGETPRINTER => string,
    ycbcrcoefficients: YCBCRCOEFFICIENTS => [f64],
    ycbcrpositioning: YCBCRPOSITIONING => u16,
    referenceblackwhite: REFERENCEBLACKWHITE => [f64],
    applicationnotes: APPLICATIONNOTES => u8,
    relatedimagefileformat: RELATEDIMAGEFILEFORMAT => string,
    relatedimagewidth: RELATEDIMAGEWIDTH => u16,
    relatedimageheight: RELATEDIMAGEHEIGHT => u16,
    rating: RATING => u16,
    ratingpercent: RATINGPERCENT => u16,
    vignettingcorrection: VIGNETTINGCORRECTION => i16,
    vignettingcorrparams: VIGNETTINGCORRPARAMS => [i16],
    chromaticaberrationcorrection: CHROMATICABERRATIONCORRECTION => i16,
    chromaticaberrationcorrparams: CHROMATICABERRATIONCORRPARAMS => [i16],
    distortioncorrection: DISTORTIONCORRECTION => i16,
    distortioncorrparams: DISTORTIONCORRPARAMS => [i16],
    sonyrawimagesize: SONYRAWIMAGESIZE => [u32],
    blacklevel: BLACKLEVEL => [u16],
    wb_rggblevels: WB_RGGBLEVELS => [i16],
    sonycroptopleft: SONYCROPTOPLEFT => [u32],
    sonycropsize: SONYCROPSIZE => [u32],
    cfarepeatpatterndim: CFAREPEATPATTERNDIM => [u16],
    cfapattern2: CFAPATTERN2 => [u8],
    copyright: COPYRIGHT => string,
    exposuretime: EXPOSURETIME => f64,
    fnumber: FNUMBER => f64,
    pixelscale: PIXELSCALE => [f64],
    iptc_naa: IPTC_NAA => u32,
    intergraphmatrix: INTERGRAPHMATRIX => [f64],
    modeltiepoint: MODELTIEPOINT => [f64],
    seminfo: SEMINFO => string,
    modeltransform: MODELTRANSFORM => [f64],
    geotiffdirectory: GEOTIFFDIRECTORY => [u16],
    geotiffdoubleparams: GEOTIFFDOUBLEPARAMS => [f64],
    geotiffasciiparams: GEOTIFFASCIIPARAMS => string,
    exposureprogram: EXPOSUREPROGRAM => u16,
    spectralsensitivity: SPECTRALSENSITIVITY => string,
    iso: ISO => [u16],
    timezoneoffset: TIMEZONEOFFSET => [i16],
    selftimermode: SELFTIMERMODE => u16,
    sensitivitytype: SENSITIVITYTYPE => u16,
    standardoutputsensitivity: STANDARDOUTPUTSENSITIVITY => u32,
    recommendedexposureindex: RECOMMENDEDEXPOSUREINDEX => u32,
    isospeed: ISOSPEED => u32,
    isospeedlatitudeyyy: ISOSPEEDLATITUDEYYY => u32,
    isospeedlatitudezzz: ISOSPEEDLATITUDEZZZ => u32,
    datetimeoriginal: DATETIMEORIGINAL => string,
    createdate: CREATEDATE => string,
    offsettime: OFFSETTIME => string,
    offsettimeoriginal: OFFSETTIMEORIGINAL => string,
    offsettimedigitized: OFFSETTIMEDIGITIZED => string,
    compressedbitsperpixel: COMPRESSEDBITSPERPIXEL => f64,
    shutterspeedvalue: SHUTTERSPEEDVALUE => f64,
    aperturevalue: APERTUREVALUE => f64,
    brightnessvalue: BRIGHTNESSVALUE => f64,
    exposurecompensation: EXPOSURECOMPENSATION => f64,
    maxaperturevalue: MAXAPERTUREVALUE => f64,
    subjectdistance: SUBJECTDISTANCE => f64,
    meteringmode: METERINGMODE => u16,
    lightsource: LIGHTSOURCE => u16,
    flash: FLASH => u16,
    focallength: FOCALLENGTH => f64,
    imagenumber: IMAGENUMBER => u32,
    securityclassification: SECURITYCLASSIFICATION => string,
    imagehistory: IMAGEHISTORY => string,
    subjectarea: SUBJECTAREA => [u16],
    subsectime: SUBSECTIME => string,
    subsectimeoriginal: SUBSECTIMEORIGINAL => string,
    subsectimedigitized: SUBSECTIMEDIGITIZED => string,
    ambienttemperature: AMBIENTTEMPERATURE => f64,
    humidity: HUMIDITY => f64,
    pressure: PRESSURE => f64,
    waterdepth: WATERDEPTH => f64,
    acceleration: ACCELERATION => f64,
    cameraelevationangle: CAMERAELEVATIONANGLE => f64,
    xiaomisettings: XIAOMISETTINGS => string,
    xiaomimodel: XIAOMIMODEL => string,
    xptitle: XPTITLE => u8,
    xpcomment: XPCOMMENT => u8,
    xpauthor: XPAUTHOR => u8,
    xpkeywords: XPKEYWORDS => u8,
    xpsubject: XPSUBJECT => u8,
    colorspace: COLORSPACE => u16,
    exifimagewidth: EXIFIMAGEWIDTH => u16,
    iso: ISO => [u16],
    exifimageheight: EXIFIMAGEHEIGHT => u16,
    relatedsoundfile: RELATEDSOUNDFILE => string,
    subjectlocation: SUBJECTLOCATION => [u16],
    customrendered: CUSTOMRENDERED => u16,
    exposuremode: EXPOSUREMODE => u16,
    whitebalance: WHITEBALANCE => u16,
    digitalzoomratio: DIGITALZOOMRATIO => f64,
    focallengthin35mmformat: FOCALLENGTHIN35MMFORMAT => u16,
    scenecapturetype: SCENECAPTURETYPE => u16,
    gaincontrol: GAINCONTROL => u16,
    contrast: CONTRAST => u16,
    saturation: SATURATION => u16,
    sharpness: SHARPNESS => u16,
    subjectdistancerange: SUBJECTDISTANCERANGE => u16,
    imageuniqueid: IMAGEUNIQUEID => string,
    ownername: OWNERNAME => string,
    serialnumber: SERIALNUMBER => string,
    lensinfo: LENSINFO => [f64],
    lensmake: LENSMAKE => string,
    lensmodel: LENSMODEL => string,
    lensserialnumber: LENSSERIALNUMBER => string,
}