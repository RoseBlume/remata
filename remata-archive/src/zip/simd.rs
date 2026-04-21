#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

use super::{
    ArchiveExtraDataRecord,
    LocalFileHeader,
    CentralDirectoryHeader,
    DataDescriptor,
    Zip,
    LOCAL_HEADER_SIG,
    CENTRAL_FILE_HEADER_SIGNATURE,
    DESC_SIG,
    ARCHIVE_EXTRA_DATA_REC_SIG
};

use std::io::ErrorKind;

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn process_avx2(buffer: &[u8], mut i: usize, zip: &mut Zip) -> usize {


    const CHUNK: usize = 32;
    let len = buffer.len();

    let p_vec = _mm256_set1_epi8(b'P' as i8);
    let k_vec = _mm256_set1_epi8(b'K' as i8);

    while i + CHUNK + 1 <= len {
        let ptr = unsafe { buffer.as_ptr().add(i) };

        let block = unsafe { _mm256_loadu_si256(ptr as *const __m256i) };
        let next  = unsafe { _mm256_loadu_si256(ptr.add(1) as *const __m256i) };

        let cmp_p = _mm256_cmpeq_epi8(block, p_vec);
        let cmp_k = _mm256_cmpeq_epi8(next, k_vec);

        let mut mask =
            (_mm256_movemask_epi8(cmp_p) &
             _mm256_movemask_epi8(cmp_k)) as u32;
        let mut next_i = i + CHUNK;

        while mask != 0 {
            let bit = mask.trailing_zeros() as usize;
            let pos = i + bit;

            if pos + 4 > len {
                return pos;
            }

            let sig = u32::from_le_bytes([
                buffer[pos],
                buffer[pos + 1],
                buffer[pos + 2],
                buffer[pos + 3],
            ]);

            let slice = &buffer[pos + 4..];

            let result = if sig == LOCAL_HEADER_SIG {
                LocalFileHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.local_file_headers.push(h);
                        c
                    })
            } else if sig == CENTRAL_FILE_HEADER_SIGNATURE {
                CentralDirectoryHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.central_directory_headers.push(h);
                        c
                    })
            } else if sig == DESC_SIG {
                DataDescriptor::parse(slice)
                    .map(|(d, c)| {
                        if let Some(last) = zip.local_file_headers.last_mut() {
                            last.merge(&d);
                        }
                        zip.data_descriptors.push(d);
                        c
                    })
            } else if sig == ARCHIVE_EXTRA_DATA_REC_SIG {
                ArchiveExtraDataRecord::parse(slice)
                    .map(|(r, c)| {
                        zip.archive_extra_data_records.push(r);
                        c
                    })
            } else {
                mask &= mask - 1;
                continue;
            };

            match result {
                Ok(consumed) => {
                    let end = pos + 4 + consumed;
                    if end > next_i {
                        next_i = end;
                    }
                }

                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                    return pos;
                }

                Err(_) => {
                    // ignore
                }
            }

            mask &= mask - 1;
        }

        i = next_i.min(buffer.len());
    }

    i
}



