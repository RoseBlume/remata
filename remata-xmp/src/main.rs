use xmp_macros::namespace_gen;
mod crd;
namespace_gen!( MyStruct {
    exposure_mode: "ExposureMode" => String,
    flash_mode: "FlashMode" => String,
});
namespace_gen!( MicrosoftXmp {
    camera_serial_number: "CameraSerialNumber" => String,
    creator_app_id: "CreatorAppID" => String,
    creator_open_with_ui_options: "CreatorOpenWithUIOptions" => String,
    date_acquired: "DateAcquired" => String,
    flash_manufacturer: "FlashManufacturer" => String,
    flash_model: "FlashModel" => String,
    item_sub_type: "ItemSubType" => String,
    last_keyword_iptc: "LastKeywordIPTC" => String,
    last_keyword_xmp: "LastKeywordXMP" => String,
    lens_manufacturer: "LensManufacturer" => String,
    lens_model: "LensModel" => String,
    rating_percent: "RatingPercent" => String,
});


namespace_gen!( Aas {
    affine_a: "AffineA" => Real,
    affine_b: "AffineB" => Real,
    affine_c: "AffineC" => Real,
    affine_d: "AffineD" => Real,

    affine_x: "AffineX" => Real,
    affine_y: "AffineY" => Real,

    crop_h: "CropH" => Integer,
    crop_w: "CropW" => Integer,
    crop_x: "CropX" => Integer,
    crop_y: "CropY" => Integer,

    curve_0x: "Curve0x" => Real,
    curve_0y: "Curve0y" => Real,
    curve_1x: "Curve1x" => Real,
    curve_1y: "Curve1y" => Real,
    curve_2x: "Curve2x" => Real,
    curve_2y: "Curve2y" => Real,
    curve_3x: "Curve3x" => Real,
    curve_3y: "Curve3y" => Real,
    curve_4x: "Curve4x" => Real,
    curve_4y: "Curve4y" => Real,
    face_balance_orig_i: "FaceBalanceOrigI" => Real,
    face_balance_orig_q: "FaceBalanceOrigQ" => Real,
    face_balance_strength: "FaceBalanceStrength" => Real,
    face_balance_warmth: "FaceBalanceWarmth" => Real,
    highlights: "Highlights" => Real,
    shadows: "Shadows" => Real,
    vibrance: "Vibrance" => Real,
});

// Tag Name	Writable	Values / Notes
// AffineA	real
// AffineB	real
// AffineC	real
// AffineD	real
// AffineX	real
// AffineY	real
// CropH	integer/
// CropW	integer/
// CropX	integer/
// CropY	integer/
// Curve0x	real
// Curve0y	real
// Curve1x	real
// Curve1y	real
// Curve2x	real
// Curve2y	real
// Curve3x	real
// Curve3y	real
// Curve4x	real
// Curve4y	real
// FaceBalanceOrigI	real
// FaceBalanceOrigQ	real
// FaceBalanceStrength	real
// FaceBalanceWarmth	real
// Highlights	real/
// Shadows	real/
// Vibrance	real/

namespace_gen!(Acdsee {
    author: "Author" => String,
    caption: "Caption" => String,
    categories: "Categories" => String,
    collections: "Collections" => String,
    datetime: "DateTime" => String, // later you can map this to a Date variant if you want
    dpp: "DPP" => String, // lang-alt (store raw for now)
    edit_status: "EditStatus" => String,
    fixture_identifier: "FixtureIdentifier" => String,
    keywords: "Keywords" => String,
    notes: "Notes" => String,
    object_cycle: "ObjectCycle" => String,
    originating_program: "OriginatingProgram" => String,
    rating: "Rating" => Real,
    rawrppused: "Rawrppused" => Bool,
    release_date: "ReleaseDate" => String,
    release_time: "ReleaseTime" => String,
    rpp: "RPP" => String, // lang-alt (raw XML for now)
    snapshots: "Snapshots" => String,
    tagged: "Tagged" => Bool,
});

// Tag Name	Writable	Values / Notes
// Author	string/
// Caption	string/
// Categories	string/
// Collections	string/
// DateTime	date/
// DPP	lang-alt/	(newer version of XML raw processing settings)
// EditStatus	string/
// FixtureIdentifier	string/
// Keywords	string/+
// Notes	string/
// ObjectCycle	string/
// OriginatingProgram	string/
// Rating	real/
// Rawrppused	boolean/
// ReleaseDate	string/
// ReleaseTime	string/
// RPP	lang-alt/	(raw processing settings in XML format)
// Snapshots	string/+
// Tagged	boolean/


