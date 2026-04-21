use remata_lnk::{
    Lnk
};

use std::fs;

fn main() {
    // Load a .lnk file (change path as needed)
    let path = "assets/lnks/CapCut.lnk";

    let data = fs::read(path)
        .expect("failed to read .lnk file");

    let lnk = Lnk::parse(&data);

    // -----------------------------
    // Pretty print
    // -----------------------------
    println!("{:#?}", lnk);

    // -----------------------------
    // Flags
    // -----------------------------
    // if let Some(flags) = lnk.flags {
    //     println!("Flags: {}", flags);

    //     if flags.contains(Flag::Unicode) {
    //         println!("→ Unicode enabled");
    //     }

    //     if flags.contains(Flag::RunAsUser) {
    //         println!("→ Runs as user");
    //     }
    // }

    // // -----------------------------
    // // File attributes
    // // -----------------------------
    // if let Some(attrs) = lnk.file_attributes {
    //     println!("Attributes: {}", attrs);

    //     if attrs.contains(Attribute::Hidden) {
    //         println!("→ File is hidden");
    //     }
    // }

    // // -----------------------------
    // // Raw bit access (if needed)
    // // -----------------------------
    // if let Some(flags) = lnk.flags {
    //     let raw = flags.bits();
    //     println!("Raw flag bits: 0x{:08X}", raw);
    // }

    // // -----------------------------
    // // Individual fields
    // // -----------------------------
    // println!("Create date: {:?}", lnk.create_date);
    // println!("Access date: {:?}", lnk.access_date);
    // println!("Modify date: {:?}", lnk.modify_date);

    // println!("Target size: {:?}", lnk.target_file_size);
    // println!("Icon index: {:?}", lnk.icon_index);
    // println!("Run window: {:?}", lnk.run_window);
    // println!("Hotkey: {:?}", lnk.hotkey);
}