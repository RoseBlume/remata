use std::fmt;
use std::collections::HashMap;
use super::{
    Xmp,
    XmpValue,
    Xmp
};

macro_rules! write_opt {
    ($f:expr, $name:expr, $val:expr) => {
        if let Some(v) = &$val {
            writeln!($f, "{}: {}", $name, v)?;
        }
    };
}


impl fmt::Display for XmpExifEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ExifEX:")?;

        // --- Basic ---
        write_opt!(f, "  Acceleration", self.acceleration);
        write_opt!(f, "  SerialNumber", self.serial_number);
        write_opt!(f, "  CameraElevationAngle", self.camera_elevation_angle);
        write_opt!(f, "  CameraFirmware", self.camera_firmware);
        write_opt!(f, "  OwnerName", self.owner_name);

        // --- Composite ---
        write_opt!(f, "  CompositeImage", self.composite_image);
        write_opt!(f, "  CompositeImageCount", self.composite_image_count);

        if let Some(v) = &self.composite_image_exposure_times {
            writeln!(f, "  {}", v)?;
        }

        // Flattened
        write_opt!(f, "  MaxExposureAll", self.comp_image_max_exposure_all);
        write_opt!(f, "  MaxExposureUsed", self.comp_image_max_exposure_used);
        write_opt!(f, "  MinExposureAll", self.comp_image_min_exposure_all);
        write_opt!(f, "  MinExposureUsed", self.comp_image_min_exposure_used);
        write_opt!(f, "  ImagesPerSequence", self.comp_image_images_per_sequence);
        write_opt!(f, "  NumSequences", self.comp_image_num_sequences);
        write_opt!(f, "  SumExposureAll", self.comp_image_sum_exposure_all);
        write_opt!(f, "  SumExposureUsed", self.comp_image_sum_exposure_used);
        write_opt!(f, "  TotalExposurePeriod", self.comp_image_total_exposure_period);

        if let Some(values) = &self.comp_image_values {
            write!(f, "  Values: [")?;
            for (i, v) in values.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            writeln!(f, "]")?;
        }

        // --- Environment ---
        write_opt!(f, "  Gamma", self.gamma);
        write_opt!(f, "  Humidity", self.humidity);
        write_opt!(f, "  Pressure", self.pressure);
        write_opt!(f, "  Temperature", self.ambient_temperature);
        write_opt!(f, "  WaterDepth", self.water_depth);

        // --- Software ---
        write_opt!(f, "  ImageEditingSoftware", self.image_editing_software);
        write_opt!(f, "  ImageEditor", self.image_editor);
        write_opt!(f, "  MetadataEditingSoftware", self.metadata_editing_software);
        write_opt!(f, "  RawDevelopingSoftware", self.raw_developing_software);
        write_opt!(f, "  Photographer", self.photographer);

        // --- Identification ---
        write_opt!(f, "  ImageTitle", self.image_title);
        write_opt!(f, "  ImageUniqueID", self.image_unique_id);
        write_opt!(f, "  InteropIndex", self.interop_index);

        // --- ISO ---
        write_opt!(f, "  ISOSpeed", self.iso_speed);
        write_opt!(f, "  ISOSpeedLatitudeYYY", self.iso_speed_latitude_yyy);
        write_opt!(f, "  ISOSpeedLatitudeZZZ", self.iso_speed_latitude_zzz);
        write_opt!(f, "  PhotographicSensitivity", self.photographic_sensitivity);
        write_opt!(f, "  RecommendedExposureIndex", self.recommended_exposure_index);
        write_opt!(f, "  SensitivityType", self.sensitivity_type);
        write_opt!(f, "  StandardOutputSensitivity", self.standard_output_sensitivity);

        // --- Lens ---
        write_opt!(f, "  LensMake", self.lens_make);
        write_opt!(f, "  LensModel", self.lens_model);
        write_opt!(f, "  LensSerialNumber", self.lens_serial_number);

        if let Some(info) = &self.lens_info {
            write!(f, "  LensInfo: [")?;
            for (i, r) in info.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", r)?;
            }
            writeln!(f, "]")?;
        }

        Ok(())
    }
}
impl fmt::Display for XmpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmpValue::String(s) => write!(f, "{}", s),
            XmpValue::Integer(i) => write!(f, "{}", i),
            XmpValue::Float(v) => write!(f, "{}", v),
            XmpValue::Boolean(b) => write!(f, "{}", b),
            XmpValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            XmpValue::Struct(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}



