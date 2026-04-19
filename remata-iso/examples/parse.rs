use std::fs::File;
use std::path::PathBuf;

use remata_iso::Iso;

fn main() -> std::io::Result<()> {
    // Build path: CARGO_MANIFEST_DIR/assets/archlinux-x86_64.iso
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets");
    path.push("archlinux-x86_64.iso");

    let file = File::open(&path)?;
    let iso = Iso::parse(file)?;
    println!("=== ISO INFO ===");
    if let Some(record) = iso.boot_record {
        println!("\nBoot Record Info:\n{}", record);
    }
    if let Some(pvd) = iso.primary_volume {
        println!("\nPrimary Volume Info:\n{}", pvd);
    }


    // println!("{}", iso.primary_volume.unwrap());
    // if let Some(pvd) = iso.primary_volume {
    //     println!("=== ISO INFO ===");

    //     if let Some(name) = pvd.volume_identifier {
    //         println!("Volume Name: {}", name);
    //     }

    //     if let Some(system) = pvd.system_identifier {
    //         println!("System: {}", system);
    //     }

    //     if let Some(blocks) = pvd.volume_block_count {
    //         println!("Blocks: {}", blocks);
    //     }

    //     if let Some(size) = pvd.logical_block_size {
    //         println!("Block Size: {}", size);
    //     }

    //     if let Some(app) = pvd.application_identifier {
    //         println!("Application: {}", app);
    //     }
    // } else {
    //     println!("No primary volume descriptor found");
    // }

    Ok(())
}