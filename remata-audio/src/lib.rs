//! # Media Metadata Parser
//!
//! A lightweight, dependency-free Rust library for extracting metadata from
//! multiple audio and container formats.
//!
//! ## Supported Formats
//!
//! This crate provides parsers for:
//!
//! - **ID3** (v1 and v2) – MP3 metadata
//! - **RIFF INFO** – WAV/AVI metadata
//! - **ASF** – Windows Media formats (WMA/WMV)
//! - **Atom (MP4/M4A)** – iTunes-style metadata
//! - **Vorbis Comments** – FLAC and Ogg containers
//! - **AIFF** – Audio Interchange File Format
//!
//! Each format is parsed into a strongly-typed Rust structure with optional fields.
//!
//! ## Design Goals
//!
//! - ✅ Zero dependencies (standard library only)
//! - ✅ Fast and minimal allocations
//! - ✅ Unified, ergonomic API
//! - ✅ Graceful handling of missing fields
//!
//! ---
//!
//! ## Quick Start
//!
//! ### Example: Parsing ID3 (MP3)
//!
//! ```no_run
//! use std::fs;
//! use remata::Id3;
//! use remata::ParserError;
//!
//! let data = fs::read("audio.mp3")?;
//!
//! let meta: Result<Id3, ParserError> = Id3::parse(&data);
//! println!("{}", meta?);
//! # Ok::<(), ParserError>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing RIFF (WAV)
//!
//! ```no_run
//! use std::fs;
//! use remata::RiffMeta;
//! use remata::ParserError;
//!
//! let data = fs::read("audio.wav")?;
//!
//! let meta: Result<RiffMeta, ParserError> = RiffMeta::parse(&data);
//!
//! if let Ok(meta) = meta {
//!     if let Some(title) = meta.title {
//!         println!("Title: {}", title);
//!     }
//! }
//! # Ok::<(), ParserError>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing ASF (WMA/WMV)
//!
//! ```no_run
//! use std::fs;
//! use remata::AsfMeta;
//! use remata::ParserError;
//!
//! let data = fs::read("audio.wma")?;
//!
//! let meta: Result<AsfMeta, ParserError> = AsfMeta::parse(&data);
//! println!("{}", meta?);
//! # Ok::<(), ParserError>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing MP4 / M4A (Atom)
//!
//! ```no_run
//! use std::fs;
//! use remata::AtomMeta;
//! use remata::ParserError;
//!
//! let data = fs::read("audio.m4a")?;
//!
//! let meta: Result<AtomMeta, ParserError> = AtomMeta::parse(&data);
//!
//! if let Ok(meta) = meta {
//!     if let Some(artist) = meta.artist {
//!         println!("Artist: {}", artist);
//!     }
//! }
//! # Ok::<(), ParserError>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing FLAC / Ogg (Vorbis Comments)
//!
//! ```no_run
//! use std::fs;
//! use remata::Vob;
//! use remata::ParserError;
//!
//! let data = fs::read("audio.flac")?;
//!
//! let meta: Result<Vob, ParserError> = Vob::parse(&data);
//! println!("{}", meta?);
//! # Ok::<(), ParserError>(())
//! ```
//!
//! ---
//!
//! ## Error Handling
//!
//! All parsers now return the unified [`ParserError`] type. This error contains
//! a human-readable message and can be propagated or logged.
//!
//! ```no_run
//! use remata::{Vob, ParserError};
//!
//! match Vob::parse(&[]) {
//!     Ok(meta) => println!("{}", meta),
//!     Err(err) => eprintln!("Error: {}", err.message),
//! }
//! ```
//!
//! ---
//!
//! ## Notes
//!
//! - Not all metadata fields are guaranteed to be present.
//! - Unknown or unsupported tags are safely ignored.
//! - Binary data (e.g., cover art) is exposed as raw bytes.
//!
//! ---
//!
//! ## Re-exports
//!
//! All metadata types are re-exported at the crate root for convenience.
#![deny(missing_docs)]
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;

mod aiff;
pub use aiff::AiffMeta;

mod id3;
pub use id3::Id3;

mod riff;
pub use riff::RiffMeta;

mod asf;
pub use asf::AsfMeta;

mod vob;
pub use vob::Vob;

mod atom;
pub use atom::AtomMeta;

mod ape;
pub use ape::Ape;

mod dsf;
pub use dsf::Dsf;

mod wavpack;
pub use wavpack::WavPack;

mod mpc;
pub use mpc::Mpc;

mod audible;
pub use audible::Audible;

