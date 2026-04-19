
use std::io::{self, Read, Seek, SeekFrom, Cursor};
use makernote_macros::{FromPrimitive};
use remata_macros::DisplayPretty;
// use crate::get_struct;


use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct IfdEntry {
    tag: u16,
    typ: u16,
    count: usize,
    value_or_offset: u32,
}
#[derive(Clone)]
pub struct NikonDecoder<R: Read + Seek> {
    reader: R,
    endian: Endian,
    base: u64,
    entries: HashMap<u16, IfdEntry>,
}

impl<R: Read + Seek> NikonDecoder<R> {
    pub fn new(mut reader: R) -> io::Result<Self> {
        let base = reader.stream_position()?;

        let mut sig = [0u8; 6];
        reader.read_exact(&mut sig)?;

        let mut endian_buf = [0u8; 2];
        loop {
            reader.read_exact(&mut endian_buf)?;
            if &endian_buf == b"II" || &endian_buf == b"MM" {
                break;
            }
        }

        let endian = if &endian_buf == b"II" {
            Endian::Little
        } else {
            Endian::Big
        };

        let _ = endian.read_u16(&mut reader)?;
        let ifd_offset = endian.read_u32(&mut reader)?;

        let ifd_base = base + ifd_offset as u64;
        reader.seek(SeekFrom::Start(ifd_base))?;

        let entry_count = endian.read_u16(&mut reader)? as usize;

        let mut entries = HashMap::new();

        for _ in 0..entry_count {
            let tag = endian.read_u16(&mut reader)?;
            let typ = endian.read_u16(&mut reader)?;
            let count = endian.read_u32(&mut reader)? as usize;
            let value_or_offset = endian.read_u32(&mut reader)?;

            if tag != 0 {
                entries.insert(tag, IfdEntry {
                    tag,
                    typ,
                    count,
                    value_or_offset,
                });
            }
        }

        Ok(Self {
            reader,
            endian,
            base,
            entries,
        })
    }
    fn value_pos(&self, e: &IfdEntry) -> Option<u64> {
        let size = type_size(e.typ) * e.count;

        if size <= 4 {
            None // inline
        } else {
            Some(self.base + e.value_or_offset as u64)
        }
    }

    fn inline_bytes(&self, e: &IfdEntry) -> [u8; 4] {
        let endian = self.endian;
        match endian {
            Endian::Little => e.value_or_offset.to_le_bytes(),
            Endian::Big => e.value_or_offset.to_be_bytes(),
        }
    }

    fn read_at<T>(&mut self, pos: u64, mut f: impl FnMut(&mut R) -> io::Result<T>) -> io::Result<T> {
        let cur = self.reader.stream_position()?;
        self.reader.seek(SeekFrom::Start(pos))?;
        let v = f(&mut self.reader)?;
        self.reader.seek(SeekFrom::Start(cur))?;
        Ok(v)
    }


