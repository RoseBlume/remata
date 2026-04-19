use std::io::{Cursor, Read};
use remata_macros::DisplayPretty;

pub mod tags;
use tags::registry::registry;

/* =========================================================
   INTERNAL IFD STRUCTURES
   ========================================================= */

#[derive(Debug)]
pub struct Ifd {
    pub entries: Vec<IfdEntry>,
}

#[derive(Debug)]
pub struct IfdEntry {
    pub tag: u16,
    pub field_type: u16,
    pub count: u32,
    pub value: Vec<u8>,
}

/* =========================================================
   PUBLIC API
   ========================================================= */

impl Exif {
    pub fn parse<R: Read>(mut reader: R) -> std::io::Result<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        let base = find_exif(&data)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "No EXIF"))?;

        let ifd = parse_ifd(&data, base)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad EXIF"))?;

        Ok(Self::from_ifd(&ifd))
    }

    fn from_ifd(ifd: &Ifd) -> Self {
        let mut exif = Self::default();

        let reg = registry();

        for entry in &ifd.entries {
            for (id, handler) in reg {
                if *id == entry.tag {
                    handler(&mut exif, &entry.value);
                    break;
                }
            }
        }

        exif
    }
}

/* =========================================================
   IFD PARSER
   ========================================================= */

fn parse_ifd(data: &[u8], base: usize) -> Option<Ifd> {
    let mut c = Cursor::new(&data[base..]);

    let mut endian = [0u8; 2];
    c.read_exact(&mut endian).ok()?;

    let le = match &endian {
        b"II" => true,
        b"MM" => false,
        _ => return None,
    };

    read_u16(&mut c, le)?; // TIFF magic

    let offset = read_u32(&mut c, le)? as usize;
    let mut c = Cursor::new(&data[base + offset..]);

    let count = read_u16(&mut c, le)? as usize;

    let mut entries = Vec::with_capacity(count);

    for _ in 0..count {
        let tag = read_u16(&mut c, le)?;
        let field_type = read_u16(&mut c, le)?;
        let count = read_u32(&mut c, le)?;

        let mut raw = [0u8; 4];
        c.read_exact(&mut raw).ok()?;

        let value = resolve(data, base, &raw, field_type, count, le);

        entries.push(IfdEntry {
            tag,
            field_type,
            count,
            value,
        });
    }

    Some(Ifd { entries })
}

/* =========================================================
   VALUE RESOLUTION (RAW ONLY)
   ========================================================= */

fn resolve(
    data: &[u8],
    base: usize,
    raw: &[u8; 4],
    ty: u16,
    count: u32,
    le: bool,
) -> Vec<u8> {
    let unit = match ty {
        1 | 2 | 7 => 1,
        3 => 2,
        4 | 9 => 4,
        5 | 10 => 8,
        _ => 1,
    };

    let size = unit * count as usize;

    if size <= 4 {
        raw[..size].to_vec()
    } else {
        let off = if le {
            u32::from_le_bytes(*raw)
        } else {
            u32::from_be_bytes(*raw)
        } as usize;

        data[base + off..base + off + size].to_vec()
    }
}

/* =========================================================
   EXIF FINDER
   ========================================================= */

fn find_exif(data: &[u8]) -> Option<usize> {
    let mut i = 0;

    while i + 4 < data.len() {
        if data[i] == 0xFF && data[i + 1] == 0xE1 {
            let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
            let start = i + 4;

            if &data[start..start + 6] == b"Exif\0\0" {
                return Some(start + 6);
            }

            i = start + len - 2;
        } else {
            i += 1;
        }
    }

    None
}

/* =========================================================
   HELPERS
   ========================================================= */

fn read_u16(c: &mut Cursor<&[u8]>, le: bool) -> Option<u16> {
    let mut b = [0; 2];
    c.read_exact(&mut b).ok()?;
    Some(if le { u16::from_le_bytes(b) } else { u16::from_be_bytes(b) })
}

