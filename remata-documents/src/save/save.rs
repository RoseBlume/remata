use std::collections::HashMap;
use std::io::{self, Read};

//
// ======================================================
// PUBLIC API
// ======================================================
//

/// Streaming HTML metadata parser.
///
/// Extracts:
/// - `<title>`
/// - `<meta name="...">`
/// - `<meta property="og:*">`
///
/// Uses a streaming buffer and does not build a DOM.
pub struct HtmlMetaStream<R: Read> {
    reader: R,
    buffer: [u8; 4096],
    len: usize,
    pos: usize,
    eof: bool,
}

/// Final extracted metadata from HTML.
#[derive(Debug, Default, Clone)]
pub struct HtmlMeta {
    /// Document title.
    pub title: Option<String>,

    /// `<meta name="...">` values.
    pub meta: HashMap<String, String>,

    /// `<meta property="...">` values (OpenGraph, etc).
    pub properties: HashMap<String, String>,

    /// Parsed keywords list (if present).
    pub keywords: Option<Vec<String>>,
}

impl HtmlMeta {
    pub fn parse<R: Read>(reader: R) -> io::Result<Self> {
        HtmlMetaStream::new(reader).parse()
    }
}

impl<R: Read> HtmlMetaStream<R> {
    /// Creates a new streaming HTML parser.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: [0; 4096],
            len: 0,
            pos: 0,
            eof: false,
        }
    }

    /// Parses the entire HTML stream into `HtmlMeta`.
    ///
    /// # Errors
    /// Returns I/O errors from the underlying reader.
    pub fn parse(mut self) -> io::Result<HtmlMeta> {
        let mut meta = HtmlMeta::default();

        while !self.eof {
            self.fill_buffer()?;

            while self.pos < self.len {
                if let Some(tag) = self.next_tag() {
                    match tag {
                        HtmlTag::Title(t) => meta.title = Some(t),
                        HtmlTag::Meta(m) => self.apply_meta(&mut meta, m),
                    }
                }
            }
        }

        Ok(meta)
    }

    /// Fills internal buffer from reader.
    fn fill_buffer(&mut self) -> io::Result<()> {
        self.len = self.reader.read(&mut self.buffer)?;
        self.pos = 0;

        if self.len == 0 {
            self.eof = true;
        }

        Ok(())
    }

    /// Finds next tag in buffer.
    fn next_tag(&mut self) -> Option<HtmlTag> {
        while self.pos < self.len {
            if self.buffer[self.pos] == b'<' {
                if let Some(end) = self.find_tag_end(self.pos) {
                    let slice = &self.buffer[self.pos..end];
                    self.pos = end;

                    return parse_tag(slice);
                }
            }
            self.pos += 1;
        }
        None
    }

    /// Finds closing `>` for a tag.
    fn find_tag_end(&self, start: usize) -> Option<usize> {
        let mut i = start;
        while i < self.len {
            if self.buffer[i] == b'>' {
                return Some(i + 1);
            }
            i += 1;
        }
        None
    }

    /// Applies parsed meta tag into output structure.
    fn apply_meta(&self, out: &mut HtmlMeta, tag: MetaTag) {
        if let Some(name) = tag.name {
            if let Some(ref content) = tag.content {
                out.meta.insert(name.clone(), content.clone().to_string());

                if name == "keywords" {
                    out.keywords = Some(
                        content
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                    );
                }
            }
        }

        if let Some(prop) = tag.property {
            if let Some(ref content) = tag.content {
                out.properties.insert(prop, content.to_string());
            }
        }
    }
}

//
// ======================================================
// TAG MODEL
// ======================================================
//

/// Parsed HTML tag types.
enum HtmlTag {
    Title(String),
    Meta(MetaTag),
}

/// Parsed `<meta>` tag.
struct MetaTag {
    name: Option<String>,
    property: Option<String>,
    content: Option<String>,
}

//
// ======================================================
// TAG PARSER (QUOTE-AWARE)
// ======================================================
//

fn parse_tag(tag: &[u8]) -> Option<HtmlTag> {
    let lower = to_lower(tag);

    if lower.starts_with("<title") {
        return Some(HtmlTag::Title(parse_title(tag)));
    }

    if !lower.starts_with("<meta") {
        return None;
    }

    let mut i = 0;
    let mut name = None;
    let mut property = None;
    let mut content = None;

    while i < tag.len() {
        skip_ws(tag, &mut i);

        let key = read_ident(tag, &mut i)?;
        skip_ws(tag, &mut i);

        if i >= tag.len() || tag[i] != b'=' {
            continue;
        }

        i += 1;
        skip_ws(tag, &mut i);

        let value = read_quoted(tag, &mut i)?;

        match key.as_str() {
            "name" => name = Some(value),
            "property" => property = Some(value),
            "content" => content = Some(value),
            _ => {}
        }
    }

    Some(HtmlTag::Meta(MetaTag {
        name,
        property,
        content,
    }))
}

//
// ======================================================
// TITLE PARSER
// ======================================================
//

fn parse_title(tag: &[u8]) -> String {
    let lower = to_lower(tag);

    let start = match lower.find("<title>") {
        Some(p) => p + 7,
        None => return String::new(),
    };

    let end = match lower.find("</title>") {
        Some(p) => p,
        None => tag.len(),
    };

    String::from_utf8_lossy(&tag[start..end])
        .trim()
        .to_string()
}

//
// ======================================================
// ATTRIBUTE TOKENIZER (QUOTE-AWARE)
// ======================================================
//

fn read_quoted(buf: &[u8], i: &mut usize) -> Option<String> {
    if *i >= buf.len() {
        return None;
    }

    let q = buf[*i];
    if q != b'"' && q != b'\'' {
        return None;
    }

    *i += 1;
    let start = *i;

    while *i < buf.len() && buf[*i] != q {
        *i += 1;
    }

    let out = std::str::from_utf8(&buf[start..*i]).ok()?.to_string();

    if *i < buf.len() {
        *i += 1;
    }

    Some(out)
}

fn read_ident(buf: &[u8], i: &mut usize) -> Option<String> {
    let start = *i;

    while *i < buf.len() {
        let c = buf[*i];
        if c == b'=' || c == b' ' || c == b'\t' || c == b'\n' {
            break;
        }
        *i += 1;
    }

    std::str::from_utf8(&buf[start..*i]).ok().map(|s| s.to_string())
}

fn skip_ws(buf: &[u8], i: &mut usize) {
    while *i < buf.len() && matches!(buf[*i], b' ' | b'\n' | b'\t' | b'\r') {
        *i += 1;
    }
}

fn to_lower(buf: &[u8]) -> String {
    buf.iter().map(|b| (*b as char).to_ascii_lowercase()).collect()
}

//
// ======================================================
// SIMD FAST PATH (FEATURE-GATED)
// ======================================================
//

#[cfg(feature = "simd-html")]
mod simd;