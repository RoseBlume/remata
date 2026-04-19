use std::collections::HashMap;
use std::io;

/// Represents metadata extracted from an HTML document.
///
/// This struct collects metadata from `<meta>` tags, `<title>`,
/// and structured metadata namespaces such as `dc`, `ncc`, `o`, and `prod`.
#[derive(Debug, Default, Clone)]
pub struct Html {
    /// Page abstract (meta name="abstract")
    pub abstract_text: Option<String>,

    /// Author of the document
    pub author: Option<String>,

    /// Classification category
    pub classification: Option<String>,

    /// Content language (meta http-equiv or name)
    pub content_language: Option<String>,

    /// Copyright information
    pub copyright: Option<String>,

    /// Dublin Core metadata block
    pub dc: Option<HtmlDc>,

    /// Description meta tag
    pub description: Option<String>,

    /// Distribution policy (public/private/etc)
    pub distribution: Option<String>,

    /// Document class metadata
    pub doc_class: Option<String>,

    /// Document rights metadata
    pub doc_rights: Option<String>,

    /// Document type metadata
    pub doc_type: Option<String>,

    /// Formatter used to generate page
    pub formatter: Option<String>,

    /// Generator name (e.g. WordPress, Hugo)
    pub generator: Option<String>,

    /// Generator version
    pub generator_version: Option<String>,

    /// Googlebot directives
    pub googlebot: Option<String>,

    /// HTTP-EQUIV metadata block
    pub http_equiv: Option<HtmlHttpEquiv>,

    /// Keywords (may be comma-separated or multiple meta tags)
    pub keywords: Option<Vec<String>>,

    /// Prevent MS Smart Tags parsing
    pub no_ms_smart_tags: Option<bool>,

    /// NCC metadata group
    pub ncc: Option<HtmlNcc>,

    /// Office metadata group
    pub office: Option<HtmlOffice>,

    /// Originator of document
    pub originator: Option<String>,

    /// Owner of document
    pub owner: Option<String>,

    /// Product metadata group
    pub prod: Option<HtmlProd>,

    /// Program ID
    pub prog_id: Option<String>,

    /// Content rating
    pub rating: Option<String>,

    /// Refresh directive (meta refresh)
    pub refresh: Option<String>,

    /// Resource type
    pub resource_type: Option<String>,

    /// Revisit interval
    pub revisit_after: Option<String>,

    /// Robots directive (index/follow rules)
    pub robots: Option<Vec<String>>,

    /// Document title (ONLY extracted from `<title>`)
    pub title: Option<String>,
}

//
// -----------------------------
// GROUP STRUCTS
// -----------------------------
//

/// Dublin Core metadata group (`dc.*`)
#[derive(Debug, Default, Clone)]
pub struct HtmlDc {
    /// Dublin Core fields (flattened key-value storage)
    pub fields: HashMap<String, String>,
}

/// HTTP-EQUIV metadata group
#[derive(Debug, Default, Clone)]
pub struct HtmlHttpEquiv {
    /// HTTP-EQUIV key-value pairs
    pub fields: HashMap<String, String>,
}

/// NCC metadata group
#[derive(Debug, Default, Clone)]
pub struct HtmlNcc {
    /// NCC metadata fields
    pub fields: HashMap<String, String>,
}

/// Office metadata group
#[derive(Debug, Default, Clone)]
pub struct HtmlOffice {
    /// Microsoft Office metadata fields
    pub fields: HashMap<String, String>,
}

/// Product metadata group
#[derive(Debug, Default, Clone)]
pub struct HtmlProd {
    /// Product metadata fields
    pub fields: HashMap<String, String>,
}

//
// -----------------------------
// TAG TYPE ENUM
// -----------------------------
//

/// Represents the type of HTML metadata tag encountered.
///
/// Used internally during parsing to classify `<meta>` tags.
#[derive(Debug, Clone, Copy)]
pub enum HtmlMetaTag {
    /// Abstract content
    Abstract,

    /// Author metadata
    Author,

    /// Classification metadata
    Classification,

    /// Content language
    ContentLanguage,

    /// Copyright information
    Copyright,

    /// Dublin Core (`dc.*`) metadata group
    Dc,

    /// Description metadata
    Description,

    /// Distribution metadata
    Distribution,

    /// Document class
    DocClass,

    /// Document rights
    DocRights,

    /// Document type
    DocType,

    /// Formatter used
    Formatter,

    /// Generator name
    Generator,

    /// Generator version
    GeneratorVersion,

    /// Googlebot directive
    GoogleBot,

    /// HTTP-EQUIV metadata group
    HttpEquiv,

    /// Keywords list
    Keywords,

