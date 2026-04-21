use remata_archive::{
    ar::{AR_MAGIC, Ar},
    gzip::{GZIP_MAGIC, Gzip},
    xz::{XZ_MAGIC, Xz},
    zip::{ZIP_SIG, Zip},
    // zst::{Zstd},
    iso::{ISO_9660_SIG, Iso},
    // sevenz::{SEVENZ_MAGIC, SevenZ},
    rar::{RAR4_SIG, Rar4, RAR5_SIG, Rar5}
};
use std::env;
use std::fs::File;
use std::io::{Seek, SeekFrom, self, Error, ErrorKind};
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let output = get_info(input_file.to_string())?;
    println!("{}", output);
    Ok(())
}

fn get_info(input_file: String) -> io::Result<String> {
    let mut file = File::open(input_file)?;
    // let data = SevenZ::parse(&mut file)?;
    // println!("Data: {}", data);

    if let Ok(ar) = Ar::parse(&mut file) {
        println!("ar");
        return Ok(format!("{}", ar));
    }
    file.seek(SeekFrom::Start(0))?;
    if let Ok(ar) = Gzip::parse(&mut file) {
        println!("Gzip");
        return Ok(format!("{}", ar));
    }
    file.seek(SeekFrom::Start(0))?;
    if let Ok(ar) = Xz::parse(&mut file) {
        println!("Xz");
        return Ok(format!("{}", ar));
    }
    file.seek(SeekFrom::Start(0))?;
    if let Ok(ar) = Zip::parse(&mut file) {
        println!("Zip");
        return Ok(format!("{}", ar));
    }
    // file.seek(SeekFrom::Start(0))?;
    // if let Ok(ar) = SevenZ::parse(&mut file) {
    //     println!("SevenZ");
    //     return Ok(format!("{}", ar));
    // }

    // file.seek(SeekFrom::Start(0))?;
    // if let Ok(ar) = Zstd::parse(&mut file) {
    //     println!("Zstd");
    //     return Ok(format!("{}", ar));
    // }
    file.seek(SeekFrom::Start(0))?;
    if let Ok(ar) = Rar4::parse(&mut file) {
        println!("Rar4");
        return Ok(format!("{}", ar));
    }
    file.seek(SeekFrom::Start(0))?;
    if let Ok(ar) = Rar5::parse(&mut file) {
        println!("Rar5");
        return Ok(format!("{}", ar));
    }

    file.seek(SeekFrom::Start(0))?;
    if let Ok(ar) = Iso::parse(&mut file) {
        println!("Iso");
        return Ok(format!("{}", ar));
    }
    Err(Error::new(ErrorKind::InvalidData, "Invalid ar header"))
}