namespace_gen!(AcdseeRegions {
    region_info: "Regions" => Struct,
    applied_dims: "RegionsAppliedToDimensions" => Struct,

    applied_h: "RegionsAppliedToDimensionsH" => Real,
    applied_w: "RegionsAppliedToDimensionsW" => Real,
    applied_unit: "RegionsAppliedToDimensionsUnit" => String,

    region_list: "RegionsRegionList" => StructArray,
    alg_area: "RegionsRegionListALGArea" => StructArray,

    alg_h: "RegionsRegionListALGAreaH" => RealArray,
    alg_w: "RegionsRegionListALGAreaW" => RealArray,
    alg_x: "RegionsRegionListALGAreaX" => RealArray,
    alg_y: "RegionsRegionListALGAreaY" => RealArray,

    dly_area: "RegionsRegionListDLYArea" => StructArray,

    dly_h: "RegionsRegionListDLYAreaH" => RealArray,
    dly_w: "RegionsRegionListDLYAreaW" => RealArray,
    dly_x: "RegionsRegionListDLYAreaX" => RealArray,
    dly_y: "RegionsRegionListDLYAreaY" => RealArray,

    name: "RegionsRegionListName" => StringArray,
    name_assign: "RegionsRegionListNameAssignType" => StringArray,
    region_type: "RegionsRegionListType" => StringArray,
});

namespace_gen!(Apdi {
    image_type: "AuxiliaryImageType" => String,
    native_format: "NativeFormat" => String,
    stored_format: "StoredFormat" => String,
});

// XMP apdi Tags
// Used in Apple HDR GainMap images.

// These tags belong to the ExifTool XMP-apdi family 1 group.

// Tag Name	Writable	Values / Notes
// AuxiliaryImageType	string
// NativeFormat	string
// StoredFormat	string

namespace_gen!(AppleFi {
    angle_info_roll: "Roll" => Integer,
    angle_info_yaw: "AngleInfoYaw" => Integer,
    confidence_level: "ConfidenceLevel" => Integer,
    face_id: "FaceID" => Integer,
    timestamp: "TimeStamp" => Integer,
});

// XMP apple_fi Tags
// Face information tags written by the Apple iPhone 5 inside the mwg-rs RegionExtensions.

// These tags belong to the ExifTool XMP-apple-fi family 1 group.

// Tag Name	Writable	Values / Notes
// AngleInfoRoll	integer
// AngleInfoYaw	integer
// ConfidenceLevel	integer
// FaceID	integer
// TimeStamp	integer

namespace_gen!(Aux {
    approximate_focus_distance: "ApproximateFocusDistance" => Rational,
    distortion_correction_already_applied: "DistortionCorrectionAlreadyApplied" => Bool,
    enhance_denoise_already_applied: "EnhanceDenoiseAlreadyApplied" => Bool,
    enhance_denoise_luma_amount: "EnhanceDenoiseLumaAmount" => String,
    enhance_denoise_version: "EnhanceDenoiseVersion" => String,
    enhance_details_already_applied: "EnhanceDetailsAlreadyApplied" => Bool,
    enhance_details_version: "EnhanceDetailsVersion" => String,
    enhance_super_resolution_already_applied: "EnhanceSuperResolutionAlreadyApplied" => Bool,
    enhance_super_resolution_scale: "EnhanceSuperResolutionScale" => Rational,
    enhance_super_resolution_version: "EnhanceSuperResolutionVersion" => String,
    firmware: "Firmware" => String,
    flash_compensation: "FlashCompensation" => Rational,
    fuji_rating_already_applied: "FujiRatingAlreadyApplied" => Bool,
    image_number: "ImageNumber" => String,
    is_merged_hdr: "IsMergedHDR" => Bool,
    is_merged_panorama: "IsMergedPanorama" => Bool,
    lateral_chromatic_aberration_correction_already_applied: "LateralChromaticAberrationCorrectionAlreadyApplied" => Bool,
    lens: "Lens" => String,
    lens_distort_info: "LensDistortInfo" => String,
    lens_id: "LensID" => String,
    lens_info: "LensInfo" => String,
    lens_serial_number: "LensSerialNumber" => String,
    neutral_density_factor: "NeutralDensityFactor" => String,
    owner_name: "OwnerName" => String,
    serial_number: "SerialNumber" => String,
    vignette_correction_already_applied: "VignetteCorrectionAlreadyApplied" => Bool,
});
// XMP Aux Tags
// Tag Name	Writable	Values / Notes
// ApproximateFocusDistance	rational	4294967295 = infinity
// DistortionCorrectionAlreadyApplied	boolean
// EnhanceDenoiseAlreadyApplied	boolean
// EnhanceDenoiseLumaAmount	string
// EnhanceDenoiseVersion	string
// EnhanceDetailsAlreadyApplied	boolean
// EnhanceDetailsVersion	string
// EnhanceSuperResolutionAlreadyApplied	boolean
// EnhanceSuperResolutionScale	rational
// EnhanceSuperResolutionVersion	string
// Firmware	string
// FlashCompensation	rational
// FujiRatingAlreadyApplied	boolean
// ImageNumber	string
// IsMergedHDR	boolean
// IsMergedPanorama	boolean
// LateralChromaticAberrationCorrectionAlreadyApplied	boolean
// Lens	string
// LensDistortInfo	string
// LensID	string
// LensInfo	string/	(4 rational values giving focal and aperture ranges)
// LensSerialNumber	string/
// NeutralDensityFactor	string
// OwnerName	string/
// SerialNumber	string/
// VignetteCorrectionAlreadyApplied	boolean

