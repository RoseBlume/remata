use std::io::{Read, Seek, SeekFrom, self};
use std::fs::File;
use crate::simd::find_bytes;
/// Find Exif Signature
pub fn find_exif_signature(buf: &[u8]) -> Option<usize> {
    // JPEG EXIF
    if let Some(pos) = find_bytes(buf, b"\xFF\xE1") {
        // check Exif header after marker (SIMD-free validation)
        let check = pos + 4 + 2; // marker + length + skip
        if buf.len() >= check + 6 && &buf[check..check + 6] == b"Exif\0\0" {
            return Some(check + 6);
        }
    }

    // PNG EXIF
    if let Some(pos) = find_bytes(buf, b"eXIf") {
        return Some(pos);
    }

    // WebP EXIF
    if let Some(pos) = find_bytes(buf, b"EXIF") {
        return Some(pos);
    }

    // HEIF EXIF
    if let Some(pos) = find_bytes(buf, b"Exif") {
        return Some(pos);
    }

    None
}

/// Finds the TIFF header inside a JPEG EXIF segment.
/// Returns absolute file offset where TIFF header begins.
pub fn find_exif_tiff_start(file: &mut File) -> io::Result<u64> {
    file.seek(SeekFrom::Start(0))?;

    let mut soi = [0u8; 2];
    file.read_exact(&mut soi)?;

    if soi != [0xFF, 0xD8] {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not a JPEG file",
        ));
    }

    loop {
        let mut marker_prefix = [0u8; 1];
        file.read_exact(&mut marker_prefix)?;

        if marker_prefix[0] != 0xFF {
            continue;
        }

        let mut marker = [0u8; 1];
        file.read_exact(&mut marker)?;

        let marker = marker[0];

        // Skip padding FFs
        if marker == 0xFF {
            continue;
        }

        // Standalone markers
        if marker == 0xD9 || marker == 0xDA {
            break;
        }
        let mut buf = [0u8; 2];
        file.read_exact(&mut buf)?;
        // Read a big-endian u16 (JPEG markers are always big-endian)
        let length = u16::from_be_bytes(buf) as u64;


        if marker == 0xE1 {
            // APP1 segment
            let mut header = [0u8; 6];
            file.read_exact(&mut header)?;

            if &header == b"Exif\0\0" {
                let pos = file.stream_position()?;
                return Ok(pos);
            } else {
                file.seek(SeekFrom::Current((length - 8) as i64))?;
            }
        } else {
            file.seek(SeekFrom::Current((length - 2) as i64))?;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No EXIF segment found",
    ))
}

pub fn find_png_exif_start(file: &mut File) -> io::Result<u64> {
    file.seek(SeekFrom::Start(8))?; // skip PNG signature

    loop {
        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            break;
        }

        let length = u32::from_be_bytes(len_buf) as u64;

        let mut chunk_type = [0u8; 4];
        file.read_exact(&mut chunk_type)?;

        if &chunk_type == b"eXIf" {
            let pos = file.stream_position()?;
            return Ok(pos);
        }

        // skip data + CRC
        file.seek(SeekFrom::Current((length + 4) as i64))?;
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No EXIF chunk in PNG",
    ))
}


pub fn find_webp_exif_start(file: &mut File) -> io::Result<u64> {
    file.seek(SeekFrom::Start(12))?; // skip RIFF header

    loop {
        let mut chunk_header = [0u8; 8];
        if file.read_exact(&mut chunk_header).is_err() {
            break;
        }

        let chunk_type = &chunk_header[0..4];
        let chunk_size = u32::from_le_bytes([
            chunk_header[4],
            chunk_header[5],
            chunk_header[6],
            chunk_header[7],
        ]) as u64;

        if chunk_type == b"EXIF" {
            let pos = file.stream_position()?;
            return Ok(pos);
        }

        // Chunks are padded to even sizes
        let skip = if chunk_size % 2 == 1 {
            chunk_size + 1
        } else {
            chunk_size
        };

        file.seek(SeekFrom::Current(skip as i64))?;
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No EXIF in WebP",
    ))
}

pub fn find_heif_exif_start(file: &mut File) -> io::Result<u64> {
    file.seek(SeekFrom::Start(0))?;

    loop {
        let mut header = [0u8; 8];
        if file.read_exact(&mut header).is_err() {
            break;
        }

        let size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as u64;
        let box_type = &header[4..8];

        if box_type == b"Exif" {
            let pos = file.stream_position()?;
            return Ok(pos);
        }

        if size < 8 {
            break;
        }

        file.seek(SeekFrom::Current((size - 8) as i64))?;
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No EXIF in HEIF",
    ))
}