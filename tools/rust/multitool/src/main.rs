use std::env;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::fs::File;
mod simd;
mod exif;
use exif::{
    ifd::Ifd,
    ParseMode
};

use remata_macros::DisplayPretty;
use remata_audio::AudioMeta;
use remata_archive::{
    Iso,
    Torrent
};
use remata_exe::ExecutableType;
use remata_lnk::Lnk;
use remata_xmp::Xmp;


#[derive(Default, DisplayPretty)]
pub struct Metadata {
    exif: Option<Vec<Ifd>>,
    xmp: Option<Xmp>,
    audio: Option<AudioMeta>,
    iso: Option<Iso>,
    torrent: Option<Torrent>,
    exe: Option<ExecutableType>,
    lnk: Option<Lnk>,
}

impl Metadata {
    pub fn parse(path: &str, mode: ParseMode) -> io::Result<Self> {
        let mut meta = Self::default();
        let mut file = File::open(path)?;

        // Read first 16 bytes (enough for most signatures)
        let mut magic = [0u8; 16];
        let n = file.read(&mut magic)?;

        // Always rewind after reading header
        file.seek(SeekFrom::Start(0))?;

        let header = &magic[..n];

        // -------------------------
        // EXIF (JPEG / TIFF)
        // -------------------------
        if header.starts_with(&[0xFF, 0xD8]) // JPEG
            || header.starts_with(b"II*\0")  // TIFF (little endian)
            || header.starts_with(b"MM\0*")  // TIFF (big endian)
        {
            if let Ok(ifds) = Ifd::from_file(path, mode) {
                meta.exif = Some(ifds);
            }
        }

        // -------------------------
        // ISO (ISO9660)
        // Offset 0x8001 = "CD001"
        // -------------------------
        {
            let mut buf = [0u8; 5];
            file.seek(SeekFrom::Start(0x8001))?;
            if file.read_exact(&mut buf).is_ok() && &buf == b"CD001" {
                file.seek(SeekFrom::Start(0))?;
                if let Ok(iso) = Iso::parse(File::open(path)?) {
                    meta.iso = Some(iso);
                }
            }
            file.seek(SeekFrom::Start(0))?;
        }

        // -------------------------
        // Torrent (bencode: starts with 'd')
        // -------------------------
        if header.first() == Some(&b'd') {
            if let Ok(torrent) = Torrent::parse(File::open(path)?) {
                meta.torrent = Some(torrent);
            }
        }

        // -------------------------
        // Windows LNK
        // Signature: 4C 00 00 00
        // -------------------------
        if header.starts_with(&[0x4C, 0x00, 0x00, 0x00]) {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            meta.lnk = Some(Lnk::parse(&buffer));
            file.seek(SeekFrom::Start(0))?;
        }

        let xmp = Xmp::from_path(path);
        meta.xmp = match xmp {
            Ok(xmp_meta) => Some(xmp_meta),
            _ => None
        };

        // -------------------------
        // Executables (delegate detection internally)
        // -------------------------
        if let Ok(Some(exe)) = ExecutableType::parse(&mut file) {
            meta.exe = Some(exe);
        }

        // -------------------------
        // Audio (already self-detecting)
        // -------------------------
        meta.audio = AudioMeta::from_path(path).ok();

        Ok(meta)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file|directory> [--output file.txt]", args[0]);
        return Ok(());
    }

    let input = &args[1];
    let mut output: Option<String> = None;
    let mut mode = ParseMode::Lenient;
    for i in 2..args.len() {
        if args[i] == "--output" && i + 1 < args.len() {
            output = Some((&args[i + 1]).to_string());
        }
        if args[i] == "--mode" && i + 1 < args.len() {
            let choice = &args[i + 1].to_lowercase();
            if choice == "lenient" { mode = ParseMode::Lenient; }
            else if choice == "strict" { mode = ParseMode::Strict; }
            else {
                println!("Invalid value for --mode");
            }
        } 
    }

    // let ifds = Ifd::from_file(input, exif::ParseMode::Lenient)?;
    // for ifd in ifds.iter() {
    //     println!("{}", ifd);
    // }
    let metadata = Metadata::parse(input, mode)?;
    match output {
        Some(out) => {
            let mut file = File::create(&out)?;
            let output_data = format!("{}", metadata);
            file.write_all(output_data.as_bytes())?;
        }
        None => println!("{}", metadata)
    }

    Ok(())
}