    /// MS Smart Tags prevention flag
    NoMsSmartTags,

    /// NCC metadata group
    Ncc,

    /// Office metadata group
    Office,

    /// Originator field
    Originator,

    /// Owner field
    Owner,

    /// Product metadata group
    Prod,

    /// Program ID
    ProgId,

    /// Rating metadata
    Rating,

    /// Refresh directive
    Refresh,

    /// Resource type
    ResourceType,

    /// Revisit-after directive
    RevisitAfter,

    /// Robots directive
    Robots,
}

//
// -----------------------------
// PARSER
// -----------------------------
//

impl Html {
    /// Parses an HTML document and extracts metadata.
    ///
    /// This function performs a lightweight scan of the HTML source and extracts:
    /// - `<meta name="...">` fields
    /// - `<meta http-equiv="...">` fields
    /// - `<title>` tag content
    /// - structured metadata groups (`dc`, `ncc`, `office`, `prod`)
    ///
    /// It does NOT build a DOM tree and is designed for streaming use.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if input cannot be read.
    pub fn parse<R: std::io::Read>(mut reader: R) -> io::Result<Self> {
        let mut html = String::new();
        reader.read_to_string(&mut html)?;

        let mut out = Html::default();

        // naive tag scanning (std-only approach)
        let lower = html.to_lowercase();

        out.title = extract_title(&html);

        out.abstract_text = extract_meta(&lower, "abstract");
        out.author = extract_meta(&lower, "author");
        out.classification = extract_meta(&lower, "classification");
        out.content_language = extract_meta(&lower, "content-language");
        out.copyright = extract_meta(&lower, "copyright");
        out.description = extract_meta(&lower, "description");
        out.distribution = extract_meta(&lower, "distribution");
        out.doc_class = extract_meta(&lower, "doc-class");
        out.doc_rights = extract_meta(&lower, "doc-rights");
        out.doc_type = extract_meta(&lower, "doc-type");
        out.formatter = extract_meta(&lower, "formatter");
        out.generator = extract_meta(&lower, "generator");
        out.generator_version = extract_meta(&lower, "generatorversion");
        out.googlebot = extract_meta(&lower, "googlebot");
        out.no_ms_smart_tags = extract_bool_meta(&lower, "mssmarttagspreventparsing");
        out.originator = extract_meta(&lower, "originator");
        out.owner = extract_meta(&lower, "owner");
        out.prog_id = extract_meta(&lower, "progid");
        out.rating = extract_meta(&lower, "rating");
        out.refresh = extract_meta(&lower, "refresh");
        out.resource_type = extract_meta(&lower, "resource-type");
        out.revisit_after = extract_meta(&lower, "revisit-after");

        out.keywords = extract_list_meta(&lower, "keywords");
        out.robots = extract_list_meta(&lower, "robots");

        // groups
        out.dc = extract_group(&lower, "dc");
        out.http_equiv = extract_group(&lower, "http-equiv");
        out.ncc = extract_group(&lower, "ncc");
        out.office = extract_group(&lower, "o");
        out.prod = extract_group(&lower, "prod");

        Ok(out)
    }
}

//
// -----------------------------
// ENUM UTIL (optional helper layer)
// -----------------------------
//

impl HtmlMetaTag {
    /// Converts a string tag name into a typed metadata variant.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "abstract" => Some(Self::Abstract),
            "author" => Some(Self::Author),
            "classification" => Some(Self::Classification),
            "content-language" => Some(Self::ContentLanguage),
            "copyright" => Some(Self::Copyright),
            "dc" => Some(Self::Dc),
            "description" => Some(Self::Description),
            "distribution" => Some(Self::Distribution),
            "doc-class" => Some(Self::DocClass),
            "doc-rights" => Some(Self::DocRights),
            "doc-type" => Some(Self::DocType),
            "formatter" => Some(Self::Formatter),
            "generator" => Some(Self::Generator),
            "generatorversion" => Some(Self::GeneratorVersion),
            "googlebot" => Some(Self::GoogleBot),
            "http-equiv" => Some(Self::HttpEquiv),
            "keywords" => Some(Self::Keywords),
            "mssmarttagspreventparsing" => Some(Self::NoMsSmartTags),
            "ncc" => Some(Self::Ncc),
            "o" => Some(Self::Office),
            "originator" => Some(Self::Originator),
            "owner" => Some(Self::Owner),
            "prod" => Some(Self::Prod),
            "progid" => Some(Self::ProgId),
            "rating" => Some(Self::Rating),
            "refresh" => Some(Self::Refresh),
            "resource-type" => Some(Self::ResourceType),
            "revisit-after" => Some(Self::RevisitAfter),
            "robots" => Some(Self::Robots),
            _ => None,
        }
    }
}