pub use remata_core::ParserError;


use remata_macros::DisplayPretty;

/// Aggregates metadata extracted from various supported audio and container formats.
///
/// This structure acts as a unified container for metadata parsed from different
/// file types and tagging systems. Each field corresponds to a specific format
/// or metadata standard, and is optional to allow flexible, format-agnostic parsing.
///
/// In typical usage, only one or a few of these fields will be populated depending
/// on the input file type and embedded metadata.
#[derive(Default, Clone, DisplayPretty)]
pub struct AudioMeta {
    /// ID3 metadata, commonly used in formats like MP3 and sometimes embedded
    /// in other containers (e.g., AIFF, WAV, or DSF).
    pub id3: Option<Id3>,

    /// Metadata extracted from AIFF (Audio Interchange File Format) files.
    pub aiff: Option<AiffMeta>,

    /// Metadata extracted from RIFF INFO chunks (used in WAV/AVI containers).
    pub riff: Option<RiffMeta>,

    /// Metadata extracted from ASF (Advanced Systems Format), used by formats
    /// such as WMA and WMV.
    pub asf: Option<AsfMeta>,

    /// Metadata and structure extracted from VOB (DVD Video Object) files.
    ///
    /// This may include audio-related information within MPEG program streams.
    pub vob: Option<Vob>,

    /// Metadata extracted from atom-based containers (e.g., MP4/M4A).
    ///
    /// Includes iTunes-style metadata and other ISO Base Media File Format atoms.
    pub atom: Option<AtomMeta>,

    /// Metadata extracted from WavPack audio files.
    pub wavpack: Option<WavPack>,

    /// Metadata extracted from DSF (DSD Stream File) audio files.
    pub dsf: Option<Dsf>,

    /// Metadata extracted from APE (Monkey's Audio) files and APE tag blocks.
    pub ape: Option<Ape>,

    /// Metadata extracted from Musepack (MPC) audio files.
    pub mpc: Option<Mpc>,

    /// Metadata extracted from Audible audiobook containers.
    ///
    /// Includes custom tag structures and embedded metadata atoms.
    pub audible: Option<Audible>,
}


impl AudioMeta {
    /// Parses metadata from a file path.
    ///
    /// This function:
    /// - Opens the file
    /// - Wraps it in a buffered reader
    /// - Delegates to [`AudioMeta::parse`]
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The format is unsupported
    /// - Parsing fails
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ParserError> {
        let file = File::open(path)
            .map_err(|e| ParserError { message: e.to_string() })?;

        let mut reader = BufReader::new(file);

        Self::parse(&mut reader)
    }