impl fmt::Display for XmpCompositeImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotComposite => "NotComposite",
            Self::GeneralComposite => "GeneralComposite",
            Self::CompositeCapturedWhileShooting => "CompositeCapturedWhileShooting",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for XmpSensitivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::StandardOutputSensitivity => "StandardOutputSensitivity",
            Self::RecommendedExposureIndex => "RecommendedExposureIndex",
            Self::ISOSpeed => "ISOSpeed",
            Self::StandardOutputSensitivityAndRecommendedExposureIndex => {
                "StandardOutputSensitivityAndRecommendedExposureIndex"
            }
            Self::StandardOutputSensitivityAndISOSpeed => {
                "StandardOutputSensitivityAndISOSpeed"
            }
            Self::RecommendedExposureIndexAndISOSpeed => {
                "RecommendedExposureIndexAndISOSpeed"
            }
            Self::StandardOutputSensitivityRecommendedExposureIndexAndISOSpeed => {
                "All"
            }
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl fmt::Display for XmpCompImageExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CompositeImageExposure:")?;

        write_opt!(f, "  MaxExposureTimesOfAll", self.max_exposure_times_of_all);
        write_opt!(f, "  MaxExposureTimesOfUsed", self.max_exposure_times_of_used);
        write_opt!(f, "  MinExposureTimesOfAll", self.min_exposure_times_of_all);
        write_opt!(f, "  MinExposureTimesOfUsed", self.min_exposure_times_of_used);
        write_opt!(f, "  NumberOfImagesInSequences", self.number_of_images_in_sequences);
        write_opt!(f, "  NumberOfSequences", self.number_of_sequences);
        write_opt!(f, "  SumOfExposureTimesOfAll", self.sum_of_exposure_times_of_all);
        write_opt!(f, "  SumOfExposureTimesOfUsed", self.sum_of_exposure_times_of_used);
        write_opt!(f, "  TotalExposurePeriod", self.total_exposure_period);

        Ok(())
    }
}

