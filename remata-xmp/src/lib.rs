//! # remata-xmp
//!
//! A lightweight, dependency-free XMP metadata extractor.
//!
//! This crate provides a minimal parser for extracting XMP metadata from binary
//! files (e.g., JPEG, PNG, WebP, TIFF) without relying on external XML libraries.
//!
//! ## Features
//!
//! - No external dependencies (std-only)
//! - Works directly on raw file data
//! - Supports:
//!   - Element values (`<ns:tag>value</ns:tag>`)
//!   - Attribute values (`ns:tag="value"`)
//! - Namespace-based organization
//!
//! ## Limitations
//!
//! - Not a full XML/RDF parser
//! - Does not fully support:
//!   - `rdf:Seq`, `rdf:Bag` (arrays)
//!   - deeply nested RDF structures
//!   - XML namespaces with aliases
//!
//! ## Example
//!
//! ```no_run
//! use remata_xmp::Xmp;
//!
//! let xmp = Xmp::from_path("image.jpg").unwrap();
//!
//! if let Some(date) = xmp.microsoft.get("DateAcquired") {
//!     println!("Date acquired: {}", date);
//! }
//! ```
//!

#![deny(missing_docs)]
use std::collections::HashMap;
use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::fmt;
/// Represents parsed XMP metadata organized by namespace.
///
/// Each field corresponds to a known XMP namespace and contains
/// key-value pairs of tag names and their associated values.
///
/// Values are stored as strings and are not type-normalized.
///
/// # Example
///
/// ```no_run
/// let xmp = Xmp::from_path("image.jpg").unwrap();
///
/// if let Some(author) = xmp.dc.get("creator") {
///     println!("Creator: {}", author);
/// }
/// ```
#[derive(Default, Clone, Debug)]
pub struct Xmp {
    /// AAS (Adobe Asset Services) metadata namespace.
    pub aas: HashMap<String, String>,

    /// ACDSee application metadata namespace.
    pub acdsee: HashMap<String, String>,

    /// ACDSee Regions metadata namespace (region-based annotations).
    pub acdsee_rs: HashMap<String, String>,

    /// Album-related metadata (organizing and grouping information).
    pub album: HashMap<String, String>,

    /// APDI metadata namespace (Adobe Photo Data Interchange).
    pub apdi: HashMap<String, String>,

    /// Apple file information metadata namespace.
    pub apple_fi: HashMap<String, String>,

    /// AST metadata namespace (application-specific tagging).
    pub ast: HashMap<String, String>,

    /// Auxiliary metadata namespace for extended properties.
    pub aux: HashMap<String, String>,

    /// CC (Creative Commons or related metadata namespace).
    pub cc: HashMap<String, String>,

    /// Cell metadata namespace (device or sensor-specific metadata).
    pub cell: HashMap<String, String>,

    /// CRD metadata namespace (camera raw or processing data).
    pub crd: HashMap<String, String>,

    /// Creator Atom metadata namespace (authoring and feed-based metadata).
    pub creator_atom: HashMap<String, String>,

    /// Camera Raw Settings (Adobe Camera Raw / Lightroom settings).
    pub crs: HashMap<String, String>,

    /// Dublin Core metadata namespace (standard descriptive metadata).
    pub dc: HashMap<String, String>,

    /// Device-specific metadata namespace.
    pub device: HashMap<String, String>,

    /// DEX metadata namespace (digital exchange metadata).
    pub dex: HashMap<String, String>,

    /// DICOM metadata namespace (medical imaging metadata).
    pub dicom: HashMap<String, String>,

    /// DigiKam metadata namespace (photo management application metadata).
    pub digi_kam: HashMap<String, String>,

    /// DJI drone metadata namespace.
    pub drone_dji: HashMap<String, String>,

    /// DWC (Darwin Core) metadata namespace (biological/specimen data).
    pub dwc: HashMap<String, String>,