    pub fn get_u8(&mut self, tag: u16) -> Option<u8> {
        let e = *self.entries.get(&tag)?;

        if type_size(e.typ) * e.count <= 4 {
            return Some(self.inline_bytes(&e)[0]);
        }

        let endian = self.endian;
        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| endian.read_u8(r)).ok()
    }

    pub fn get_u16(&mut self, tag: u16) -> Option<u16> {
        let e = *self.entries.get(&tag)?;

        if type_size(e.typ) * e.count <= 4 {
            let b = self.inline_bytes(&e);
            return Some(u16::from_le_bytes([b[0], b[1]]));
        }

        let endian = self.endian;
        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| endian.read_u16(r)).ok()
    }

    pub fn get_u32(&mut self, tag: u16) -> Option<u32> {
        let e = *self.entries.get(&tag)?;

        if type_size(e.typ) * e.count <= 4 {
            return Some(e.value_or_offset);
        }

        let endian = self.endian;
        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| endian.read_u32(r)).ok()
    }

    pub fn get_u64(&mut self, tag: u16) -> Option<u64> {
        let endian = self.endian;
        let pos = self.value_pos(self.entries.get(&tag)?)?;

        self.read_at(pos, |r| {
            let hi = endian.read_u32(r)? as u64;
            let lo = endian.read_u32(r)? as u64;
            Ok((hi << 32) | lo)
        }).ok()
    }

    // -------------------------
    // Signed helpers
    // -------------------------

    pub fn get_i8(&mut self, tag: u16) -> Option<i8> {
        self.get_u8(tag).map(|v| v as i8)
    }

    pub fn get_i16(&mut self, tag: u16) -> Option<i16> {
        self.get_u16(tag).map(|v| v as i16)
    }

    pub fn get_i32(&mut self, tag: u16) -> Option<i32> {
        self.get_u32(tag).map(|v| v as i32)
    }

    pub fn get_i64(&mut self, tag: u16) -> Option<i64> {
        self.get_u64(tag).map(|v| v as i64)
    }

    // -------------------------
    // String
    // -------------------------

    pub fn get_string(&mut self, tag: u16) -> Option<String> {
        let e = *self.entries.get(&tag)?;

        if type_size(e.typ) * e.count <= 4 {
            let b = self.inline_bytes(&e);
            return Some(
                String::from_utf8_lossy(&b[..e.count])
                    .trim_end_matches('\0')
                    .to_string()
            );
        }

        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| {
            let mut buf = vec![0u8; e.count];
            r.read_exact(&mut buf)?;
            Ok(String::from_utf8_lossy(&buf)
                .trim_end_matches('\0')
                .to_string())
        }).ok()
    }

    // -------------------------
    // Arrays
    // -------------------------

    pub fn get_array_u<const N: usize>(&mut self, tag: u16) -> Option<[u32; N]> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;

        if type_size(e.typ) * e.count <= 4 {
            let mut arr = [0u32; N];
            let b = self.inline_bytes(&e);

            for i in 0..N.min(1) {
                arr[i] = u32::from_le_bytes(b);
            }
            return Some(arr);
        }

        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| {
            let mut arr = [0u32; N];
            for i in 0..N {
                arr[i] = endian.read_u32(r)?;
            }
            Ok(arr)
        }).ok()
    }

    pub fn get_vector_u16(&mut self, tag: u16, max: usize) -> Option<Vec<u16>> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;

        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| {
            let mut v = Vec::new();
            for _ in 0..e.count.min(max) {
                v.push(endian.read_u16(r)?);
            }
            Ok(v)
        }).ok()
    }

    // -------------------------
    // Rationals
    // -------------------------

    pub fn get_rational_u32(&mut self, tag: u16) -> Option<Rational> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;

        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| {
            let num = endian.read_u32(r)? as i64;
            let den = endian.read_u32(r)? as i64;
            Ok(Rational { num, den: den.max(1) })
        }).ok()
    }

    pub fn get_rational_i32(&mut self, tag: u16) -> Option<Rational> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;

        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| {
            let num = endian.read_i32(r)? as i64;
            let den = endian.read_i32(r)? as i64;
            Ok(Rational { num, den: den.max(1) })
        }).ok()
    }

    pub fn get_rational_u16(&mut self, tag: u16) -> Option<Rational> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let num = endian.read_u16(r)? as i64;
            let den = endian.read_u16(r)? as i64;
            Ok(Rational { num, den: den.max(1) })
        }).ok()
    }

    pub fn get_rational_u8(&mut self, tag: u16) -> Option<Rational> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let num = endian.read_u8(r)? as i64;
            let den = endian.read_u8(r)? as i64;
            Ok(Rational { num, den: den.max(1) })
        }).ok()
    }

    pub fn get_rational_u64(&mut self, tag: u16) -> Option<Rational> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let num = endian.read_u64(r)? as i64;
            let den = endian.read_u64(r)? as i64;
            Ok(Rational { num, den: den.max(1) })
        }).ok()
    }

    pub fn get_array_rational_u32<const N: usize>(
        &mut self,
        tag: u16,
    ) -> Option<[Rational; N]> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;

        let pos = self.value_pos(&e)?;

        self.read_at(pos, |r| {
            let mut arr = [Rational { num: 0, den: 1 }; N];

            for i in 0..N {
                let num = endian.read_u32(r)? as i64;
                let den = endian.read_u32(r)? as i64;
                arr[i] = Rational { num, den: den.max(1) };
            }

            Ok(arr)
        }).ok()
    }

    pub fn get_array_rational_i32<const N: usize>(
        &mut self,
        tag: u16,
    ) -> Option<[Rational; N]> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let mut arr = [Rational { num: 0, den: 1 }; N];
            for i in 0..N {
                let num = endian.read_i32(r)? as i64;
                let den = endian.read_i32(r)? as i64;
                arr[i] = Rational { num, den: den.max(1) };
            }
            Ok(arr)
        }).ok()
    }

    pub fn get_array_rational_u16<const N: usize>(
        &mut self,
        tag: u16,
    ) -> Option<[Rational; N]> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let mut arr = [Rational { num: 0, den: 1 }; N];
            for i in 0..N {
                let num = endian.read_u16(r)? as i64;
                let den = endian.read_u16(r)? as i64;
                arr[i] = Rational { num, den: den.max(1) };
            }
            Ok(arr)
        }).ok()
    }

    pub fn get_array_rational_u8<const N: usize>(
        &mut self,
        tag: u16,
    ) -> Option<[Rational; N]> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let mut arr = [Rational { num: 0, den: 1 }; N];
            for i in 0..N {
                let num = endian.read_u8(r)? as i64;
                let den = endian.read_u8(r)? as i64;
                arr[i] = Rational { num, den: den.max(1) };
            }
            Ok(arr)
        }).ok()
    }

    pub fn get_array_rational_u64<const N: usize>(
        &mut self,
        tag: u16,
    ) -> Option<[Rational; N]> {
        let e = *self.entries.get(&tag)?;
        let endian = self.endian;
        let pos = self.value_pos(&e);

        self.read_at(pos?, |r| {
            let mut arr = [Rational { num: 0, den: 1 }; N];
            for i in 0..N {
                let num = endian.read_u64(r)? as i64;
                let den = endian.read_u64(r)? as i64;
                arr[i] = Rational { num, den: den.max(1) };
            }
            Ok(arr)
        }).ok()
    }

}

