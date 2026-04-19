pub struct GpsInfo {
    pub gps_version_id: [u8; 4], // 0x0000
    pub latitude_ref: Option<LatitudeRef>, // 0x0001
    pub latitude: [u64; 3], // 0x0002
    pub longitude_ref: Option<LongitudeRef>, // 0x0003
    pub longitude: [u64; 3], // 0x0004
    pub altitude_ref: Option<AltitudeRef>, // 0x0005
    pub altitude: Option<u64>, // 0x0006
    pub timestamp: [u64; 3], // 0x0007
    pub satellites: Option<String>, // 0x0008
    pub status: Option<Status>, // 0x0009
    pub measure_mode: Option<MeasureMode>, // 0x000a
    pub dop: Option<f64>, // 0x000b
    pub speed_ref: Option<SpeedRef>, // 0x000c
    pub speed: Option<f64>, // 0x000d
    pub track_ref: Option<TrackRef>, // 0x000e
    pub track: Option<f64>, // 0x000f
    pub img_direction_ref: Option<ImageDirectionRef>, // 0x0010
    pub img_direction: Option<f64>, // 0x0011
    pub map_datum: Option<String>, // 0x0012
    pub dest_latitude_ref: Option<LatitudeRef>, // 0x0013
    pub dest_latitude: Option<[u64; 3]>, // 0x0014
    pub dest_longitude_ref: Option<LongitudeRef>, // 0x0015
    pub dest_longitude: Option<[u64; 3]>, // 0x0016
    pub dest_bearing_ref: Option<TrackRef>, // 0x0017
    pub dest_bearing: Option<f64>, // 0x0018
    pub dest_distance_ref: Option<DistanceRef>, // 0x0019
    pub dest_distance: Option<f64>, // 0x001a
    pub processing_method: Option<ProcessingMethod>, // 0x001b
    pub area_information: Option<String>, // 0x001c
    pub date_stamp: Option<String>, // 0x001d
    pub differential: Option<Differential>, // 0x001e
    pub h_positioning_error: Option<f64>, // 0x001f
}

pub enum LatitudeRef {
    North,
    South,
    Unknown
}

// 'N' = North
// 'S' = South

pub enum LongitudeRef {
    East,
    West,
    Unknown
}

// 'E' = East
// 'W' = West

pub enum AltitudeRef {
    AboveSeaLevel = 0,
    BelowSeaLevel = 1,
    PositiveSeaLevel = 2,
    NegativeSeaLevel = 3
}

// 0 = Above Sea Level
// 1 = Below Sea Level
// 2 = Positive Sea Level (sea-level ref)
// 3 = Negative Sea Level (sea-level ref)

pub enum Status {
    MeasurementActive,
    MeasurementVoid
}

// 'A' = Measurement Active
// 'V' = Measurement Void

pub enum MeasureMode {
    TwoDimensionalMeasurement,
    ThreeDimensionalMeasurement
}

// 2 = 2-Dimensional Measurement
// 3 = 3-Dimensional Measurement

pub enum SpeedRef {
    KmH,
    MpH,
    Knots
}

// 'K' = km/h
// 'M' = mph
// 'N' = knots

pub enum TrackRef {
    MagneticNorth,
    TrueNorth
}

// 'M' = Magnetic North
// 'T' = True North

pub enum ImageDirectionRef {
    MagneticNorth,
    TrueNorth
}

// 'M' = Magnetic North
// 'T' = True North

pub enum DistanceRef {
    Kilometeres,
    Miles,
    NauticalMiles,
    Unknown
}
// 'K' = Kilometers
// 'M' = Miles
// 'N' = Nautical Miles

pub enum Differential {
    NoCorrection = 0,
    DifferentialCorrected = 1,
}

// 0 = No Correction
// 1 = Differential Corrected

pub enum ProcessingMethod {
    Gps,
    CellId,
    Wlan,
    Manual
}

// "GPS", "CELLID", "WLAN" or "MANUAL"