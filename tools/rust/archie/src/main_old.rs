use anyhow::Result;
use std::{
    fs::File,
    path::{Path, PathBuf},
    io::{Read, Write}
};

use walkdir::WalkDir;

use tar::Builder;
// use zstd::stream::Encoder as ZstdEncoder;
// use xz2::write::XzEncoder;
// use ar::Builder as ArBuilder;

use remata_archive::Ar;

#[derive(Debug)]
enum Format {
    Tar,
    TarZstd,
    TarXz,
    ArFlat,        // correct ar usage (THIS is the proper one)
    ArTarZstd,     // ar containing tar.zst blob (optional mode)
    ArTarXz,
}

impl Format {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "tar" => Some(Self::Tar),
            "tar-zstd" => Some(Self::TarZstd),
            "tar-xz" => Some(Self::TarXz),
            "ar" => Some(Self::ArFlat),
            "ar-zstd" => Some(Self::ArTarZstd),
            "ar-xz" => Some(Self::ArTarXz),
            _ => None,
        }
    }
}
use std::env;
fn main() -> Result<()> {
    // let args = Args::parse();
    let args: Vec<String> = env::args().collect();
    let format = &args[1];
    let input = Path::new(&args[2]);
    let output = Path::new(&args[3]);

    let format = Format::parse(format)
        .expect("Invalid format. Use tar, tar-zstd, tar-xz, ar, ar-zstd, ar-xz");

    match format {
        // Format::Tar => create_tar(&args.input, &args.output)?,
        // Format::TarZstd => create_tar_zstd(&args.input, &args.output)?,
        // Format::TarXz => create_tar_xz(&args.input, &args.output)?,
        Format::ArFlat => create_ar_flat(&input, &output)?,
        // Format::ArTarZstd => create_ar_compressed(&args.input, &args.output, Compression::Zstd)?,
        // Format::ArTarXz => create_ar_compressed(&args.input, &args.output, Compression::Xz)?,
        _ => {}
    }

    println!("Created: {}", &output.display());
    let mut out_file = File::open("archive.ar")?;
    let text = Ar::parse(&mut out_file)?;
    println!("Info: {}", text);
    // let out_file_path = format!("{}", &output.display());

    Ok(())
}

fn append_dir(tar: &mut Builder<Box<dyn std::io::Write>>, input: &Path) -> Result<()> {
    for entry in WalkDir::new(input).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let rel = path.strip_prefix(input)?;
            tar.append_path_with_name(path, rel)?;
        }
    }
    Ok(())
}

// fn create_tar(input: &Path, output: &Path) -> Result<()> {
//     let file = File::create(output)?;
//     let mut tar = tar::Builder::new(file);
//     append_dir(&mut tar, input)?;
//     tar.finish()?;
//     Ok(())
// }

// fn create_tar_zstd(input: &Path, output: &Path) -> Result<()> {
//     let file = File::create(output)?;
//     let encoder = zstd::stream::Encoder::new(file, 0)?.auto_finish();
//     let mut tar = tar::Builder::new(encoder);
//     append_dir(&mut tar, input)?;
//     tar.finish()?;
//     Ok(())
// }

// fn create_tar_xz(input: &Path, output: &Path) -> Result<()> {
//     let file = File::create(output)?;
//     let encoder = xz2::write::XzEncoder::new(file, 6);
//     let mut tar = tar::Builder::new(encoder);
//     append_dir(&mut tar, input)?;
//     tar.finish()?;
//     Ok(())
// }

fn create_ar_flat(input: &Path, output: &Path) -> Result<()> {
    let output_file = File::create(output)?;
    let mut builder = ar::Builder::new(output_file);

    if input.is_file() {
        builder.append_path(input)?;
        return Ok(());
    }

    for entry in WalkDir::new(input).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            builder.append_path(path)?;
        }
    }

    Ok(())
}
enum Compression {
    Zstd,
    Xz,
}

// fn create_ar_compressed(input: &Path, output: &Path, comp: Compression) -> Result<()> {
//     let file = File::create(output)?;
//     let mut ar = ArBuilder::new(BufWriter::new(file));

//     let mut tar_buf = Vec::new();
//     {
//         let mut tar = Builder::new(&mut tar_buf);
//         append_dir(&mut tar, input)?;
//         tar.finish()?;
//     }

//     match comp {
//         Compression::Zstd => {
//             let mut encoder = ZstdEncoder::new(Vec::new(), 0)?;
//             encoder.write_all(&tar_buf)?;
//             let compressed = encoder.finish()?;
//             ar.append("archive.tar.zst", &mut &compressed[..])?;
//         }
//         Compression::Xz => {
//             let mut encoder = XzEncoder::new(Vec::new(), 6);
//             encoder.write_all(&tar_buf)?;
//             let compressed = encoder.finish()?;
//             ar.append("archive.tar.xz", &mut &compressed[..])?;
//         }
//     }

//     ar.finish()?;
//     Ok(())
// }

// fn create_ar_tar_zstd(input: &Path, output: &Path) -> Result<()> {
//     let mut tar_buf = Vec::new();
//     {
//         let mut tar = tar::Builder::new(&mut tar_buf);
//         append_dir(&mut tar, input)?;
//         tar.finish()?;
//     }

//     let mut encoder = zstd::stream::encode_all(&tar_buf[..], 0)?;

//     let file = File::create(output)?;
//     let mut builder = ar::Builder::new(file);

//     builder.append("archive.tar.zst", &mut &encoder[..])?;

//     Ok(())
// }