use std::env;
use std::fs;
use std::path::Path;
use std::process;

use remata_exif::ExifData;

fn process_path(path: &Path) {
    match ExifData::from_path(path.to_string_lossy().as_ref()) {
        Ok(exif) => {
            println!("==== {} ====", path.display());
            println!("{:#?}", exif);

            println!("\n---- Summary ----");
            println!("{}", exif);
            println!();
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
        }
    }
}

fn process_dir(dir: &Path) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", dir.display(), e);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            // Recursive (optional)
            process_dir(&path);
        } else if path.is_file() {
            process_path(&path);
        }
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next();

    let input = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: cargo run -- <file_or_directory>");
            process::exit(1);
        }
    };

    let path = Path::new(&input);

    if path.is_file() {
        process_path(path);
    } else if path.is_dir() {
        process_dir(path);
    } else {
        eprintln!("Invalid path: {}", input);
        process::exit(1);
    }
}