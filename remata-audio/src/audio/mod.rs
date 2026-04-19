mod aiff;
pub use aiff::AiffMeta;

mod id3;
pub use id3::{Id3};

mod riff;
pub use riff::RiffMeta;

mod asf;
pub use asf::AsfMeta;

mod vob;
pub use vob::Vob;

mod atom;
pub use atom::AtomMeta;

use crate::error::ParserError;
// use id3::{Id3V1, Id3V2};


#[derive(Default, Debug, Clone)]
pub struct AudioMeta {
    pub id3: Option<Id3>,
    pub aiff: Option<AiffMeta>,
    pub riff: Option<RiffMeta>,
    pub asf: Option<AsfMeta>,
    pub vob: Option<Vob>,
    pub atom: Option<AtomMeta>,
}

impl AudioMeta {
    pub fn parse(data: &[u8]) -> Result<Self, ParserError> {
        let mut meta = AudioMeta::default();

        // ------------------------
        // ID3v2 (header at start)
        // ------------------------
        if data.len() >= 10 && &data[0..3] == b"ID3" {
            if let Ok(id3) = Id3::parse(data) {
                meta.id3 = Some(id3);
            }
        }
        // ------------------------
        // ID3v1 (footer only if no v2)
        // ------------------------
        else if data.len() >= 128 {
            let footer = &data[data.len() - 128..];
            if &footer[0..3] == b"TAG" {
                if let Ok(id3) = Id3::parse(data) {
                    meta.id3 = Some(id3);
                }
            }
        }

        // ------------------------
        // ASF (WMA/WMV)
        // ------------------------
        if is_asf_header(data) {
            if let Ok(asf) = AsfMeta::parse(data) {
                meta.asf = Some(asf);
            }
        }

        // ------------------------
        // RIFF (WAV/AVI/WEBP container)
        // ------------------------
        if data.len() >= 12 && &data[0..4] == b"RIFF" {
            // Optional: distinguish WebP vs WAV/AVI
            // WEBP = "RIFF....WEBP"
            if &data[8..12] != b"WEBP" {
                if let Ok(riff) = RiffMeta::parse(data) {
                    meta.riff = Some(riff);
                }
            }
        }

        // ------------------------
        // AIFF (FORM header)
        // ------------------------
        if data.len() >= 12 && &data[0..4] == b"FORM" {
            if let Ok(aiff) = AiffMeta::parse(data) {
                meta.aiff = Some(aiff);
            }
        }

        // ------------------------
        // MP4 / Atom (ftyp box)
        // ------------------------
        if data.len() >= 12 && &data[4..8] == b"ftyp" {
            if let Ok(atom) = AtomMeta::parse(data) {
                meta.atom = Some(atom);
            }
        }

        // ------------------------
        // VOB / MPEG (program stream)
        // ------------------------
        if data.len() >= 4 && data[0..4] == [0x00, 0x00, 0x01, 0xBA] {
            if let Ok(vob) = Vob::parse(data) {
                meta.vob = Some(vob);
            }
        }

        // ------------------------
        // Final validation
        // ------------------------
        if meta.id3.is_none()
            && meta.asf.is_none()
            && meta.riff.is_none()
            && meta.aiff.is_none()
            && meta.atom.is_none()
            && meta.vob.is_none()
        {
            return Err(ParserError::new("No supported metadata format found"));
        }

        Ok(meta)
    }
}

impl MetaExt for AudioMeta {
    fn title(&self) -> Option<String> {
        self.atom.as_ref().and_then(|m| m.title())
            .or_else(|| self.id3.as_ref().and_then(|m| m.title()))
            .or_else(|| self.asf.as_ref().and_then(|m| m.title()))
            .or_else(|| self.riff.as_ref().and_then(|m| m.title()))
            .or_else(|| self.aiff.as_ref().and_then(|m| m.title()))
            .or_else(|| self.vob.as_ref().and_then(|m| m.title()))
    }

    fn artist(&self) -> Option<String> {
        self.atom.as_ref().and_then(|m| m.artist())
            .or_else(|| self.id3.as_ref().and_then(|m| m.artist()))
            .or_else(|| self.asf.as_ref().and_then(|m| m.artist()))
            .or_else(|| self.riff.as_ref().and_then(|m| m.artist()))
            .or_else(|| self.aiff.as_ref().and_then(|m| m.artist()))
            .or_else(|| self.vob.as_ref().and_then(|m| m.artist()))
    }

    fn album(&self) -> Option<String> {
        self.atom.as_ref().and_then(|m| m.album())
            .or_else(|| self.id3.as_ref().and_then(|m| m.album()))
            .or_else(|| self.asf.as_ref().and_then(|m| m.album()))
            .or_else(|| self.riff.as_ref().and_then(|m| m.album()))
            .or_else(|| self.aiff.as_ref().and_then(|m| m.album()))
            .or_else(|| self.vob.as_ref().and_then(|m| m.album()))
    }

    fn genre(&self) -> Option<String> {
        self.atom.as_ref().and_then(|m| m.genre())
            .or_else(|| self.id3.as_ref().and_then(|m| m.genre()))
            .or_else(|| self.asf.as_ref().and_then(|m| m.genre()))
            .or_else(|| self.riff.as_ref().and_then(|m| m.genre()))
            .or_else(|| self.aiff.as_ref().and_then(|m| m.genre()))
            .or_else(|| self.vob.as_ref().and_then(|m| m.genre()))
    }

    fn year(&self) -> Option<String> {
        self.atom.as_ref().and_then(|m| m.year())
            .or_else(|| self.id3.as_ref().and_then(|m| m.year()))
            .or_else(|| self.asf.as_ref().and_then(|m| m.year()))
            .or_else(|| self.riff.as_ref().and_then(|m| m.year()))
            .or_else(|| self.aiff.as_ref().and_then(|m| m.year()))
            .or_else(|| self.vob.as_ref().and_then(|m| m.year()))
    }

    fn art(&self) -> Option<MetaArt> {
        self.id3.as_ref().and_then(|m| m.art())
            .or_else(|| self.atom.as_ref().and_then(|m| m.art()))
            .or_else(|| self.asf.as_ref().and_then(|m| m.art()))
            .or_else(|| self.riff.as_ref().and_then(|m| m.art()))
            .or_else(|| self.vob.as_ref().and_then(|m| m.art()))
            .or_else(|| self.aiff.as_ref().and_then(|m| m.art()))
    }
}

// ------------------------
// Format detection helper
// ------------------------

fn is_asf_header(data: &[u8]) -> bool {
    const ASF_HEADER_OBJECT: [u8; 16] = [
        0x30, 0x26, 0xB2, 0x75,
        0x8E, 0x66, 0xCF, 0x11,
        0xA6, 0xD9, 0x00, 0xAA,
        0x00, 0x62, 0xCE, 0x6C,
    ];

    data.len() >= 16 && data[..16] == ASF_HEADER_OBJECT
}


pub enum MetaArt {
    Bin(Vec<u8>),
    Url(String)
}

pub trait MetaExt {
    fn title(&self) -> Option<String>;
    fn artist(&self) -> Option<String>;
    fn album(&self) -> Option<String>;
    fn genre(&self) -> Option<String>;
    fn year(&self) -> Option<String>;
    fn art(&self) -> Option<MetaArt>;
}