    /// Parses audio metadata from any seekable reader.
    ///
    /// This function:
    /// - Detects the underlying audio/container format using magic bytes
    /// - Attempts to parse multiple metadata formats (ID3, RIFF, ASF, etc.)
    /// - Supports modern formats like WavPack, DSF, and APE
    /// - Safely resets the reader between parsing attempts
    ///
    /// # Behavior
    /// - Multiple metadata formats may coexist (e.g., APE + ID3)
    /// - Parsing is best-effort: failures in one format do not stop others
    /// - Returns an error only if *no* supported format is found
    ///
    /// # Errors
    /// Returns `ParserError` if:
    /// - The reader cannot be read/seeked
    /// - No supported format is detected
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParserError> {
        let mut meta = AudioMeta::default();

        // Read enough bytes for detection
        let mut header = [0u8; 16];
        let read = reader.read(&mut header)?;
        reader.seek(SeekFrom::Start(0))?;

        // ------------------------
        // Format Detection FIRST
        // ------------------------

        // ---- WavPack ----
        if read >= 4 && &header[0..4] == b"wvpk" {
            if let Ok(v) = WavPack::parse(&mut *reader) {
                meta.wavpack = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ---- DSF ----
        if read >= 4 && &header[0..4] == b"DSD " {
            if let Ok(v) = Dsf::parse(&mut *reader) {
                meta.dsf = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ---- APE ----
        if read >= 4 && &header[0..4] == b"MAC " {
            if let Ok(v) = Ape::parse(&mut *reader) {
                meta.ape = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ---- MPC ----
        if read >= 3 && &header[0..3] == b"MP+" {
            if let Ok(v) = Mpc::parse(&mut *reader) {
                meta.mpc = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ---- Audible (.aa / m4b atoms) ----
        if read >= 8 {
            let mut probe = [0u8; 8];
            reader.read_exact(&mut probe)?;
            reader.seek(SeekFrom::Start(0))?;

            // MP4-style container (Audible lives inside this)
            if &probe[4..8] == b"ftyp" {
                if let Ok(v) = Audible::parse(&mut *reader) {
                    meta.audible = Some(v);
                }
                reader.seek(SeekFrom::Start(0))?;
            }
        }

        // ------------------------
        // ID3 (header or footer)
        // ------------------------

        if read >= 3 && &header[0..3] == b"ID3" {
            if let Ok(v) = Id3::parse(&mut *reader) {
                meta.id3 = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        } else {
            if let Ok(len) = reader.seek(SeekFrom::End(0)) {
                if len >= 128 {
                    reader.seek(SeekFrom::End(-128))?;

                    let mut tag = [0u8; 3];
                    if reader.read_exact(&mut tag).is_ok() && &tag == b"TAG" {
                        reader.seek(SeekFrom::Start(0))?;
                        if let Ok(v) = Id3::parse(&mut *reader) {
                            meta.id3 = Some(v);
                        }
                    }
                }
                reader.seek(SeekFrom::Start(0))?;
            }
        }

        // ------------------------
        // ASF (WMA/WMV)
        // ------------------------
        if read >= 16 && is_asf_header(&header) {
            if let Ok(v) = AsfMeta::parse(&mut *reader) {
                meta.asf = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ------------------------
        // RIFF (WAV/AVI)
        // ------------------------
        if read >= 12 && &header[0..4] == b"RIFF" {
            let mut riff_check = [0u8; 12];
            reader.read_exact(&mut riff_check)?;
            if &riff_check[8..12] != b"WEBP" {
                reader.seek(SeekFrom::Start(0))?;
                if let Ok(v) = RiffMeta::parse(&mut *reader) {
                    meta.riff = Some(v);
                }
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ------------------------
        // AIFF
        // ------------------------
        if read >= 4 && &header[0..4] == b"FORM" {
            if let Ok(v) = AiffMeta::parse(&mut *reader) {
                meta.aiff = Some(v);
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ------------------------
        // MP4 / Atom
        // ------------------------
        if read >= 8 {
            let mut ftyp = [0u8; 8];
            reader.read_exact(&mut ftyp)?;
            if &ftyp[4..8] == b"ftyp" {
                reader.seek(SeekFrom::Start(0))?;
                if let Ok(v) = AtomMeta::parse(&mut *reader) {
                    meta.atom = Some(v);
                }
            }
            reader.seek(SeekFrom::Start(0))?;
        }

        // ------------------------
        // VOB / MPEG / FLAC / OGG
        // ------------------------
        if read >= 4 {
            if &header[0..4] == [0x00, 0x00, 0x01, 0xBA]
                || &header[0..4] == b"fLaC"
                || &header[0..4] == b"OggS"
            {
                if let Ok(v) = Vob::parse(&mut *reader) {
                    meta.vob = Some(v);
                }
                reader.seek(SeekFrom::Start(0))?;
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
            && meta.wavpack.is_none()
            && meta.dsf.is_none()
            && meta.ape.is_none()
            && meta.mpc.is_none()
            && meta.audible.is_none()
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

/// Represents artwork associated with audio metadata.
///
/// Artwork can either be embedded directly as binary data
/// (such as JPEG or PNG bytes) or referenced via an external URL.
pub enum MetaArt {
    /// Embedded binary artwork data.
    ///
    /// Typically contains raw image bytes (e.g., JPEG, PNG).
    /// The exact format is not enforced and must be inferred
    /// by the consumer.
    Bin(Vec<u8>),

    /// URL pointing to externally hosted artwork.
    ///
    /// This may reference cover art stored on a remote server.
    Url(String),
}

/// Extension trait for accessing common metadata fields across formats.
///
/// This trait provides a unified interface over multiple audio metadata
/// implementations (e.g., ID3, APE, RIFF, MP4 atoms), allowing consumers
/// to retrieve common properties without needing to know the underlying format.
///
/// Implementations typically map format-specific fields into a shared view.
pub trait MetaExt {
    /// Attempts to get song title using a best effort approach
    fn title(&self) -> Option<String>;
    /// Attempts to get song artist using a best effort approach
    fn artist(&self) -> Option<String>;
    /// Attempts to get album title using a best effort approach
    fn album(&self) -> Option<String>;
    /// Attempts to get song genre using a best effort approach
    fn genre(&self) -> Option<String>;
    /// Attempts to get song year using a best effort approach
    fn year(&self) -> Option<String>;
    /// Attempts to get cover art using a best effort approach
    fn art(&self) -> Option<MetaArt>;
}