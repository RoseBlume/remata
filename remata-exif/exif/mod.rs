
mod helpers;
mod tags;

use helpers::{read_string, read_rational};

pub use tags::{
    FlashMode,
    MeteringMode,
    Contrast,
    LightSource,
    ExposureProgram,
    Saturation,
    Sharpness,
    WhiteBalance,
    SubjectDistanceRange,
    PhotoMetricInterpretation,
    SceneCaptureType,
    Compression
};

use std::time::Duration;
use crate::ParserError;


pub struct Exif {
    pub make: Option<String>, // 0x010f
    pub model: Option<String>, // 0x0110	
    pub compression: Option<Compression>, // 0x0103 // New
    pub camera_model: Option<String>, // 0xc614	UniqueCameraModel or use prefered 0xc615	LocalizedCameraModel
    pub camera_serial_number: Option<String>, // 0xc62f
    pub exposure_time: Option<Duration>, // 0x829a comes from a u64 
    pub iso_speed: Option<u16>, // 0x8827
    pub exposure_bias: Option<i16>, // 0x9204 values from -99.99 to 99.99
    pub focal_length: Option<u64>, // 0x920a
    pub max_aperture: Option<u64>, // 0x9205
    pub metering_mode: Option<MeteringMode>, // 0x9207
    pub subject_distance: Option<SubjectDistanceRange>, // 0xa40c
    pub flash_mode: Option<FlashMode>, // 0x9209
    pub focal_length_35mm: Option<u16>, // 0xa405
    pub lens_make: Option<String>, // 0xa433
    pub lens_model: Option<String>, // 0xa434
    pub lens_serial: Option<String>, // 0xa435
    pub contrast: Option<Contrast>, // 0xfe54
    pub brightness: Option<u64>, // 0x9203	
    pub light_source: Option<LightSource>, // 0x9208
    pub exposure_program: Option<ExposureProgram>, // 0x8822
    pub saturation: Option<Saturation>, // 0xfe55
    pub sharpness: Option<Sharpness>, // 0xfe56	
    pub white_balance: Option<WhiteBalance>, // 0xa403
    pub photometric_iterpretation: Option<PhotoMetricInterpretation>, // 0x0106
    pub scene_capture_type: Option<SceneCaptureType>, //0xa406
    pub digital_zoom: Option<u64>, // 0xa404	
    pub exif_version: Option<ExifVersion>
}

impl Exif {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        if data.len() < 8 {
            return Err(ParserError::new("Invalid Format"));
        }

        let le = match &data[0..2] {
            b"II" => true,
            b"MM" => false,
            _ => return Err(ParserError::new("Invalid Format")),
        };

        let read_u16 = |b: &[u8]| {
            if le {
                u16::from_le_bytes([b[0], b[1]])
            } else {
                u16::from_be_bytes([b[0], b[1]])
            }
        };

        let read_u32 = |b: &[u8]| {
            if le {
                u32::from_le_bytes([b[0], b[1], b[2], b[3]])
            } else {
                u32::from_be_bytes([b[0], b[1], b[2], b[3]])
            }
        };

        let ifd_offset = read_u32(&data[4..8]) as usize;
        let mut exif = Exif {
            make: None,
            model: None,
            compression: None,
            camera_model: None,
            camera_serial_number: None,
            exposure_time: None,
            iso_speed: None,
            exposure_bias: None,
            focal_length: None,
            max_aperture: None,
            metering_mode: None,
            subject_distance: None,
            flash_mode: None,
            focal_length_35mm: None,
            lens_make: None,
            lens_model: None,
            lens_serial: None,
            contrast: None,
            brightness: None,
            light_source: None,
            exposure_program: None,
            saturation: None,
            sharpness: None,
            white_balance: None,
            photometric_iterpretation: None,
            scene_capture_type: None,
            digital_zoom: None,
            exif_version: None,
        };

        parse_ifd(data, ifd_offset, &mut exif, &read_u16, &read_u32)?;

        Ok(exif)
    }
}


fn parse_ifd(
    data: &[u8],
    offset: usize,
    exif: &mut Exif,
    read_u16: &dyn Fn(&[u8]) -> u16,
    read_u32: &dyn Fn(&[u8]) -> u32,
) -> Result<(), ParserError> {
    if offset + 2 > data.len() {
        return Ok(());
    }

    let count = read_u16(&data[offset..offset + 2]) as usize;

    for i in 0..count {
        let base = offset + 2 + i * 12;
        if base + 12 > data.len() {
            break;
        }

        let tag = read_u16(&data[base..base + 2]);
        let _field_type = read_u16(&data[base + 2..base + 4]);
        let count = read_u32(&data[base + 4..base + 8]);
        let value_offset = &data[base + 8..base + 12];

        let value_ptr = if count * 4 <= 4 {
            base + 8
        } else {
            read_u32(value_offset) as usize
        };

        match tag {
            0x010F => exif.make = read_string(data, value_ptr, count as usize),
            0x0110 => exif.model = read_string(data, value_ptr, count as usize),
            0x0103 => {
                let v = read_u16(&data[value_ptr..]);
                exif.compression = Some(Compression::from_u16(v));
            }
            0xC614 | 0xC615 => {
                exif.camera_model = read_string(data, value_ptr, count as usize)
            }
            0xC62F => {
                exif.camera_serial_number = read_string(data, value_ptr, count as usize)
            }
            0x829A => {
                if let Some(v) = read_rational(data, value_ptr) {
                    exif.exposure_time = Some(Duration::from_secs_f64(v));
                }
            }
            0x8827 => exif.iso_speed = Some(read_u16(&data[value_ptr..])),
            0x9207 => {
                let v = read_u16(&data[value_ptr..]);
                exif.metering_mode = Some(MeteringMode::from_u16(v));
            }
            0x9209 => {
                let v = read_u16(&data[value_ptr..]);
                exif.flash_mode = Some(FlashMode::from_u16(v));
            }
            0x9208 => {
                let v = read_u16(&data[value_ptr..]);
                exif.light_source = Some(LightSource::from_u16(v));
            }
            0x8822 => {
                let v = read_u16(&data[value_ptr..]);
                exif.exposure_program = Some(ExposureProgram::from_u16(v));
            }
            0xA403 => {
                let v = read_u16(&data[value_ptr..]);
                exif.white_balance = Some(WhiteBalance::from_u16(v));
            }
            0xFE54 => {
                let v = read_u16(&data[value_ptr..]);
                exif.contrast = Some(Contrast::from_u16(v));
            }
            0xa406 => {
                let v = read_u16(&data[value_ptr..]);
                exif.scene_capture_type = Some(SceneCaptureType::from_u16(v));
            }
            0xFE55 => {
                let v = read_u16(&data[value_ptr..]);
                exif.saturation = Some(Saturation::from_u16(v));
            }
            0xFE56 => {
                let v = read_u16(&data[value_ptr..]);
                exif.sharpness = Some(Sharpness::from_u16(v));
            }
            0x0106 => {
                let v = read_u16(&data[value_ptr..]);
                exif.photometric_iterpretation =
                    Some(PhotoMetricInterpretation::from_u16(v));
            }
            _ => {}
        }
    }

    Ok(())
}





pub enum ExifVersion {
    V1,
    V1_1,
    V2,
    V2_1,
    V2_2,
    V2_21,
    V2_3,
    V2_31,
    V2_32,
    V3
}



