use std::fs::File;
use std::io::{self, Read};

const TAG_MAKE: u16 = 0x010F;

#[derive(Debug, Clone)]
enum Endian {
    Little,
    Big,
}

fn read_u16(data: &[u8], offset: usize, endian: Endian) -> u16 {
    let b1 = data[offset] as u16;
    let b2 = data[offset + 1] as u16;

    match endian {
        Endian::Little => b1 | (b2 << 8),
        Endian::Big => (b1 << 8) | b2,
    }
}

fn read_u32(data: &[u8], offset: usize, endian: Endian) -> u32 {
    let b1 = data[offset] as u32;
    let b2 = data[offset + 1] as u32;
    let b3 = data[offset + 2] as u32;
    let b4 = data[offset + 3] as u32;

    match endian {
        Endian::Little => b1 | (b2 << 8) | (b3 << 16) | (b4 << 24),
        Endian::Big => (b1 << 24) | (b2 << 16) | (b3 << 8) | b4,
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("assets/images/First.JPG")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 8 {
        println!("Not a valid TIFF file");
        return Ok(());
    }

    // TIFF header
    let endian = match &data[0..2] {
        b"II" => Endian::Little,
        b"MM" => Endian::Big,
        _ => {
            println!("Invalid TIFF header");
            return Ok(());
        }
    };

    // Must be 42
    let magic = read_u16(&data, 2, endian.clone());
    if magic != 42 {
        println!("Invalid TIFF magic number");
        return Ok(());
    }

    // Offset to first IFD
    let ifd_offset = read_u32(&data, 4, endian.clone()) as usize;

    if ifd_offset >= data.len() {
        println!("Invalid IFD offset");
        return Ok(());
    }

    let num_entries = read_u16(&data, ifd_offset, endian.clone()) as usize;
    let mut offset = ifd_offset + 2;

    let mut make_value: Option<String> = None;

    for _ in 0..num_entries {
        let tag = read_u16(&data, offset, endian.clone());
        let _type = read_u16(&data, offset + 2, endian.clone());
        let _count = read_u32(&data, offset + 4, endian.clone());
        let value_or_offset = read_u32(&data, offset + 8, endian.clone());

        if tag == TAG_MAKE {
            // Type 2 = ASCII string in EXIF/TIFF
            let str_offset = value_or_offset as usize;

            if str_offset < data.len() {
                let mut end = str_offset;

                while end < data.len() && data[end] != 0 {
                    end += 1;
                }

                let s = String::from_utf8_lossy(&data[str_offset..end]).to_string();
                make_value = Some(s);
            }
        }

        offset += 12;
    }

    match make_value {
        Some(v) => println!("Camera Make: {}", v),
        None => println!("Make tag (0x010F) not found"),
    }

    Ok(())
}