//
// -----------------------------
// INTERNAL HELPERS (STUBBED)
// -----------------------------
//
use std::collections::HashMap;

//
// -----------------------------
// META EXTRACTION (SINGLE VALUE)
// -----------------------------
//

/// Extracts a single `<meta name="key" content="...">` value.
///
/// This is a lightweight scanner (no DOM, no regex).
fn extract_meta(html: &str, key: &str) -> Option<String> {
    let key = key.to_lowercase();

    let mut i = 0;
    let bytes = html.as_bytes();

    while i + 5 < bytes.len() {
        if bytes[i] == b'<' && bytes[i + 1] == b'm' {
            // naive "<meta"
            if let Some(end) = find_tag_end(bytes, i) {
                let tag = &html[i..end];

                if tag_contains(tag, "meta") && tag_contains(tag, &format!("name=\"{key}\"")) {
                    if let Some(content) = extract_attr(tag, "content") {
                        return Some(content);
                    }
                }

                i = end;
                continue;
            }
        }
        i += 1;
    }

    None
}

//
// -----------------------------
// BOOLEAN META
// -----------------------------
//

/// Extracts a boolean meta flag (present = true).
fn extract_bool_meta(html: &str, key: &str) -> Option<bool> {
    let key = key.to_lowercase();

    let mut i = 0;
    let bytes = html.as_bytes();

    while i + 5 < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(end) = find_tag_end(bytes, i) {
                let tag = &html[i..end];

                if tag_contains(tag, "meta") && tag_contains(tag, &format!("name=\"{key}\"")) {
                    return Some(true);
                }

                i = end;
                continue;
            }
        }
        i += 1;
    }

    None
}

//
// -----------------------------
// LIST META (keywords, robots)
// -----------------------------
//

/// Extracts comma/space-separated meta content into a Vec<String>.
fn extract_list_meta(html: &str, key: &str) -> Option<Vec<String>> {
    let raw = extract_meta(html, key)?;
    let parts = raw
        .split(|c| c == ',' || c == ' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    if parts.is_empty() {
        None
    } else {
        Some(parts)
    }
}

//
// -----------------------------
// TITLE EXTRACTION
// -----------------------------
//

/// Extracts `<title>...</title>` content.
fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    let start = lower.find("<title>")?;
    let end = lower.find("</title>")?;

    if end > start {
        let content = &html[start + 7..end];
        return Some(content.trim().to_string());
    }

    None
}

//
// -----------------------------
// GROUP EXTRACTION (dc, ncc, o, prod, http-equiv)
// -----------------------------
//

/// Extracts grouped metadata such as `dc.*`, `ncc.*`, etc.
fn extract_group(html: &str, prefix: &str) -> Option<HashMap<String, String>> {
    let prefix = prefix.to_lowercase();
    let mut map = HashMap::new();

    let mut i = 0;
    let bytes = html.as_bytes();

    while i + 5 < bytes.len() {
        if bytes[i] == b'<' {
            if let Some(end) = find_tag_end(bytes, i) {
                let tag = &html[i..end];

                if tag_contains(tag, "meta") {
                    if let Some(name) = extract_attr(tag, "name") {
                        let lname = name.to_lowercase();

                        if lname.starts_with(&prefix) {
                            if let Some(content) = extract_attr(tag, "content") {
                                map.insert(name, content);
                            }
                        }
                    }
                }

                i = end;
                continue;
            }
        }

        i += 1;
    }

    if map.is_empty() {
        None
    } else {
        Some(map)
    }
}

//
// -----------------------------
// ATTRIBUTE EXTRACTION
// -----------------------------
//

/// Extracts an attribute value from a tag (`content="..."`, `name="..."`).
fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let attr = attr.to_lowercase();
    let lower = tag.to_lowercase();

    let pattern = format!("{attr}=");
    let start = lower.find(&pattern)?;

    let rest = &tag[start + pattern.len()..].trim_start();

    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let rest = &rest[1..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

//
// -----------------------------
// TAG UTILITIES
// -----------------------------
//

/// Checks if a substring exists in a tag (case-insensitive).
fn tag_contains(tag: &str, needle: &str) -> bool {
    tag.to_lowercase().contains(&needle.to_lowercase())
}

/// Finds end of an HTML tag (`>`), respecting no nested parsing.
fn find_tag_end(bytes: &[u8], mut i: usize) -> Option<usize> {
    while i < bytes.len() {
        if bytes[i] == b'>' {
            return Some(i + 1);
        }
        i += 1;
    }
    None
}