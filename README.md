# Remata

A lightweight, dependency-free Rust library for extracting metadata from multiple audio and container formats.

## ✨ Features

- 🔍 Supports multiple formats:
  - **ID3** (v1 & v2) – MP3
  - **RIFF INFO** – WAV / AVI
  - **ASF** – WMA / WMV
  - **Atom (MP4/M4A)** – iTunes metadata
  - **Vorbis Comments** – FLAC / Ogg
  - **AIFF**
- ⚡ Zero dependencies (std only)
- 🧩 Strongly-typed metadata structs
- 🛡 Graceful handling of missing fields
- 🖼 Optional binary data support (e.g. cover art)

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
remata = "1"
```

## Example
```rs
use std::env;
use std::fs;
use std::process;
use remata::{
    AtomMeta,
    Id3,
    RiffMeta,
    AsfMeta,
    AiffMeta,
    Vob
};

pub enum Meta {
    Atom(AtomMeta),
    Id3(Id3),
    Aiff(AiffMeta),
    Asf(AsfMeta),
    Riff(RiffMeta),
}

fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Read the file
    let data = match fs::read(file_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    if data.starts_with(b"fLaC") || data.starts_with(b"OggS") {
        // If you still have VOB parser
        match Vob::parse(&data) {
            Ok(v) => println!("{}", v),
            Err(e) => {
                eprintln!("Failed to parse VOB file: {}", e.message);
                process::exit(1);
            }
        }
    } else if data.starts_with(b"ID3") {
        // ID3 tag
        match Id3::parse(&data) {
            Ok(id) => println!("{}", id),
            Err(e) => {
                eprintln!("Failed to parse ID3: {}", e.message);
                process::exit(1);
            }
        }
    } else if data.len() > 8 && &data[4..8] == b"ftyp" {
        // MP4/M4A files start with 'ftyp'
        match AtomMeta::parse(&data) {
            Ok(meta) => println!("{}", meta),
            Err(e) => {
                eprintln!("Failed to parse MP4/M4A metadata: {}", e.message);
                process::exit(1);
            }
        }
    } else if data.starts_with(b"FORM") && data.len() > 8 && &data[8..12] == b"AIFF" {
        // AIFF files
        match AiffMeta::parse(&data) {
            Ok(aiff) => println!("{}", aiff),
            Err(e) => {
                eprintln!("Failed to parse AIFF metadata: {}", e.message);
                process::exit(1);
            }
        }
    } else if data.starts_with(b"RIFF") && data.len() > 8 && &data[8..12] == b"WAVE" {
        // RIFF/WAVE files
        match RiffMeta::parse(&data) {
            Ok(riff) => println!("{}", riff),
            Err(e) => {
                eprintln!("Failed to parse RIFF metadata: {}", e.message);
                process::exit(1);
            }
        }
    } else if data.starts_with(b"0&\xB2\x75") {
        // ASF files (Windows Media)
        match AsfMeta::parse(&data) {
            Ok(asf) => println!("{}", asf),
            Err(e) => {
                eprintln!("Failed to parse ASF metadata: {}", e.message);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Unsupported file format");
        process::exit(1);
    }
}
```