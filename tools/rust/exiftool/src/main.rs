use std::env;
use std::io;
mod helpers;
mod ifd;
mod tags;
mod gps;
mod starts;
mod process;
use process::process_path;

#[derive(Clone, Copy)]
pub enum ParseMode {
    Strict,
    Lenient,
}



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file|directory> [--output file.txt]", args[0]);
        return Ok(());
    }

    let input = &args[1];
    let mut output: Option<&str> = None;

    for i in 2..args.len() {
        if args[i] == "--output" && i + 1 < args.len() {
            output = Some(&args[i + 1]);
        }
    }

    let ifds = ifd::Ifd::from_file(input, ParseMode::Lenient)?;
    for ifd in ifds.iter() {
        println!("{}", ifd);
    }
    Ok(())
}


