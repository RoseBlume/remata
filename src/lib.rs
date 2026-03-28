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
mod error;
mod readers;

pub use readers::{
    AiffMeta,
    Id3,
    RiffMeta,
    AsfMeta,
    Vob,
    AtomMeta,
};
pub use error::ParserError;