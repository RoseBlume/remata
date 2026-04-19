use xmp_macros::namespace_gen;
use super::{
    XmpValue,
    XmpStructValue
};
namespace_gen!(Crd {
    already_applied: "AlreadyApplied" => Bool,

    auto_brightness: "AutoBrightness" => Bool,
    auto_contrast: "AutoContrast" => Bool,
    auto_exposure: "AutoExposure" => Bool,
    auto_lateral_ca: "AutoLateralCA" => Integer,
    auto_shadows: "AutoShadows" => Bool,
    auto_tone: "AutoTone" => Bool,

    auto_tone_digest: "AutoToneDigest" => String,
    auto_tone_digest_no_sat: "AutoToneDigestNoSat" => String,

    auto_white_version: "AutoWhiteVersion" => Integer,

    blacks_2012: "Blacks2012" => Integer,
    blue_hue: "BlueHue" => Integer,
    blue_saturation: "BlueSaturation" => Integer,
    brightness: "Brightness" => Integer,

    camera_model_restriction: "CameraModelRestriction" => String,
    camera_profile: "CameraProfile" => String,
    camera_profile_digest: "CameraProfileDigest" => String,

    chromatic_aberration_b: "ChromaticAberrationB" => Integer,
    chromatic_aberration_r: "ChromaticAberrationR" => Integer,

    clarity: "Clarity" => Integer,
    clarity_2012: "Clarity2012" => Integer,

    clipboard_aspect_ratio: "ClipboardAspectRatio" => Integer,
    clipboard_orientation: "ClipboardOrientation" => Integer,

    cluster: "Cluster" => String,

    color_grade_blending: "ColorGradeBlending" => Integer,
    color_grade_global_hue: "ColorGradeGlobalHue" => Integer,
    color_grade_global_lum: "ColorGradeGlobalLum" => Integer,
    color_grade_global_sat: "ColorGradeGlobalSat" => Integer,
    color_grade_highlight_lum: "ColorGradeHighlightLum" => Integer,
    color_grade_midtone_hue: "ColorGradeMidtoneHue" => Integer,
    color_grade_midtone_lum: "ColorGradeMidtoneLum" => Integer,
    color_grade_midtone_sat: "ColorGradeMidtoneSat" => Integer,
    color_grade_shadow_lum: "ColorGradeShadowLum" => Integer,

    color_noise_reduction: "ColorNoiseReduction" => Integer,
    color_noise_reduction_detail: "ColorNoiseReductionDetail" => Integer,
    color_noise_reduction_smoothness: "ColorNoiseReductionSmoothness" => Integer,

    color_variance: "ColorVariance" => Real,

    compatible_version: "CompatibleVersion" => String,
    contact_info: "ContactInfo" => String,

    contrast: "Contrast" => Integer,
    contrast_2012: "Contrast2012" => Integer,

    converter: "Converter" => String,
    convert_to_grayscale: "ConvertToGrayscale" => Bool,

    // Rational-valued field (explicitly requested behavior)
    flash_compensation: "FlashCompensation" => Rational,
});

namespace_gen!(CrdCircularGradientBasedCorrections {
    correction_name: "CircularGradientBasedCorrectionsCorrectionName" => String,
    correction_sync_id: "CircularGradientBasedCorrectionsCorrectionSyncID" => String,
    what: "CircularGradientBasedCorrectionsWhat" => String,

    blacks_2012: "CircularGradientBasedCorrectionsLocalBlacks2012" => Real,
    brightness: "CircularGradientBasedCorrectionsLocalBrightness" => Real,
    clarity: "CircularGradientBasedCorrectionsLocalClarity" => Real,
    clarity_2012: "CircularGradientBasedCorrectionsLocalClarity2012" => Real,
    contrast: "CircularGradientBasedCorrectionsLocalContrast" => Real,
    contrast_2012: "CircularGradientBasedCorrectionsLocalContrast2012" => Real,
    defringe: "CircularGradientBasedCorrectionsLocalDefringe" => Real,
    dehaze: "CircularGradientBasedCorrectionsLocalDehaze" => Real,
    exposure: "CircularGradientBasedCorrectionsLocalExposure" => Real,
    exposure_2012: "CircularGradientBasedCorrectionsLocalExposure2012" => Real,
    highlights_2012: "CircularGradientBasedCorrectionsLocalHighlights2012" => Real,
    hue: "CircularGradientBasedCorrectionsLocalHue" => Real,
    luminance_noise: "CircularGradientBasedCorrectionsLocalLuminanceNoise" => Real,
    moire: "CircularGradientBasedCorrectionsLocalMoire" => Real,
    saturation: "CircularGradientBasedCorrectionsLocalSaturation" => Real,
    shadows_2012: "CircularGradientBasedCorrectionsLocalShadows2012" => Real,
    sharpness: "CircularGradientBasedCorrectionsLocalSharpness" => Real,
    temperature: "CircularGradientBasedCorrectionsLocalTemperature" => Real,
    texture: "CircularGradientBasedCorrectionsLocalTexture" => Real,
    tint: "CircularGradientBasedCorrectionsLocalTint" => Real,
    toning_hue: "CircularGradientBasedCorrectionsLocalToningHue" => Real,
    toning_saturation: "CircularGradientBasedCorrectionsLocalToningSaturation" => Real,
    whites_2012: "CircularGradientBasedCorrectionsLocalWhites2012" => Real,
});

namespace_gen!(CrdCrop {
    copyright: "Copyright" => String,

    crop_angle: "CropAngle" => Real,
    crop_bottom: "CropBottom" => Real,
    crop_constrain_to_unit_square: "CropConstrainToUnitSquare" => Integer,
    crop_constrain_to_warp: "CropConstrainToWarp" => Integer,
    crop_height: "CropHeight" => Real,
    crop_left: "CropLeft" => Real,
    crop_right: "CropRight" => Real,
    crop_top: "CropTop" => Real,
    crop_unit: "CropUnit" => Integer,
    crop_units: "CropUnits" => Integer,
    crop_width: "CropWidth" => Real,

    default_auto_gray: "DefaultAutoGray" => Bool,
    default_auto_tone: "DefaultAutoTone" => Bool,
    defaults_specific_to_iso: "DefaultsSpecificToISO" => Bool,
    defaults_specific_to_serial: "DefaultsSpecificToSerial" => Bool,

    defringe: "Defringe" => Integer,
    defringe_green_amount: "DefringeGreenAmount" => Integer,
    defringe_green_hue_hi: "DefringeGreenHueHi" => Integer,
    defringe_green_hue_lo: "DefringeGreenHueLo" => Integer,
    defringe_purple_amount: "DefringePurpleAmount" => Integer,
    defringe_purple_hue_hi: "DefringePurpleHueHi" => Integer,
    defringe_purple_hue_lo: "DefringePurpleHueLo" => Integer,

    dehaze: "Dehaze" => Real,
    exposure: "Exposure" => Real,
    exposure_2012: "Exposure2012" => Real,

    fill_light: "FillLight" => Integer,

    grain_amount: "GrainAmount" => Integer,
    grain_frequency: "GrainFrequency" => Integer,
    grain_seed: "GrainSeed" => Integer,
    grain_size: "GrainSize" => Integer,

    group: "Group" => String,
    has_crop: "HasCrop" => Bool,
    has_settings: "HasSettings" => Bool,

    hdr_edit_mode: "HDREditMode" => Integer,
    hdr_max_value: "HDRMaxValue" => Real,

    highlight_2012: "Highlight2012" => Integer,
    highlight_recovery: "HighlightRecovery" => Integer,
    highlights_2012: "Highlights2012" => Integer,

    hue_adjustment_aqua: "HueAdjustmentAqua" => Integer,
    hue_adjustment_blue: "HueAdjustmentBlue" => Integer,
    hue_adjustment_green: "HueAdjustmentGreen" => Integer,
    hue_adjustment_magenta: "HueAdjustmentMagenta" => Integer,
    hue_adjustment_orange: "HueAdjustmentOrange" => Integer,
    hue_adjustment_purple: "HueAdjustmentPurple" => Integer,
    hue_adjustment_red: "HueAdjustmentRed" => Integer,
    hue_adjustment_yellow: "HueAdjustmentYellow" => Integer,

    incremental_temperature: "IncrementalTemperature" => Integer,
    incremental_tint: "IncrementalTint" => Integer,

    jpeg_handling: "JPEGHandling" => String,
});

namespace_gen!(CrdPerspective {
    perspective_aspect: "PerspectiveAspect" => Integer,
    perspective_horizontal: "PerspectiveHorizontal" => Integer,
    perspective_rotate: "PerspectiveRotate" => Real,
    perspective_scale: "PerspectiveScale" => Integer,
});

namespace_gen!(CrdDepthBasedCorrections {
    correction_active: "DepthBasedCorrCorrectionActive" => Bool,
    correction_amount: "DepthBasedCorrCorrectionAmount" => Real,

    local_corrected_depth: "DepthBasedCorrLocalCorrectedDepth" => Real,
    curve_refine_saturation: "DepthBasedCorrLocalCurveRefineSaturation" => Real,

    correction_sync_id: "DepthBasedCorrCorrectionSyncID" => String,
    what: "DepthBasedCorrWhat" => String,
});

namespace_gen!(CrdDepthMapInfo {
    depth_source: "DepthMapInfoDepthSource" => String,

    base_highlight_guide_input_digest: "DepthMapInfoBaseHighlightGuideInputDigest" => String,
    base_highlight_guide_table: "DepthMapInfoBaseHighlightGuideTable" => String,
    base_highlight_guide_version: "DepthMapInfoBaseHighlightGuideVersion" => String,

    base_layered_depth_input_digest: "DepthMapInfoBaseLayeredDepthInputDigest" => String,
    base_layered_depth_table: "DepthMapInfoBaseLayeredDepthTable" => String,
    base_layered_depth_version: "DepthMapInfoBaseLayeredDepthVersion" => String,

    base_raw_depth_input_digest: "DepthMapInfoBaseRawDepthInputDigest" => String,
    base_raw_depth_table: "DepthMapInfoBaseRawDepthTable" => String,
    base_raw_depth_version: "DepthMapInfoBaseRawDepthVersion" => String,
});

namespace_gen!(CrdLensBlur {
    active: "LensBlurActive" => Bool,
    amount: "LensBlurAmount" => Real,

    bokeh_aspect: "LensBlurBokehAspect" => Real,
    bokeh_rotation: "LensBlurBokehRotation" => Real,
    bokeh_shape: "LensBlurBokehShape" => Real,
    bokeh_shape_detail: "LensBlurBokehShapeDetail" => Real,

    cat_eye_amount: "LensBlurCatEyeAmount" => Real,
    cat_eye_scale: "LensBlurCatEyeScale" => Real,

    focal_range: "LensBlurFocalRange" => String,
    focal_range_source: "LensBlurFocalRangeSource" => Real,

    highlights_boost: "LensBlurHighlightsBoost" => Real,
    highlights_threshold: "LensBlurHighlightsThreshold" => Real,

    sampled_area: "LensBlurSampledArea" => String,
    sampled_range: "LensBlurSampledRange" => String,

    spherical_aberration: "LensBlurSphericalAberration" => Real,
    subject_range: "LensBlurSubjectRange" => String,

    version: "LensBlurVersion" => String,
});


namespace_gen!(CrdXmp {
    perspective_upright: "PerspectiveUpright" => Integer,
    perspective_vertical: "PerspectiveVertical" => Integer,
    perspective_x: "PerspectiveX" => Real,
    perspective_y: "PerspectiveY" => Real,

    point_colors: "PointColors" => Array,

    post_crop_vignette_amount: "PostCropVignetteAmount" => Integer,
    post_crop_vignette_feather: "PostCropVignetteFeather" => Integer,
    post_crop_vignette_highlight_contrast: "PostCropVignetteHighlightContrast" => Integer,
    post_crop_vignette_midpoint: "PostCropVignetteMidpoint" => Integer,
    post_crop_vignette_roundness: "PostCropVignetteRoundness" => Integer,
    post_crop_vignette_style: "PostCropVignetteStyle" => Integer,

    preset_type: "PresetType" => String,
    process_version: "ProcessVersion" => String,

    range_mask: "RangeMask" => Struct,

    range_mask_map_info: "RangeMaskMapInfo" => Struct,
    range_mask_lab_max: "RangeMaskMapInfoLabMax" => String,
    range_mask_lab_min: "RangeMaskMapInfoLabMin" => String,
    range_mask_lum_eq: "RangeMaskMapInfoLumEq" => Array,
    range_mask_rgb_max: "RangeMaskMapInfoRGBMax" => String,
    range_mask_rgb_min: "RangeMaskMapInfoRGBMin" => String,

    raw_file_name: "RawFileName" => String,
    red_eye_info: "RedEyeInfo" => Array,

    red_hue: "RedHue" => Integer,
    red_saturation: "RedSaturation" => Integer,

    retouch_areas: "RetouchAreas" => Array,
    retouch_area_feather: "RetouchAreaFeather" => Real,
    retouch_area_method: "RetouchAreaMethod" => String,
    retouch_area_offset_y: "RetouchAreaOffsetY" => Real,
    retouch_area_opacity: "RetouchAreaOpacity" => Real,
    retouch_area_seed: "RetouchAreaSeed" => Integer,
    retouch_area_source_state: "RetouchAreaSourceState" => String,
    retouch_area_source_x: "RetouchAreaSourceX" => Real,
    retouch_area_spot_type: "RetouchAreaSpotType" => String,

    retouch_area_mask_feather: "RetouchAreaMaskFeather" => Real,
    retouch_area_mask_flow: "RetouchAreaMaskFlow" => Real,
    retouch_area_mask_left: "RetouchAreaMaskLeft" => Real,
    retouch_area_mask_right: "RetouchAreaMaskRight" => Real,
    retouch_area_mask_top: "RetouchAreaMaskTop" => Real,
    retouch_area_mask_bottom: "RetouchAreaMaskBottom" => Real,
    retouch_area_mask_full_x: "RetouchAreaMaskFullX" => Real,
    retouch_area_mask_full_y: "RetouchAreaMaskFullY" => Real,
    retouch_area_mask_midpoint: "RetouchAreaMaskMidpoint" => Real,
    retouch_area_mask_roundness: "RetouchAreaMaskRoundness" => Real,
    retouch_area_mask_radius: "RetouchAreaMaskRadius" => Real,
    retouch_area_mask_x: "RetouchAreaMaskX" => Real,
    retouch_area_mask_y: "RetouchAreaMaskY" => Real,
    retouch_area_mask_value: "RetouchAreaMaskValue" => Real,
    retouch_area_mask_version: "RetouchAreaMaskVersion" => Integer,
    retouch_area_mask_mask_active: "RetouchAreaMaskMaskActive" => Bool,
    retouch_area_mask_mask_inverted: "RetouchAreaMaskMaskInverted" => Bool,
    retouch_area_mask_mask_blend_mode: "RetouchAreaMaskMaskBlendMode" => Integer,
    retouch_area_mask_mask_name: "RetouchAreaMaskMaskName" => String,

    saturation: "Saturation" => Integer,
    saturation_adjustment_aqua: "SaturationAdjustmentAqua" => Integer,
    saturation_adjustment_blue: "SaturationAdjustmentBlue" => Integer,
    saturation_adjustment_green: "SaturationAdjustmentGreen" => Integer,
    saturation_adjustment_magenta: "SaturationAdjustmentMagenta" => Integer,
    saturation_adjustment_orange: "SaturationAdjustmentOrange" => Integer,
    saturation_adjustment_purple: "SaturationAdjustmentPurple" => Integer,
    saturation_adjustment_red: "SaturationAdjustmentRed" => Integer,
    saturation_adjustment_yellow: "SaturationAdjustmentYellow" => Integer,

    sdr_blend: "SDRBlend" => Real,
    sdr_brightness: "SDRBrightness" => Real,
    sdr_contrast: "SDRContrast" => Real,
    sdr_highlights: "SDRHighlights" => Real,
    sdr_shadows: "SDRShadows" => Real,
    sdr_whites: "SDRWhites" => Real,

    shadows: "Shadows" => Integer,
    shadows_2012: "Shadows2012" => Integer,
    shadow_tint: "ShadowTint" => Integer,

    sharpness: "Sharpness" => Integer,
    sharpen_detail: "SharpenDetail" => Integer,
    sharpen_edge_masking: "SharpenEdgeMasking" => Integer,
    sharpen_radius: "SharpenRadius" => Real,
    smoothness: "Smoothness" => Integer,

    tone_curve: "ToneCurve" => Array,
    tone_curve_blue: "ToneCurveBlue" => Array,
    tone_curve_green: "ToneCurveGreen" => Array,
    tone_curve_red: "ToneCurveRed" => Array,
    tone_map_strength: "ToneMapStrength" => Real,

    upright_center_mode: "UprightCenterMode" => Integer,
    upright_center_norm_x: "UprightCenterNormX" => Real,
    upright_center_norm_y: "UprightCenterNormY" => Real,
    upright_focal_length_35mm: "UprightFocalLength35mm" => Real,
    upright_focal_mode: "UprightFocalMode" => Integer,
    upright_preview: "UprightPreview" => Bool,
    upright_version: "UprightVersion" => Integer,

    uuid: "UUID" => String,
    version: "Version" => String,

    vibrance: "Vibrance" => Integer,
    vignette_amount: "VignetteAmount" => Integer,
    vignette_midpoint: "VignetteMidpoint" => Integer,

    color_temperature: "ColorTemperature" => Integer,
    tint: "Tint" => Integer,

    texture: "Texture" => Integer,
    tone_map_strength_alt: "ToneMapStrength" => Real,

    white_balance: "WhiteBalance" => String,
    whites_2012: "Whites2012" => Integer,
});