    /// ET metadata namespace (extended tagging information).
    pub et: HashMap<String, String>,

    /// EXIF metadata namespace (camera capture information).
    pub exif: HashMap<String, String>,

    /// Extended EXIF metadata namespace (additional EXIF extensions).
    pub exif_ex: HashMap<String, String>,

    /// Expression Media metadata namespace (Microsoft Expression Media).
    pub expression_media: HashMap<String, String>,

    /// Extensis metadata namespace (portfolio/asset management metadata).
    pub extensis: HashMap<String, String>,

    /// FPV metadata namespace (first-person video/drone metadata).
    pub fpv: HashMap<String, String>,

    /// Google Audio metadata namespace.
    pub g_audio: HashMap<String, String>,

    /// Google Camera metadata namespace.
    pub g_camera: HashMap<String, String>,

    /// Google Container metadata namespace (structural media container data).
    pub g_container: HashMap<String, String>,

    /// Google Creations metadata namespace.
    pub g_creations: HashMap<String, String>,

    /// Google Depth metadata namespace (depth maps / 3D info).
    pub g_depth: HashMap<String, String>,

    /// Getty Images metadata namespace.
    pub getty: HashMap<String, String>,

    /// Google Focus metadata namespace.
    pub g_focus: HashMap<String, String>,

    /// Google Image metadata namespace.
    pub g_image: HashMap<String, String>,

    /// Google Panorama metadata namespace.
    pub g_pano: HashMap<String, String>,

    /// Google Spherical metadata namespace (360° imagery).
    pub g_spherical: HashMap<String, String>,

    /// HDR metadata namespace (high dynamic range imaging data).
    pub hdr: HashMap<String, String>,

    /// HDR gain map metadata namespace (HDR tone mapping data).
    pub hdr_gain_map: HashMap<String, String>,

    /// HDR Gain Map metadata namespace (alternate HDR representation).
    pub hdrgm: HashMap<String, String>,

    /// ICS metadata namespace (calendar/event-related metadata).
    pub ics: HashMap<String, String>,

    /// IPTC Core metadata namespace (standard editorial metadata).
    pub iptc_core: HashMap<String, String>,

    /// IPTC Extension metadata namespace (extended IPTC fields).
    pub iptc_ext: HashMap<String, String>,

    /// Lightroom image metadata namespace.
    pub l_image: HashMap<String, String>,

    /// Adobe Lightroom metadata namespace.
    pub lr: HashMap<String, String>,

    /// MediaPro metadata namespace (asset management software).
    pub mediapro: HashMap<String, String>,

    /// Microsoft Photo metadata namespace.
    pub microsoft: HashMap<String, String>,

    /// MP metadata namespace (multi-purpose metadata group).
    pub mp: HashMap<String, String>,

    /// MP1 metadata namespace (versioned media metadata group).
    pub mp1: HashMap<String, String>,

    /// MWG Collation metadata namespace (Metadata Working Group).
    pub mwg_coll: HashMap<String, String>,

    /// MWG Keywords metadata namespace.
    pub mwg_kw: HashMap<String, String>,

    /// MWG Regions metadata namespace.
    pub mwg_rs: HashMap<String, String>,

    /// Nine metadata namespace (application-specific grouping).
    pub nine: HashMap<String, String>,

    /// Panorama metadata namespace.
    pub panorama: HashMap<String, String>,

    /// PDF metadata namespace.
    pub pdf: HashMap<String, String>,

    /// PDF/X metadata namespace (print-ready PDF standard metadata).
    pub pdfx: HashMap<String, String>,

    /// Photo Mechanic metadata namespace.
    pub photomech: HashMap<String, String>,

    /// Photoshop metadata namespace.
    pub photoshop: HashMap<String, String>,

    /// Pixel Live metadata namespace.
    pub pixel_live: HashMap<String, String>,

    /// PLUS metadata namespace (licensing and usage rights).
    pub plus: HashMap<String, String>,

