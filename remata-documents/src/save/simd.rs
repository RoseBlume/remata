pub fn contains_meta(buf: &[u8]) -> bool {
    let needle = b"<meta";

    let mut i = 0;
    while i + 16 <= buf.len() {
        let chunk = &buf[i..i + 16];

        if chunk.windows(5).any(|w| w == needle) {
            return true;
        }

        i += 16;
    }

    buf.windows(5).any(|w| w == needle)
}