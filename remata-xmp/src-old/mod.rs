use crate::ParserError;
use std::fmt;
use std::collections::HashMap;
// mod disp;
mod exifex;
pub mod helpers;
use helpers::{
    as_string,
    as_i64,
    as_i32,
    as_rational,
    split_ns
};
pub use exifex::{
    XmpExifEx,
    XmpCompImageExp,
    XmpSensitivityType,
    XmpCompositeImage
};
#[derive(Debug, Clone)]
pub enum XmpValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<XmpValue>),
    Struct(HashMap<String, XmpValue>),
}

#[derive(Debug, Default)]
pub struct Xmp {
    pub aas: Option<HashMap<String, XmpValue>>,
    pub acdsee: Option<HashMap<String, XmpValue>>,
    pub acdsee_rs: Option<HashMap<String, XmpValue>>,
    pub album: Option<HashMap<String, XmpValue>>,
    pub apdi: Option<HashMap<String, XmpValue>>,
    pub apple_fi: Option<HashMap<String, XmpValue>>,
    pub ast: Option<HashMap<String, XmpValue>>,
    pub aux: Option<HashMap<String, XmpValue>>,
    pub cc: Option<HashMap<String, XmpValue>>,
    pub cell: Option<HashMap<String, XmpValue>>,
    pub crd: Option<HashMap<String, XmpValue>>,
    pub creator_atom: Option<HashMap<String, XmpValue>>,
    pub crs: Option<HashMap<String, XmpValue>>,
    pub dc: Option<HashMap<String, XmpValue>>,
    pub device: Option<HashMap<String, XmpValue>>,
    pub dex: Option<HashMap<String, XmpValue>>,
    pub dicom: Option<HashMap<String, XmpValue>>,
    pub digi_kam: Option<HashMap<String, XmpValue>>,
    pub drone_dji: Option<HashMap<String, XmpValue>>,
    pub dwc: Option<HashMap<String, XmpValue>>,
    pub et: Option<HashMap<String, XmpValue>>,
    pub exif: Option<HashMap<String, XmpValue>>,
    pub exif_ex: Option<XmpExifEx>,
    pub expressionmedia: Option<HashMap<String, XmpValue>>,
    pub extensis: Option<HashMap<String, XmpValue>>,
    pub fpv: Option<HashMap<String, XmpValue>>,
    pub g_audio: Option<HashMap<String, XmpValue>>,
    pub g_camera: Option<HashMap<String, XmpValue>>,
    pub g_container: Option<HashMap<String, XmpValue>>,
    pub g_creations: Option<HashMap<String, XmpValue>>,
    pub g_depth: Option<HashMap<String, XmpValue>>,
    pub getty: Option<HashMap<String, XmpValue>>,
    pub g_focus: Option<HashMap<String, XmpValue>>,
    pub g_image: Option<HashMap<String, XmpValue>>,
    pub g_pano: Option<HashMap<String, XmpValue>>,
    pub g_spherical: Option<HashMap<String, XmpValue>>,
    pub hdr: Option<HashMap<String, XmpValue>>,
    pub hdr_gain_map: Option<HashMap<String, XmpValue>>,
    pub hdrgm: Option<HashMap<String, XmpValue>>,
    pub ics: Option<HashMap<String, XmpValue>>,
    pub iptc_core: Option<HashMap<String, XmpValue>>,
    pub iptc_ext: Option<HashMap<String, XmpValue>>,
    pub l_image: Option<HashMap<String, XmpValue>>,
    pub lr: Option<HashMap<String, XmpValue>>,
    pub mediapro: Option<HashMap<String, XmpValue>>,
    pub microsoft: Option<HashMap<String, XmpValue>>,
    pub mp: Option<HashMap<String, XmpValue>>,
    pub mp1: Option<HashMap<String, XmpValue>>,
    pub mwg_coll: Option<HashMap<String, XmpValue>>,
    pub mwg_kw: Option<HashMap<String, XmpValue>>,
    pub mwg_rs: Option<HashMap<String, XmpValue>>,
    pub nine: Option<HashMap<String, XmpValue>>,
    pub panorama: Option<HashMap<String, XmpValue>>,
    pub pdf: Option<HashMap<String, XmpValue>>,
    pub pdfx: Option<HashMap<String, XmpValue>>,
    pub photomech: Option<HashMap<String, XmpValue>>,
    pub photoshop: Option<HashMap<String, XmpValue>>,
    pub pixel_live: Option<HashMap<String, XmpValue>>,
    pub plus: Option<HashMap<String, XmpValue>>,
    pub pmi: Option<HashMap<String, XmpValue>>,
    pub prism: Option<HashMap<String, XmpValue>>,
    pub prl: Option<HashMap<String, XmpValue>>,
    pub prm: Option<HashMap<String, XmpValue>>,
    pub pur: Option<HashMap<String, XmpValue>>,
    pub rdf: Option<HashMap<String, XmpValue>>,
    pub sdc: Option<HashMap<String, XmpValue>>,
    pub seal: Option<HashMap<String, XmpValue>>,
    pub swf: Option<HashMap<String, XmpValue>>,
    pub tiff: Option<HashMap<String, XmpValue>>,
    pub x: Option<HashMap<String, XmpValue>>,
    pub xmp: Option<HashMap<String, XmpValue>>,
    pub xmp_bj: Option<HashMap<String, XmpValue>>,
    pub xmp_dm: Option<HashMap<String, XmpValue>>,
    pub xmp_dsa: Option<HashMap<String, XmpValue>>,
    pub xmp_mm: Option<HashMap<String, XmpValue>>,
    pub xmp_note: Option<HashMap<String, XmpValue>>,
    pub xmp_plus: Option<HashMap<String, XmpValue>>,
    pub xmp_rights: Option<HashMap<String, XmpValue>>,
    pub xmp_tpg: Option<HashMap<String, XmpValue>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    pub num: i32,
    pub den: i32,
}

