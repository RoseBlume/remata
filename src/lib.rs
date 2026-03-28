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
//! use your_crate_name::Id3;
//!
//! let data = fs::read("audio.mp3")?;
//!
//! let meta = Id3::parse(&data)?;
//! println!("{}", meta);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing RIFF (WAV)
//!
//! ```no_run
//! use std::fs;
//! use your_crate_name::RiffMeta;
//!
//! let data = fs::read("audio.wav")?;
//!
//! let meta = RiffMeta::parse(&data)?;
//!
//! if let Some(title) = meta.title {
//!     println!("Title: {}", title);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing ASF (WMA/WMV)
//!
//! ```no_run
//! use std::fs;
//! use your_crate_name::AsfMeta;
//!
//! let data = fs::read("audio.wma")?;
//!
//! let meta = AsfMeta::parse(&data)?;
//! println!("{}", meta);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing MP4 / M4A (Atom)
//!
//! ```no_run
//! use std::fs;
//! use your_crate_name::AtomMeta;
//!
//! let data = fs::read("audio.m4a")?;
//!
//! let meta = AtomMeta::parse(&data)?;
//!
//! if let Some(artist) = meta.artist {
//!     println!("Artist: {}", artist);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ---
//!
//! ### Example: Parsing FLAC / Ogg (Vorbis Comments)
//!
//! ```no_run
//! use std::fs;
//! use your_crate_name::Vob;
//!
//! let data = fs::read("audio.flac")?;
//!
//! let meta = Vob::parse(&data)?;
//!
//! println!("{}", meta);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ---
//!
//! ## Error Handling
//!
//! Each parser returns its own error type (e.g. `Id3Error`, `RiffError`, etc.).
//! These errors contain human-readable messages and can be easily propagated.
//!
//! ```no_run
//! use your_crate_name::Vob;
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
//! - Binary data (e.g. cover art) is exposed as raw bytes.
//!
//! ---
//!
//! ## Re-exports
//!
//! This crate re-exports all metadata types at the root for convenience.

mod readers;

pub use readers::{
    AiffMeta,
    Id3,
    RiffMeta,
    AsfMeta,
    Vob,
    AtomMeta,
};