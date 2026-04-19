use std::env;
use std::fs::File;
use std::io::{Read, Write, BufReader};

fn extract_xmp(data: &[u8]) -> Option<String> {
    let start_tag = b"<x:xmpmeta";
    let end_tag = b"</x:xmpmeta>";

    if let Some(start) = data.windows(start_tag.len()).position(|w| w == start_tag) {
        if let Some(end) = data.windows(end_tag.len()).position(|w| w == end_tag) {
            let end = end + end_tag.len();
            let slice = &data[start..end];
            if let Ok(xmp_str) = String::from_utf8(slice.to_vec()) {
                return Some(xmp_str);
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: xmpdump <image_file> <output_file>");
        return;
    }

    let file_path = &args[1];
    let output_path = &args[2];
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    if let Err(e) = reader.read_to_end(&mut buffer) {
        eprintln!("Error reading file: {}", e);
        return;
    }

    match extract_xmp(&buffer) {
        Some(xmp) => {
            let mut file = File::create(output_path).expect("Failed to create file");
            file.write_all(xmp.as_bytes()).expect("Failed to write to file");
            // println!("{}", xmp)
        },
        None => println!("No XMP metadata found."),
    }
}