impl Xmp {
    pub fn insert(&mut self, namespace: &str, key: String, value: XmpValue) {
        let target = match namespace {
            "aas" => &mut self.aas,
            "acdsee" => &mut self.acdsee,
            "acdsee-rs" => &mut self.acdsee_rs,
            "album" => &mut self.album,
            "apdi" => &mut self.apdi,
            "apple-fi" => &mut self.apple_fi,
            "ast" => &mut self.ast,
            "aux" => &mut self.aux,
            "cc" => &mut self.cc,
            "cell" => &mut self.cell,
            "crd" => &mut self.crd,
            "creatorAtom" => &mut self.creator_atom,
            "crs" => &mut self.crs,
            "dc" => &mut self.dc,
            "Device" => &mut self.device,
            "dex" => &mut self.dex,
            "DICOM" => &mut self.dicom,
            "digiKam" => &mut self.digi_kam,
            "drone-dji" => &mut self.drone_dji,
            "dwc" => &mut self.dwc,
            "et" => &mut self.et,
            "exif" => &mut self.exif,
            "exifEX" => {
                let exif = self.exif_ex.get_or_insert_with(|| XmpExifEx {
                    acceleration: None,
                    serial_number: None,
                    camera_elevation_angle: None,
                    camera_firmware: None,
                    owner_name: None,
                    composite_image: None,
                    composite_image_count: None,
                    composite_image_exposure_times: None,
                    comp_image_max_exposure_all: None,
                    comp_image_max_exposure_used: None,
                    comp_image_min_exposure_all: None,
                    comp_image_min_exposure_used: None,
                    comp_image_images_per_sequence: None,
                    comp_image_num_sequences: None,
                    comp_image_sum_exposure_all: None,
                    comp_image_sum_exposure_used: None,
                    comp_image_total_exposure_period: None,
                    comp_image_values: None,
                    gamma: None,
                    humidity: None,
                    pressure: None,
                    ambient_temperature: None,
                    water_depth: None,
                    image_editing_software: None,
                    image_editor: None,
                    metadata_editing_software: None,
                    raw_developing_software: None,
                    photographer: None,
                    image_title: None,
                    image_unique_id: None,
                    interop_index: None,
                    iso_speed: None,
                    iso_speed_latitude_yyy: None,
                    iso_speed_latitude_zzz: None,
                    photographic_sensitivity: None,
                    recommended_exposure_index: None,
                    sensitivity_type: None,
                    standard_output_sensitivity: None,
                    lens_make: None,
                    lens_model: None,
                    lens_serial_number: None,
                    lens_info: None,
                });

                exif.insert(&key, value);
                return;
            },
            "expressionmedia" => &mut self.expressionmedia,
            "extensis" => &mut self.extensis,
            "fpv" => &mut self.fpv,
            "GAudio" => &mut self.g_audio,
            "GCamera" => &mut self.g_camera,
            "GContainer" => &mut self.g_container,
            "GCreations" => &mut self.g_creations,
            "GDepth" => &mut self.g_depth,
            "getty" => &mut self.getty,
            "GFocus" => &mut self.g_focus,
            "GImage" => &mut self.g_image,
            "GPano" => &mut self.g_pano,
            "GSpherical" => &mut self.g_spherical,
            "hdr" => &mut self.hdr,
            "HDRGainMap" => &mut self.hdr_gain_map,
            "hdrgm" => &mut self.hdrgm,
            "ics" => &mut self.ics,
            "iptcCore" => &mut self.iptc_core,
            "iptcExt" => &mut self.iptc_ext,
            "LImage" => &mut self.l_image,
            "lr" => &mut self.lr,
            "mediapro" => &mut self.mediapro,
            "microsoft" => &mut self.microsoft,
            "MP" => &mut self.mp,
            "MP1" => &mut self.mp1,
            "mwg-coll" => &mut self.mwg_coll,
            "mwg-kw" => &mut self.mwg_kw,
            "mwg-rs" => &mut self.mwg_rs,
            "nine" => &mut self.nine,
            "panorama" => &mut self.panorama,
            "pdf" => &mut self.pdf,
            "pdfx" => &mut self.pdfx,
            "photomech" => &mut self.photomech,
            "photoshop" => &mut self.photoshop,
            "PixelLive" => &mut self.pixel_live,
            "plus" => &mut self.plus,
            "pmi" => &mut self.pmi,
            "prism" => &mut self.prism,
            "prl" => &mut self.prl,
            "prm" => &mut self.prm,
            "pur" => &mut self.pur,
            "rdf" => &mut self.rdf,
            "sdc" => &mut self.sdc,
            "seal" => &mut self.seal,
            "swf" => &mut self.swf,
            "tiff" => &mut self.tiff,
            "x" => &mut self.x,
            "xmp" => &mut self.xmp,
            "xmpBJ" => &mut self.xmp_bj,
            "xmpDM" => &mut self.xmp_dm,
            "xmpDSA" => &mut self.xmp_dsa,
            "xmpMM" => &mut self.xmp_mm,
            "xmpNote" => &mut self.xmp_note,
            "xmpPLUS" => &mut self.xmp_plus,
            "xmpRights" => &mut self.xmp_rights,
            "xmpTPg" => &mut self.xmp_tpg,
            _ => return,
        };

        let map = target.get_or_insert_with(HashMap::new);
        map.insert(key, value);
    }

    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let s = std::str::from_utf8(data)
            .map_err(|e| ParserError::new(&e.to_string()))?;