    /// PMI metadata namespace (Product Manufacturing Information).
    pub pmi: HashMap<String, String>,

    /// Prism metadata namespace (color/profile metadata).
    pub prism: HashMap<String, String>,

    /// PRL metadata namespace (application-specific resource metadata).
    pub prl: HashMap<String, String>,

    /// PRM metadata namespace (parameter/resource metadata).
    pub prm: HashMap<String, String>,

    /// PUR metadata namespace (purification or processing metadata).
    pub pur: HashMap<String, String>,

    /// RDF metadata namespace (Resource Description Framework structure).
    pub rdf: HashMap<String, String>,

    /// SDC metadata namespace (structured data container).
    pub sdc: HashMap<String, String>,

    /// SEAL metadata namespace (security or sealing metadata).
    pub seal: HashMap<String, String>,

    /// SWF metadata namespace (Flash content metadata).
    pub swf: HashMap<String, String>,

    /// TIFF metadata namespace (image file format metadata).
    pub tiff: HashMap<String, String>,

    /// Generic X namespace (custom or unknown extensions).
    pub x: HashMap<String, String>,

    /// XMP core metadata namespace.
    pub xmp: HashMap<String, String>,

    /// XMP BJ metadata namespace (application-specific extension).
    pub xmp_bj: HashMap<String, String>,

    /// XMP DM metadata namespace (dynamic media metadata).
    pub xmp_dm: HashMap<String, String>,

    /// XMP DSA metadata namespace (digital signature/auth metadata).
    pub xmp_dsa: HashMap<String, String>,

    /// XMP MM metadata namespace (media management metadata).
    pub xmp_mm: HashMap<String, String>,

    /// XMP Note metadata namespace (annotation/comment metadata).
    pub xmp_note: HashMap<String, String>,

    /// XMP PLUS metadata namespace (rights/licensing extension).
    pub xmp_plus: HashMap<String, String>,

    /// XMP Rights Management metadata namespace.
    pub xmp_rights: HashMap<String, String>,

    /// XMP TPG metadata namespace (third-party group extensions).
    pub xmp_tpg: HashMap<String, String>,
}