// XMP crd Tags
// Adobe Camera Raw Defaults tags.

// These tags belong to the ExifTool XMP-crd family 1 group.

// Tag Name	Writable	Values / Notes
// AlreadyApplied	boolean/	 
// AutoBrightness	boolean/	 
// AutoContrast	boolean/	 
// AutoExposure	boolean/	 
// AutoLateralCA	integer/
// AutoShadows	boolean/	 
// AutoTone	boolean/	 
// AutoToneDigest	string/	 
// AutoToneDigestNoSat	string/	 
// AutoWhiteVersion	integer/	 
// Blacks2012	integer/
// BlueHue	integer/	 
// BlueSaturation	integer/	 
// Brightness	integer/	 
// CameraModelRestriction	string/
// CameraProfile	string/	 
// CameraProfileDigest	string/	 
// ChromaticAberrationB	integer/	 
// ChromaticAberrationR	integer/	 
// CircularGradientBasedCorrections	struct+	--> Correction Struct
// CircGradBasedCorrActive	boolean/_	(CircularGradientBasedCorrectionsCorrectionActive)
// CircGradBasedCorrAmount	real/_	(CircularGradientBasedCorrectionsCorrectionAmount)
// CircGradBasedCorrMasks	struct_+	--> CorrectionMask Struct
// (CircularGradientBasedCorrectionsCorrectionMasks)
// CircGradBasedCorrMaskAlpha	real/_	(CircularGradientBasedCorrectionsCorrectionMasksAlpha)
// CircGradBasedCorrMaskAngle	real/_	(CircularGradientBasedCorrectionsCorrectionMasksAngle)
// CircGradBasedCorrMaskBottom	real/_	(CircularGradientBasedCorrectionsCorrectionMasksBottom)
// CircGradBasedCorrMaskCenterValue	real/_	(CircularGradientBasedCorrectionsCorrectionMasksCenterValue)
// CircGradBasedCorrMaskCenterWeight	real/_	(CircularGradientBasedCorrectionsCorrectionMasksCenterWeight)
// CircGradBasedCorrMaskRange	struct_+	--> CorrRangeMask Struct
// (CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMask; called CorrectionRangeMask by the spec)
// CircGradBasedCorrMaskRangeAreaModels	struct_+	--> AreaModels Struct
// (CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModels)
// CircGradBasedCorrMaskRangeAreaModelsComponents	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsAreaComponents)
// CircGradBasedCorrMaskRangeAreaModelsColorSampleInfo	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// CircGradBasedCorrMaskRangeColorAmount	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskColorAmount)
// CircGradBasedCorrMaskRangeDepthFeather	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthFeather)
// CircGradBasedCorrMaskRangeDepthMax	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMax)
// CircGradBasedCorrMaskRangeDepthMin	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMin)
// CircGradBasedCorrMaskRangeInvert	boolean/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskInvert)
// CircGradBasedCorrMaskRangeLumFeather	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumFeather)
// CircGradBasedCorrMaskRangeLuminanceDepthSampleInfo	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLuminanceDepthSampleInfo)
// CircGradBasedCorrMaskRangeLumMax	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMax)
// CircGradBasedCorrMaskRangeLumMin	real/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMin)
// CircGradBasedCorrMaskRangeLumRange	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumRange)
// CircGradBasedCorrMaskRangeSampleType	integer/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskSampleType)
// CircGradBasedCorrMaskRangeType	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskType)
// CircGradBasedCorrMaskRangeVersion	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskVersion)
// CircGradBasedCorrMaskDabs	string/_	(CircularGradientBasedCorrectionsCorrectionMasksDabs)
// CircGradBasedCorrMaskFeather	real/_	(CircularGradientBasedCorrectionsCorrectionMasksFeather)
// CircGradBasedCorrMaskFlipped	boolean/_	(CircularGradientBasedCorrectionsCorrectionMasksFlipped)
// CircGradBasedCorrMaskFlow	real/_	(CircularGradientBasedCorrectionsCorrectionMasksFlow)
// CircGradBasedCorrMaskFullX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksFullX)
// CircGradBasedCorrMaskFullY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksFullY)
// CircGradBasedCorrMaskInputDigest	string/_	(CircularGradientBasedCorrectionsCorrectionMasksInputDigest)
// CircGradBasedCorrMaskLeft	real/_	(CircularGradientBasedCorrectionsCorrectionMasksLeft)
// CircGradBasedCorrMaskMaskActive	boolean/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskActive)
// CircGradBasedCorrMaskMaskBlendMode	integer/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskBlendMode)
// CircGradBasedCorrMaskMaskDigest	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskDigest)
// CircGradBasedCorrMaskMaskInverted	boolean/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskInverted)
// CircGradBasedCorrMaskMaskName	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskName)
// CircGradBasedCorrMaskMasks	struct_+	--> CorrectionMask Struct
// (CircularGradientBasedCorrectionsCorrectionMasksMasks)
// CircGradBasedCorrMaskMasksAlpha	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksAlpha)
// CircGradBasedCorrMaskMasksAngle	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksAngle)
// CircGradBasedCorrMaskMasksBottom	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksBottom)
// CircGradBasedCorrMaskMasksCenterValue	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksCenterValue)
// CircGradBasedCorrMaskMasksCenterWeight	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksCenterWeight)
// CircGradBasedCorrMaskMasksDabs	string/_+	(CircularGradientBasedCorrectionsCorrectionMasksMasksDabs)
// CircGradBasedCorrMaskMasksFeather	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksFeather)
// CircGradBasedCorrMaskMasksFlipped	boolean/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksFlipped)
// CircGradBasedCorrMaskMasksFlow	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksFlow)
// CircGradBasedCorrMaskMasksFullX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksFullX)
// CircGradBasedCorrMaskMasksFullY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksFullY)
// CircGradBasedCorrMaskMasksInputDigest	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksInputDigest)
// CircGradBasedCorrMaskMasksLeft	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksLeft)
// CircGradBasedCorrMaskMasksMaskActive	boolean/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskActive)
// CircGradBasedCorrMaskMasksMaskBlendMode	integer/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskBlendMode)
// CircGradBasedCorrMaskMasksMaskDigest	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskDigest)
// CircGradBasedCorrMaskMasksMaskInverted	boolean/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskInverted)
// CircGradBasedCorrMaskMasksMaskName	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskName)
// CircGradBasedCorrMaskMasksMaskSubType	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskSubType)
// CircGradBasedCorrMaskMasksMaskSyncID	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskSyncID)
// CircGradBasedCorrMaskMasksValue	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskValue)
// CircGradBasedCorrMaskMasksMaskVersion	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMaskVersion)
// CircGradBasedCorrMaskMasksMidpoint	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksMidpoint)
// CircGradBasedCorrMaskMasksOrigin	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksOrigin)
// CircGradBasedCorrMaskMasksPerimeterValue	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksPerimeterValue)
// CircGradBasedCorrMaskMasksRadius	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksRadius)
// CircGradBasedCorrMaskMasksReferencePoint	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksReferencePoint)
// CircGradBasedCorrMaskMasksRight	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksRight)
// CircGradBasedCorrMaskMasksRoundness	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksRoundness)
// CircGradBasedCorrMaskMasksSizeX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksSizeX)
// CircGradBasedCorrMaskMasksSizeY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksSizeY)
// CircGradBasedCorrMaskMasksTop	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksTop)
// CircGradBasedCorrMaskMaskSubType	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskSubType)
// CircGradBasedCorrMaskMasksVersion	integer/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksVersion)
// CircGradBasedCorrMaskMasksWhat	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksWhat)
// CircGradBasedCorrMaskMasksWholeImageArea	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksWholeImageArea)
// CircGradBasedCorrMaskMasksX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksX)
// CircGradBasedCorrMaskMasksY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksY)
// CircGradBasedCorrMaskMaskSyncID	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskSyncID)
// CircGradBasedCorrMaskMasksZeroX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksZeroX)
// CircGradBasedCorrMaskMasksZeroY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMasksZeroY)
// CircGradBasedCorrMaskValue	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskValue)
// CircGradBasedCorrMaskMaskVersion	string/_	(CircularGradientBasedCorrectionsCorrectionMasksMaskVersion)
// CircGradBasedCorrMaskMidpoint	real/_	(CircularGradientBasedCorrectionsCorrectionMasksMidpoint)
// CircGradBasedCorrMaskOrigin	string/_	(CircularGradientBasedCorrectionsCorrectionMasksOrigin)
// CircGradBasedCorrMaskPerimeterValue	real/_	(CircularGradientBasedCorrectionsCorrectionMasksPerimeterValue)
// CircGradBasedCorrMaskRadius	real/_	(CircularGradientBasedCorrectionsCorrectionMasksRadius)
// CircGradBasedCorrMaskReferencePoint	string/_	(CircularGradientBasedCorrectionsCorrectionMasksReferencePoint)
// CircGradBasedCorrMaskRight	real/_	(CircularGradientBasedCorrectionsCorrectionMasksRight)
// CircGradBasedCorrMaskRoundness	real/_	(CircularGradientBasedCorrectionsCorrectionMasksRoundness)
// CircGradBasedCorrMaskSizeX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksSizeX)
// CircGradBasedCorrMaskSizeY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksSizeY)
// CircGradBasedCorrMaskTop	real/_	(CircularGradientBasedCorrectionsCorrectionMasksTop)
// CircGradBasedCorrMaskVersion	integer/_	(CircularGradientBasedCorrectionsCorrectionMasksVersion)
// CircGradBasedCorrMaskWhat	string/_	(CircularGradientBasedCorrectionsCorrectionMasksWhat)
// CircGradBasedCorrMaskWholeImageArea	string/_	(CircularGradientBasedCorrectionsCorrectionMasksWholeImageArea)
// CircGradBasedCorrMaskX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksX)
// CircGradBasedCorrMaskY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksY)
// CircGradBasedCorrMaskZeroX	real/_	(CircularGradientBasedCorrectionsCorrectionMasksZeroX)
// CircGradBasedCorrMaskZeroY	real/_	(CircularGradientBasedCorrectionsCorrectionMasksZeroY)
// CircGradBasedCorrCorrectionName	string/_+	(CircularGradientBasedCorrectionsCorrectionName)
// CircGradBasedCorrRangeMask	struct_+	--> CorrRangeMask Struct
// (CircularGradientBasedCorrectionsCorrectionRangeMask; called CorrectionRangeMask by the spec)
// CircGradBasedCorrRangeMaskAreaModels	struct_+	--> AreaModels Struct
// (CircularGradientBasedCorrectionsCorrectionRangeMaskAreaModels)
// CircGradBasedCorrRangeMaskAreaModelsComponents	string/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskAreaModelsAreaComponents)
// CircGradBasedCorrRangeMaskAreaModelsColorSampleInfo	string/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// CircGradBasedCorrRangeMaskColorAmount	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskColorAmount)
// CircGradBasedCorrRangeMaskDepthFeather	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskDepthFeather)
// CircGradBasedCorrRangeMaskDepthMax	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskDepthMax)
// CircGradBasedCorrRangeMaskDepthMin	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskDepthMin)
// CircGradBasedCorrRangeMaskInvert	boolean/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskInvert)
// CircGradBasedCorrRangeMaskLumFeather	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskLumFeather)
// CircGradBasedCorrRangeMaskLuminanceDepthSampleInfo	string/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskLuminanceDepthSampleInfo)
// CircGradBasedCorrRangeMaskLumMax	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskLumMax)
// CircGradBasedCorrRangeMaskLumMin	real/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskLumMin)
// CircGradBasedCorrRangeMaskLumRange	string/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskLumRange)
// CircGradBasedCorrRangeMaskSampleType	integer/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskSampleType)
// CircGradBasedCorrRangeMaskType	string/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskType)
// CircGradBasedCorrRangeMaskVersion	string/_+	(CircularGradientBasedCorrectionsCorrectionRangeMaskVersion)
// CircGradBasedCorrCorrectionSyncID	string/_+	(CircularGradientBasedCorrectionsCorrectionSyncID)
// CircGradBasedCorrBlacks2012	real/_	(CircularGradientBasedCorrectionsLocalBlacks2012)
// CircGradBasedCorrBrightness	real/_	(CircularGradientBasedCorrectionsLocalBrightness)
// CircGradBasedCorrClarity	real/_	(CircularGradientBasedCorrectionsLocalClarity)
// CircGradBasedCorrClarity2012	real/_	(CircularGradientBasedCorrectionsLocalClarity2012)
// CircGradBasedCorrContrast	real/_	(CircularGradientBasedCorrectionsLocalContrast)
// CircGradBasedCorrContrast2012	real/_	(CircularGradientBasedCorrectionsLocalContrast2012)
// CircGradBasedCorrDefringe	real/_	(CircularGradientBasedCorrectionsLocalDefringe)
// CircGradBasedCorrDehaze	real/_	(CircularGradientBasedCorrectionsLocalDehaze)
// CircGradBasedCorrExposure	real/_	(CircularGradientBasedCorrectionsLocalExposure)
// CircGradBasedCorrExposure2012	real/_	(CircularGradientBasedCorrectionsLocalExposure2012)
// CircGradBasedCorrHighlights2012	real/_	(CircularGradientBasedCorrectionsLocalHighlights2012)
// CircGradBasedCorrHue	real/_	(CircularGradientBasedCorrectionsLocalHue)
// CircGradBasedCorrLuminanceNoise	real/_	(CircularGradientBasedCorrectionsLocalLuminanceNoise)
// CircGradBasedCorrMoire	real/_	(CircularGradientBasedCorrectionsLocalMoire)
// CircGradBasedCorrSaturation	real/_	(CircularGradientBasedCorrectionsLocalSaturation)
// CircGradBasedCorrShadows2012	real/_	(CircularGradientBasedCorrectionsLocalShadows2012)
// CircGradBasedCorrSharpness	real/_	(CircularGradientBasedCorrectionsLocalSharpness)
// CircGradBasedCorrTemperature	real/_	(CircularGradientBasedCorrectionsLocalTemperature)
// CircGradBasedCorrTexture	real/_	(CircularGradientBasedCorrectionsLocalTexture)
// CircGradBasedCorrTint	real/_	(CircularGradientBasedCorrectionsLocalTint)
// CircGradBasedCorrToningHue	real/_	(CircularGradientBasedCorrectionsLocalToningHue)
// CircGradBasedCorrToningSaturation	real/_	(CircularGradientBasedCorrectionsLocalToningSaturation)
// CircGradBasedCorrWhites2012	real/_	(CircularGradientBasedCorrectionsLocalWhites2012)
// CircGradBasedCorrWhat	string/_	(CircularGradientBasedCorrectionsWhat)
// Clarity	integer/	 
// Clarity2012	integer/
// ClipboardAspectRatio	integer/	 
// ClipboardOrientation	integer/	 
// Cluster	string/	 
// ColorGradeBlending	integer/	 
// ColorGradeGlobalHue	integer/	 
// ColorGradeGlobalLum	integer/	 
// ColorGradeGlobalSat	integer/	 
// ColorGradeHighlightLum	integer/	 
// ColorGradeMidtoneHue	integer/	 
// ColorGradeMidtoneLum	integer/	 
// ColorGradeMidtoneSat	integer/	 
// ColorGradeShadowLum	integer/	 
// ColorNoiseReduction	integer/
// ColorNoiseReductionDetail	integer/	 
// ColorNoiseReductionSmoothness	integer/	 
// ColorVariance	real/+	 
// CompatibleVersion	string/
// ContactInfo	string/	 
// Contrast	integer/	 
// Contrast2012	integer/	 
// Converter	string/	 
// ConvertToGrayscale	boolean/	 
// Copyright	string/	 
// CropAngle	real/	 
// CropBottom	real/
// CropConstrainToUnitSquare	integer/	 
// CropConstrainToWarp	integer/	 
// CropHeight	real/	 
// CropLeft	real/
// CropRight	real/	 
// CropTop	real/	 
// CropUnit	integer/	0 = pixels
// 1 = inches
// 2 = cm
// CropUnits	integer/	0 = pixels
// 1 = inches
// 2 = cm
// CropWidth	real/	 
// DefaultAutoGray	boolean/	 
// DefaultAutoTone	boolean/	 
// DefaultsSpecificToISO	boolean/	 
// DefaultsSpecificToSerial	boolean/	 
// Defringe	integer/	 
// DefringeGreenAmount	integer/
// DefringeGreenHueHi	integer/	 
// DefringeGreenHueLo	integer/	 
// DefringePurpleAmount	integer/	 
// DefringePurpleHueHi	integer/
// DefringePurpleHueLo	integer/	 
// Dehaze	real/	 
// DepthBasedCorrections	struct+	--> DepthBasedCorr Struct
// DepthBasedCorrCorrectionActive	boolean/_+	(DepthBasedCorrectionsCorrectionActive)
// DepthBasedCorrCorrectionAmount	real/_+	(DepthBasedCorrectionsCorrectionAmount)
// DepthBasedCorrMask	struct_+	--> CorrectionMask Struct
// (DepthBasedCorrectionsCorrectionMasks)
// DepthBasedCorrMaskAlpha	real/_	(DepthBasedCorrectionsCorrectionMasksAlpha)
// DepthBasedCorrMaskAngle	real/_	(DepthBasedCorrectionsCorrectionMasksAngle)
// DepthBasedCorrMaskBottom	real/_	(DepthBasedCorrectionsCorrectionMasksBottom)
// DepthBasedCorrMaskCenterValue	real/_	(DepthBasedCorrectionsCorrectionMasksCenterValue)
// DepthBasedCorrMaskCenterWeight	real/_	(DepthBasedCorrectionsCorrectionMasksCenterWeight)
// DepthBasedCorrMaskRange	struct_+	--> CorrRangeMask Struct
// (DepthBasedCorrectionsCorrectionMasksCorrectionRangeMask; called CorrectionRangeMask by the spec)
// DepthBasedCorrMaskRangeAreaModels	struct_+	--> AreaModels Struct
// (DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModels)
// DepthBasedCorrMaskRangeAreaModelsComponents	string/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsAreaComponents)
// DepthBasedCorrMaskRangeAreaModelsColorSampleInfo	string/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// DepthBasedCorrMaskRangeColorAmount	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskColorAmount)
// DepthBasedCorrMaskRangeDepthFeather	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthFeather)
// DepthBasedCorrMaskRangeDepthMax	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMax)
// DepthBasedCorrMaskRangeDepthMin	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMin)
// DepthBasedCorrMaskRangeInvert	boolean/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskInvert)
// DepthBasedCorrMaskRangeLumFeather	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumFeather)
// DepthBasedCorrMaskRangeLuminanceDepthSampleInfo	string/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskLuminanceDepthSampleInfo)
// DepthBasedCorrMaskRangeLumMax	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMax)
// DepthBasedCorrMaskRangeLumMin	real/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMin)
// DepthBasedCorrMaskRangeLumRange	string/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumRange)
// DepthBasedCorrMaskRangeSampleType	integer/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskSampleType)
// DepthBasedCorrMaskRangeType	string/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskType)
// DepthBasedCorrMaskRangeVersion	string/_+	(DepthBasedCorrectionsCorrectionMasksCorrectionRangeMaskVersion)
// DepthBasedCorrMaskDabs	string/_+	(DepthBasedCorrectionsCorrectionMasksDabs)
// DepthBasedCorrMaskFeather	real/_	(DepthBasedCorrectionsCorrectionMasksFeather)
// DepthBasedCorrMaskFlipped	boolean/_	(DepthBasedCorrectionsCorrectionMasksFlipped)
// DepthBasedCorrMaskFlow	real/_	(DepthBasedCorrectionsCorrectionMasksFlow)
// DepthBasedCorrMaskFullX	real/_	(DepthBasedCorrectionsCorrectionMasksFullX)
// DepthBasedCorrMaskFullY	real/_	(DepthBasedCorrectionsCorrectionMasksFullY)
// DepthBasedCorrMaskInputDigest	string/_	(DepthBasedCorrectionsCorrectionMasksInputDigest)
// DepthBasedCorrMaskLeft	real/_	(DepthBasedCorrectionsCorrectionMasksLeft)
// DepthBasedCorrMaskMaskActive	boolean/_	(DepthBasedCorrectionsCorrectionMasksMaskActive)
// DepthBasedCorrMaskMaskBlendMode	integer/_	(DepthBasedCorrectionsCorrectionMasksMaskBlendMode)
// DepthBasedCorrMaskMaskDigest	string/_	(DepthBasedCorrectionsCorrectionMasksMaskDigest)
// DepthBasedCorrMaskMaskInverted	boolean/_	(DepthBasedCorrectionsCorrectionMasksMaskInverted)
// DepthBasedCorrMaskMaskName	string/_	(DepthBasedCorrectionsCorrectionMasksMaskName)
// DepthBasedCorrMaskMasks	struct_+	--> CorrectionMask Struct
// (DepthBasedCorrectionsCorrectionMasksMasks)
// DepthBasedCorrMaskMasksAlpha	real/_	(DepthBasedCorrectionsCorrectionMasksMasksAlpha)
// DepthBasedCorrMaskMasksAngle	real/_	(DepthBasedCorrectionsCorrectionMasksMasksAngle)
// DepthBasedCorrMaskMasksBottom	real/_	(DepthBasedCorrectionsCorrectionMasksMasksBottom)
// DepthBasedCorrMaskMasksCenterValue	real/_	(DepthBasedCorrectionsCorrectionMasksMasksCenterValue)
// DepthBasedCorrMaskMasksCenterWeight	real/_	(DepthBasedCorrectionsCorrectionMasksMasksCenterWeight)
// DepthBasedCorrMaskMasksDabs	string/_+	(DepthBasedCorrectionsCorrectionMasksMasksDabs)
// DepthBasedCorrMaskMasksFeather	real/_	(DepthBasedCorrectionsCorrectionMasksMasksFeather)
// DepthBasedCorrMaskMasksFlipped	boolean/_	(DepthBasedCorrectionsCorrectionMasksMasksFlipped)
// DepthBasedCorrMaskMasksFlow	real/_	(DepthBasedCorrectionsCorrectionMasksMasksFlow)
// DepthBasedCorrMaskMasksFullX	real/_	(DepthBasedCorrectionsCorrectionMasksMasksFullX)
// DepthBasedCorrMaskMasksFullY	real/_	(DepthBasedCorrectionsCorrectionMasksMasksFullY)
// DepthBasedCorrMaskMasksInputDigest	string/_	(DepthBasedCorrectionsCorrectionMasksMasksInputDigest)
// DepthBasedCorrMaskMasksLeft	real/_	(DepthBasedCorrectionsCorrectionMasksMasksLeft)
// DepthBasedCorrMaskMasksMaskActive	boolean/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskActive)
// DepthBasedCorrMaskMasksMaskBlendMode	integer/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskBlendMode)
// DepthBasedCorrMaskMasksMaskDigest	string/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskDigest)
// DepthBasedCorrMaskMasksMaskInverted	boolean/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskInverted)
// DepthBasedCorrMaskMasksMaskName	string/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskName)
// DepthBasedCorrMaskMasksMaskSubType	string/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskSubType)
// DepthBasedCorrMaskMasksMaskSyncID	string/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskSyncID)
// DepthBasedCorrMaskMasksValue	real/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskValue)
// DepthBasedCorrMaskMasksMaskVersion	string/_	(DepthBasedCorrectionsCorrectionMasksMasksMaskVersion)
// DepthBasedCorrMaskMasksMidpoint	real/_	(DepthBasedCorrectionsCorrectionMasksMasksMidpoint)
// DepthBasedCorrMaskMasksOrigin	string/_	(DepthBasedCorrectionsCorrectionMasksMasksOrigin)
// DepthBasedCorrMaskMasksPerimeterValue	real/_	(DepthBasedCorrectionsCorrectionMasksMasksPerimeterValue)
// DepthBasedCorrMaskMasksRadius	real/_	(DepthBasedCorrectionsCorrectionMasksMasksRadius)
// DepthBasedCorrMaskMasksReferencePoint	string/_	(DepthBasedCorrectionsCorrectionMasksMasksReferencePoint)
// DepthBasedCorrMaskMasksRight	real/_	(DepthBasedCorrectionsCorrectionMasksMasksRight)
// DepthBasedCorrMaskMasksRoundness	real/_	(DepthBasedCorrectionsCorrectionMasksMasksRoundness)
// DepthBasedCorrMaskMasksSizeX	real/_	(DepthBasedCorrectionsCorrectionMasksMasksSizeX)
// DepthBasedCorrMaskMasksSizeY	real/_	(DepthBasedCorrectionsCorrectionMasksMasksSizeY)
// DepthBasedCorrMaskMasksTop	real/_	(DepthBasedCorrectionsCorrectionMasksMasksTop)
// DepthBasedCorrMaskMaskSubType	string/_	(DepthBasedCorrectionsCorrectionMasksMaskSubType)
// DepthBasedCorrMaskMasksVersion	integer/_	(DepthBasedCorrectionsCorrectionMasksMasksVersion)
// DepthBasedCorrMaskMasksWhat	string/_	(DepthBasedCorrectionsCorrectionMasksMasksWhat)
// DepthBasedCorrMaskMasksWholeImageArea	string/_	(DepthBasedCorrectionsCorrectionMasksMasksWholeImageArea)
// DepthBasedCorrMaskMasksX	real/_	(DepthBasedCorrectionsCorrectionMasksMasksX)
// DepthBasedCorrMaskMasksY	real/_	(DepthBasedCorrectionsCorrectionMasksMasksY)
// DepthBasedCorrMaskMaskSyncID	string/_	(DepthBasedCorrectionsCorrectionMasksMaskSyncID)
// DepthBasedCorrMaskMasksZeroX	real/_	(DepthBasedCorrectionsCorrectionMasksMasksZeroX)
// DepthBasedCorrMaskMasksZeroY	real/_	(DepthBasedCorrectionsCorrectionMasksMasksZeroY)
// DepthBasedCorrMaskValue	real/_	(DepthBasedCorrectionsCorrectionMasksMaskValue)
// DepthBasedCorrMaskMaskVersion	string/_	(DepthBasedCorrectionsCorrectionMasksMaskVersion)
// DepthBasedCorrMaskMidpoint	real/_	(DepthBasedCorrectionsCorrectionMasksMidpoint)
// DepthBasedCorrMaskOrigin	string/_	(DepthBasedCorrectionsCorrectionMasksOrigin)
// DepthBasedCorrMaskPerimeterValue	real/_	(DepthBasedCorrectionsCorrectionMasksPerimeterValue)
// DepthBasedCorrMaskRadius	real/_	(DepthBasedCorrectionsCorrectionMasksRadius)
// DepthBasedCorrMaskReferencePoint	string/_	(DepthBasedCorrectionsCorrectionMasksReferencePoint)
// DepthBasedCorrMaskRight	real/_	(DepthBasedCorrectionsCorrectionMasksRight)
// DepthBasedCorrMaskRoundness	real/_	(DepthBasedCorrectionsCorrectionMasksRoundness)
// DepthBasedCorrMaskSizeX	real/_	(DepthBasedCorrectionsCorrectionMasksSizeX)
// DepthBasedCorrMaskSizeY	real/_	(DepthBasedCorrectionsCorrectionMasksSizeY)
// DepthBasedCorrMaskTop	real/_	(DepthBasedCorrectionsCorrectionMasksTop)
// DepthBasedCorrMaskVersion	integer/_	(DepthBasedCorrectionsCorrectionMasksVersion)
// DepthBasedCorrMaskWhat	string/_	(DepthBasedCorrectionsCorrectionMasksWhat)
// DepthBasedCorrMaskWholeImageArea	string/_	(DepthBasedCorrectionsCorrectionMasksWholeImageArea)
// DepthBasedCorrMaskX	real/_	(DepthBasedCorrectionsCorrectionMasksX)
// DepthBasedCorrMaskY	real/_	(DepthBasedCorrectionsCorrectionMasksY)
// DepthBasedCorrMaskZeroX	real/_	(DepthBasedCorrectionsCorrectionMasksZeroX)
// DepthBasedCorrMaskZeroY	real/_	(DepthBasedCorrectionsCorrectionMasksZeroY)
// DepthBasedCorrCorrectionSyncID	string/_+	(DepthBasedCorrectionsCorrectionSyncID)
// DepthBasedCorrLocalCorrectedDepth	real/_+	(DepthBasedCorrectionsLocalCorrectedDepth)
// DepthBasedCorrLocalCurveRefineSaturation	real/_+	(DepthBasedCorrectionsLocalCurveRefineSaturation)
// DepthBasedCorrWhat	string/_+	(DepthBasedCorrectionsWhat)
// DepthMapInfo	struct	--> DepthMapInfo Struct
// DepthMapInfoBaseHighlightGuideInputDigest	string/_	 
// DepthMapInfoBaseHighlightGuideTable	string/_	 
// DepthMapInfoBaseHighlightGuideVersion	string/_	 
// DepthMapInfoBaseLayeredDepthInputDigest	string/_	 
// DepthMapInfoBaseLayeredDepthTable	string/_	 
// DepthMapInfoBaseLayeredDepthVersion	string/_	 
// DepthMapInfoBaseRawDepthInputDigest	string/_	 
// DepthMapInfoBaseRawDepthTable	string/_	 
// DepthMapInfoBaseRawDepthVersion	string/_	 
// DepthMapInfoDepthSource	string/_	 
// Description	lang-alt/
// DNGIgnoreSidecars	boolean/	 
// Exposure	real/	 
// Exposure2012	real/	 
// FillLight	integer/
// GradientBasedCorrections	struct+	--> Correction Struct
// GradientBasedCorrActive	boolean/_	(GradientBasedCorrectionsCorrectionActive)
// GradientBasedCorrAmount	real/_	(GradientBasedCorrectionsCorrectionAmount)
// GradientBasedCorrMasks	struct_+	--> CorrectionMask Struct
// (GradientBasedCorrectionsCorrectionMasks)
// GradientBasedCorrMaskAlpha	real/_	(GradientBasedCorrectionsCorrectionMasksAlpha)
// GradientBasedCorrMaskAngle	real/_	(GradientBasedCorrectionsCorrectionMasksAngle)
// GradientBasedCorrMaskBottom	real/_	(GradientBasedCorrectionsCorrectionMasksBottom)
// GradientBasedCorrMaskCenterValue	real/_	(GradientBasedCorrectionsCorrectionMasksCenterValue)
// GradientBasedCorrMaskCenterWeight	real/_	(GradientBasedCorrectionsCorrectionMasksCenterWeight)
// GradientBasedCorrMaskRange	struct_+	--> CorrRangeMask Struct
// (GradientBasedCorrectionsCorrectionMasksCorrectionRangeMask; called CorrectionRangeMask by the spec)
// GradientBasedCorrMaskRangeAreaModels	struct_+	--> AreaModels Struct
// (GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModels)
// GradientBasedCorrMaskRangeAreaModelsComponents	string/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsAreaComponents)
// GradientBasedCorrMaskRangeAreaModelsColorSampleInfo	string/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// GradientBasedCorrMaskRangeColorAmount	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskColorAmount)
// GradientBasedCorrMaskRangeDepthFeather	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthFeather)
// GradientBasedCorrMaskRangeDepthMax	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMax)
// GradientBasedCorrMaskRangeDepthMin	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMin)
// GradientBasedCorrMaskRangeInvert	boolean/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskInvert)
// GradientBasedCorrMaskRangeLumFeather	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumFeather)
// GradientBasedCorrMaskRangeLuminanceDepthSampleInfo	string/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLuminanceDepthSampleInfo)
// GradientBasedCorrMaskRangeLumMax	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMax)
// GradientBasedCorrMaskRangeLumMin	real/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMin)
// GradientBasedCorrMaskRangeLumRange	string/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumRange)
// GradientBasedCorrMaskRangeSampleType	integer/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskSampleType)
// GradientBasedCorrMaskRangeType	string/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskType)
// GradientBasedCorrMaskRangeVersion	string/_+	(GradientBasedCorrectionsCorrectionMasksCorrectionRangeMaskVersion)
// GradientBasedCorrMaskDabs	string/_	(GradientBasedCorrectionsCorrectionMasksDabs)
// GradientBasedCorrMaskFeather	real/_	(GradientBasedCorrectionsCorrectionMasksFeather)
// GradientBasedCorrMaskFlipped	boolean/_	(GradientBasedCorrectionsCorrectionMasksFlipped)
// GradientBasedCorrMaskFlow	real/_	(GradientBasedCorrectionsCorrectionMasksFlow)
// GradientBasedCorrMaskFullX	real/_	(GradientBasedCorrectionsCorrectionMasksFullX)
// GradientBasedCorrMaskFullY	real/_	(GradientBasedCorrectionsCorrectionMasksFullY)
// GradientBasedCorrMaskInputDigest	string/_	(GradientBasedCorrectionsCorrectionMasksInputDigest)
// GradientBasedCorrMaskLeft	real/_	(GradientBasedCorrectionsCorrectionMasksLeft)
// GradientBasedCorrMaskMaskActive	boolean/_	(GradientBasedCorrectionsCorrectionMasksMaskActive)
// GradientBasedCorrMaskMaskBlendMode	integer/_	(GradientBasedCorrectionsCorrectionMasksMaskBlendMode)
// GradientBasedCorrMaskMaskDigest	string/_	(GradientBasedCorrectionsCorrectionMasksMaskDigest)
// GradientBasedCorrMaskMaskInverted	boolean/_	(GradientBasedCorrectionsCorrectionMasksMaskInverted)
// GradientBasedCorrMaskMaskName	string/_	(GradientBasedCorrectionsCorrectionMasksMaskName)
// GradientBasedCorrMaskMasks	struct_+	--> CorrectionMask Struct
// (GradientBasedCorrectionsCorrectionMasksMasks)
// GradientBasedCorrMaskMasksAlpha	real/_	(GradientBasedCorrectionsCorrectionMasksMasksAlpha)
// GradientBasedCorrMaskMasksAngle	real/_	(GradientBasedCorrectionsCorrectionMasksMasksAngle)
// GradientBasedCorrMaskMasksBottom	real/_	(GradientBasedCorrectionsCorrectionMasksMasksBottom)
// GradientBasedCorrMaskMasksCenterValue	real/_	(GradientBasedCorrectionsCorrectionMasksMasksCenterValue)
// GradientBasedCorrMaskMasksCenterWeight	real/_	(GradientBasedCorrectionsCorrectionMasksMasksCenterWeight)
// GradientBasedCorrMaskMasksDabs	string/_+	(GradientBasedCorrectionsCorrectionMasksMasksDabs)
// GradientBasedCorrMaskMasksFeather	real/_	(GradientBasedCorrectionsCorrectionMasksMasksFeather)
// GradientBasedCorrMaskMasksFlipped	boolean/_	(GradientBasedCorrectionsCorrectionMasksMasksFlipped)
// GradientBasedCorrMaskMasksFlow	real/_	(GradientBasedCorrectionsCorrectionMasksMasksFlow)
// GradientBasedCorrMaskMasksFullX	real/_	(GradientBasedCorrectionsCorrectionMasksMasksFullX)
// GradientBasedCorrMaskMasksFullY	real/_	(GradientBasedCorrectionsCorrectionMasksMasksFullY)
// GradientBasedCorrMaskMasksInputDigest	string/_	(GradientBasedCorrectionsCorrectionMasksMasksInputDigest)
// GradientBasedCorrMaskMasksLeft	real/_	(GradientBasedCorrectionsCorrectionMasksMasksLeft)
// GradientBasedCorrMaskMasksMaskActive	boolean/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskActive)
// GradientBasedCorrMaskMasksMaskBlendMode	integer/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskBlendMode)
// GradientBasedCorrMaskMasksMaskDigest	string/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskDigest)
// GradientBasedCorrMaskMasksMaskInverted	boolean/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskInverted)
// GradientBasedCorrMaskMasksMaskName	string/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskName)
// GradientBasedCorrMaskMasksMaskSubType	string/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskSubType)
// GradientBasedCorrMaskMasksMaskSyncID	string/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskSyncID)
// GradientBasedCorrMaskMasksValue	real/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskValue)
// GradientBasedCorrMaskMasksMaskVersion	string/_	(GradientBasedCorrectionsCorrectionMasksMasksMaskVersion)
// GradientBasedCorrMaskMasksMidpoint	real/_	(GradientBasedCorrectionsCorrectionMasksMasksMidpoint)
// GradientBasedCorrMaskMasksOrigin	string/_	(GradientBasedCorrectionsCorrectionMasksMasksOrigin)
// GradientBasedCorrMaskMasksPerimeterValue	real/_	(GradientBasedCorrectionsCorrectionMasksMasksPerimeterValue)
// GradientBasedCorrMaskMasksRadius	real/_	(GradientBasedCorrectionsCorrectionMasksMasksRadius)
// GradientBasedCorrMaskMasksReferencePoint	string/_	(GradientBasedCorrectionsCorrectionMasksMasksReferencePoint)
// GradientBasedCorrMaskMasksRight	real/_	(GradientBasedCorrectionsCorrectionMasksMasksRight)
// GradientBasedCorrMaskMasksRoundness	real/_	(GradientBasedCorrectionsCorrectionMasksMasksRoundness)
// GradientBasedCorrMaskMasksSizeX	real/_	(GradientBasedCorrectionsCorrectionMasksMasksSizeX)
// GradientBasedCorrMaskMasksSizeY	real/_	(GradientBasedCorrectionsCorrectionMasksMasksSizeY)
// GradientBasedCorrMaskMasksTop	real/_	(GradientBasedCorrectionsCorrectionMasksMasksTop)
// GradientBasedCorrMaskMaskSubType	string/_	(GradientBasedCorrectionsCorrectionMasksMaskSubType)
// GradientBasedCorrMaskMasksVersion	integer/_	(GradientBasedCorrectionsCorrectionMasksMasksVersion)
// GradientBasedCorrMaskMasksWhat	string/_	(GradientBasedCorrectionsCorrectionMasksMasksWhat)
// GradientBasedCorrMaskMasksWholeImageArea	string/_	(GradientBasedCorrectionsCorrectionMasksMasksWholeImageArea)
// GradientBasedCorrMaskMasksX	real/_	(GradientBasedCorrectionsCorrectionMasksMasksX)
// GradientBasedCorrMaskMasksY	real/_	(GradientBasedCorrectionsCorrectionMasksMasksY)
// GradientBasedCorrMaskMaskSyncID	string/_	(GradientBasedCorrectionsCorrectionMasksMaskSyncID)
// GradientBasedCorrMaskMasksZeroX	real/_	(GradientBasedCorrectionsCorrectionMasksMasksZeroX)
// GradientBasedCorrMaskMasksZeroY	real/_	(GradientBasedCorrectionsCorrectionMasksMasksZeroY)
// GradientBasedCorrMaskValue	real/_	(GradientBasedCorrectionsCorrectionMasksMaskValue)
// GradientBasedCorrMaskMaskVersion	string/_	(GradientBasedCorrectionsCorrectionMasksMaskVersion)
// GradientBasedCorrMaskMidpoint	real/_	(GradientBasedCorrectionsCorrectionMasksMidpoint)
// GradientBasedCorrMaskOrigin	string/_	(GradientBasedCorrectionsCorrectionMasksOrigin)
// GradientBasedCorrMaskPerimeterValue	real/_	(GradientBasedCorrectionsCorrectionMasksPerimeterValue)
// GradientBasedCorrMaskRadius	real/_	(GradientBasedCorrectionsCorrectionMasksRadius)
// GradientBasedCorrMaskReferencePoint	string/_	(GradientBasedCorrectionsCorrectionMasksReferencePoint)
// GradientBasedCorrMaskRight	real/_	(GradientBasedCorrectionsCorrectionMasksRight)
// GradientBasedCorrMaskRoundness	real/_	(GradientBasedCorrectionsCorrectionMasksRoundness)
// GradientBasedCorrMaskSizeX	real/_	(GradientBasedCorrectionsCorrectionMasksSizeX)
// GradientBasedCorrMaskSizeY	real/_	(GradientBasedCorrectionsCorrectionMasksSizeY)
// GradientBasedCorrMaskTop	real/_	(GradientBasedCorrectionsCorrectionMasksTop)
// GradientBasedCorrMaskVersion	integer/_	(GradientBasedCorrectionsCorrectionMasksVersion)
// GradientBasedCorrMaskWhat	string/_	(GradientBasedCorrectionsCorrectionMasksWhat)
// GradientBasedCorrMaskWholeImageArea	string/_	(GradientBasedCorrectionsCorrectionMasksWholeImageArea)
// GradientBasedCorrMaskX	real/_	(GradientBasedCorrectionsCorrectionMasksX)
// GradientBasedCorrMaskY	real/_	(GradientBasedCorrectionsCorrectionMasksY)
// GradientBasedCorrMaskZeroX	real/_	(GradientBasedCorrectionsCorrectionMasksZeroX)
// GradientBasedCorrMaskZeroY	real/_	(GradientBasedCorrectionsCorrectionMasksZeroY)
// GradientBasedCorrCorrectionName	string/_+	(GradientBasedCorrectionsCorrectionName)
// GradientBasedCorrRangeMask	struct_+	--> CorrRangeMask Struct
// (GradientBasedCorrectionsCorrectionRangeMask; called CorrectionRangeMask by the spec)
// GradientBasedCorrRangeMaskAreaModels	struct_+	--> AreaModels Struct
// (GradientBasedCorrectionsCorrectionRangeMaskAreaModels)
// GradientBasedCorrRangeMaskAreaModelsComponents	string/_+	(GradientBasedCorrectionsCorrectionRangeMaskAreaModelsAreaComponents)
// GradientBasedCorrRangeMaskAreaModelsColorSampleInfo	string/_+	(GradientBasedCorrectionsCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// GradientBasedCorrRangeMaskColorAmount	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskColorAmount)
// GradientBasedCorrRangeMaskDepthFeather	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskDepthFeather)
// GradientBasedCorrRangeMaskDepthMax	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskDepthMax)
// GradientBasedCorrRangeMaskDepthMin	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskDepthMin)
// GradientBasedCorrRangeMaskInvert	boolean/_+	(GradientBasedCorrectionsCorrectionRangeMaskInvert)
// GradientBasedCorrRangeMaskLumFeather	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskLumFeather)
// GradientBasedCorrRangeMaskLuminanceDepthSampleInfo	string/_+	(GradientBasedCorrectionsCorrectionRangeMaskLuminanceDepthSampleInfo)
// GradientBasedCorrRangeMaskLumMax	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskLumMax)
// GradientBasedCorrRangeMaskLumMin	real/_+	(GradientBasedCorrectionsCorrectionRangeMaskLumMin)
// GradientBasedCorrRangeMaskLumRange	string/_+	(GradientBasedCorrectionsCorrectionRangeMaskLumRange)
// GradientBasedCorrRangeMaskSampleType	integer/_+	(GradientBasedCorrectionsCorrectionRangeMaskSampleType)
// GradientBasedCorrRangeMaskType	string/_+	(GradientBasedCorrectionsCorrectionRangeMaskType)
// GradientBasedCorrRangeMaskVersion	string/_+	(GradientBasedCorrectionsCorrectionRangeMaskVersion)
// GradientBasedCorrCorrectionSyncID	string/_+	(GradientBasedCorrectionsCorrectionSyncID)
// GradientBasedCorrBlacks2012	real/_	(GradientBasedCorrectionsLocalBlacks2012)
// GradientBasedCorrBrightness	real/_	(GradientBasedCorrectionsLocalBrightness)
// GradientBasedCorrClarity	real/_	(GradientBasedCorrectionsLocalClarity)
// GradientBasedCorrClarity2012	real/_	(GradientBasedCorrectionsLocalClarity2012)
// GradientBasedCorrContrast	real/_	(GradientBasedCorrectionsLocalContrast)
// GradientBasedCorrContrast2012	real/_	(GradientBasedCorrectionsLocalContrast2012)
// GradientBasedCorrDefringe	real/_	(GradientBasedCorrectionsLocalDefringe)
// GradientBasedCorrDehaze	real/_	(GradientBasedCorrectionsLocalDehaze)
// GradientBasedCorrExposure	real/_	(GradientBasedCorrectionsLocalExposure)
// GradientBasedCorrExposure2012	real/_	(GradientBasedCorrectionsLocalExposure2012)
// GradientBasedCorrHighlights2012	real/_	(GradientBasedCorrectionsLocalHighlights2012)
// GradientBasedCorrHue	real/_	(GradientBasedCorrectionsLocalHue)
// GradientBasedCorrLuminanceNoise	real/_	(GradientBasedCorrectionsLocalLuminanceNoise)
// GradientBasedCorrMoire	real/_	(GradientBasedCorrectionsLocalMoire)
// GradientBasedCorrSaturation	real/_	(GradientBasedCorrectionsLocalSaturation)
// GradientBasedCorrShadows2012	real/_	(GradientBasedCorrectionsLocalShadows2012)
// GradientBasedCorrSharpness	real/_	(GradientBasedCorrectionsLocalSharpness)
// GradientBasedCorrTemperature	real/_	(GradientBasedCorrectionsLocalTemperature)
// GradientBasedCorrTexture	real/_	(GradientBasedCorrectionsLocalTexture)
// GradientBasedCorrTint	real/_	(GradientBasedCorrectionsLocalTint)
// GradientBasedCorrToningHue	real/_	(GradientBasedCorrectionsLocalToningHue)
// GradientBasedCorrToningSaturation	real/_	(GradientBasedCorrectionsLocalToningSaturation)
// GradientBasedCorrWhites2012	real/_	(GradientBasedCorrectionsLocalWhites2012)
// GradientBasedCorrWhat	string/_	(GradientBasedCorrectionsWhat)
// GrainAmount	integer/
// GrainFrequency	integer/	 
// GrainSeed	integer/	 
// GrainSize	integer/	 
// GrayMixerAqua	integer/	 
// GrayMixerBlue	integer/	 
// GrayMixerGreen	integer/	 
// GrayMixerMagenta	integer/	 
// GrayMixerOrange	integer/
// GrayMixerPurple	integer/	 
// GrayMixerRed	integer/	 
// GrayMixerYellow	integer/	 
// GreenHue	integer/	 
// GreenSaturation	integer/
// Group	lang-alt/	 
// HasCrop	boolean/	 
// HasSettings	boolean/	 
// HDREditMode	integer/
// HDRMaxValue	real/	 
// Highlight2012	integer/	 
// HighlightRecovery	integer/	 
// Highlights2012	integer/	 
// HueAdjustmentAqua	integer/
// HueAdjustmentBlue	integer/	 
// HueAdjustmentGreen	integer/	 
// HueAdjustmentMagenta	integer/	 
// HueAdjustmentOrange	integer/
// HueAdjustmentPurple	integer/	 
// HueAdjustmentRed	integer/	 
// HueAdjustmentYellow	integer/	 
// IncrementalTemperature	integer/
// IncrementalTint	integer/	 
// JPEGHandling	string/	 
// LensBlur	struct	--> LensBlur Struct
// LensBlurActive	boolean/_	 
// LensBlurAmount	real/_	(LensBlurBlurAmount)
// LensBlurBokehAspect	real/_	 
// LensBlurBokehRotation	real/_	 
// LensBlurBokehShape	real/_	 
// LensBlurBokehShapeDetail	real/_	 
// LensBlurCatEyeAmount	real/_
// LensBlurCatEyeScale	real/_	 
// LensBlurFocalRange	string/_	 
// LensBlurFocalRangeSource	real/_	 
// LensBlurHighlightsBoost	real/_	 
// LensBlurHighlightsThreshold	real/_	 
// LensBlurSampledArea	string/_	 
// LensBlurSampledRange	string/_
// LensBlurSphericalAberration	real/_	 
// LensBlurSubjectRange	string/_	 
// LensBlurVersion	string/_	 
// LensManualDistortionAmount	integer/
// LensProfileChromaticAberrationScale	integer/	 
// LensProfileDigest	string/	 
// LensProfileDistortionScale	integer/	 
// LensProfileEnable	integer/	 
// LensProfileFilename	string/
// LensProfileIsEmbedded	boolean/	 
// LensProfileMatchKeyCameraModelName	string/	 
// LensProfileMatchKeyExifMake	string/	 
// LensProfileMatchKeyExifModel	string/	 
// LensProfileMatchKeyIsRaw	boolean/
// LensProfileMatchKeyLensID	string/	 
// LensProfileMatchKeyLensInfo	string/	 
// LensProfileMatchKeyLensName	string/	 
// LensProfileMatchKeySensorFormatFactor	real/
// LensProfileName	string/	 
// LensProfileSetup	string/	 
// LensProfileVignettingScale	integer/	 
// Look	struct	--> Look Struct
// LookAmount	string/_	 
// LookCluster	string/_	 
// LookCopyright	string/_	 
// LookGroup	lang-alt/_	 
// LookName	string/	(NOT a flattened tag!)
// LookParameters	struct_	--> LookParms Struct
// LookParametersCameraProfile	string/_	 
// LookParametersClarity2012	string/_	 
// LookParametersConvertToGrayscale	string/_
// LookParametersHighlights2012	string/_	 
// LookParametersLookTable	string/_	 
// LookParametersProcessVersion	string/_	 
// LookParametersShadows2012	string/_	 
// LookParametersToneCurvePV2012	string/_+
// LookParametersToneCurvePV2012Blue	string/_+	 
// LookParametersToneCurvePV2012Green	string/_+	 
// LookParametersToneCurvePV2012Red	string/_+	 
// LookParametersVersion	string/_	 
// LookSupportsAmount	string/_	 
// LookSupportsMonochrome	string/_	 
// LookSupportsOutputReferred	string/_	 
// LookUUID	string/_
// LuminanceAdjustmentAqua	integer/	 
// LuminanceAdjustmentBlue	integer/	 
// LuminanceAdjustmentGreen	integer/	 
// LuminanceAdjustmentMagenta	integer/	 
// LuminanceAdjustmentOrange	integer/
// LuminanceAdjustmentPurple	integer/	 
// LuminanceAdjustmentRed	integer/	 
// LuminanceAdjustmentYellow	integer/	 
// LuminanceNoiseReductionContrast	integer/	 
// LuminanceNoiseReductionDetail	integer/
// LuminanceSmoothing	integer/	 
// MaskGroupBasedCorrections	struct+	--> Correction Struct
// MaskGroupBasedCorrActive	boolean/_	(MaskGroupBasedCorrectionsCorrectionActive)
// MaskGroupBasedCorrAmount	real/_	(MaskGroupBasedCorrectionsCorrectionAmount)
// MaskGroupBasedCorrMask	struct_+	--> CorrectionMask Struct
// (MaskGroupBasedCorrectionsCorrectionMasks)
// MaskGroupBasedCorrMaskAlpha	real/_	(MaskGroupBasedCorrectionsCorrectionMasksAlpha)
// MaskGroupBasedCorrMaskAngle	real/_	(MaskGroupBasedCorrectionsCorrectionMasksAngle)
// MaskGroupBasedCorrMaskBottom	real/_	(MaskGroupBasedCorrectionsCorrectionMasksBottom)
// MaskGroupBasedCorrMaskCenterValue	real/_	(MaskGroupBasedCorrectionsCorrectionMasksCenterValue)
// MaskGroupBasedCorrMaskCenterWeight	real/_	(MaskGroupBasedCorrectionsCorrectionMasksCenterWeight)
// MaskGroupBasedCorrMaskRange	struct_+	--> CorrRangeMask Struct
// (MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMask; called CorrectionRangeMask by the spec)
// MaskGroupBasedCorrMaskRangeAreaModels	struct_+	--> AreaModels Struct
// (MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModels)
// MaskGroupBasedCorrMaskRangeAreaModelsComponents	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsAreaComponents)
// MaskGroupBasedCorrMaskRangeAreaModelsColorSampleInfo	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// MaskGroupBasedCorrMaskRangeColorAmount	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskColorAmount)
// MaskGroupBasedCorrMaskRangeDepthFeather	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthFeather)
// MaskGroupBasedCorrMaskRangeDepthMax	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMax)
// MaskGroupBasedCorrMaskRangeDepthMin	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMin)
// MaskGroupBasedCorrMaskRangeInvert	boolean/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskInvert)
// MaskGroupBasedCorrMaskRangeLumFeather	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumFeather)
// MaskGroupBasedCorrMaskRangeLuminanceDepthSampleInfo	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskLuminanceDepthSampleInfo)
// MaskGroupBasedCorrMaskRangeLumMax	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMax)
// MaskGroupBasedCorrMaskRangeLumMin	real/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMin)
// MaskGroupBasedCorrMaskRangeLumRange	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumRange)
// MaskGroupBasedCorrMaskRangeSampleType	integer/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskSampleType)
// MaskGroupBasedCorrMaskRangeType	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskType)
// MaskGroupBasedCorrMaskRangeVersion	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksCorrectionRangeMaskVersion)
// MaskGroupBasedCorrMaskDabs	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksDabs)
// MaskGroupBasedCorrMaskFeather	real/_	(MaskGroupBasedCorrectionsCorrectionMasksFeather)
// MaskGroupBasedCorrMaskFlipped	boolean/_	(MaskGroupBasedCorrectionsCorrectionMasksFlipped)
// MaskGroupBasedCorrMaskFlow	real/_	(MaskGroupBasedCorrectionsCorrectionMasksFlow)
// MaskGroupBasedCorrMaskFullX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksFullX)
// MaskGroupBasedCorrMaskFullY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksFullY)
// MaskGroupBasedCorrMaskInputDigest	string/_	(MaskGroupBasedCorrectionsCorrectionMasksInputDigest)
// MaskGroupBasedCorrMaskLeft	real/_	(MaskGroupBasedCorrectionsCorrectionMasksLeft)
// MaskGroupBasedCorrMaskMaskActive	boolean/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskActive)
// MaskGroupBasedCorrMaskMaskBlendMode	integer/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskBlendMode)
// MaskGroupBasedCorrMaskMaskDigest	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskDigest)
// MaskGroupBasedCorrMaskMaskInverted	boolean/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskInverted)
// MaskGroupBasedCorrMaskMaskName	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskName)
// MaskGroupBasedCorrMaskMasks	struct_+	--> CorrectionMask Struct
// (MaskGroupBasedCorrectionsCorrectionMasksMasks)
// MaskGroupBasedCorrMaskMasksAlpha	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksAlpha)
// MaskGroupBasedCorrMaskMasksAngle	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksAngle)
// MaskGroupBasedCorrMaskMasksBottom	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksBottom)
// MaskGroupBasedCorrMaskMasksCenterValue	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksCenterValue)
// MaskGroupBasedCorrMaskMasksCenterWeight	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksCenterWeight)
// MaskGroupBasedCorrMaskMasksDabs	string/_+	(MaskGroupBasedCorrectionsCorrectionMasksMasksDabs)
// MaskGroupBasedCorrMaskMasksFeather	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksFeather)
// MaskGroupBasedCorrMaskMasksFlipped	boolean/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksFlipped)
// MaskGroupBasedCorrMaskMasksFlow	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksFlow)
// MaskGroupBasedCorrMaskMasksFullX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksFullX)
// MaskGroupBasedCorrMaskMasksFullY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksFullY)
// MaskGroupBasedCorrMaskMasksInputDigest	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksInputDigest)
// MaskGroupBasedCorrMaskMasksLeft	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksLeft)
// MaskGroupBasedCorrMaskMasksMaskActive	boolean/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskActive)
// MaskGroupBasedCorrMaskMasksMaskBlendMode	integer/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskBlendMode)
// MaskGroupBasedCorrMaskMasksMaskDigest	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskDigest)
// MaskGroupBasedCorrMaskMasksMaskInverted	boolean/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskInverted)
// MaskGroupBasedCorrMaskMasksMaskName	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskName)
// MaskGroupBasedCorrMaskMasksMaskSubType	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskSubType)
// MaskGroupBasedCorrMaskMasksMaskSyncID	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskSyncID)
// MaskGroupBasedCorrMaskMasksValue	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskValue)
// MaskGroupBasedCorrMaskMasksMaskVersion	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMaskVersion)
// MaskGroupBasedCorrMaskMasksMidpoint	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksMidpoint)
// MaskGroupBasedCorrMaskMasksOrigin	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksOrigin)
// MaskGroupBasedCorrMaskMasksPerimeterValue	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksPerimeterValue)
// MaskGroupBasedCorrMaskMasksRadius	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksRadius)
// MaskGroupBasedCorrMaskMasksReferencePoint	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksReferencePoint)
// MaskGroupBasedCorrMaskMasksRight	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksRight)
// MaskGroupBasedCorrMaskMasksRoundness	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksRoundness)
// MaskGroupBasedCorrMaskMasksSizeX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksSizeX)
// MaskGroupBasedCorrMaskMasksSizeY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksSizeY)
// MaskGroupBasedCorrMaskMasksTop	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksTop)
// MaskGroupBasedCorrMaskMaskSubType	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskSubType)
// MaskGroupBasedCorrMaskMasksVersion	integer/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksVersion)
// MaskGroupBasedCorrMaskMasksWhat	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksWhat)
// MaskGroupBasedCorrMaskMasksWholeImageArea	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksWholeImageArea)
// MaskGroupBasedCorrMaskMasksX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksX)
// MaskGroupBasedCorrMaskMasksY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksY)
// MaskGroupBasedCorrMaskMaskSyncID	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskSyncID)
// MaskGroupBasedCorrMaskMasksZeroX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksZeroX)
// MaskGroupBasedCorrMaskMasksZeroY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMasksZeroY)
// MaskGroupBasedCorrMaskValue	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskValue)
// MaskGroupBasedCorrMaskMaskVersion	string/_	(MaskGroupBasedCorrectionsCorrectionMasksMaskVersion)
// MaskGroupBasedCorrMaskMidpoint	real/_	(MaskGroupBasedCorrectionsCorrectionMasksMidpoint)
// MaskGroupBasedCorrMaskOrigin	string/_	(MaskGroupBasedCorrectionsCorrectionMasksOrigin)
// MaskGroupBasedCorrMaskPerimeterValue	real/_	(MaskGroupBasedCorrectionsCorrectionMasksPerimeterValue)
// MaskGroupBasedCorrMaskRadius	real/_	(MaskGroupBasedCorrectionsCorrectionMasksRadius)
// MaskGroupBasedCorrMaskReferencePoint	string/_	(MaskGroupBasedCorrectionsCorrectionMasksReferencePoint)
// MaskGroupBasedCorrMaskRight	real/_	(MaskGroupBasedCorrectionsCorrectionMasksRight)
// MaskGroupBasedCorrMaskRoundness	real/_	(MaskGroupBasedCorrectionsCorrectionMasksRoundness)
// MaskGroupBasedCorrMaskSizeX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksSizeX)
// MaskGroupBasedCorrMaskSizeY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksSizeY)
// MaskGroupBasedCorrMaskTop	real/_	(MaskGroupBasedCorrectionsCorrectionMasksTop)
// MaskGroupBasedCorrMaskVersion	integer/_	(MaskGroupBasedCorrectionsCorrectionMasksVersion)
// MaskGroupBasedCorrMaskWhat	string/_	(MaskGroupBasedCorrectionsCorrectionMasksWhat)
// MaskGroupBasedCorrMaskWholeImageArea	string/_	(MaskGroupBasedCorrectionsCorrectionMasksWholeImageArea)
// MaskGroupBasedCorrMaskX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksX)
// MaskGroupBasedCorrMaskY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksY)
// MaskGroupBasedCorrMaskZeroX	real/_	(MaskGroupBasedCorrectionsCorrectionMasksZeroX)
// MaskGroupBasedCorrMaskZeroY	real/_	(MaskGroupBasedCorrectionsCorrectionMasksZeroY)
// MaskGroupBasedCorrCorrectionName	string/_+	(MaskGroupBasedCorrectionsCorrectionName)
// MaskGroupBasedCorrRangeMask	struct_+	--> CorrRangeMask Struct
// (MaskGroupBasedCorrectionsCorrectionRangeMask; called CorrectionRangeMask by the spec)
// MaskGroupBasedCorrRangeMaskAreaModels	struct_+	--> AreaModels Struct
// (MaskGroupBasedCorrectionsCorrectionRangeMaskAreaModels)
// MaskGroupBasedCorrRangeMaskAreaModelsComponents	string/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskAreaModelsAreaComponents)
// MaskGroupBasedCorrRangeMaskAreaModelsColorSampleInfo	string/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// MaskGroupBasedCorrRangeMaskColorAmount	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskColorAmount)
// MaskGroupBasedCorrRangeMaskDepthFeather	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskDepthFeather)
// MaskGroupBasedCorrRangeMaskDepthMax	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskDepthMax)
// MaskGroupBasedCorrRangeMaskDepthMin	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskDepthMin)
// MaskGroupBasedCorrRangeMaskInvert	boolean/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskInvert)
// MaskGroupBasedCorrRangeMaskLumFeather	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskLumFeather)
// MaskGroupBasedCorrRangeMaskLuminanceDepthSampleInfo	string/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskLuminanceDepthSampleInfo)
// MaskGroupBasedCorrRangeMaskLumMax	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskLumMax)
// MaskGroupBasedCorrRangeMaskLumMin	real/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskLumMin)
// MaskGroupBasedCorrRangeMaskLumRange	string/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskLumRange)
// MaskGroupBasedCorrRangeMaskSampleType	integer/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskSampleType)
// MaskGroupBasedCorrRangeMaskType	string/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskType)
// MaskGroupBasedCorrRangeMaskVersion	string/_+	(MaskGroupBasedCorrectionsCorrectionRangeMaskVersion)
// MaskGroupBasedCorrCorrectionSyncID	string/_+	(MaskGroupBasedCorrectionsCorrectionSyncID)
// MaskGroupBasedCorrBlacks2012	real/_	(MaskGroupBasedCorrectionsLocalBlacks2012)
// MaskGroupBasedCorrBrightness	real/_	(MaskGroupBasedCorrectionsLocalBrightness)
// MaskGroupBasedCorrClarity	real/_	(MaskGroupBasedCorrectionsLocalClarity)
// MaskGroupBasedCorrClarity2012	real/_	(MaskGroupBasedCorrectionsLocalClarity2012)
// MaskGroupBasedCorrContrast	real/_	(MaskGroupBasedCorrectionsLocalContrast)
// MaskGroupBasedCorrContrast2012	real/_	(MaskGroupBasedCorrectionsLocalContrast2012)
// MaskGroupBasedCorrDefringe	real/_	(MaskGroupBasedCorrectionsLocalDefringe)
// MaskGroupBasedCorrDehaze	real/_	(MaskGroupBasedCorrectionsLocalDehaze)
// MaskGroupBasedCorrExposure	real/_	(MaskGroupBasedCorrectionsLocalExposure)
// MaskGroupBasedCorrExposure2012	real/_	(MaskGroupBasedCorrectionsLocalExposure2012)
// MaskGroupBasedCorrHighlights2012	real/_	(MaskGroupBasedCorrectionsLocalHighlights2012)
// MaskGroupBasedCorrHue	real/_	(MaskGroupBasedCorrectionsLocalHue)
// MaskGroupBasedCorrLuminanceNoise	real/_	(MaskGroupBasedCorrectionsLocalLuminanceNoise)
// MaskGroupBasedCorrMoire	real/_	(MaskGroupBasedCorrectionsLocalMoire)
// MaskGroupBasedCorrSaturation	real/_	(MaskGroupBasedCorrectionsLocalSaturation)
// MaskGroupBasedCorrShadows2012	real/_	(MaskGroupBasedCorrectionsLocalShadows2012)
// MaskGroupBasedCorrSharpness	real/_	(MaskGroupBasedCorrectionsLocalSharpness)
// MaskGroupBasedCorrTemperature	real/_	(MaskGroupBasedCorrectionsLocalTemperature)
// MaskGroupBasedCorrTexture	real/_	(MaskGroupBasedCorrectionsLocalTexture)
// MaskGroupBasedCorrTint	real/_	(MaskGroupBasedCorrectionsLocalTint)
// MaskGroupBasedCorrToningHue	real/_	(MaskGroupBasedCorrectionsLocalToningHue)
// MaskGroupBasedCorrToningSaturation	real/_	(MaskGroupBasedCorrectionsLocalToningSaturation)
// MaskGroupBasedCorrWhites2012	real/_	(MaskGroupBasedCorrectionsLocalWhites2012)
// MaskGroupBasedCorrWhat	string/_	(MaskGroupBasedCorrectionsWhat)
// MoireFilter	string/	'Off' = Off
// 'On' = On
// Name	lang-alt/	 
// NegativeCacheLargePreviewSize	integer/	 
// NegativeCacheMaximumSize	real/	 
// NegativeCachePath	string/
// OverrideLookVignette	boolean/	 
// PaintBasedCorrections	struct+	--> Correction Struct
// PaintCorrectionActive	boolean/_	(PaintBasedCorrectionsCorrectionActive)
// PaintCorrectionAmount	real/_	(PaintBasedCorrectionsCorrectionAmount)
// PaintBasedCorrectionMasks	struct_+	--> CorrectionMask Struct
// (PaintBasedCorrectionsCorrectionMasks)
// PaintCorrectionMaskAlpha	real/_	(PaintBasedCorrectionsCorrectionMasksAlpha)
// PaintCorrectionMaskAngle	real/_	(PaintBasedCorrectionsCorrectionMasksAngle)
// PaintCorrectionMaskBottom	real/_	(PaintBasedCorrectionsCorrectionMasksBottom)
// PaintCorrectionMaskCenterValue	real/_	(PaintBasedCorrectionsCorrectionMasksCenterValue)
// PaintCorrectionMaskCenterWeight	real/_	(PaintBasedCorrectionsCorrectionMasksCenterWeight)
// PaintCorrectionMaskRange	struct_+	--> CorrRangeMask Struct
// (PaintBasedCorrectionsCorrectionMasksCorrectionRangeMask; called CorrectionRangeMask by the spec)
// PaintCorrectionMaskRangeAreaModels	struct_+	--> AreaModels Struct
// (PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModels)
// PaintCorrectionMaskRangeAreaModelsComponents	string/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsAreaComponents)
// PaintCorrectionMaskRangeAreaModelsColorSampleInfo	string/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// PaintCorrectionMaskRangeColorAmount	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskColorAmount)
// PaintCorrectionMaskRangeDepthFeather	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthFeather)
// PaintCorrectionMaskRangeDepthMax	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMax)
// PaintCorrectionMaskRangeDepthMin	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskDepthMin)
// PaintCorrectionMaskRangeInvert	boolean/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskInvert)
// PaintCorrectionMaskRangeLumFeather	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumFeather)
// PaintCorrectionMaskRangeLuminanceDepthSampleInfo	string/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskLuminanceDepthSampleInfo)
// PaintCorrectionMaskRangeLumMax	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMax)
// PaintCorrectionMaskRangeLumMin	real/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumMin)
// PaintCorrectionMaskRangeLumRange	string/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskLumRange)
// PaintCorrectionMaskRangeSampleType	integer/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskSampleType)
// PaintCorrectionMaskRangeType	string/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskType)
// PaintCorrectionMaskRangeVersion	string/_+	(PaintBasedCorrectionsCorrectionMasksCorrectionRangeMaskVersion)
// PaintCorrectionMaskDabs	string/_	(PaintBasedCorrectionsCorrectionMasksDabs)
// PaintCorrectionMaskFeather	real/_	(PaintBasedCorrectionsCorrectionMasksFeather)
// PaintCorrectionMaskFlipped	boolean/_	(PaintBasedCorrectionsCorrectionMasksFlipped)
// PaintCorrectionMaskFlow	real/_	(PaintBasedCorrectionsCorrectionMasksFlow)
// PaintCorrectionMaskFullX	real/_	(PaintBasedCorrectionsCorrectionMasksFullX)
// PaintCorrectionMaskFullY	real/_	(PaintBasedCorrectionsCorrectionMasksFullY)
// PaintCorrectionMaskInputDigest	string/_	(PaintBasedCorrectionsCorrectionMasksInputDigest)
// PaintCorrectionMaskLeft	real/_	(PaintBasedCorrectionsCorrectionMasksLeft)
// PaintCorrectionMaskMaskActive	boolean/_	(PaintBasedCorrectionsCorrectionMasksMaskActive)
// PaintCorrectionMaskMaskBlendMode	integer/_	(PaintBasedCorrectionsCorrectionMasksMaskBlendMode)
// PaintCorrectionMaskMaskDigest	string/_	(PaintBasedCorrectionsCorrectionMasksMaskDigest)
// PaintCorrectionMaskMaskInverted	boolean/_	(PaintBasedCorrectionsCorrectionMasksMaskInverted)
// PaintCorrectionMaskMaskName	string/_	(PaintBasedCorrectionsCorrectionMasksMaskName)
// PaintCorrectionMaskMasks	struct_+	--> CorrectionMask Struct
// (PaintBasedCorrectionsCorrectionMasksMasks)
// PaintCorrectionMaskMasksAlpha	real/_	(PaintBasedCorrectionsCorrectionMasksMasksAlpha)
// PaintCorrectionMaskMasksAngle	real/_	(PaintBasedCorrectionsCorrectionMasksMasksAngle)
// PaintCorrectionMaskMasksBottom	real/_	(PaintBasedCorrectionsCorrectionMasksMasksBottom)
// PaintCorrectionMaskMasksCenterValue	real/_	(PaintBasedCorrectionsCorrectionMasksMasksCenterValue)
// PaintCorrectionMaskMasksCenterWeight	real/_	(PaintBasedCorrectionsCorrectionMasksMasksCenterWeight)
// PaintCorrectionMaskMasksDabs	string/_+	(PaintBasedCorrectionsCorrectionMasksMasksDabs)
// PaintCorrectionMaskMasksFeather	real/_	(PaintBasedCorrectionsCorrectionMasksMasksFeather)
// PaintCorrectionMaskMasksFlipped	boolean/_	(PaintBasedCorrectionsCorrectionMasksMasksFlipped)
// PaintCorrectionMaskMasksFlow	real/_	(PaintBasedCorrectionsCorrectionMasksMasksFlow)
// PaintCorrectionMaskMasksFullX	real/_	(PaintBasedCorrectionsCorrectionMasksMasksFullX)
// PaintCorrectionMaskMasksFullY	real/_	(PaintBasedCorrectionsCorrectionMasksMasksFullY)
// PaintCorrectionMaskMasksInputDigest	string/_	(PaintBasedCorrectionsCorrectionMasksMasksInputDigest)
// PaintCorrectionMaskMasksLeft	real/_	(PaintBasedCorrectionsCorrectionMasksMasksLeft)
// PaintCorrectionMaskMasksMaskActive	boolean/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskActive)
// PaintCorrectionMaskMasksMaskBlendMode	integer/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskBlendMode)
// PaintCorrectionMaskMasksMaskDigest	string/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskDigest)
// PaintCorrectionMaskMasksMaskInverted	boolean/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskInverted)
// PaintCorrectionMaskMasksMaskName	string/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskName)
// PaintCorrectionMaskMasksMaskSubType	string/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskSubType)
// PaintCorrectionMaskMasksMaskSyncID	string/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskSyncID)
// PaintCorrectionMaskMasksValue	real/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskValue)
// PaintCorrectionMaskMasksMaskVersion	string/_	(PaintBasedCorrectionsCorrectionMasksMasksMaskVersion)
// PaintCorrectionMaskMasksMidpoint	real/_	(PaintBasedCorrectionsCorrectionMasksMasksMidpoint)
// PaintCorrectionMaskMasksOrigin	string/_	(PaintBasedCorrectionsCorrectionMasksMasksOrigin)
// PaintCorrectionMaskMasksPerimeterValue	real/_	(PaintBasedCorrectionsCorrectionMasksMasksPerimeterValue)
// PaintCorrectionMaskMasksRadius	real/_	(PaintBasedCorrectionsCorrectionMasksMasksRadius)
// PaintCorrectionMaskMasksReferencePoint	string/_	(PaintBasedCorrectionsCorrectionMasksMasksReferencePoint)
// PaintCorrectionMaskMasksRight	real/_	(PaintBasedCorrectionsCorrectionMasksMasksRight)
// PaintCorrectionMaskMasksRoundness	real/_	(PaintBasedCorrectionsCorrectionMasksMasksRoundness)
// PaintCorrectionMaskMasksSizeX	real/_	(PaintBasedCorrectionsCorrectionMasksMasksSizeX)
// PaintCorrectionMaskMasksSizeY	real/_	(PaintBasedCorrectionsCorrectionMasksMasksSizeY)
// PaintCorrectionMaskMasksTop	real/_	(PaintBasedCorrectionsCorrectionMasksMasksTop)
// PaintCorrectionMaskMaskSubType	string/_	(PaintBasedCorrectionsCorrectionMasksMaskSubType)
// PaintCorrectionMaskMasksVersion	integer/_	(PaintBasedCorrectionsCorrectionMasksMasksVersion)
// PaintCorrectionMaskMasksWhat	string/_	(PaintBasedCorrectionsCorrectionMasksMasksWhat)
// PaintCorrectionMaskMasksWholeImageArea	string/_	(PaintBasedCorrectionsCorrectionMasksMasksWholeImageArea)
// PaintCorrectionMaskMasksX	real/_	(PaintBasedCorrectionsCorrectionMasksMasksX)
// PaintCorrectionMaskMasksY	real/_	(PaintBasedCorrectionsCorrectionMasksMasksY)
// PaintCorrectionMaskMaskSyncID	string/_	(PaintBasedCorrectionsCorrectionMasksMaskSyncID)
// PaintCorrectionMaskMasksZeroX	real/_	(PaintBasedCorrectionsCorrectionMasksMasksZeroX)
// PaintCorrectionMaskMasksZeroY	real/_	(PaintBasedCorrectionsCorrectionMasksMasksZeroY)
// PaintCorrectionMaskValue	real/_	(PaintBasedCorrectionsCorrectionMasksMaskValue)
// PaintCorrectionMaskMaskVersion	string/_	(PaintBasedCorrectionsCorrectionMasksMaskVersion)
// PaintCorrectionMaskMidpoint	real/_	(PaintBasedCorrectionsCorrectionMasksMidpoint)
// PaintCorrectionMaskOrigin	string/_	(PaintBasedCorrectionsCorrectionMasksOrigin)
// PaintCorrectionMaskPerimeterValue	real/_	(PaintBasedCorrectionsCorrectionMasksPerimeterValue)
// PaintCorrectionMaskRadius	real/_	(PaintBasedCorrectionsCorrectionMasksRadius)
// PaintCorrectionMaskReferencePoint	string/_	(PaintBasedCorrectionsCorrectionMasksReferencePoint)
// PaintCorrectionMaskRight	real/_	(PaintBasedCorrectionsCorrectionMasksRight)
// PaintCorrectionMaskRoundness	real/_	(PaintBasedCorrectionsCorrectionMasksRoundness)
// PaintCorrectionMaskSizeX	real/_	(PaintBasedCorrectionsCorrectionMasksSizeX)
// PaintCorrectionMaskSizeY	real/_	(PaintBasedCorrectionsCorrectionMasksSizeY)
// PaintCorrectionMaskTop	real/_	(PaintBasedCorrectionsCorrectionMasksTop)
// PaintCorrectionMaskVersion	integer/_	(PaintBasedCorrectionsCorrectionMasksVersion)
// PaintCorrectionMaskWhat	string/_	(PaintBasedCorrectionsCorrectionMasksWhat)
// PaintCorrectionMaskWholeImageArea	string/_	(PaintBasedCorrectionsCorrectionMasksWholeImageArea)
// PaintCorrectionMaskX	real/_	(PaintBasedCorrectionsCorrectionMasksX)
// PaintCorrectionMaskY	real/_	(PaintBasedCorrectionsCorrectionMasksY)
// PaintCorrectionMaskZeroX	real/_	(PaintBasedCorrectionsCorrectionMasksZeroX)
// PaintCorrectionMaskZeroY	real/_	(PaintBasedCorrectionsCorrectionMasksZeroY)
// PaintCorrectionCorrectionName	string/_+	(PaintBasedCorrectionsCorrectionName)
// PaintCorrectionRangeMask	struct_+	--> CorrRangeMask Struct
// (PaintBasedCorrectionsCorrectionRangeMask; called CorrectionRangeMask by the spec)
// PaintCorrectionRangeMaskAreaModels	struct_+	--> AreaModels Struct
// (PaintBasedCorrectionsCorrectionRangeMaskAreaModels)
// PaintCorrectionRangeMaskAreaModelsComponents	string/_+	(PaintBasedCorrectionsCorrectionRangeMaskAreaModelsAreaComponents)
// PaintCorrectionRangeMaskAreaModelsColorSampleInfo	string/_+	(PaintBasedCorrectionsCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// PaintCorrectionRangeMaskColorAmount	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskColorAmount)
// PaintCorrectionRangeMaskDepthFeather	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskDepthFeather)
// PaintCorrectionRangeMaskDepthMax	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskDepthMax)
// PaintCorrectionRangeMaskDepthMin	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskDepthMin)
// PaintCorrectionRangeMaskInvert	boolean/_+	(PaintBasedCorrectionsCorrectionRangeMaskInvert)
// PaintCorrectionRangeMaskLumFeather	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskLumFeather)
// PaintCorrectionRangeMaskLuminanceDepthSampleInfo	string/_+	(PaintBasedCorrectionsCorrectionRangeMaskLuminanceDepthSampleInfo)
// PaintCorrectionRangeMaskLumMax	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskLumMax)
// PaintCorrectionRangeMaskLumMin	real/_+	(PaintBasedCorrectionsCorrectionRangeMaskLumMin)
// PaintCorrectionRangeMaskLumRange	string/_+	(PaintBasedCorrectionsCorrectionRangeMaskLumRange)
// PaintCorrectionRangeMaskSampleType	integer/_+	(PaintBasedCorrectionsCorrectionRangeMaskSampleType)
// PaintCorrectionRangeMaskType	string/_+	(PaintBasedCorrectionsCorrectionRangeMaskType)
// PaintCorrectionRangeMaskVersion	string/_+	(PaintBasedCorrectionsCorrectionRangeMaskVersion)
// PaintCorrectionCorrectionSyncID	string/_+	(PaintBasedCorrectionsCorrectionSyncID)
// PaintCorrectionBlacks2012	real/_	(PaintBasedCorrectionsLocalBlacks2012)
// PaintCorrectionBrightness	real/_	(PaintBasedCorrectionsLocalBrightness)
// PaintCorrectionClarity	real/_	(PaintBasedCorrectionsLocalClarity)
// PaintCorrectionClarity2012	real/_	(PaintBasedCorrectionsLocalClarity2012)
// PaintCorrectionContrast	real/_	(PaintBasedCorrectionsLocalContrast)
// PaintCorrectionContrast2012	real/_	(PaintBasedCorrectionsLocalContrast2012)
// PaintCorrectionDefringe	real/_	(PaintBasedCorrectionsLocalDefringe)
// PaintCorrectionDehaze	real/_	(PaintBasedCorrectionsLocalDehaze)
// PaintCorrectionExposure	real/_	(PaintBasedCorrectionsLocalExposure)
// PaintCorrectionExposure2012	real/_	(PaintBasedCorrectionsLocalExposure2012)
// PaintCorrectionHighlights2012	real/_	(PaintBasedCorrectionsLocalHighlights2012)
// PaintCorrectionHue	real/_	(PaintBasedCorrectionsLocalHue)
// PaintCorrectionLuminanceNoise	real/_	(PaintBasedCorrectionsLocalLuminanceNoise)
// PaintCorrectionMoire	real/_	(PaintBasedCorrectionsLocalMoire)
// PaintCorrectionSaturation	real/_	(PaintBasedCorrectionsLocalSaturation)
// PaintCorrectionShadows2012	real/_	(PaintBasedCorrectionsLocalShadows2012)
// PaintCorrectionSharpness	real/_	(PaintBasedCorrectionsLocalSharpness)
// PaintCorrectionTemperature	real/_	(PaintBasedCorrectionsLocalTemperature)
// PaintCorrectionTexture	real/_	(PaintBasedCorrectionsLocalTexture)
// PaintCorrectionTint	real/_	(PaintBasedCorrectionsLocalTint)
// PaintCorrectionToningHue	real/_	(PaintBasedCorrectionsLocalToningHue)
// PaintCorrectionToningSaturation	real/_	(PaintBasedCorrectionsLocalToningSaturation)
// PaintCorrectionWhites2012	real/_	(PaintBasedCorrectionsLocalWhites2012)
// PaintCorrectionWhat	string/_	(PaintBasedCorrectionsWhat)
// ParametricDarks	integer/	 
// ParametricHighlights	integer/	 
// ParametricHighlightSplit	integer/	 
// ParametricLights	integer/	 
// ParametricMidtoneSplit	integer/	 
// ParametricShadows	integer/
// ParametricShadowSplit	integer/	 
// PerspectiveAspect	integer/	 
// PerspectiveHorizontal	integer/
// PerspectiveRotate	real/	 
// PerspectiveScale	integer/	 
// PerspectiveUpright	integer/	
// 0 = Off
// 1 = Auto
// 2 = Full	  	3 = Level
// 4 = Vertical
// 5 = Guided
// PerspectiveVertical	integer/	 
// PerspectiveX	real/	 
// PerspectiveY	real/	 
// PointColors	string/+	 
// PostCropVignetteAmount	integer/
// PostCropVignetteFeather	integer/	 
// PostCropVignetteHighlightContrast	integer/	 
// PostCropVignetteMidpoint	integer/	 
// PostCropVignetteRoundness	integer/	 
// PostCropVignetteStyle	integer/	1 = Highlight Priority
// 2 = Color Priority
// 3 = Paint Overlay
// PresetType	string/
// ProcessVersion	string/	 
// RangeMask	struct	--> RangeMask Struct
// (tag ID is 'RangeMaskMapInfo')
// RangeMaskMapInfo	struct_	--> MapInfo Struct
// (RangeMaskMapInfoRangeMaskMapInfo)
// RangeMaskMapInfoLabMax	string/_	(RangeMaskMapInfoRangeMaskMapInfoLabMax)
// RangeMaskMapInfoLabMin	string/_	(RangeMaskMapInfoRangeMaskMapInfoLabMin)
// RangeMaskMapInfoLumEq	string/_+	(RangeMaskMapInfoRangeMaskMapInfoLumEq)
// RangeMaskMapInfoRGBMax	string/_	(RangeMaskMapInfoRangeMaskMapInfoRGBMax)
// RangeMaskMapInfoRGBMin	string/_	(RangeMaskMapInfoRangeMaskMapInfoRGBMin)
// RawFileName	string/	 
// RedEyeInfo	string/+	 
// RedHue	integer/	 
// RedSaturation	integer/
// RetouchAreas	struct+	--> RetouchArea Struct
// RetouchAreaFeather	real/_	(RetouchAreasFeather)
// RetouchAreaMasks	struct_+	--> CorrectionMask Struct
// (RetouchAreasMasks)
// RetouchAreaMaskAlpha	real/_	(RetouchAreasMasksAlpha)
// RetouchAreaMaskAngle	real/_	(RetouchAreasMasksAngle)
// RetouchAreaMaskBottom	real/_	(RetouchAreasMasksBottom)
// RetouchAreaMaskCenterValue	real/_	(RetouchAreasMasksCenterValue)
// RetouchAreaMaskCenterWeight	real/_	(RetouchAreasMasksCenterWeight)
// RetouchAreaMaskRange	struct_+	--> CorrRangeMask Struct
// (RetouchAreasMasksCorrectionRangeMask; called CorrectionRangeMask by the spec)
// RetouchAreaMaskRangeAreaModels	struct_+	--> AreaModels Struct
// (RetouchAreasMasksCorrectionRangeMaskAreaModels)
// RetouchAreaMaskRangeAreaModelsComponents	string/_+	(RetouchAreasMasksCorrectionRangeMaskAreaModelsAreaComponents)
// RetouchAreaMaskRangeAreaModelsColorSampleInfo	string/_+	(RetouchAreasMasksCorrectionRangeMaskAreaModelsColorRangeMaskAreaSampleInfo)
// RetouchAreaMaskRangeColorAmount	real/_+	(RetouchAreasMasksCorrectionRangeMaskColorAmount)
// RetouchAreaMaskRangeDepthFeather	real/_+	(RetouchAreasMasksCorrectionRangeMaskDepthFeather)
// RetouchAreaMaskRangeDepthMax	real/_+	(RetouchAreasMasksCorrectionRangeMaskDepthMax)
// RetouchAreaMaskRangeDepthMin	real/_+	(RetouchAreasMasksCorrectionRangeMaskDepthMin)
// RetouchAreaMaskRangeInvert	boolean/_+	(RetouchAreasMasksCorrectionRangeMaskInvert)
// RetouchAreaMaskRangeLumFeather	real/_+	(RetouchAreasMasksCorrectionRangeMaskLumFeather)
// RetouchAreaMaskRangeLuminanceDepthSampleInfo	string/_+	(RetouchAreasMasksCorrectionRangeMaskLuminanceDepthSampleInfo)
// RetouchAreaMaskRangeLumMax	real/_+	(RetouchAreasMasksCorrectionRangeMaskLumMax)
// RetouchAreaMaskRangeLumMin	real/_+	(RetouchAreasMasksCorrectionRangeMaskLumMin)
// RetouchAreaMaskRangeLumRange	string/_+	(RetouchAreasMasksCorrectionRangeMaskLumRange)
// RetouchAreaMaskRangeSampleType	integer/_+	(RetouchAreasMasksCorrectionRangeMaskSampleType)
// RetouchAreaMaskRangeType	string/_+	(RetouchAreasMasksCorrectionRangeMaskType)
// RetouchAreaMaskRangeVersion	string/_+	(RetouchAreasMasksCorrectionRangeMaskVersion)
// RetouchAreaMaskDabs	string/_	(RetouchAreasMasksDabs)
// RetouchAreaMaskFeather	real/_	(RetouchAreasMasksFeather)
// RetouchAreaMaskFlipped	boolean/_	(RetouchAreasMasksFlipped)
// RetouchAreaMaskFlow	real/_	(RetouchAreasMasksFlow)
// RetouchAreaMaskFullX	real/_	(RetouchAreasMasksFullX)
// RetouchAreaMaskFullY	real/_	(RetouchAreasMasksFullY)
// RetouchAreaMaskInputDigest	string/_	(RetouchAreasMasksInputDigest)
// RetouchAreaMaskLeft	real/_	(RetouchAreasMasksLeft)
// RetouchAreaMaskMaskActive	boolean/_	(RetouchAreasMasksMaskActive)
// RetouchAreaMaskMaskBlendMode	integer/_	(RetouchAreasMasksMaskBlendMode)
// RetouchAreaMaskMaskDigest	string/_	(RetouchAreasMasksMaskDigest)
// RetouchAreaMaskMaskInverted	boolean/_	(RetouchAreasMasksMaskInverted)
// RetouchAreaMaskMaskName	string/_	(RetouchAreasMasksMaskName)
// RetouchAreaMaskMasks	struct_+	--> CorrectionMask Struct
// (RetouchAreasMasksMasks)
// RetouchAreaMaskMasksAlpha	real/_	(RetouchAreasMasksMasksAlpha)
// RetouchAreaMaskMasksAngle	real/_	(RetouchAreasMasksMasksAngle)
// RetouchAreaMaskMasksBottom	real/_	(RetouchAreasMasksMasksBottom)
// RetouchAreaMaskMasksCenterValue	real/_	(RetouchAreasMasksMasksCenterValue)
// RetouchAreaMaskMasksCenterWeight	real/_	(RetouchAreasMasksMasksCenterWeight)
// RetouchAreaMaskMasksDabs	string/_+	(RetouchAreasMasksMasksDabs)
// RetouchAreaMaskMasksFeather	real/_	(RetouchAreasMasksMasksFeather)
// RetouchAreaMaskMasksFlipped	boolean/_	(RetouchAreasMasksMasksFlipped)
// RetouchAreaMaskMasksFlow	real/_	(RetouchAreasMasksMasksFlow)
// RetouchAreaMaskMasksFullX	real/_	(RetouchAreasMasksMasksFullX)
// RetouchAreaMaskMasksFullY	real/_	(RetouchAreasMasksMasksFullY)
// RetouchAreaMaskMasksInputDigest	string/_	(RetouchAreasMasksMasksInputDigest)
// RetouchAreaMaskMasksLeft	real/_	(RetouchAreasMasksMasksLeft)
// RetouchAreaMaskMasksMaskActive	boolean/_	(RetouchAreasMasksMasksMaskActive)
// RetouchAreaMaskMasksMaskBlendMode	integer/_	(RetouchAreasMasksMasksMaskBlendMode)
// RetouchAreaMaskMasksMaskDigest	string/_	(RetouchAreasMasksMasksMaskDigest)
// RetouchAreaMaskMasksMaskInverted	boolean/_	(RetouchAreasMasksMasksMaskInverted)
// RetouchAreaMaskMasksMaskName	string/_	(RetouchAreasMasksMasksMaskName)
// RetouchAreaMaskMasksMaskSubType	string/_	(RetouchAreasMasksMasksMaskSubType)
// RetouchAreaMaskMasksMaskSyncID	string/_	(RetouchAreasMasksMasksMaskSyncID)
// RetouchAreaMaskMasksValue	real/_	(RetouchAreasMasksMasksMaskValue)
// RetouchAreaMaskMasksMaskVersion	string/_	(RetouchAreasMasksMasksMaskVersion)
// RetouchAreaMaskMasksMidpoint	real/_	(RetouchAreasMasksMasksMidpoint)
// RetouchAreaMaskMasksOrigin	string/_	(RetouchAreasMasksMasksOrigin)
// RetouchAreaMaskMasksPerimeterValue	real/_	(RetouchAreasMasksMasksPerimeterValue)
// RetouchAreaMaskMasksRadius	real/_	(RetouchAreasMasksMasksRadius)
// RetouchAreaMaskMasksReferencePoint	string/_	(RetouchAreasMasksMasksReferencePoint)
// RetouchAreaMaskMasksRight	real/_	(RetouchAreasMasksMasksRight)
// RetouchAreaMaskMasksRoundness	real/_	(RetouchAreasMasksMasksRoundness)
// RetouchAreaMaskMasksSizeX	real/_	(RetouchAreasMasksMasksSizeX)
// RetouchAreaMaskMasksSizeY	real/_	(RetouchAreasMasksMasksSizeY)
// RetouchAreaMaskMasksTop	real/_	(RetouchAreasMasksMasksTop)
// RetouchAreaMaskMaskSubType	string/_	(RetouchAreasMasksMaskSubType)
// RetouchAreaMaskMasksVersion	integer/_	(RetouchAreasMasksMasksVersion)
// RetouchAreaMaskMasksWhat	string/_	(RetouchAreasMasksMasksWhat)
// RetouchAreaMaskMasksWholeImageArea	string/_	(RetouchAreasMasksMasksWholeImageArea)
// RetouchAreaMaskMasksX	real/_	(RetouchAreasMasksMasksX)
// RetouchAreaMaskMasksY	real/_	(RetouchAreasMasksMasksY)
// RetouchAreaMaskMaskSyncID	string/_	(RetouchAreasMasksMaskSyncID)
// RetouchAreaMaskMasksZeroX	real/_	(RetouchAreasMasksMasksZeroX)
// RetouchAreaMaskMasksZeroY	real/_	(RetouchAreasMasksMasksZeroY)
// RetouchAreaMaskValue	real/_	(RetouchAreasMasksMaskValue)
// RetouchAreaMaskMaskVersion	string/_	(RetouchAreasMasksMaskVersion)
// RetouchAreaMaskMidpoint	real/_	(RetouchAreasMasksMidpoint)
// RetouchAreaMaskOrigin	string/_	(RetouchAreasMasksOrigin)
// RetouchAreaMaskPerimeterValue	real/_	(RetouchAreasMasksPerimeterValue)
// RetouchAreaMaskRadius	real/_	(RetouchAreasMasksRadius)
// RetouchAreaMaskReferencePoint	string/_	(RetouchAreasMasksReferencePoint)
// RetouchAreaMaskRight	real/_	(RetouchAreasMasksRight)
// RetouchAreaMaskRoundness	real/_	(RetouchAreasMasksRoundness)
// RetouchAreaMaskSizeX	real/_	(RetouchAreasMasksSizeX)
// RetouchAreaMaskSizeY	real/_	(RetouchAreasMasksSizeY)
// RetouchAreaMaskTop	real/_	(RetouchAreasMasksTop)
// RetouchAreaMaskVersion	integer/_	(RetouchAreasMasksVersion)
// RetouchAreaMaskWhat	string/_	(RetouchAreasMasksWhat)
// RetouchAreaMaskWholeImageArea	string/_	(RetouchAreasMasksWholeImageArea)
// RetouchAreaMaskX	real/_	(RetouchAreasMasksX)
// RetouchAreaMaskY	real/_	(RetouchAreasMasksY)
// RetouchAreaMaskZeroX	real/_	(RetouchAreasMasksZeroX)
// RetouchAreaMaskZeroY	real/_	(RetouchAreasMasksZeroY)
// RetouchAreaMethod	string/_	(RetouchAreasMethod)
// RetouchAreaOffsetY	real/_	(RetouchAreasOffsetY)
// RetouchAreaOpacity	real/_	(RetouchAreasOpacity)
// RetouchAreaSeed	integer/_	(RetouchAreasSeed)
// RetouchAreaSourceState	string/_	(RetouchAreasSourceState)
// RetouchAreaSourceX	real/_	(RetouchAreasSourceX)
// RetouchAreaSpotType	string/_	(RetouchAreasSpotType)
// RetouchInfo	string/+	 
// Saturation	integer/	 
// SaturationAdjustmentAqua	integer/	 
// SaturationAdjustmentBlue	integer/
// SaturationAdjustmentGreen	integer/	 
// SaturationAdjustmentMagenta	integer/	 
// SaturationAdjustmentOrange	integer/	 
// SaturationAdjustmentPurple	integer/	 
// SaturationAdjustmentRed	integer/	 
// SaturationAdjustmentYellow	integer/	 
// SDRBlend	real/	 
// SDRBrightness	real/	 
// SDRContrast	real/	 
// SDRHighlights	real/	 
// SDRShadows	real/	 
// SDRWhites	real/
// Shadows	integer/	 
// Shadows2012	integer/	 
// ShadowTint	integer/	 
// SharpenDetail	integer/	 
// SharpenEdgeMasking	integer/	 
// SharpenRadius	real/	 
// Sharpness	integer/	 
// ShortName	lang-alt/
// Smoothness	integer/	 
// SortName	lang-alt/	 
// SplitToningBalance	integer/	(also used for newer ColorGrade settings)
// SplitToningHighlightHue	integer/	(also used for newer ColorGrade settings)
// SplitToningHighlightSaturation	integer/	(also used for newer ColorGrade settings)
// SplitToningShadowHue	integer/	(also used for newer ColorGrade settings)
// SplitToningShadowSaturation	integer/	(also used for newer ColorGrade settings)
// SupportsAmount	boolean/	 
// SupportsColor	boolean/	 
// SupportsHighDynamicRange	boolean/	 
// SupportsMonochrome	boolean/	 
// SupportsNormalDynamicRange	boolean/	 
// SupportsOutputReferred	boolean/
// SupportsSceneReferred	boolean/	 
// ColorTemperature	integer/	(tag ID is 'Temperature')
// Texture	integer/	 
// TIFFHandling	string/	 
// Tint	integer/	 
// ToggleStyleAmount	integer/	 
// ToggleStyleDigest	string/	 
// ToneCurve	string/+	 
// ToneCurveBlue	string/+
// ToneCurveGreen	string/+	 
// ToneCurveName	string/	'Custom' = Custom
// 'Linear' = Linear
// 'Medium Contrast' = Medium Contrast
// 'Strong Contrast' = Strong Contrast
// ToneCurveName2012	string/	 
// ToneCurvePV2012	string/+	 
// ToneCurvePV2012Blue	string/+	 
// ToneCurvePV2012Green	string/+	 
// ToneCurvePV2012Red	string/+	 
// ToneCurveRed	string/+	 
// ToneMapStrength	real/
// UprightCenterMode	integer/	 
// UprightCenterNormX	real/	 
// UprightCenterNormY	real/	 
// UprightDependentDigest	string/	 
// UprightFocalLength35mm	real/
// UprightFocalMode	integer/	 
// UprightFourSegments_0	string/	 
// UprightFourSegments_1	string/	 
// UprightFourSegments_2	string/
// UprightFourSegments_3	string/	 
// UprightFourSegmentsCount	integer/	 
// UprightGuidedDependentDigest	string/	 
// UprightPreview	boolean/
// UprightTransform_0	string/	 
// UprightTransform_1	string/	 
// UprightTransform_2	string/	 
// UprightTransform_3	string/	 
// UprightTransform_4	string/
// UprightTransform_5	string/	 
// UprightTransformCount	integer/	 
// UprightVersion	integer/	 
// UUID	string/
// Version	string/	 
// Vibrance	integer/	 
// VignetteAmount	integer/	 
// VignetteMidpoint	integer/	 
// What	string/
// WhiteBalance	string/
// 'As Shot' = As Shot
// 'Auto' = Auto
// 'Cloudy' = Cloudy
// 'Custom' = Custom
// 'Daylight' = Daylight
// 'Flash' = Flash
// 'Fluorescent' = Fluorescent
// 'Shade' = Shade
// 'Tungsten' = Tungsten
// Whites2012	integer/