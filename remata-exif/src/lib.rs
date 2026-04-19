use remata_macros::{DisplayPretty};
mod helpers;
use helpers::{
    dms_to_deg,
    rational_to_f64,
    parse_ifd,
    read_ascii,
    find_exif_tiff_start,
    // read_srational_strings,

};



use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
mod gps;
use gps::Gps;
mod enums;
use enums::{
    Orientation,
    Compression,
    ResolutionUnit,
    YCbCrPositioning,
    Flash
};

mod tags;
use tags::TAGS;

mod endian;
use endian::Endian;


#[derive(Debug)]
pub struct ExtraExif {
    tag_id: u16,
    tag_name: String,
    tag_value: String
}

impl ExtraExif {
    pub fn new(tag_id: u16, tag_name: String, tag_value: String) -> Self {
        Self {
            tag_id,
            tag_name,
            tag_value
        }
    }
}

impl std::fmt::Display for ExtraExif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  {:04X} ({}): {:?}", self.tag_id, self.tag_name, self.tag_value)
    }
}
// use remata_macros::FromPrimitive;
#[derive(Default, DisplayPretty)]
pub struct ExifData {
    pub make: Option<String>,            // 0x010F
    pub model: Option<String>,           // 0x0110
    pub datetime: Option<String>,        // 0x0132

    pub orientation: Option<Orientation>,        // 0x0112
    pub compression: Option<Compression>, // 0x0103

    pub software: Option<String>,        // 0x0131

    pub artist: Option<String>,          // 0x013B

    pub exposure_time: Option<String>,   // 0x829A
    pub f_number: Option<String>,        // 0x829D
    pub iso: Option<u32>,                // 0x8827

    pub x_resolution: Option<String>, // 011A
    pub y_resolution: Option<String>, // 011B
    pub resolution_unit: Option<ResolutionUnit>, // 0x0128

    pub focal_length: Option<String>,    // 0x920A
    pub y_cb_cr_subsampling: Option<YCbCrSubSampling>, // 0x212
    pub y_cb_cr_positioning: Option<YCbCrPositioning>, // 0x0213
    pub gps: Option<Gps>,
    pub thumbnail_offset: Option<u64>, // 0x0201
    pub thumbnail_length: Option<u64>, // 0x0202
    pub image_description: Option<String>, // 0x010E
    pub flash: Option<Flash>, // 0x9209

    pub extra: Vec<ExtraExif>,
}

