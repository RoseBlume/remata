use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

use xmp_codegen::compile_many_html;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  xmp_codegen <input-path> <output-file>");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  xmp_codegen schemas/ generated.rs");
        eprintln!("  xmp_codegen schemas/page.html generated.rs");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    let pages = collect_html_files(&input_path);

    if pages.is_empty() {
        eprintln!("No HTML files found in input path: {:?}", input_path);
        std::process::exit(1);
    }

    let contents: Vec<String> = pages
        .iter()
        .map(|p| fs::read_to_string(p).unwrap_or_else(|_| {
            panic!("Failed to read file: {:?}", p);
        }))
        .collect();

    let output = compile_many_html(&contents);

    fs::write(&output_path, output).unwrap_or_else(|_| {
        panic!("Failed to write output file: {:?}", output_path);
    });

    println!(
        "Generated {} from {} file(s) → {:?}",
        output_path.display(),
        contents.len(),
        output_path
    );
}

/// Collects all `.html` files from either:
/// - a single file
/// - a directory (recursive not required, but could be added later)
fn collect_html_files(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        return vec![path.to_path_buf()];
    }

    if path.is_dir() {
        return fs::read_dir(path)
            .unwrap_or_else(|_| panic!("Cannot read directory: {:?}", path))
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if path.extension()? == "html" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
    }

    vec![]
}