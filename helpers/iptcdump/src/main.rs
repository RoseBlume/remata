use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}

fn extract_iptc_from_jpeg(data: &[u8]) -> Option<Vec<u8>> {
    // let app13_marker = [0xFF, 0xED]; // APP13 segment
    let photoshop_sig = b"Photoshop 3.0\0";

    let mut i = 0;

    while i + 4 < data.len() {
        if data[i] == 0xFF && data[i + 1] == 0xED {
            // segment length (big-endian)
            let len = ((data[i + 2] as usize) << 8) | (data[i + 3] as usize);

            let start = i + 4;
            let end = (start + len - 2).min(data.len());

            let segment = &data[start..end];

            if let Some(pos) = find_subslice(segment, photoshop_sig) {
                let after_sig = pos + photoshop_sig.len();

                if after_sig < segment.len() {
                    // Remaining data is Photoshop Image Resources
                    // IPTC is inside resource blocks (we extract raw tail for simplicity)
                    return Some(segment[after_sig..].to_vec());
                }
            }

            i = end;
        } else {
            i += 1;
        }
    }

    None
}

fn hex_dump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}: ", i * 16);

        for b in chunk {
            print!("{:02x} ", b);
        }

        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }

        print!(" |");

        for b in chunk {
            let c = if b.is_ascii_graphic() || *b == b' ' {
                *b as char
            } else {
                '.'
            };
            print!("{}", c);
        }

        println!("|");
    }
}

fn main() -> io::Result<()> {
    let path = "assets/images/First.JPG";
    let data = read_file(path)?;

    match extract_iptc_from_jpeg(&data) {
        Some(iptc) => {
            println!("IPTC block found (hex dump):");
            hex_dump(&iptc);
        }
        None => {
            println!("No IPTC data found.");
        }
    }

    Ok(())
}