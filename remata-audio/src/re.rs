use std::env;
use std::process;
use remata::{
    AudioMeta
};



fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    match AudioMeta::from_path(file_path) {
        Ok(info) => println!("{}", info),
        Err(e) => {
            println!("{}", e.message);
            std::process::exit(1);
        },
    }
    // println!("")

    // Read the file



}