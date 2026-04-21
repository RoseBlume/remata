use remata_xmp::Xmp;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() == 2 {
        (&args[1]).to_string()
    }
    else if args.len() > 2 {
        println!("Too many arguments.");
        println!("Usage: tester <file>");
        process::exit(1);
    }
    else {
        println!("Usage: tester <file>");
        process::exit(1);
    };
    let info = Xmp::from_path(input_path).unwrap();
    println!("{}", info);
}