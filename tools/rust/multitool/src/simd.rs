#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86_64")]
pub fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {


    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }

    unsafe {
        let len = haystack.len();
        let n = needle.len();
        let mut i = 0;

        let first = _mm_set1_epi8(needle[0] as i8);

        while i + 16 <= len {
            let ptr = haystack.as_ptr().add(i) as *const __m128i;
            let block = _mm_loadu_si128(ptr);

            let cmp = _mm_cmpeq_epi8(block, first);
            let mask = _mm_movemask_epi8(cmp);

            if mask != 0 {
                let mut m = mask as u32;

                while m != 0 {
                    let bit = m.trailing_zeros() as usize;
                    let pos = i + bit;

                    if pos + n <= len && &haystack[pos..pos + n] == needle {
                        return Some(pos);
                    }

                    m &= m - 1;
                }
            }

            i += 16;
        }

        for j in i..=len.saturating_sub(n) {
            if &haystack[j..j + n] == needle {
                return Some(j);
            }
        }
    }

    None
}

#[cfg(target_arch = "aarch64")]
pub fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {


    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }

    unsafe {
        let len = haystack.len();
        let n = needle.len();
        let mut i = 0;

        let first = vdupq_n_u8(needle[0]);

        while i + 16 <= len {
            let ptr = haystack.as_ptr().add(i);
            let block = vld1q_u8(ptr);

            let cmp = vceqq_u8(block, first);

            let high = vshrq_n_u8(cmp, 7);
            let packed = vreinterpretq_u64_u8(high);

            let mask =
                vgetq_lane_u64(packed, 0) |
                (vgetq_lane_u64(packed, 1) << 8);

            let mut m = mask;

            while m != 0 {
                let bit = m.trailing_zeros() as usize;
                let pos = i + bit;

                if pos + n <= len && &haystack[pos..pos + n] == needle {
                    return Some(pos);
                }

                m &= m - 1;
            }

            i += 16;
        }

        for j in i..=len.saturating_sub(n) {
            if &haystack[j..j + n] == needle {
                return Some(j);
            }
        }
    }

    None
}