// get_struct!(MyStruct {
//     iso: get_u16(0x0002),
//     lens: get_u8(0x0083),
// });

// #[macro_export]
// macro_rules! get_struct {
//     ($dec:expr, $name:ident { $($field:ident : $expr:expr),* $(,)? }) => {{
//         #[derive(Debug)]
//         struct $name {
//             $(pub $field: _),*
//         }

//         $name {
//             $(
//                 $field: $expr,
//             )*
//         }
//     }};
// }




#[derive(Clone, Copy, Default, DisplayPretty)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}

#[derive(Clone, Copy, DisplayPretty)]
pub enum Endian {
    Little,
    Big,
}

impl Endian {
    // -------------------------
    // core byte readers
    // -------------------------

    #[inline]
    fn read_exact<const N: usize, R: Read>(&self, r: &mut R) -> io::Result<[u8; N]> {
        let mut b = [0u8; N];
        r.read_exact(&mut b)?;
        Ok(b)
    }

    // -------------------------
    // unsigned integers
    // -------------------------

    pub fn read_u8<R: Read>(&self, r: &mut R) -> io::Result<u8> {
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        Ok(b[0])
    }

    pub fn read_u16<R: Read>(&self, r: &mut R) -> io::Result<u16> {
        let b = self.read_exact::<2, _>(r)?;
        Ok(match self {
            Endian::Little => u16::from_le_bytes(b),
            Endian::Big => u16::from_be_bytes(b),
        })
    }

    pub fn read_u32<R: Read>(&self, r: &mut R) -> io::Result<u32> {
        let b = self.read_exact::<4, _>(r)?;
        Ok(match self {
            Endian::Little => u32::from_le_bytes(b),
            Endian::Big => u32::from_be_bytes(b),
        })
    }

    pub fn read_u64<R: Read>(&self, r: &mut R) -> io::Result<u64> {
        let b = self.read_exact::<8, _>(r)?;
        Ok(match self {
            Endian::Little => u64::from_le_bytes(b),
            Endian::Big => u64::from_be_bytes(b),
        })
    }

    // -------------------------
    // signed integers
    // -------------------------

    pub fn read_i8<R: Read>(&self, r: &mut R) -> io::Result<i8> {
        Ok(self.read_u8(r)? as i8)
    }

    pub fn read_i16<R: Read>(&self, r: &mut R) -> io::Result<i16> {
        let b = self.read_exact::<2, _>(r)?;
        Ok(match self {
            Endian::Little => i16::from_le_bytes(b),
            Endian::Big => i16::from_be_bytes(b),
        })
    }

    pub fn read_i32<R: Read>(&self, r: &mut R) -> io::Result<i32> {
        let b = self.read_exact::<4, _>(r)?;
        Ok(match self {
            Endian::Little => i32::from_le_bytes(b),
            Endian::Big => i32::from_be_bytes(b),
        })
    }