fn read_u32(c: &mut Cursor<&[u8]>, le: bool) -> Option<u32> {
    let mut b = [0; 4];
    c.read_exact(&mut b).ok()?;
    Some(if le { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
}

#[derive(Default, DisplayPretty, Debug)]
pub struct Exif {
    pub interopindex: Option<String>,
    pub processingsoftware: Option<String>,
    pub subfiletype: Option<u32>,
    pub oldsubfiletype: Option<u16>,
    pub imagewidth: Option<u32>,
    pub imageheight: Option<u32>,
    pub bitspersample: Option<Vec<u16>>,
    pub compression: Option<u16>,
    pub photometricinterpretation: Option<u16>,
    pub thresholding: Option<u16>,
    pub cellwidth: Option<u16>,
    pub celllength: Option<u16>,
    pub fillorder: Option<u16>,
    pub documentname: Option<String>,
    pub imagedescription: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub orientation: Option<u16>,
    pub samplesperpixel: Option<u16>,
    pub rowsperstrip: Option<u32>,
    pub minsamplevalue: Option<u16>,
    pub maxsamplevalue: Option<u16>,
    pub xresolution: Option<f64>,
    pub yresolution: Option<f64>,
    pub planarconfiguration: Option<u16>,
    pub pagename: Option<String>,
    pub xposition: Option<f64>,
    pub yposition: Option<f64>,
    pub grayresponseunit: Option<u16>,
    pub resolutionunit: Option<u16>,
    pub pagenumber: Option<Vec<u16>>,
    pub transferfunction: Option<Vec<u16>>,
    pub software: Option<String>,
    pub modifydate: Option<String>,
    pub artist: Option<String>,
    pub hostcomputer: Option<String>,
    pub predictor: Option<u16>,
    pub whitepoint: Option<Vec<f64>>,
    pub primarychromaticities: Option<Vec<f64>>,
    pub halftonehints: Option<Vec<u16>>,
    pub tilewidth: Option<u32>,
    pub tilelength: Option<u32>,
    pub inkset: Option<u16>,
    pub targetprinter: Option<String>,
    pub ycbcrcoefficients: Option<Vec<f64>>,
    pub ycbcrpositioning: Option<u16>,
    pub referenceblackwhite: Option<Vec<f64>>,
    pub applicationnotes: Option<u8>,
    pub relatedimagefileformat: Option<String>,
    pub relatedimagewidth: Option<u16>,
    pub relatedimageheight: Option<u16>,
    pub rating: Option<u16>,
    pub ratingpercent: Option<u16>,
    pub vignettingcorrection: Option<i16>,
    pub vignettingcorrparams: Option<Vec<i16>>,
    pub chromaticaberrationcorrection: Option<i16>,
    pub chromaticaberrationcorrparams: Option<Vec<i16>>,
    pub distortioncorrection: Option<i16>,
    pub distortioncorrparams: Option<Vec<i16>>,
    pub sonyrawimagesize: Option<Vec<u32>>,
    pub blacklevel: Option<Vec<u16>>,
    pub wb_rggblevels: Option<Vec<i16>>,
    pub sonycroptopleft: Option<Vec<u32>>,
    pub sonycropsize: Option<Vec<u32>>,
    pub cfarepeatpatterndim: Option<Vec<u16>>,
    pub cfapattern2: Option<Vec<u8>>,
    pub copyright: Option<String>,
    pub exposuretime: Option<f64>,
    pub fnumber: Option<f64>,
    pub pixelscale: Option<Vec<f64>>,
    pub iptc_naa: Option<u32>,
    pub intergraphmatrix: Option<Vec<f64>>,
    pub modeltiepoint: Option<Vec<f64>>,
    pub seminfo: Option<String>,
    pub modeltransform: Option<Vec<f64>>,
    pub geotiffdirectory: Option<Vec<u16>>,
    pub geotiffdoubleparams: Option<Vec<f64>>,
    pub geotiffasciiparams: Option<String>,
    pub exposureprogram: Option<u16>,
    pub spectralsensitivity: Option<String>,
    pub iso: Option<Vec<u16>>,
    pub timezoneoffset: Option<Vec<i16>>,
    pub selftimermode: Option<u16>,
    pub sensitivitytype: Option<u16>,
    pub standardoutputsensitivity: Option<u32>,
    pub recommendedexposureindex: Option<u32>,
    pub isospeed: Option<u32>,
    pub isospeedlatitudeyyy: Option<u32>,
    pub isospeedlatitudezzz: Option<u32>,
    pub datetimeoriginal: Option<String>,
    pub createdate: Option<String>,
    pub offsettime: Option<String>,
    pub offsettimeoriginal: Option<String>,
    pub offsettimedigitized: Option<String>,
    pub compressedbitsperpixel: Option<f64>,
    pub shutterspeedvalue: Option<f64>,
    pub aperturevalue: Option<f64>,
    pub brightnessvalue: Option<f64>,
    pub exposurecompensation: Option<f64>,
    pub maxaperturevalue: Option<f64>,
    pub subjectdistance: Option<f64>,
    pub meteringmode: Option<u16>,
    pub lightsource: Option<u16>,
    pub flash: Option<u16>,
    pub focallength: Option<f64>,
    pub imagenumber: Option<u32>,
    pub securityclassification: Option<String>,
    pub imagehistory: Option<String>,
    pub subjectarea: Option<Vec<u16>>,
    pub subsectime: Option<String>,
    pub subsectimeoriginal: Option<String>,
    pub subsectimedigitized: Option<String>,
    pub ambienttemperature: Option<f64>,
    pub humidity: Option<f64>,
    pub pressure: Option<f64>,
    pub waterdepth: Option<f64>,
    pub acceleration: Option<f64>,
    pub cameraelevationangle: Option<f64>,
    pub xiaomisettings: Option<String>,
    pub xiaomimodel: Option<String>,
    pub xptitle: Option<u8>,
    pub xpcomment: Option<u8>,
    pub xpauthor: Option<u8>,
    pub xpkeywords: Option<u8>,
    pub xpsubject: Option<u8>,
    pub colorspace: Option<u16>,
    pub exifimagewidth: Option<u16>,
    pub exifimageheight: Option<u16>,
    pub relatedsoundfile: Option<String>,
    pub subjectlocation: Option<Vec<u16>>,
    pub customrendered: Option<u16>,
    pub exposuremode: Option<u16>,
    pub whitebalance: Option<u16>,
    pub digitalzoomratio: Option<f64>,
    pub focallengthin35mmformat: Option<u16>,
    pub scenecapturetype: Option<u16>,
    pub gaincontrol: Option<u16>,
    pub contrast: Option<u16>,
    pub saturation: Option<u16>,
    pub sharpness: Option<u16>,
    pub subjectdistancerange: Option<u16>,
    pub imageuniqueid: Option<String>,
    pub ownername: Option<String>,
    pub serialnumber: Option<String>,
    pub lensinfo: Option<Vec<f64>>,
    pub lensmake: Option<String>,
    pub lensmodel: Option<String>,
    pub lensserialnumber: Option<String>,
    pub imagetitle: Option<String>,
    pub photographer: Option<String>,
    pub imageeditor: Option<String>,
    pub camerafirmware: Option<String>,
    pub rawdevelopingsoftware: Option<String>,
    pub imageeditingsoftware: Option<String>,
    pub metadataeditingsoftware: Option<String>,
    pub compositeimage: Option<u16>,
    pub compositeimagecount: Option<Vec<u16>>,
    pub gdalmetadata: Option<String>,
    pub gdalnodata: Option<String>,
    pub gamma: Option<f64>,
    pub dngversion: Option<Vec<u8>>,
    pub dngbackwardversion: Option<Vec<u8>>,
    pub uniquecameramodel: Option<String>,
    pub localizedcameramodel: Option<String>,
    pub linearizationtable: Option<Vec<u16>>,
    pub blacklevelrepeatdim: Option<Vec<u16>>,
    pub blackleveldeltah: Option<Vec<f64>>,
    pub blackleveldeltav: Option<Vec<f64>>,
    pub whitelevel: Option<Vec<u32>>,
    pub defaultscale: Option<Vec<f64>>,
    pub defaultcroporigin: Option<Vec<u32>>,
    pub defaultcropsize: Option<Vec<u32>>,
    pub colormatrix1: Option<Vec<f64>>,
    pub colormatrix2: Option<Vec<f64>>,
    pub cameracalibration1: Option<Vec<f64>>,
    pub cameracalibration2: Option<Vec<f64>>,
    pub reductionmatrix1: Option<Vec<f64>>,
    pub reductionmatrix2: Option<Vec<f64>>,
    pub analogbalance: Option<Vec<f64>>,
    pub asshotneutral: Option<Vec<f64>>,
    pub asshotwhitexy: Option<Vec<f64>>,
    pub baselineexposure: Option<f64>,
    pub baselinenoise: Option<f64>,
    pub baselinesharpness: Option<f64>,
    pub bayergreensplit: Option<u32>,
    pub linearresponselimit: Option<f64>,
    pub cameraserialnumber: Option<String>,
    pub dnglensinfo: Option<Vec<f64>>,
    pub chromablurradius: Option<f64>,
    pub antialiasstrength: Option<f64>,
    pub shadowscale: Option<f64>,
    pub makernotesafety: Option<u16>,
    pub calibrationilluminant1: Option<u16>,
    pub calibrationilluminant2: Option<u16>,
    pub bestqualityscale: Option<f64>,
    pub rawdatauniqueid: Option<Vec<u8>>,
    pub originalrawfilename: Option<String>,
    pub activearea: Option<Vec<u32>>,
    pub maskedareas: Option<Vec<u32>>,
    pub asshotpreprofilematrix: Option<Vec<f64>>,
    pub currentpreprofilematrix: Option<Vec<f64>>,
    pub colorimetricreference: Option<u16>,
    pub cameracalibrationsig: Option<String>,
    pub profilecalibrationsig: Option<String>,
    pub asshotprofilename: Option<String>,
    pub noisereductionapplied: Option<f64>,
    pub profilename: Option<String>,
    pub profilehuesatmapdims: Option<Vec<u32>>,
    pub profilehuesatmapdata1: Option<Vec<f32>>,
    pub profilehuesatmapdata2: Option<Vec<f32>>,
    pub profiletonecurve: Option<Vec<f32>>,
    pub profileembedpolicy: Option<u32>,
    pub profilecopyright: Option<String>,
    pub forwardmatrix1: Option<Vec<f64>>,
    pub forwardmatrix2: Option<Vec<f64>>,
    pub previewapplicationname: Option<String>,
    pub previewapplicationversion: Option<String>,
    pub previewsettingsname: Option<String>,
    pub previewsettingsdigest: Option<u8>,
    pub previewcolorspace: Option<u32>,
    pub previewdatetime: Option<String>,
    pub rawimagedigest: Option<Vec<u8>>,
    pub originalrawfiledigest: Option<Vec<u8>>,
    pub profilelooktabledims: Option<Vec<u32>>,
    pub profilelooktabledata: Option<Vec<f32>>,
    pub noiseprofile: Option<Vec<f64>>,
    pub timecodes: Option<Vec<u8>>,
    pub framerate: Option<f64>,
    pub tstop: Option<Vec<f64>>,
    pub reelname: Option<String>,
    pub originaldefaultfinalsize: Option<Vec<u32>>,
    pub originalbestqualitysize: Option<Vec<u32>>,
    pub originaldefaultcropsize: Option<Vec<f64>>,
    pub cameralabel: Option<String>,
    pub profilehuesatmapencoding: Option<u32>,
    pub profilelooktableencoding: Option<u32>,
    pub baselineexposureoffset: Option<f64>,
    pub defaultblackrender: Option<u32>,
    pub newrawimagedigest: Option<Vec<u8>>,
    pub rawtopreviewgain: Option<f64>,
    pub cacheversion: Option<u32>,
    pub defaultusercrop: Option<Vec<f64>>,
    pub depthformat: Option<u16>,
    pub depthnear: Option<f64>,
    pub depthfar: Option<f64>,
    pub depthunits: Option<u16>,
    pub depthmeasuretype: Option<u16>,
    pub enhanceparams: Option<String>,
    pub calibrationilluminant3: Option<u16>,
    pub cameracalibration3: Option<Vec<f64>>,
    pub colormatrix3: Option<Vec<f64>>,
    pub forwardmatrix3: Option<Vec<f64>>,
    pub profilehuesatmapdata3: Option<Vec<f32>>,
    pub reductionmatrix3: Option<Vec<f64>>,
    pub columninterleavefactor: Option<u32>,
    pub profilegroupname: Option<String>,
    pub jxldistance: Option<f32>,
    pub jxleffort: Option<u32>,
    pub jxldecodespeed: Option<u32>,
    pub seal: Option<String>,
    pub offsetschema: Option<i32>,
    pub lens: Option<String>,
    pub rawfile: Option<String>,
    pub converter: Option<String>,
    pub exposure: Option<String>,
    pub shadows: Option<String>,
    pub brightness: Option<String>,
    pub smoothness: Option<String>,
    pub moirefilter: Option<String>,
}