impl fmt::Display for Xmp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // helper to print one namespace block
        fn write_ns(
            f: &mut fmt::Formatter<'_>,
            name: &str,
            map: &HashMap<String, String>,
        ) -> fmt::Result {
            if map.is_empty() {
                return Ok(());
            }

            writeln!(f, "{}:", name)?;
            for (k, v) in map {
                writeln!(f, "  {}: {}", k, v)?;
            }
            writeln!(f)?;
            Ok(())
        }

        write_ns(f, "AAS", &self.aas)?;
        write_ns(f, "ACDSee", &self.acdsee)?;
        write_ns(f, "ACDSee Regions", &self.acdsee_rs)?;
        write_ns(f, "Album", &self.album)?;
        write_ns(f, "APDI", &self.apdi)?;
        write_ns(f, "Apple FI", &self.apple_fi)?;
        write_ns(f, "AST", &self.ast)?;
        write_ns(f, "AUX", &self.aux)?;
        write_ns(f, "Creative Commons", &self.cc)?;
        write_ns(f, "Cell", &self.cell)?;
        write_ns(f, "CRD", &self.crd)?;
        write_ns(f, "Creator Atom", &self.creator_atom)?;
        write_ns(f, "Camera Raw Settings", &self.crs)?;
        write_ns(f, "Dublin Core", &self.dc)?;
        write_ns(f, "Device", &self.device)?;
        write_ns(f, "DEX", &self.dex)?;
        write_ns(f, "DICOM", &self.dicom)?;
        write_ns(f, "DigiKam", &self.digi_kam)?;
        write_ns(f, "DJI Drone", &self.drone_dji)?;
        write_ns(f, "Darwin Core", &self.dwc)?;
        write_ns(f, "ET", &self.et)?;
        write_ns(f, "EXIF", &self.exif)?;
        write_ns(f, "EXIF Extended", &self.exif_ex)?;
        write_ns(f, "Expression Media", &self.expression_media)?;
        write_ns(f, "Extensis", &self.extensis)?;
        write_ns(f, "FPV", &self.fpv)?;
        write_ns(f, "Google Audio", &self.g_audio)?;
        write_ns(f, "Google Camera", &self.g_camera)?;
        write_ns(f, "Google Container", &self.g_container)?;
        write_ns(f, "Google Creations", &self.g_creations)?;
        write_ns(f, "Google Depth", &self.g_depth)?;
        write_ns(f, "Getty", &self.getty)?;
        write_ns(f, "Google Focus", &self.g_focus)?;
        write_ns(f, "Google Image", &self.g_image)?;
        write_ns(f, "Google Panorama", &self.g_pano)?;
        write_ns(f, "Google Spherical", &self.g_spherical)?;
        write_ns(f, "HDR", &self.hdr)?;
        write_ns(f, "HDR Gain Map", &self.hdr_gain_map)?;
        write_ns(f, "HDR Gain Map", &self.hdrgm)?;
        write_ns(f, "ICS", &self.ics)?;
        write_ns(f, "IPTC Core", &self.iptc_core)?;
        write_ns(f, "IPTC Extension", &self.iptc_ext)?;
        write_ns(f, "Lightroom Image", &self.l_image)?;
        write_ns(f, "Lightroom", &self.lr)?;
        write_ns(f, "MediaPro", &self.mediapro)?;
        write_ns(f, "Microsoft Photo", &self.microsoft)?;
        write_ns(f, "MP", &self.mp)?;
        write_ns(f, "MP1", &self.mp1)?;
        write_ns(f, "MWG Collections", &self.mwg_coll)?;
        write_ns(f, "MWG Keywords", &self.mwg_kw)?;
        write_ns(f, "MWG Rating", &self.mwg_rs)?;
        write_ns(f, "Nine", &self.nine)?;
        write_ns(f, "Panorama", &self.panorama)?;
        write_ns(f, "PDF", &self.pdf)?;
        write_ns(f, "PDF Extension", &self.pdfx)?;
        write_ns(f, "PhotoMechanic", &self.photomech)?;
        write_ns(f, "Photoshop", &self.photoshop)?;
        write_ns(f, "Pixel Live", &self.pixel_live)?;
        write_ns(f, "Plus", &self.plus)?;
        write_ns(f, "PMI", &self.pmi)?;
        write_ns(f, "Prism", &self.prism)?;
        write_ns(f, "PRL", &self.prl)?;
        write_ns(f, "PRM", &self.prm)?;
        write_ns(f, "PUR", &self.pur)?;
        write_ns(f, "RDF", &self.rdf)?;
        write_ns(f, "SDC", &self.sdc)?;
        write_ns(f, "Seal", &self.seal)?;
        write_ns(f, "SWF", &self.swf)?;
        write_ns(f, "TIFF", &self.tiff)?;
        write_ns(f, "X", &self.x)?;
        write_ns(f, "XMP", &self.xmp)?;
        write_ns(f, "XMP Basic Job", &self.xmp_bj)?;
        write_ns(f, "XMP Dynamic Media", &self.xmp_dm)?;
        write_ns(f, "XMP Data Structures", &self.xmp_dsa)?;
        write_ns(f, "XMP Media Management", &self.xmp_mm)?;
        write_ns(f, "XMP Note", &self.xmp_note)?;
        write_ns(f, "XMP Plus", &self.xmp_plus)?;
        write_ns(f, "XMP Rights", &self.xmp_rights)?;
        write_ns(f, "XMP Text Page", &self.xmp_tpg)?;

        Ok(())
    }
}