    pub fn read_i64<R: Read>(&self, r: &mut R) -> io::Result<i64> {
        let b = self.read_exact::<8, _>(r)?;
        Ok(match self {
            Endian::Little => i64::from_le_bytes(b),
            Endian::Big => i64::from_be_bytes(b),
        })
    }

    // -------------------------
    // rational readers
    // -------------------------

    pub fn read_rational_u32<R: Read>(&self, r: &mut R) -> io::Result<Rational> {
        let num = self.read_u32(r)? as i64;
        let den = self.read_u32(r)? as i64;
        Ok(Rational {
            num,
            den: if den == 0 { 1 } else { den },
        })
    }

    pub fn read_rational_i32<R: Read>(&self, r: &mut R) -> io::Result<Rational> {
        let num = self.read_i32(r)? as i64;
        let den = self.read_i32(r)? as i64;
        Ok(Rational {
            num,
            den: if den == 0 { 1 } else { den },
        })
    }

    pub fn read_rational_u16<R: Read>(&self, r: &mut R) -> io::Result<Rational> {
        let num = self.read_u16(r)? as i64;
        let den = self.read_u16(r)? as i64;
        Ok(Rational {
            num,
            den: if den == 0 { 1 } else { den },
        })
    }

    pub fn read_rational_u8<R: Read>(&self, r: &mut R) -> io::Result<Rational> {
        let num = self.read_u8(r)? as i64;
        let den = self.read_u8(r)? as i64;
        Ok(Rational {
            num,
            den: if den == 0 { 1 } else { den },
        })
    }

    pub fn read_rational_u64<R: Read>(&self, r: &mut R) -> io::Result<Rational> {
        let num = self.read_u64(r)? as i64;
        let den = self.read_u64(r)? as i64;
        Ok(Rational {
            num,
            den: if den == 0 { 1 } else { den },
        })
    }
}


#[derive(Default, DisplayPretty)]
pub struct NikonMakerNotes {
    pub maker_note_version: Option<[u8; 4]>,        // 0x0001
    pub iso: Option<[u16; 2]>,                      // 0x0002
    pub color_mode: Option<String>,                 // 0x0003
    pub quality: Option<String>,                    // 0x0004
    pub white_balance: Option<String>,              // 0x0005
    pub sharpness: Option<String>,                  // 0x0006
    pub focus_mode: Option<String>,                 // 0x0007
    pub flash_setting: Option<String>,              // 0x0008
    pub flash_type: Option<String>,                 // 0x0009

    pub white_balance_fine_tune: Option<Vec<i16>>,  // 0x000b
    pub wb_rb_levels: Option<[Rational; 4]>,        // 0x000c

    pub iso_selection: Option<String>,              // 0x000f
    pub iso_setting: Option<[u16; 2]>,              // 0x0013

    pub image_boundary: Option<[u16; 4]>,           // 0x0016
    pub exposure_bracket_value: Option<Rational>,   // 0x0019

    pub image_processing: Option<String>,           // 0x001a
    pub crop_hi_speed: Option<[CropHighSpeed; 7]>,  // 0x001b

    pub lens_type: Option<LensType>,                // 0x0083

    pub active_d_lighting: Option<ActiveDLighting>, // 0x0022
    pub vignette_control: Option<VignetteControl>,  // 0x002a
    pub shutter_mode: Option<ShutterMode>,          // 0x0034
    pub image_size_raw: Option<ImageSizeRaw>,       // 0x003e
    pub jpg_compression: Option<JpgCompression>,    // 0x0044

}
#[derive(Default, Clone, Copy, DisplayPretty)]
pub struct NikonWorldTime {
    pub time_zone: Option<i16>,                 // index 0
    pub daylight_savings: Option<DaylightSavings>, // index 2
    pub date_display_format: Option<DateDisplayFormat>, // index 3
}