namespace_gen!(Cc {
    attribution_name: "AttributionName" => String,
    attribution_url: "AttributionURL" => String,
    deprecated_on: "DeprecatedOn" => String, // could later become Date
    jurisdiction: "Jurisdiction" => String,
    legal_code: "LegalCode" => String,
    more_permissions: "MorePermissions" => String,
    permits: "Permits" => StringArray,   // cc:DerivativeWorks, Distribution, etc.
    prohibits: "Prohibits" => StringArray, // cc:CommercialUse, HighIncomeNationUse
    requires: "Requires" => StringArray,  // cc:Attribution, ShareAlike, etc.
    use_guidelines: "UseGuidelines" => String,
});
// XMP cc Tags
// Creative Commons namespace tags. Note that the CC specification for XMP is non-existent, so ExifTool must make some assumptions about the format of the specific properties in XMP (see http://creativecommons.org/ns).

// These tags belong to the ExifTool XMP-cc family 1 group.

// Tag Name	Writable	Values / Notes
// AttributionName	string
// AttributionURL	string
// DeprecatedOn	date
// Jurisdiction	string
// LegalCode	string
// License	string
// MorePermissions	string
// Permits	string+	'cc:DerivativeWorks' = Derivative Works
// 'cc:Distribution' = Distribution
// 'cc:Reproduction' = Reproduction
// 'cc:Sharing' = Sharing
// Prohibits	string+	'cc:CommercialUse' = Commercial Use
// 'cc:HighIncomeNationUse' = High Income Nation Use
// Requires	string+
// 'cc:Attribution' = Attribution
// 'cc:Copyleft' = Copyleft
// 'cc:LesserCopyleft' = Lesser Copyleft
// 'cc:Notice' = Notice
// 'cc:ShareAlike' = Share Alike
// 'cc:SourceCode' = Source Code
// UseGuidelines	string
use std::fmt::Debug;
#[derive(Debug, Clone)]
pub struct XmpStructValue {
    pub fields: std::collections::HashMap<String, XmpValue>,
}



#[derive(Debug, Clone)]
pub enum XmpValue {
    String(String),
    Integer(i64),
    Rational {
        numerator: i64,
        denominator: i64,
    },
    Bool(bool),
    Real(f64),

    Struct(XmpStructValue),
    Array(Vec<XmpValue>),
}



impl XmpValue {
    pub fn parse(kind: &str, value: &str) -> Option<Self> {
        match kind {
            "String" => Some(Self::String(value.to_string())),

            "Integer" => value.parse().ok().map(Self::Integer),

            "Real" => value.parse().ok().map(Self::Real),

            "Bool" => match value.to_lowercase().as_str() {
                "true" => Some(Self::Bool(true)),
                "false" => Some(Self::Bool(false)),
                _ => None,
            },

            "Rational" => {
                let mut parts = value.split('/');
                let n = parts.next()?.parse().ok()?;
                let d = parts.next()?.parse().ok()?;
                Some(Self::Rational { numerator: n, denominator: d })
            }

            _ => None,
        }
    }
}
fn main() {
    // let mut s =  {
    //     exposure_mode: None,
    //     flash_mode: None,
    // };
    let mut s = AcdseeRegions::default();
    s.insert("RegionsAppliedToDimensionsUnit", "ThisIsMyInfo", "String");

    println!("{:?}", s);
}