impl ExifData {
    pub fn parse<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let file_len = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(0))?;
        let mut exif = ExifData::default();

        // ---- Detect format ----
        let mut magic = [0u8; 2];
        reader.read_exact(&mut magic)?;

        let tiff_start = if magic == [0xFF, 0xD8] {
            find_exif_tiff_start(&mut reader)?
        } else {
            0
        };

        reader.seek(SeekFrom::Start(tiff_start))?;

        // ---- TIFF header ----
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;

        let endian = match &header[0..2] {
            b"II" => Endian::Little,
            b"MM" => Endian::Big,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid TIFF header",
                ))
            }
        };

        let magic = match endian {
            Endian::Little => u16::from_le_bytes([header[2], header[3]]),
            Endian::Big => u16::from_be_bytes([header[2], header[3]]),
        };

        if magic != 42 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid TIFF magic",
            ));
        }

        let first_ifd_offset = match endian {
            Endian::Little => u32::from_le_bytes([header[4], header[5], header[6], header[7]]),
            Endian::Big => u32::from_be_bytes([header[4], header[5], header[6], header[7]]),
        };

        // ---- Walk IFDs ----
        let mut current_offset = first_ifd_offset;

        while let Some((entries, next_offset)) =
            parse_ifd(&mut reader, tiff_start, current_offset, endian, file_len)?
        {
            for e in &entries {
                match e.tag {
                    // ---- ASCII ----
                    0x010F => {
                        if let Ok(s) = read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count) {
                            exif.make = Some(s);
                        }
                    }
                    0x0110 => {
                        if let Ok(s) = read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count) {
                            exif.model = Some(s);
                        }
                    }
                    0x0132 => {
                        if let Ok(s) = read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count) {
                            exif.datetime = Some(s);
                        }
                    }
                    0x0131 => {
                        if let Ok(s) = read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count) {
                            exif.software = Some(s);
                        }
                    }
                    0x013B => {
                        if let Ok(s) = read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count) {
                            exif.artist = Some(s);
                        }
                    }
                    0x010E => {
                        if let Ok(s) = read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count) {
                            exif.image_description = Some(s);
                        }
                    }

                    // ---- SHORT ----
                    0x0112 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.orientation = v.first()
                                .and_then(|v| Orientation::try_from(*v as u16).ok());
                        }
                    }
                    0x0103 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.compression = v.first()
                                .and_then(|v| Compression::from_u16(*v as u16));
                        }
                    }
                    0x0128 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.resolution_unit = v.first()
                                .and_then(|v| ResolutionUnit::try_from(*v as u16).ok());
                        }
                    }
                    0x0213 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.y_cb_cr_positioning = v.first()
                                .and_then(|v| YCbCrPositioning::try_from(*v as u16).ok());
                        }
                    }
                    0x9209 | 0x920b => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.flash = v.first()
                                .and_then(|v| Flash::try_from(*v as u16).ok());
                        }
                    }
                    0x0201 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.thumbnail_offset = v.first().and_then(|v| Some(*v as u64));
                        }
                    }

                    0x0202 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.thumbnail_length = v.first().and_then(|v| Some(*v as u64));
                        }
                    }

                    // ---- RATIONAL ----
                    0x011A => {
                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, e) {
                            exif.x_resolution = v.first().cloned();
                        }
                    }
                    0x011B => {
                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, e) {
                            exif.y_resolution = v.first().cloned();
                        }
                    }
                    0x829A => {
                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, e) {
                            exif.exposure_time = v.first().cloned();
                        }
                    }
                    0x829D => {
                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, e) {
                            exif.f_number = v.first().cloned();
                        }
                    }
                    0xA405 => {
                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, e) {
                            exif.focal_length = v.first().cloned();
                        }
                    }

                    // ---- ISO ----
                    0x8827 => {
                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, e) {
                            exif.iso = v.first().map(|v| *v as u32);
                        }
                    }

                    // ---- SubIFDs ----
                    0x8769 => {
                        if let Some((sub_entries, _)) =
                            parse_ifd(&mut reader, tiff_start, e.value_offset_or_inline, endian, file_len)?
                        {
                            for sub in &sub_entries {
                                match sub.tag {
                                    0x829A => {
                                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, sub) {
                                            exif.exposure_time = v.first().cloned();
                                        }
                                    }
                                    0x829D => {
                                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, sub) {
                                            exif.f_number = v.first().cloned();
                                        }
                                    }
                                    0x8827 => {
                                        if let Ok(v) = endian.read_numeric_values(&mut reader, tiff_start, sub) {
                                            exif.iso = v.first().map(|v| *v as u32);
                                        }
                                    }
                                    0x920A => {
                                        if let Ok(v) = endian.read_rational_strings(&mut reader, tiff_start, sub) {
                                            exif.focal_length = v.first().cloned();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    // ---- GPS ----
                    0x8825 => {
                        if let Ok(gps) =
                            Gps::parse_ifd(&mut reader, tiff_start, e.value_offset_or_inline, endian, file_len)
                        {
                            exif.gps = Some(gps);
                        }
                    }

                    // ---- Extra ----
                    _ => {
                        let name = helpers::tag_name(e.tag).to_string();

                        let value = match e.value_type {
                            IfdType::Ascii => {
                                read_ascii(&mut reader, tiff_start, e.value_offset_or_inline, e.count)
                                    .unwrap_or_else(|_| "<error>".into())
                            }

                            IfdType::Rational => {
                                endian.read_rational_strings(&mut reader, tiff_start, e)
                                    .map(|v| format!("{:?}", v))
                                    .unwrap_or_else(|_| "<error>".into())
                            }

                            IfdType::SRational => {
                                endian.read_srational_strings(&mut reader, tiff_start, e)
                                    .map(|v| format!("{:?}", v))
                                    .unwrap_or_else(|_| "<error>".into())
                            }

                            IfdType::Byte
                            | IfdType::Short
                            | IfdType::Long
                            | IfdType::SLong
                            | IfdType::Undefined => {
                                endian.read_numeric_values(&mut reader, tiff_start, e)
                                    .map(|v| format!("{:?}", v))
                                    .unwrap_or_else(|_| "<error>".into())
                            }

                            _ => format!("type={:?}, count={}", e.value_type, e.count),
                        };

                        exif.extra.push(ExtraExif::new(e.tag, name, value));
                    }
                }
            }

            if next_offset == 0 {
                break;
            }

            current_offset = next_offset;
        }

        Ok(exif)
    }

    pub fn from_path(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Self::parse(file)
    }
}



mod ifd;
use ifd::{
    IfdType,
    // Rational,
    // SRational,
    // IfdValue,
    IfdEntry,
    // Ifd
};

use crate::enums::YCbCrSubSampling;



