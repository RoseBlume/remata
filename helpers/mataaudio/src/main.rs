use std::env;
use std::process;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use remata_audio::{
    AudioMeta
};
mod locations;


fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let mut file_path = args[1].as_str();
    file_path = match file_path {
        "MUSIC" => &*locations::MUSIC_FOLDER_PATH,
        "DOCUMENTS" => &*locations::DOCUMENTS_FOLDER_PATH,
        "DOWNLOADS" => &*locations::DOWNLOADS_FOLDER_PATH,
        "PICTURES" => &*locations::PICTURES_FOLDER_PATH,
        "VIDEOS" => &*locations::VIDEOS_FOLDER_PATH,
        _ => &args[1]
    };
    let output_path = &args[2];
    let path = Path::new(file_path);
    if !path.exists() {
        println!("Error: Path does not exist");
        process::exit(1);
    }
    if path.is_symlink() {
        println!("Error: Symlinks not accepted");
        process::exit(1);
    }
    println!("Collecting Files");
    let files: Vec<String> = if path.is_dir() {
        collect_files(file_path.to_string(), &["mp3", "m4a", "flac", "wav", "ogg", "wma"])
    }
    else {
        vec![file_path.to_string()]
    };
    println!("Getting Metadata");
    let metadata: Vec<AudioMeta> = get_meta(files.clone());

    println!("Outputting Metadata");
    let mut file = File::create(output_path).unwrap();
    for i in 0..metadata.len() {
        let content = format!("file_location: {}\nFormat: {}", files[i], metadata[i]);
        file.write_all(content.as_bytes()).expect("Failed to write");
    }
}



pub fn get_meta(files: Vec<String>) -> Vec<AudioMeta> {
    let mut meta: Vec<AudioMeta> = Vec::new();
    for file in files.iter() {
        match AudioMeta::from_path(file) {
            Ok(info) => meta.push(info),
            Err(e) => {
                println!("{}", e.message);
            }
        }
    }

    meta
}

pub fn collect_files(path: String, accepted_extensions: &[&str]) -> Vec<String> {
    let mut files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files(path.to_string_lossy().to_string(), accepted_extensions));
            } else if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if accepted_extensions.contains(&ext_str) {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    files
}
