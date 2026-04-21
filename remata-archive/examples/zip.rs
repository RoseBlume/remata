use std::env;
use std::fs::File;
use std::io::{self};
use remata_archive::Zip;
fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let mut file = File::open(input_file)?;
    if let Ok(ar) = Zip::parse(&mut file) {
        println!("Zip");
        println!("{}", ar);
    }
    Ok(())
}