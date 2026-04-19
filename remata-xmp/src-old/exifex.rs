// use super::Rational;
use crate::{
    XmpValue,
    Rational
};
use crate::helpers::{
    as_string,
    as_i64,
    as_i32,
    as_rational
};
#[derive(Debug, Default)]
pub struct XmpExifEx {
    // Basic
    pub acceleration: Option<Rational>,
    pub serial_number: Option<String>, // BodySerialNumber
    pub camera_elevation_angle: Option<Rational>,
    pub camera_firmware: Option<String>,
    pub owner_name: Option<String>, // CameraOwnerName

    // Composite image
    pub composite_image: Option<XmpCompositeImage>,
    pub composite_image_count: Option<i64>,
    pub composite_image_exposure_times: Option<XmpCompImageExp>,

    // Flattened (duplicate convenience tags from struct)
    pub comp_image_max_exposure_all: Option<Rational>,
    pub comp_image_max_exposure_used: Option<Rational>,
    pub comp_image_min_exposure_all: Option<Rational>,
    pub comp_image_min_exposure_used: Option<Rational>,
    pub comp_image_images_per_sequence: Option<i64>,
    pub comp_image_num_sequences: Option<i64>,
    pub comp_image_sum_exposure_all: Option<Rational>,
    pub comp_image_sum_exposure_used: Option<Rational>,
    pub comp_image_total_exposure_period: Option<Rational>,
    pub comp_image_values: Option<Vec<Rational>>,

    // Image/environment
    pub gamma: Option<Rational>,
    pub humidity: Option<Rational>,
    pub pressure: Option<Rational>,
    pub ambient_temperature: Option<Rational>, // Temperature
    pub water_depth: Option<Rational>,

    // Software / authorship
    pub image_editing_software: Option<String>,
    pub image_editor: Option<String>,
    pub metadata_editing_software: Option<String>,
    pub raw_developing_software: Option<String>,
    pub photographer: Option<String>,

    // Identification
    pub image_title: Option<String>,
    pub image_unique_id: Option<String>,
    pub interop_index: Option<String>,

    // ISO / sensitivity
    pub iso_speed: Option<i64>,
    pub iso_speed_latitude_yyy: Option<i64>,
    pub iso_speed_latitude_zzz: Option<i64>,
    pub photographic_sensitivity: Option<i64>,
    pub recommended_exposure_index: Option<i64>,
    pub sensitivity_type: Option<i32>,
    pub standard_output_sensitivity: Option<i64>,

    // Lens
    pub lens_make: Option<String>,
    pub lens_model: Option<String>,
    pub lens_serial_number: Option<String>,
    pub lens_info: Option<Vec<Rational>>, // LensSpecification (rational+)
}

impl XmpExifEx {
    pub fn insert(&mut self, key: &str, value: XmpValue) {
        match key {
            // --- Basic ---
            "Acceleration" => self.acceleration = as_rational(&value),
            "BodySerialNumber" => self.serial_number = as_string(&value),
            "CameraElevationAngle" => self.camera_elevation_angle = as_rational(&value),
            "CameraFirmware" => self.camera_firmware = as_string(&value),
            "CameraOwnerName" => self.owner_name = as_string(&value),

            // --- Composite image ---
            "CompositeImage" => {
                self.composite_image = as_i32(&value).map(|v| match v {
                    1 => XmpCompositeImage::NotComposite,
                    2 => XmpCompositeImage::GeneralComposite,
                    3 => XmpCompositeImage::CompositeCapturedWhileShooting,
                    _ => XmpCompositeImage::Unknown,
                });
            }
            "CompositeImageCount" => self.composite_image_count = as_i64(&value),

            // --- Environment ---
            "Gamma" => self.gamma = as_rational(&value),
            "Humidity" => self.humidity = as_rational(&value),
            "Pressure" => self.pressure = as_rational(&value),
            "Temperature" => self.ambient_temperature = as_rational(&value),
            "WaterDepth" => self.water_depth = as_rational(&value),

            // --- Software ---
            "ImageEditingSoftware" => self.image_editing_software = as_string(&value),
            "ImageEditor" => self.image_editor = as_string(&value),
            "MetadataEditingSoftware" => self.metadata_editing_software = as_string(&value),
            "RawDevelopingSoftware" => self.raw_developing_software = as_string(&value),
            "Photographer" => self.photographer = as_string(&value),

            // --- Identification ---
            "ImageTitle" => self.image_title = as_string(&value),
            "ImageUniqueID" => self.image_unique_id = as_string(&value),
            "InteropIndex" => self.interop_index = as_string(&value),

            // --- ISO ---
            "ISOSpeed" => self.iso_speed = as_i64(&value),
            "ISOSpeedLatitudeyyy" => self.iso_speed_latitude_yyy = as_i64(&value),
            "ISOSpeedLatitudezzz" => self.iso_speed_latitude_zzz = as_i64(&value),
            "PhotographicSensitivity" => self.photographic_sensitivity = as_i64(&value),
            "RecommendedExposureIndex" => self.recommended_exposure_index = as_i64(&value),
            "SensitivityType" => self.sensitivity_type = as_i32(&value),
            "StandardOutputSensitivity" => self.standard_output_sensitivity = as_i64(&value),

            // --- Lens ---
            "LensMake" => self.lens_make = as_string(&value),
            "LensModel" => self.lens_model = as_string(&value),
            "LensSerialNumber" => self.lens_serial_number = as_string(&value),

            _ => {}
        }
    }
}
#[derive(Debug, Default)]
pub struct XmpCompImageExp {
    pub max_exposure_times_of_all: Option<Rational>,
    pub max_exposure_times_of_used: Option<Rational>,
    pub min_exposure_times_of_all: Option<Rational>,
    pub min_exposure_times_of_used: Option<Rational>,
    pub number_of_images_in_sequences: Option<i64>,
    pub number_of_sequences: Option<i64>,
    pub sum_of_exposure_times_of_all: Option<Rational>,
    pub sum_of_exposure_times_of_used: Option<Rational>,
    pub total_exposure_period: Option<Rational>,
}

// Field Name	Writable	Values / Notes
// MaxExposureTimesOfAll	rational
// MaxExposureTimesOfUsed	rational
// MinExposureTimesOfAll	rational
// MinExposureTimesOfUsed	rational
// NumberOfImagesInSequences	integer
// NumberOfSequences	integer
// SumOfExposureTimesOfAll	rational
// SumOfExposureTimesOfUsed	rational
// TotalExposurePeriod	rational
// Values	rational+
#[derive(Debug)]
pub enum XmpSensitivityType {
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
#[derive(Debug)]
pub enum XmpCompositeImage {
    NotComposite = 1,
    GeneralComposite = 2,
    CompositeCapturedWhileShooting = 3,
    Unknown
}

// 0 = Unknown
// 1 = Not a Composite Image
// 2 = General Composite Image
// 3 = Composite Image Captured While Shooting