macro_rules! get_struct {
    (
        $dec:expr,
        $tag:expr,
        $struct:ident {
            $(
                $field:ident : $ty:ident $(=> $map:ident)? @ $offset:expr
            ),* $(,)?
        }
    ) => {{
        let mut result = $struct::default();

        if let Some(e) = $dec.entries.get(&$tag).copied() {
            let pos = $dec.value_pos(&e);

            if let Some(pos) = pos {
                let endian = $dec.endian;

                let _ = $dec.read_at(pos, |r| {
                    $(
                        {
                            use std::io::{Seek, SeekFrom};

                            r.seek(SeekFrom::Start(pos + $offset))?;

                            let raw = match stringify!($ty) {
                                "u8" => endian.read_u8(r)? as u64,
                                "u16" => endian.read_u16(r)? as u64,
                                "u32" => endian.read_u32(r)? as u64,
                                "i8" => endian.read_u8(r)? as i8 as i64 as u64,
                                "i16" => endian.read_i16(r)? as i64 as u64,
                                "i32" => endian.read_i32(r)? as i64 as u64,
                                _ => 0,
                            };

                            result.$field = Some(
                                get_struct!(@map raw, $ty $(=> $map)?)
                            );
                        }
                    )*

                    Ok(())
                });
            }
        }

        result
    }};

    // -------------------------
    // Mapping helpers
    // -------------------------

    (@map $raw:expr, u8 => $map:ident) => {
        $map::from($raw as u8)
    };

    (@map $raw:expr, u16 => $map:ident) => {
        $map::from($raw as u16)
    };

    (@map $raw:expr, u32 => $map:ident) => {
        $map::from($raw as u32)
    };

    (@map $raw:expr, i8 => $map:ident) => {
        $map::from($raw as i8)
    };

    (@map $raw:expr, i16 => $map:ident) => {
        $map::from($raw as i16)
    };

    (@map $raw:expr, i32 => $map:ident) => {
        $map::from($raw as i32)
    };

    (@map $raw:expr, u8) => { $raw as u8 };
    (@map $raw:expr, u16) => { $raw as u16 };
    (@map $raw:expr, u32) => { $raw as u32 };
    (@map $raw:expr, i8) => { $raw as i8 };
    (@map $raw:expr, i16) => { $raw as i16 };
    (@map $raw:expr, i32) => { $raw as i32 };
}

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum DaylightSavings {
    #[value = 0] No,
    #[value = 0] Yes,
    Unknown(u8),
}

// impl From<u8> for DaylightSavings {
//     fn from(v: u8) -> Self {
//         match v {
//             0 => DaylightSavings::No,
//             1 => DaylightSavings::Yes,
//             x => DaylightSavings::Unknown(x),
//         }
//     }
// }

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum DateDisplayFormat {
    #[value = 0] YMD,
    #[value = 1] MDY,
    #[value = 2] DMY,
    Unknown(u8),
}

// impl From<u8> for DateDisplayFormat {
//     fn from(v: u8) -> Self {
//         match v {
//             0 => DateDisplayFormat::YMD,
//             1 => DateDisplayFormat::MDY,
//             2 => DateDisplayFormat::DMY,
//             _ => DateDisplayFormat::Unknown(v),
//         }
//     }
// }