        let mut xmp = Xmp::default();

        let mut i = 0;
        let bytes = s.as_bytes();

        let mut stack: Vec<(String, String)> = Vec::new(); // (ns, tag)

        while i < bytes.len() {
            if bytes[i] == b'<' {
                // Find end of tag
                let start = i + 1;
                let mut end = start;

                while end < bytes.len() && bytes[end] != b'>' {
                    end += 1;
                }

                if end >= bytes.len() {
                    break;
                }

                let raw_tag = &s[start..end].trim();

                // Move cursor past '>'
                i = end + 1;

                // Skip declarations/comments
                if raw_tag.starts_with('?') || raw_tag.starts_with('!') {
                    continue;
                }

                // Closing tag
                if raw_tag.starts_with('/') {
                    stack.pop();
                    continue;
                }

                // Self-closing
                let self_closing = raw_tag.ends_with('/');

                // Strip attributes
                let tag_name = raw_tag
                    .trim_end_matches('/')
                    .split_whitespace()
                    .next()
                    .unwrap_or("");

                if let Some((ns, tag)) = split_ns(tag_name) {
                    if !self_closing {
                        stack.push((ns.to_string(), tag.to_string()));

                        // Extract text until next '<'
                        let text_start = i;
                        let mut text_end = i;

                        while text_end < bytes.len() && bytes[text_end] != b'<' {
                            text_end += 1;
                        }

                        if text_end > text_start {
                            let value = s[text_start..text_end].trim();

                            if !value.is_empty() {
                                xmp.insert(
                                    ns,
                                    tag.to_string(),
                                    XmpValue::String(value.to_string()),
                                );
                            }
                        }
                    }
                }
            } else {
                i += 1;
            }
        }

        Ok(xmp)
    }
}






impl Rational {
    fn from_str(s: &str) -> Option<Self> {
        let mut parts = s.split('/');
        let num = parts.next()?.parse().ok()?;
        let den = parts.next()?.parse().ok()?;
        Some(Self { num, den })
    }
}