#[target_feature(enable = "sse2")]
#[cfg(target_arch = "x86_64")]
pub unsafe fn process_sse2(buffer: &[u8], mut i: usize, zip: &mut Zip) -> usize {
    const CHUNK: usize = 16;
    let len = buffer.len();

    let p_vec = _mm_set1_epi8(b'P' as i8);
    let k_vec = _mm_set1_epi8(b'K' as i8);

    while i + CHUNK + 1 <= len {
        let ptr = unsafe { buffer.as_ptr().add(i) };

        let block = unsafe { _mm_loadu_si128(ptr as *const __m128i) };
        let next  = unsafe { _mm_loadu_si128(ptr.add(1) as *const __m128i) };

        let cmp_p = _mm_cmpeq_epi8(block, p_vec);
        let cmp_k = _mm_cmpeq_epi8(next, k_vec);

        let mut mask =
            (_mm_movemask_epi8(cmp_p) & _mm_movemask_epi8(cmp_k)) as u32;

        let mut next_i = i + CHUNK;

        while mask != 0 {
            let bit = mask.trailing_zeros() as usize;
            let pos = i + bit;

            if pos + 4 > len {
                return pos;
            }

            let sig = u32::from_le_bytes([
                buffer[pos],
                buffer[pos + 1],
                buffer[pos + 2],
                buffer[pos + 3],
            ]);

            let slice = &buffer[pos + 4..];

            let result = if sig == LOCAL_HEADER_SIG {
                LocalFileHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.local_file_headers.push(h);
                        c
                    })
            } else if sig == CENTRAL_FILE_HEADER_SIGNATURE {
                CentralDirectoryHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.central_directory_headers.push(h);
                        c
                    })
            } else if sig == DESC_SIG {
                DataDescriptor::parse(slice)
                    .map(|(d, c)| {
                        if let Some(last) = zip.local_file_headers.last_mut() {
                            last.merge(&d);
                        }
                        zip.data_descriptors.push(d);
                        c
                    })
            } else if sig == ARCHIVE_EXTRA_DATA_REC_SIG {
                ArchiveExtraDataRecord::parse(slice)
                    .map(|(r, c)| {
                        zip.archive_extra_data_records.push(r);
                        c
                    })
            } else {
                mask &= mask - 1;
                continue;
            };

            match result {
                Ok(consumed) => {
                    let end = pos + 4 + consumed;
                    if end > next_i {
                        next_i = end;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return pos;
                }
                Err(_) => {}
            }

            mask &= mask - 1;
        }

        i = next_i.min(len);
    }

    i
}


#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn process_neon(buffer: &[u8], mut i: usize, zip: &mut Zip) -> usize {
    const CHUNK: usize = 16;
    let len = buffer.len();

    let p_vec = vdupq_n_u8(b'P');
    let k_vec = vdupq_n_u8(b'K');

    while i + CHUNK + 1 <= len {
        let ptr = buffer.as_ptr().add(i);

        let block = vld1q_u8(ptr);
        let next  = vld1q_u8(ptr.add(1));

        // Compare bytes
        let cmp_p = vceqq_u8(block, p_vec);
        let cmp_k = vceqq_u8(next, k_vec);

        // AND both comparisons
        let combined = vandq_u8(cmp_p, cmp_k);

        // NEON has no movemask → extract to array
        let mut bytes = [0u8; 16];
        vst1q_u8(bytes.as_mut_ptr(), combined);

        let mut mask: u32 = 0;
        for j in 0..16 {
            if bytes[j] != 0 {
                mask |= 1 << j;
            }
        }

        let mut next_i = i + CHUNK;

        while mask != 0 {
            let bit = mask.trailing_zeros() as usize;
            let pos = i + bit;

            if pos + 4 > len {
                return pos;
            }

            let sig = u32::from_le_bytes([
                buffer[pos],
                buffer[pos + 1],
                buffer[pos + 2],
                buffer[pos + 3],
            ]);

            let slice = &buffer[pos + 4..];

            let result = if sig == LOCAL_HEADER_SIG {
                LocalFileHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.local_file_headers.push(h);
                        c
                    })
            } else if sig == CENTRAL_FILE_HEADER_SIGNATURE {
                CentralDirectoryHeader::parse(slice)
                    .map(|(h, c)| {
                        zip.central_directory_headers.push(h);
                        c
                    })
            } else if sig == DESC_SIG {
                DataDescriptor::parse(slice)
                    .map(|(d, c)| {
                        if let Some(last) = zip.local_file_headers.last_mut() {
                            last.merge(&d);
                        }
                        zip.data_descriptors.push(d);
                        c
                    })
            } else if sig == ARCHIVE_EXTRA_DATA_REC_SIG {
                ArchiveExtraDataRecord::parse(slice)
                    .map(|(r, c)| {
                        zip.archive_extra_data_records.push(r);
                        c
                    })
            } else {
                mask &= mask - 1;
                continue;
            };

            match result {
                Ok(consumed) => {
                    let end = pos + 4 + consumed;
                    if end > next_i {
                        next_i = end;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return pos;
                }
                Err(_) => {}
            }

            mask &= mask - 1;
        }

        i = next_i.min(len);
    }

    i
}