pub fn parse(data: &[u8]) -> io::Result<NikonMakerNotes> {
    let cursor = Cursor::new(data);
    let mut dec = NikonDecoder::new(cursor)?;

    let mut notes = NikonMakerNotes::default();

    // -------------------------
    // Fixed fields
    // -------------------------
    if let Some(v) = dec.get_array_u::<4>(0x0001) {
        let arr = [v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8];
        println!("[0x0001] MakerNoteVersion → {:?}", arr);
        notes.maker_note_version = Some(arr);
    }

    if let Some(v) = dec.get_array_u::<2>(0x0002) {
        let arr = [v[0] as u16, v[1] as u16];
        println!("[0x0002] ISO → {:?}", arr);
        notes.iso = Some(arr);
    }

    if let Some(v) = dec.get_u8(0x0083) {
        let lens = LensType::from(v);
        println!("[0x0083] LensType → {:?}", lens);
        notes.lens_type = Some(lens);
    }

    // -------------------------
    // Enum fields
    // -------------------------
    if let Some(v) = dec.get_u16(0x0022) {
        let val = ActiveDLighting::from(v);
        println!("[0x0022] ActiveDLighting → {:?}", val);
        notes.active_d_lighting = Some(val);
    }

    if let Some(v) = dec.get_u16(0x002a) {
        let val = VignetteControl::from(v);
        println!("[0x002a] VignetteControl → {:?}", val);
        notes.vignette_control = Some(val);
    }

    if let Some(v) = dec.get_u16(0x0034) {
        let val = ShutterMode::from(v);
        println!("[0x0034] ShutterMode → {:?}", val);
        notes.shutter_mode = Some(val);
    }

    if let Some(v) = dec.get_u16(0x003e) {
        let val = ImageSizeRaw::from(v);
        println!("[0x003e] ImageSizeRAW → {:?}", val);
        notes.image_size_raw = Some(val);
    }

    if let Some(v) = dec.get_u16(0x0044) {
        let val = JpgCompression::from(v);
        println!("[0x0044] JPGCompression → {:?}", val);
        notes.jpg_compression = Some(val);
    }

    // -------------------------
    // Strings
    // -------------------------
    if let Some(s) = dec.get_string(0x0003) {
        println!("[0x0003] ColorMode → {}", s);
        notes.color_mode = Some(s);
    }

    // -------------------------
    // i16 vector
    // -------------------------
    if let Some(v) = dec.get_vector_u16(0x000b, 64) {
        let v: Vec<i16> = v.into_iter().map(|x| x as i16).collect();
        println!("[0x000b] WhiteBalanceFineTune → {:?}", v);
        notes.white_balance_fine_tune = Some(v);
    }

    // -------------------------
    // Rational array
    // -------------------------
    if let Some(arr) = dec.get_array_rational_u32::<4>(0x000c) {
        println!("[0x000c] WB_RBLevels → {:?}", arr);
        notes.wb_rb_levels = Some(arr);
    }

    // -------------------------
    // CropHiSpeed
    // -------------------------
    if let Some(v) = dec.get_vector_u16(0x001b, 7) {
        let mut arr = [CropHighSpeed::Off; 7];

        for (i, val) in v.into_iter().enumerate().take(7) {
            arr[i] = CropHighSpeed::from(val);
        }

        println!("[0x001b] CropHiSpeed → {:?}", arr);
        notes.crop_hi_speed = Some(arr);
    }

    // -------------------------
    // ✅ WorldTime (STRUCT MACRO)
    // -------------------------
    let world_time = get_struct!(dec, 0x001f, NikonWorldTime {
        time_zone: i16 @ 0,
        daylight_savings: u8 => DaylightSavings @ 2,
        date_display_format: u8 => DateDisplayFormat @ 3,
    });

    if world_time.time_zone.is_some() {
        println!("[0x001f] WorldTime → {:?}", world_time);
        // (optional) add to your main struct if desired
        // notes.world_time = Some(world_time);
    }

    Ok(notes)
}

fn type_size(t: u16) -> usize {
    match t {
        1 | 2 | 6 | 7 => 1,  // BYTE, ASCII, SBYTE, UNDEF
        3 | 8 => 2,         // SHORT
        4 | 9 => 4,         // LONG
        5 | 10 => 8,        // RATIONAL
        _ => 1,
    }
}

#[derive(Default, Clone, Copy, DisplayPretty)]
pub struct LensType {
    pub mf: bool,
    pub d: bool,
    pub g: bool,
    pub vr: bool,
    pub bit4: bool,
    pub ft1: bool,
    pub e: bool,
    pub af_p: bool,
}

impl From<u8> for LensType {
    fn from(v: u8) -> Self {
        Self {
            mf: v & (1 << 0) != 0,
            d: v & (1 << 1) != 0,
            g: v & (1 << 2) != 0,
            vr: v & (1 << 3) != 0,
            bit4: v & (1 << 4) != 0,
            ft1: v & (1 << 5) != 0,
            e: v & (1 << 6) != 0,
            af_p: v & (1 << 7) != 0,
        }
    }
}

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum ActiveDLighting {
    #[value = 0] Off,
    #[value = 1] Low,
    #[value = 3] Normal,
    #[value = 5] High,
    #[value = 7] ExtraHigh,
    #[value = 8] ExtraHigh1,
    #[value = 9] ExtraHigh2,
    #[value = 10] ExtraHigh3,
    #[value = 11] ExtraHigh4,
    #[value = 65535] Auto,
    Unknown(u16),
}

// impl From<u16> for ActiveDLighting {
//     fn from(v: u16) -> Self {
//         match v {
//             0 => Self::Off,
//             1 => Self::Low,
//             3 => Self::Normal,
//             5 => Self::High,
//             7 => Self::ExtraHigh,
//             8 => Self::ExtraHigh1,
//             9 => Self::ExtraHigh2,
//             10 => Self::ExtraHigh3,
//             11 => Self::ExtraHigh4,
//             65535 => Self::Auto,
//             _ => Self::Unknown(v),
//         }
//     }
// }

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum JpgCompression {
    #[value = 1] SizePriority,
    #[value = 3] OptimalQuality,
    Unknown(u16),
}

