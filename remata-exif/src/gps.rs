use std::io::{self, Read, Seek, SeekFrom};
use remata_macros::DisplayPretty;
use super::{
    Endian,
    parse_ifd,
    read_ascii,
    rational_to_f64,
    dms_to_deg
};
#[derive(DisplayPretty, Default)]
pub struct Gps {
    pub version_id: Option<[u8; 4]>,

    pub latitude_ref: Option<char>,
    pub latitude: Option<f64>,

    pub longitude_ref: Option<char>,
    pub longitude: Option<f64>,

    pub altitude_ref: Option<u8>,
    pub altitude: Option<f64>,

    pub timestamp: Option<String>,

    pub satellites: Option<String>,
    pub status: Option<char>,
    pub measure_mode: Option<String>,

    pub dop: Option<f64>,

    pub speed_ref: Option<char>,
    pub speed: Option<f64>,

    pub track_ref: Option<char>,
    pub track: Option<f64>,

    pub img_direction_ref: Option<char>,
    pub img_direction: Option<f64>,

    pub map_datum: Option<String>,

    pub dest_latitude_ref: Option<char>,
    pub dest_latitude: Option<f64>,

    pub dest_longitude_ref: Option<char>,
    pub dest_longitude: Option<f64>,

    pub dest_bearing_ref: Option<char>,
    pub dest_bearing: Option<f64>,

    pub dest_distance_ref: Option<char>,
    pub dest_distance: Option<f64>,

    pub processing_method: Option<Vec<u8>>,
    pub area_information: Option<Vec<u8>>,

    pub date_stamp: Option<String>,

    pub differential: Option<u16>,
    pub h_positioning_error: Option<f64>,
}

impl Gps {
    pub fn parse_ifd<R: Read + Seek>(
        reader: &mut R,
        base_offset: u64,
        offset: u32,
        endian: Endian,
        file_len: u64
    ) -> io::Result<Self> {
        let mut gps = Self::default();

        if let Some((entries, _)) = parse_ifd(reader, base_offset, offset, endian, file_len)? {
            let mut lat_raw = None;
            let mut lon_raw = None;
            let mut dest_lat_raw = None;
            let mut dest_lon_raw = None;

            for e in entries {
                match e.tag {
                    0x0000 => {
                        let vals = endian.read_numeric_values(reader, base_offset, &e)?;
                        if vals.len() == 4 {
                            gps.version_id = Some([
                                vals[0] as u8,
                                vals[1] as u8,
                                vals[2] as u8,
                                vals[3] as u8,
                            ]);
                        }
                    }

                    0x0001 => {
                        gps.latitude_ref = read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                            .ok()
                            .and_then(|s| s.chars().next());
                    }

                    0x0002 => {
                        lat_raw = Some(endian.read_rational(reader, base_offset, e.value_offset_or_inline, 3)?);
                    }

                    0x0003 => {
                        gps.longitude_ref = read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                            .ok()
                            .and_then(|s| s.chars().next());
                    }

                    0x0004 => {
                        lon_raw = Some(endian.read_rational(reader, base_offset, e.value_offset_or_inline, 3)?);
                    }

                    0x0005 => {
                        gps.altitude_ref = endian
                            .read_numeric_values(reader, base_offset, &e)
                            .ok()
                            .and_then(|v| v.first().copied())
                            .map(|v| v as u8);
                    }

                    0x0006 => {
                        if let Some((n, d)) = endian
                            .read_rational(reader, base_offset, e.value_offset_or_inline, 1)?
                            .first()
                        {
                            gps.altitude = rational_to_f64(*n, *d);
                        }
                    }

                    0x0007 => {
                        let t = endian.read_rational(reader, base_offset, e.value_offset_or_inline, 3)?;
                        if t.len() == 3 {
                            gps.timestamp = Some(format!(
                                "{:02}:{:02}:{:02} UTC",
                                t[0].0 / t[0].1,
                                t[1].0 / t[1].1,
                                t[2].0 / t[2].1
                            ));
                        }
                    }

                    0x0008 => {
                        gps.satellites =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count).ok();
                    }

                    0x0009 => {
                        gps.status = read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                            .ok()
                            .and_then(|s| s.chars().next());
                    }

                    0x000A => {
                        gps.measure_mode =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count).ok();
                    }

                    0x000B => {
                        if let Some((n, d)) = endian
                            .read_rational(reader, base_offset, e.value_offset_or_inline, 1)?
                            .first()
                        {
                            gps.dop = rational_to_f64(*n, *d);
                        }
                    }

                    0x000C => {
                        gps.speed_ref = read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                            .ok()
                            .and_then(|s| s.chars().next());
                    }

                    0x000D => {
                        if let Some((n, d)) = endian
                            .read_rational(reader, base_offset, e.value_offset_or_inline, 1)?
                            .first()
                        {
                            gps.speed = rational_to_f64(*n, *d);
                        }
                    }

                    0x000E => {
                        gps.track_ref = read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                            .ok()
                            .and_then(|s| s.chars().next());
                    }

                    0x000F => {
                        if let Some((n, d)) = endian
                            .read_rational(reader, base_offset, e.value_offset_or_inline, 1)?
                            .first()
                        {
                            gps.track = rational_to_f64(*n, *d);
                        }
                    }

                    0x0010 => {
                        gps.img_direction_ref =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                                .ok()
                                .and_then(|s| s.chars().next());
                    }

                    0x0011 => {
                        if let Some((n, d)) = endian
                            .read_rational(reader, base_offset, e.value_offset_or_inline, 1)?
                            .first()
                        {
                            gps.img_direction = rational_to_f64(*n, *d);
                        }
                    }

                    0x0012 => {
                        gps.map_datum =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count).ok();
                    }

                    0x0013 => {
                        gps.dest_latitude_ref =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                                .ok()
                                .and_then(|s| s.chars().next());
                    }

                    0x0014 => {
                        dest_lat_raw = Some(endian.read_rational(
                            reader,
                            base_offset,
                            e.value_offset_or_inline,
                            3,
                        )?);
                    }

                    0x0015 => {
                        gps.dest_longitude_ref =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count)
                                .ok()
                                .and_then(|s| s.chars().next());
                    }