impl Xmp {
    /// Inserts a tag-value pair into the appropriate namespace.
    ///
    /// # Parameters
    ///
    /// - `namespace`: The XMP namespace (e.g., `"dc"`, `"exif"`)
    /// - `tag_name`: The tag name within the namespace
    /// - `value`: The associated value
    ///
    /// # Returns
    ///
    /// Returns the previous value if the key already existed.
    ///
    /// # Notes
    ///
    /// - Unknown namespaces are ignored
    /// - Values are stored as raw strings
    pub fn insert(&mut self, namespace: &str, tag_name: &str, value: &str) -> Option<String> {
        match namespace {
            "aas" => self.aas.insert(tag_name.to_string(), value.to_string()),
            "ACDSee" => self.acdsee.insert(tag_name.to_string(), value.to_string()),
            "ACDSeeRegions" => self.acdsee_rs.insert(tag_name.to_string(), value.to_string()),
            "Album" => self.album.insert(tag_name.to_string(), value.to_string()),
            "apdi" => self.apdi.insert(tag_name.to_string(), value.to_string()),
            "apple_fi" => self.apple_fi.insert(tag_name.to_string(), value.to_string()),
            "ast" => self.ast.insert(tag_name.to_string(), value.to_string()),
            "aux" => self.aux.insert(tag_name.to_string(), value.to_string()),
            "cc" => self.cc.insert(tag_name.to_string(), value.to_string()),
            "cell" => self.cell.insert(tag_name.to_string(), value.to_string()),
            "crd" => self.crd.insert(tag_name.to_string(), value.to_string()),
            "creatorAtom" => self.creator_atom.insert(tag_name.to_string(), value.to_string()),
            "crs" => self.crs.insert(tag_name.to_string(), value.to_string()),
            "dc" => self.dc.insert(tag_name.to_string(), value.to_string()),
            "device" => self.device.insert(tag_name.to_string(), value.to_string()),
            "dex" => self.dex.insert(tag_name.to_string(), value.to_string()),
            "dicom" => self.dicom.insert(tag_name.to_string(), value.to_string()),
            "digi_kam" => self.digi_kam.insert(tag_name.to_string(), value.to_string()),
            "drone_dji" => self.drone_dji.insert(tag_name.to_string(), value.to_string()),
            "dwc" => self.dwc.insert(tag_name.to_string(), value.to_string()),
            "et" => self.et.insert(tag_name.to_string(), value.to_string()),
            "exif" => self.exif.insert(tag_name.to_string(), value.to_string()),
            "exif_ex" => self.exif_ex.insert(tag_name.to_string(), value.to_string()),
            "expression_media" => self.expression_media.insert(tag_name.to_string(), value.to_string()),
            "extensis" => self.extensis.insert(tag_name.to_string(), value.to_string()),
            "fpv" => self.fpv.insert(tag_name.to_string(), value.to_string()),
            "Gaudio" => self.g_audio.insert(tag_name.to_string(), value.to_string()),
            "Gcamera" => self.g_camera.insert(tag_name.to_string(), value.to_string()),
            "Gcontainer" => self.g_container.insert(tag_name.to_string(), value.to_string()),
            "Gcreations" => self.g_creations.insert(tag_name.to_string(), value.to_string()),
            "Gdepth" => self.g_depth.insert(tag_name.to_string(), value.to_string()),
            "getty" => self.getty.insert(tag_name.to_string(), value.to_string()),
            "Gfocus" => self.g_focus.insert(tag_name.to_string(), value.to_string()),
            "Gimage" => self.g_image.insert(tag_name.to_string(), value.to_string()),
            "Gpano" => self.g_pano.insert(tag_name.to_string(), value.to_string()),
            "Gspherical" => self.g_spherical.insert(tag_name.to_string(), value.to_string()),
            "hdr_metadata" => self.hdr.insert(tag_name.to_string(), value.to_string()),
            "HDRGainMap" => self.hdr_gain_map.insert(tag_name.to_string(), value.to_string()),
            "hdrgm" => self.hdrgm.insert(tag_name.to_string(), value.to_string()),
            "ics" => self.ics.insert(tag_name.to_string(), value.to_string()),
            "Iptc4xmpCore" => self.iptc_core.insert(tag_name.to_string(), value.to_string()),
            "Iptc4xmpExt" => self.iptc_ext.insert(tag_name.to_string(), value.to_string()),
            "l_image" => self.l_image.insert(tag_name.to_string(), value.to_string()),
            "lr" => self.lr.insert(tag_name.to_string(), value.to_string()),
            "mediapro" => self.mediapro.insert(tag_name.to_string(), value.to_string()),
            "MicrosoftPhoto" => self.microsoft.insert(tag_name.to_string(), value.to_string()),
            "mp" => self.mp.insert(tag_name.to_string(), value.to_string()),
            "mp1" => self.mp1.insert(tag_name.to_string(), value.to_string()),
            "mwg_coll" => self.mwg_coll.insert(tag_name.to_string(), value.to_string()),
            "mwg_kw" => self.mwg_kw.insert(tag_name.to_string(), value.to_string()),
            "mwg_rs" => self.mwg_rs.insert(tag_name.to_string(), value.to_string()),
            "nine" => self.nine.insert(tag_name.to_string(), value.to_string()),
            "panorama" => self.panorama.insert(tag_name.to_string(), value.to_string()),
            "pdf" => self.pdf.insert(tag_name.to_string(), value.to_string()),
            "pdfx" => self.pdfx.insert(tag_name.to_string(), value.to_string()),
            "photomechanic" => self.photomech.insert(tag_name.to_string(), value.to_string()),
            "photoshop" => self.photoshop.insert(tag_name.to_string(), value.to_string()),
            "PixelLive" => self.pixel_live.insert(tag_name.to_string(), value.to_string()),
            "plus" => self.plus.insert(tag_name.to_string(), value.to_string()),
            "pmi" => self.pmi.insert(tag_name.to_string(), value.to_string()),
            "prism" => self.prism.insert(tag_name.to_string(), value.to_string()),
            "prl" => self.prl.insert(tag_name.to_string(), value.to_string()),
            "prm" => self.prm.insert(tag_name.to_string(), value.to_string()),
            "pur" => self.pur.insert(tag_name.to_string(), value.to_string()),
            "rdf" => self.rdf.insert(tag_name.to_string(), value.to_string()),
            "sdc" => self.sdc.insert(tag_name.to_string(), value.to_string()),
            "seal" => self.seal.insert(tag_name.to_string(), value.to_string()),
            "swf" => self.swf.insert(tag_name.to_string(), value.to_string()),
            "tiff" => self.tiff.insert(tag_name.to_string(), value.to_string()),
            "x" => self.x.insert(tag_name.to_string(), value.to_string()),
            "xmp" => self.xmp.insert(tag_name.to_string(), value.to_string()),
            "xmpBj" => self.xmp_bj.insert(tag_name.to_string(), value.to_string()),
            "xmpDm" => self.xmp_dm.insert(tag_name.to_string(), value.to_string()),
            "xmpDsa" => self.xmp_dsa.insert(tag_name.to_string(), value.to_string()),
            "xmpMm" => self.xmp_mm.insert(tag_name.to_string(), value.to_string()),
            "xmpNote" => self.xmp_note.insert(tag_name.to_string(), value.to_string()),
            "xmpPlus" => self.xmp_plus.insert(tag_name.to_string(), value.to_string()),
            "xmpRights" => self.xmp_rights.insert(tag_name.to_string(), value.to_string()),
            "xmpTpg" => self.xmp_tpg.insert(tag_name.to_string(), value.to_string()),
            _ => None
        }
    }
    /// Parses XMP metadata from a file path.
    ///
    /// This is a convenience wrapper around [`from_reader`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// let xmp = Xmp::from_path("image.jpg").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        Self::from_reader(file)
    }
        /// Parses XMP metadata from a reader.
    ///
    /// This method scans the input for an XMP packet and extracts
    /// metadata into namespace-specific maps.
    ///
    /// # Behavior
    ///
    /// - Searches for `<x:xmpmeta>` block
    /// - Extracts:
    ///   - Element values (`<ns:tag>value</ns:tag>`)
    ///   - Attribute values (`ns:tag="value"`)
    /// - Ignores structural XML tags (e.g., `rdf:*`)
    ///
    /// # Errors
    ///
    /// Returns an error if the reader cannot be read.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use remata_xmp::Xmp;
    ///
    /// let file = File::open("image.jpg").unwrap();
    /// let xmp = Xmp::from_reader(file).unwrap();
    /// ```
    pub fn from_reader<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut xmp = Xmp::default();

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let content = match extract_xmp_packet(&buf) {
            Some(s) => s,
            None => return Ok(xmp),
        };

        let mut i = 0;
        let bytes = content.as_bytes();

        while i < bytes.len() {
            if bytes[i] == b'<' {
                // Skip closing tags
                if i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                    i += 2;
                    continue;
                }

                if let Some(end) = content[i..].find('>') {
                    let tag_start = i + 1;
                    let tag_end = i + end;

                    let tag_full = &content[tag_start..tag_end];
                    // 🔥 NEW: extract attributes
                    for (attr_key, attr_val) in parse_attributes(tag_full) {
                        if let Some((ns, tag)) = split_ns_tag_str(attr_key) {
                            if ns != "x" {
                                xmp.insert(ns, tag, attr_val);
                            }
                        }
                    }
                    let tag_name = tag_full.split_whitespace().next().unwrap_or("");

                    let close_tag = format!("</{}>", tag_name);
                    let value_start = tag_end + 1;

                    if let Some(close_pos) = content[value_start..].find(&close_tag) {
                        let value_end = value_start + close_pos;
                        let value = content[value_start..value_end].trim();

                        // ✅ Only store if it's a leaf node
                        if !value.is_empty() && !value.contains('<') {
                            if let Some((ns, tag)) = split_ns_tag_str(tag_name) {
                                if ns != "rdf" && ns != "x" {
                                    xmp.insert(ns, tag, value);
                                }
                            }
                        }

                        // 🔥 KEY FIX: DO NOT skip children
                        i += 1;
                        continue;
                    }

                    i = tag_end + 1;
                    continue;
                }
            }

            i += 1;
        }

        Ok(xmp)
    }

}