// impl From<u16> for JpgCompression {
//     fn from(v: u16) -> Self {
//         match v {
//             1 => Self::SizePriority,
//             3 => Self::OptimalQuality,
//             _ => Self::Unknown(v),
//         }
//     }
// }

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum ImageSizeRaw {
    #[value = 1] Large,
    #[value = 2] Medium,
    #[value = 3] Small,
    Unknown(u16),
}

// impl From<u16> for ImageSizeRaw {
//     fn from(v: u16) -> Self {
//         match v {
//             1 => Self::Large,
//             2 => Self::Medium,
//             3 => Self::Small,
//             _ => Self::Unknown(v),
//         }
//     }
// }

#[derive(FromPrimitive, Clone, Copy, DisplayPretty)]
pub enum ShutterMode {
    #[value = 0] Mechanical,
    #[value = 16] Electronic,
    #[value = 48] ElectronicFrontCurtain,
    #[value = 64] ElectronicMovie,
    #[value = 80] AutoMechanical,
    #[value = 81] AutoElectronicFrontCurtain,
    #[value = 96] ElectronicHighSpeed,
    Unknown(u16),
}

// #[derive(Clone, Copy, DisplayPretty)]
// pub enum ShutterMode {
//     Mechanical,
//     Electronic,
//     ElectronicFrontCurtain,
//     ElectronicMovie,
//     AutoMechanical,
//     AutoElectronicFrontCurtain,
//     ElectronicHighSpeed,
//     Unknown(u16),
// }

// impl From<u16> for ShutterMode {
//     fn from(v: u16) -> Self {
//         match v {
//             0 => Self::Mechanical,
//             16 => Self::Electronic,
//             48 => Self::ElectronicFrontCurtain,
//             64 => Self::ElectronicMovie,
//             80 => Self::AutoMechanical,
//             81 => Self::AutoElectronicFrontCurtain,
//             96 => Self::ElectronicHighSpeed,
//             _ => Self::Unknown(v),
//         }
//     }
// }

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum VignetteControl {
    #[value = 0] Off,
    #[value = 1] Low,
    #[value = 3] Normal,
    #[value = 5] High,
    Unknown(u16),
}

// impl From<u16> for VignetteControl {
//     fn from(v: u16) -> Self {
//         match v {
//             0 => Self::Off,
//             1 => Self::Low,
//             3 => Self::Normal,
//             5 => Self::High,
//             _ => Self::Unknown(v),
//         }
//     }
// }

#[derive(Clone, Copy, DisplayPretty, FromPrimitive)]
pub enum CropHighSpeed {
    #[value = 0] Off,
    #[value = 1] Crop1_3x,
    #[value = 2] DxCrop,
    #[value = 3] Crop5_4,
    #[value = 4] Crop3_2,
    #[value = 6] Crop16_9,
    #[value = 8] Crop2_7x,
    #[value = 9] DxMovie16_9,
    #[value = 10] Crop1_3Movie,
    #[value = 11] FxUncropped,
    #[value = 12] DxUncropped,
    #[value = 13] Crop2_8Movie,
    #[value = 14] Crop1_4Movie,
    #[value = 15] Crop1_5Movie,
    #[value = 17] Fx1_1,
    #[value = 18] Dx1_1,
    Unknown(u16),
}

// impl From<u16> for CropHighSpeed {
//     fn from(v: u16) -> Self {
//         match v {
//             0 => Self::Off,
//             1 => Self::Crop1_3x,
//             2 => Self::DxCrop,
//             3 => Self::Crop5_4,
//             4 => Self::Crop3_2,
//             6 => Self::Crop16_9,
//             8 => Self::Crop2_7x,
//             9 => Self::DxMovie16_9,
//             10 => Self::Crop1_3Movie,
//             11 => Self::FxUncropped,
//             12 => Self::DxUncropped,
//             13 => Self::Crop2_8Movie,
//             14 => Self::Crop1_4Movie,
//             15 => Self::Crop1_5Movie,
//             17 => Self::Fx1_1,
//             18 => Self::Dx1_1,
//             _ => Self::Unknown(v),
//         }
//     }
// }