impl fmt::Display for Xmp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_ns(
            f: &mut fmt::Formatter<'_>,
            name: &str,
            map: &Option<HashMap<String, XmpValue>>,
        ) -> fmt::Result {
            if let Some(map) = map {
                if map.is_empty() {
                    return Ok(());
                }

                writeln!(f, "{}:", name)?;

                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();

                for key in keys {
                    let value = &map[key];
                    writeln!(f, "  {}: {}", key, value)?;
                }
            }
            Ok(())
        }

        write_ns(f, "aas", &self.aas)?;
        write_ns(f, "acdsee", &self.acdsee)?;
        write_ns(f, "acdsee-rs", &self.acdsee_rs)?;
        write_ns(f, "album", &self.album)?;
        write_ns(f, "apdi", &self.apdi)?;
        write_ns(f, "apple-fi", &self.apple_fi)?;
        write_ns(f, "ast", &self.ast)?;
        write_ns(f, "aux", &self.aux)?;
        write_ns(f, "cc", &self.cc)?;
        write_ns(f, "cell", &self.cell)?;
        write_ns(f, "crd", &self.crd)?;
        write_ns(f, "creatorAtom", &self.creator_atom)?;
        write_ns(f, "crs", &self.crs)?;
        write_ns(f, "dc", &self.dc)?;
        write_ns(f, "Device", &self.device)?;
        write_ns(f, "dex", &self.dex)?;
        write_ns(f, "DICOM", &self.dicom)?;
        write_ns(f, "digiKam", &self.digi_kam)?;
        write_ns(f, "drone-dji", &self.drone_dji)?;
        write_ns(f, "dwc", &self.dwc)?;
        write_ns(f, "et", &self.et)?;
        write_ns(f, "exif", &self.exif)?;
        write_ns(f, "exifEX", &self.exif_ex)?;
        write_ns(f, "expressionmedia", &self.expressionmedia)?;
        write_ns(f, "extensis", &self.extensis)?;
        write_ns(f, "fpv", &self.fpv)?;
        write_ns(f, "GAudio", &self.g_audio)?;
        write_ns(f, "GCamera", &self.g_camera)?;
        write_ns(f, "GContainer", &self.g_container)?;
        write_ns(f, "GCreations", &self.g_creations)?;
        write_ns(f, "GDepth", &self.g_depth)?;
        write_ns(f, "getty", &self.getty)?;
        write_ns(f, "GFocus", &self.g_focus)?;
        write_ns(f, "GImage", &self.g_image)?;
        write_ns(f, "GPano", &self.g_pano)?;
        write_ns(f, "GSpherical", &self.g_spherical)?;
        write_ns(f, "hdr", &self.hdr)?;
        write_ns(f, "HDRGainMap", &self.hdr_gain_map)?;
        write_ns(f, "hdrgm", &self.hdrgm)?;
        write_ns(f, "ics", &self.ics)?;
        write_ns(f, "iptcCore", &self.iptc_core)?;
        write_ns(f, "iptcExt", &self.iptc_ext)?;
        write_ns(f, "LImage", &self.l_image)?;
        write_ns(f, "lr", &self.lr)?;
        write_ns(f, "mediapro", &self.mediapro)?;
        write_ns(f, "microsoft", &self.microsoft)?;
        write_ns(f, "MP", &self.mp)?;
        write_ns(f, "MP1", &self.mp1)?;
        write_ns(f, "mwg-coll", &self.mwg_coll)?;
        write_ns(f, "mwg-kw", &self.mwg_kw)?;
        write_ns(f, "mwg-rs", &self.mwg_rs)?;
        write_ns(f, "nine", &self.nine)?;
        write_ns(f, "panorama", &self.panorama)?;
        write_ns(f, "pdf", &self.pdf)?;
        write_ns(f, "pdfx", &self.pdfx)?;
        write_ns(f, "photomech", &self.photomech)?;
        write_ns(f, "photoshop", &self.photoshop)?;
        write_ns(f, "PixelLive", &self.pixel_live)?;
        write_ns(f, "plus", &self.plus)?;
        write_ns(f, "pmi", &self.pmi)?;
        write_ns(f, "prism", &self.prism)?;
        write_ns(f, "prl", &self.prl)?;
        write_ns(f, "prm", &self.prm)?;
        write_ns(f, "pur", &self.pur)?;
        write_ns(f, "rdf", &self.rdf)?;
        write_ns(f, "sdc", &self.sdc)?;
        write_ns(f, "seal", &self.seal)?;
        write_ns(f, "swf", &self.swf)?;
        write_ns(f, "tiff", &self.tiff)?;
        write_ns(f, "x", &self.x)?;
        write_ns(f, "xmp", &self.xmp)?;
        write_ns(f, "xmpBJ", &self.xmp_bj)?;
        write_ns(f, "xmpDM", &self.xmp_dm)?;
        write_ns(f, "xmpDSA", &self.xmp_dsa)?;
        write_ns(f, "xmpMM", &self.xmp_mm)?;
        write_ns(f, "xmpNote", &self.xmp_note)?;
        write_ns(f, "xmpPLUS", &self.xmp_plus)?;
        write_ns(f, "xmpRights", &self.xmp_rights)?;
        write_ns(f, "xmpTPg", &self.xmp_tpg)?;

        Ok(())
    }
}



impl fmt::Display for XmpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmpValue::String(s) => {
                write!(f, "\"{}\"", s)
            }

            XmpValue::Integer(i) => {
                write!(f, "{}", i)
            }

            XmpValue::Float(fl) => {
                write!(f, "{}", fl)
            }

            XmpValue::Boolean(b) => {
                write!(f, "{}", b)
            }

            XmpValue::Array(arr) => {
                write!(f, "[")?;

                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }

                write!(f, "]")
            }

            XmpValue::Struct(map) => {
                write!(f, "{{")?;

                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();

                let mut first = true;
                for key in keys {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;

                    let value = &map[key];
                    write!(f, "{}: {}", key, value)?;
                }

                write!(f, "}}")
            }
        }
    }
}