fn split_ns_tag_str(tag: &str) -> Option<(&str, &str)> {
    let mut parts = tag.splitn(2, ':');
    let ns = parts.next()?;
    let tag = parts.next()?;
    Some((ns, tag))
}

fn extract_xmp_packet(data: &[u8]) -> Option<String> {
    let start_tag = b"<x:xmpmeta";
    let end_tag = b"</x:xmpmeta>";

    let start = data.windows(start_tag.len())
        .position(|w| w == start_tag)?;

    let end = data.windows(end_tag.len())
        .position(|w| w == end_tag)?;

    let end = end + end_tag.len();

    let slice = &data[start..end];

    // Convert ONLY the XML part to UTF-8
    std::str::from_utf8(slice).ok().map(|s| s.to_string())
}

fn parse_attributes(tag_full: &str) -> Vec<(&str, &str)> {
    let mut attrs = Vec::new();

    let mut parts = tag_full.split_whitespace();

    // skip tag name
    parts.next();

    for part in parts {
        if let Some(eq_pos) = part.find('=') {
            let key = &part[..eq_pos];
            let mut value = &part[eq_pos + 1..];

            // remove quotes
            if value.starts_with('"') {
                value = &value[1..];
            }
            if value.ends_with('"') {
                value = &value[..value.len() - 1];
            }

            attrs.push((key, value));
        }
    }

    attrs
}