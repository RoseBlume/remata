use remata_exe::windows::Windows;
use std::fs::File;
use std::path::PathBuf;
fn main() -> std::io::Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets");
    path.push("demo.exe");
    let mut file = File::open(&path)?;
    let pe = Windows::parse(&mut file)?;

    println!("{}", pe);
    // if let Some(ver_info) = pe.version_info {
    //     println!("Portable Version Info:\n{}", ver_info);
    // }
    // else {
    //     println!("No Info Found");
    // }
    Ok(())
}