                    0x0016 => {
                        dest_lon_raw = Some(endian.read_rational(
                            reader,
                            base_offset,
                            e.value_offset_or_inline,
                            3,
                        )?);
                    }

                    0x001B | 0x001C => {
                        // streamed + bounded read
                        let size = (e.count as usize).min(1024);
                        let mut buf = vec![0u8; size];

                        let absolute = base_offset + e.value_offset_or_inline as u64;
                        reader.seek(SeekFrom::Start(absolute))?;
                        reader.read_exact(&mut buf)?;

                        if e.tag == 0x001B {
                            gps.processing_method = Some(buf);
                        } else {
                            gps.area_information = Some(buf);
                        }
                    }

                    0x001D => {
                        gps.date_stamp =
                            read_ascii(reader, base_offset, e.value_offset_or_inline, e.count).ok();
                    }

                    0x001E => {
                        gps.differential = endian
                            .read_numeric_values(reader, base_offset, &e)
                            .ok()
                            .and_then(|v| v.first().copied())
                            .map(|v| v as u16);
                    }

                    0x001F => {
                        if let Some((n, d)) = endian
                            .read_rational(reader, base_offset, e.value_offset_or_inline, 1)?
                            .first()
                        {
                            gps.h_positioning_error = rational_to_f64(*n, *d);
                        }
                    }

                    _ => {}
                }
            }

            // ---- coordinate conversion ----
            if let (Some(v), Some(r)) = (lat_raw, gps.latitude_ref) {
                if let Some(mut d) = dms_to_deg(&v) {
                    if r == 'S' {
                        d = -d;
                    }
                    gps.latitude = Some(d);
                }
            }

            if let (Some(v), Some(r)) = (lon_raw, gps.longitude_ref) {
                if let Some(mut d) = dms_to_deg(&v) {
                    if r == 'W' {
                        d = -d;
                    }
                    gps.longitude = Some(d);
                }
            }

            if let (Some(v), Some(r)) = (dest_lat_raw, gps.dest_latitude_ref) {
                if let Some(mut d) = dms_to_deg(&v) {
                    if r == 'S' {
                        d = -d;
                    }
                    gps.dest_latitude = Some(d);
                }
            }

            if let (Some(v), Some(r)) = (dest_lon_raw, gps.dest_longitude_ref) {
                if let Some(mut d) = dms_to_deg(&v) {
                    if r == 'W' {
                        d = -d;
                    }
                    gps.dest_longitude = Some(d);
                }
            }
        }

        Ok(gps)
    }
}