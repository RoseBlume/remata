//! HTML Metadata Extraction
//!
//! This module provides a lightweight, dependency-free parser for extracting
//! metadata from HTML documents.
//!
//! It focuses on `<meta>` and `<title>` elements and maps their contents into
//! strongly-typed Rust structures grouped by metadata schema.
//!
//! ## 📦 Supported Metadata Groups
//!
//! The parser organizes metadata into the following structures:
//!
//! - [`HtmlMetadata::general`] – Standard HTML `<meta name="...">` tags
//! - [`HtmlMetadata::dc`] – Dublin Core metadata
//! - [`HtmlMetadata::http_equiv`] – `<meta http-equiv="...">` headers
//! - [`HtmlMetadata::ncc`] – NCC metadata (commonly used in DAISY documents)
//! - [`HtmlMetadata::office`] – Microsoft Office-generated metadata
//! - [`HtmlMetadata::prod`] – Production-related metadata tags
//! - [`HtmlMetadata::vw96`] – VW96 metadata tags
//! - [`HtmlMetadata::title`] – Contents of the `<title>` element
//! - [`HtmlMetadata::other`] – Unknown or unsupported metadata tags
//!
//! ## 🔍 Extraction Behavior
//!
//! - Tag and attribute names are matched case-insensitively
//! - `<meta name="...">` and `<meta http-equiv="...">` are both supported
//! - The `<title>` element is extracted separately
//! - If a metadata key belongs to multiple schemas (e.g. `author`, `subject`),
//!   it is assigned to all relevant fields
//! - Unrecognized keys are stored in [`HtmlMetadata::other`]
//!
//! ## ⚠️ Limitations
//!
//! This module does **not** implement a full HTML parser.
//!
//! - Parsing is based on simple string scanning
//! - Malformed or complex HTML may not be handled correctly
//! - Attribute parsing is minimal and not HTML5-compliant
//! - Nested or unusual tag formatting may produce incomplete results
//!
//! ## 💡 Intended Use
//!
//! This module is best suited for:
//!
//! - Metadata extraction from well-formed HTML
//! - Lightweight indexing or analysis tools
//! - Environments where external dependencies are undesirable
//!
//! It is **not recommended** for:
//!
//! - Rendering HTML
//! - Full DOM parsing
//! - Handling arbitrary or untrusted HTML input
//!
//! ## 🧪 Example
//!
//! ```rust
//! use remata_documents::HtmlMetadata;
//!
//! let html = r#"
//!     <html>
//!         <head>
//!             <title>Example Page</title>
//!             <meta name="author" content="Alice">
//!             <meta name="description" content="Demo page">
//!             <meta http-equiv="refresh" content="30">
//!         </head>
//!     </html>
//! "#;
//!
//! let meta = HtmlMetadata::from_html(html);
//!
//! assert_eq!(meta.title, Some("Example Page".to_string()));
//! assert_eq!(meta.general.author, Some("Alice".to_string()));
//! assert_eq!(meta.general.description, Some("Demo page".to_string()));
//! assert_eq!(meta.http_equiv.refresh, Some("30".to_string()));
//! ```
//!
//! ## 🧠 Design Notes
//!
//! - This module prioritizes **zero dependencies** over strict correctness
//! - Metadata fields are **explicitly typed** rather than dynamically stored
//! - Some schemas contain overlapping keys; duplication is intentional
//! - Unknown metadata is preserved instead of discarded
//!
//! Future improvements may include:
//!
//! - More robust attribute parsing
//! - Expanded schema coverage
//! - Optional integration with a full HTML parser
//! - Streaming or incremental parsing
//!

use std::collections::HashMap;
/// Represents all extracted metadata from an HTML document.
///
/// This struct aggregates multiple metadata schemas into a single
/// unified structure.
///
/// Each field corresponds to a known metadata namespace.
///
/// Unknown or unsupported metadata keys are stored in [`other`].
#[derive(Debug, Default)]
pub struct HtmlMetadata {
    /// General HTML metadata (standard `<meta name="...">` tags)
    pub general: GeneralMeta,

    /// Dublin Core metadata
    pub dc: DcMeta,

    /// HTTP-equiv metadata (`<meta http-equiv="...">`)
    pub http_equiv: HttpEquivMeta,

    /// NCC metadata (used in DAISY / talking books)
    pub ncc: NccMeta,

    /// Microsoft Office-generated metadata
    pub office: OfficeMeta,

    /// Production-related metadata
    pub prod: ProdMeta,

    /// VW96 metadata tags
    pub vw96: Vw96Meta,

    /// Content of the `<title>` element
    pub title: Option<String>,

    /// Storage for unrecognized or unsupported metadata tags
    pub other: HashMap<String, String>,
}

// ---------------- GENERAL ----------------

/// Standard HTML metadata fields.
///
/// These are commonly found in `<meta name="...">` tags
/// and represent general-purpose document metadata.
#[derive(Debug, Default)]
pub struct GeneralMeta {
    /// Short summary of the document
    pub abstract_: Option<String>,

    /// Author of the document
    pub author: Option<String>,

    /// Classification or category
    pub classification: Option<String>,

    /// Content language (e.g., "en", "fr")
    pub content_language: Option<String>,

    /// Copyright information
    pub copyright: Option<String>,

    /// Description or summary of the document
    pub description: Option<String>,

    /// Distribution scope
    pub distribution: Option<String>,

    /// Document classification type
    pub doc_class: Option<String>,

    /// Rights associated with the document
    pub doc_rights: Option<String>,

    /// Document type
    pub doc_type: Option<String>,

    /// Formatting tool or system
    pub formatter: Option<String>,

    /// Generator software
    pub generator: Option<String>,

    /// Version of generator
    pub generatorversion: Option<String>,

    /// Google bot instructions
    pub googlebot: Option<String>,

    /// Keywords associated with the document
    pub keywords: Option<String>,

    /// Prevent Microsoft Smart Tags parsing
    pub mssmarttagspreventparsing: Option<String>,

    /// Originator of the document
    pub originator: Option<String>,

    /// Owner of the document
    pub owner: Option<String>,

    /// Programmatic identifier
    pub progid: Option<String>,

    /// Content rating
    pub rating: Option<String>,

    /// Refresh instruction
    pub refresh: Option<String>,

    /// Resource type
    pub resource_type: Option<String>,

    /// Suggested revisit interval
    pub revisit_after: Option<String>,

    /// Robots directives
    pub robots: Option<String>,
}

/// Dublin Core (DC) metadata.
///
/// This schema is widely used for describing digital resources and is
/// commonly embedded in HTML via `<meta name="...">` tags.
///
/// Many of these fields are also used in XML-based formats and XMP metadata.
#[derive(Debug, Default)]
pub struct DcMeta {
    /// Entity responsible for making contributions to the content
    pub contributor: Option<String>,

    /// Spatial or temporal topic of the resource
    pub coverage: Option<String>,

    /// Primary creator or author of the content
    pub creator: Option<String>,

    /// Date associated with the resource (creation, publication, etc.)
    pub date: Option<String>,

    /// Description or abstract of the content
    pub description: Option<String>,

    /// File format or physical medium (e.g., "text/html")
    pub format: Option<String>,

    /// Unique identifier for the resource (e.g., URI, ISBN)
    pub identifier: Option<String>,

    /// Language of the content (e.g., "en", "fr")
    pub language: Option<String>,

    /// Entity responsible for publishing the resource
    pub publisher: Option<String>,

    /// Related resource(s)
    pub relation: Option<String>,

    /// Rights management or usage terms
    pub rights: Option<String>,

    /// Source from which the resource is derived
    pub source: Option<String>,

    /// Subject or topic keywords
    pub subject: Option<String>,

    /// Title of the resource
    pub title: Option<String>,

    /// Nature or genre of the content
    pub type_: Option<String>,
}


/// Metadata derived from `http-equiv` attributes.
///
/// These simulate HTTP headers inside HTML.
#[derive(Debug, Default)]
pub struct HttpEquivMeta {
    /// Cache control directives
    pub cache_control: Option<String>,

    /// Content disposition header
    pub content_disposition: Option<String>,

    /// Content language
    pub content_language: Option<String>,

    /// Script MIME type
    pub content_script_type: Option<String>,

    /// Style MIME type
    pub content_style_type: Option<String>,

    /// Content type (e.g., text/html; charset=UTF-8)
    pub content_type: Option<String>,

    /// Default style sheet
    pub default_style: Option<String>,

    /// Expiration date/time
    pub expires: Option<String>,

    /// Cache extension directives
    pub ext_cache: Option<String>,

    /// Image toolbar visibility
    pub imagetoolbar: Option<String>,

    /// Misc proprietary tag
    pub lotus: Option<String>,

    /// Page entry transition
    pub page_enter: Option<String>,

    /// Page exit transition
    pub page_exit: Option<String>,

    /// PICS label
    pub pics_label: Option<String>,

    /// HTTP pragma directive
    pub pragma: Option<String>,

    /// Refresh interval or redirect
    pub refresh: Option<String>,

    /// Reply-to header
    pub reply_to: Option<String>,

    /// Cookie setting
    pub set_cookie: Option<String>,

    /// Site entry transition
    pub site_enter: Option<String>,

    /// Site exit transition
    pub site_exit: Option<String>,

    /// Vary header
    pub vary: Option<String>,

    /// Target window
    pub window_target: Option<String>,
}

/// NCC metadata (Navigation Control Center).
///
/// This schema is commonly used in DAISY digital talking books and
/// other structured multimedia documents.
///
/// Note: This struct currently represents a simplified subset of NCC fields.
#[derive(Debug, Default)]
pub struct NccMeta {
    /// Character encoding used in the document
    pub charset: Option<String>,

    /// Depth of the navigation hierarchy
    pub depth: Option<String>,

    /// File-related metadata (used as a general fallback for multiple NCC fields)
    pub files: Option<String>,

    /// Tool or software used to generate the document
    pub generator: Option<String>,

    /// Revision identifier or version information
    pub revision: Option<String>,
}

/// Microsoft Office metadata.
///
/// These fields are typically generated when HTML files are exported
/// from Microsoft Office applications (e.g., Word, Excel).
#[derive(Debug, Default)]
pub struct OfficeMeta {
    /// Author of the document
    pub author: Option<String>,

    /// Document category
    pub category: Option<String>,

    /// Organization or company associated with the document
    pub company: Option<String>,

    /// Creation date of the document
    pub created: Option<String>,

    /// Description or summary
    pub description: Option<String>,

    /// Keywords associated with the document
    pub keywords: Option<String>,

    /// Last user to modify the document
    pub last_author: Option<String>,

    /// Manager or responsible person
    pub manager: Option<String>,

    /// Number of pages
    pub pages: Option<String>,

    /// Subject or topic of the document
    pub subject: Option<String>,

    /// Template used to create the document
    pub template: Option<String>,

    /// Version or revision number
    pub version: Option<String>,
}



/// Production metadata.
///
/// These fields are typically used in specialized publishing or
/// document processing workflows.
#[derive(Debug, Default)]
pub struct ProdMeta {
    /// Recording engineer or responsible technician
    pub recengineer: Option<String>,

    /// Recording or production location
    pub reclocation: Option<String>,
}

/// VW96 metadata.
///
/// A minimal schema containing VW96-specific metadata fields.
/// These are rarely used and typically appear in niche workflows.
#[derive(Debug, Default)]
pub struct Vw96Meta {
    /// Type of object represented by the document
    pub objecttype: Option<String>,
}

// ======================================================
// ================= PARSER =============================
// ======================================================

impl HtmlMetadata {
    /// Parses an HTML string and extracts metadata.
    ///
    /// This method scans the input for:
    /// - `<meta>` tags
    /// - `<title>` tag
    ///
    /// It assigns recognized metadata keys into structured fields
    /// and stores unknown keys in [`HtmlMetadata::other`].
    ///
    /// ## Arguments
    ///
    /// * `html` - A string slice containing raw HTML
    ///
    /// ## Returns
    ///
    /// A populated [`HtmlMetadata`] struct.
    ///
    /// ## Notes
    ///
    /// - Parsing is case-insensitive for tag names and attributes
    /// - Only basic HTML structures are supported
    /// - Malformed HTML may lead to incomplete extraction
    ///
    /// ## Example
    ///
    /// ```rust
    /// let html = r#"<meta name="author" content="Alice">"#;
    /// let meta = HtmlMetadata::from_html(html);
    ///
    /// assert_eq!(meta.general.author, Some("Alice".to_string()));
    /// ```
    pub fn from_html(html: &str) -> Self {
        let mut meta = HtmlMetadata::default();

        let mut i = 0;
        let bytes = html.as_bytes();

        while i < bytes.len() {
            if bytes[i] == b'<' {
                if let Some((tag_name, attrs, new_i)) = parse_tag(html, i) {
                    let tag = tag_name.to_ascii_lowercase();

                    if tag == "meta" {
                        handle_meta_tag(&mut meta, &attrs);
                    } else if tag == "title" {
                        if let Some((content, end_i)) = extract_tag_text(html, new_i, "title") {
                            meta.title = Some(content);
                            i = end_i;
                            continue;
                        }
                    }

                    i = new_i;
                    continue;
                }
            }

            i += 1;
        }

        meta
    }
}


fn parse_tag(input: &str, start: usize) -> Option<(String, Vec<(String, String)>, usize)> {
    let bytes = input.as_bytes();
    let mut i = start + 1;

    // Read tag name
    let name_start = i;
    while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
        i += 1;
    }

    let tag_name = input[name_start..i].to_string();

    let mut attrs = Vec::new();

    // Parse attributes
    while i < bytes.len() && bytes[i] != b'>' {
        skip_whitespace(bytes, &mut i);

        if i >= bytes.len() || bytes[i] == b'>' {
            break;
        }

        // key
        let key_start = i;
        while i < bytes.len()
            && bytes[i] != b'='
            && !bytes[i].is_ascii_whitespace()
            && bytes[i] != b'>'
        {
            i += 1;
        }

        let key = input[key_start..i].to_string().to_ascii_lowercase();

        skip_whitespace(bytes, &mut i);

        let mut value = String::new();

        if i < bytes.len() && bytes[i] == b'=' {
            i += 1;
            skip_whitespace(bytes, &mut i);

            if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let quote = bytes[i];
                i += 1;
                let val_start = i;

                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }

                value = input[val_start..i].to_string();

                if i < bytes.len() {
                    i += 1;
                }
            } else {
                let val_start = i;
                while i < bytes.len()
                    && !bytes[i].is_ascii_whitespace()
                    && bytes[i] != b'>'
                {
                    i += 1;
                }

                value = input[val_start..i].to_string();
            }
        }

        attrs.push((key, value));
    }

    if i < bytes.len() && bytes[i] == b'>' {
        i += 1;
    }

    Some((tag_name, attrs, i))
}

fn skip_whitespace(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn handle_meta_tag(meta: &mut HtmlMetadata, attrs: &[(String, String)]) {
    let mut name = None;
    let mut content = None;
    let mut http_equiv = None;

    for (k, v) in attrs {
        match k.as_str() {
            "name" => name = Some(v),
            "content" => content = Some(v),
            "http-equiv" => http_equiv = Some(v),
            _ => {}
        }
    }

    if let (Some(n), Some(c)) = (name, content) {
        assign_name(meta, n, c);
    }

    if let (Some(eq), Some(c)) = (http_equiv, content) {
        assign_http_equiv(meta, eq, c);
    }
}

fn extract_tag_text(input: &str, start: usize, tag: &str) -> Option<(String, usize)> {
    let lower = input.to_ascii_lowercase();
    let end_tag = format!("</{}>", tag);

    let end = lower[start..].find(&end_tag)? + start;

    let content = input[start..end].trim().to_string();

    Some((content, end + end_tag.len()))
}

fn assign_name(meta: &mut HtmlMetadata, key: &str, val: &str) {
    let k = key.to_lowercase();
    let v = val.to_string();

    // ---------------- GENERAL ----------------
    if k == "abstract" {
        meta.general.abstract_ = Some(v);
        return;
    }

    if k == "author" {
        meta.general.author = Some(v.clone());
        meta.office.author = Some(v);
        return;
    }

    if k == "classification" {
        meta.general.classification = Some(v);
        return;
    }

    if k == "content-language" {
        meta.general.content_language = Some(v.clone());
        meta.http_equiv.content_language = Some(v);
        return;
    }

    if k == "copyright" {
        meta.general.copyright = Some(v);
        return;
    }

    if k == "description" {
        meta.general.description = Some(v.clone());
        meta.dc.description = Some(v.clone());
        meta.office.description = Some(v);
        return;
    }

    if k == "distribution" {
        meta.general.distribution = Some(v);
        return;
    }

    if k == "doc-class" {
        meta.general.doc_class = Some(v);
        return;
    }

    if k == "doc-rights" {
        meta.general.doc_rights = Some(v);
        return;
    }

    if k == "doc-type" {
        meta.general.doc_type = Some(v);
        return;
    }

    if k == "formatter" {
        meta.general.formatter = Some(v);
        return;
    }

    if k == "generator" {
        meta.general.generator = Some(v.clone());
        meta.ncc.generator = Some(v);
        return;
    }

    if k == "generatorversion" {
        meta.general.generatorversion = Some(v);
        return;
    }

    if k == "googlebot" {
        meta.general.googlebot = Some(v);
        return;
    }

    if k == "keywords" {
        meta.general.keywords = Some(v.clone());
        meta.office.keywords = Some(v);
        return;
    }

    if k == "mssmarttagspreventparsing" {
        meta.general.mssmarttagspreventparsing = Some(v);
        return;
    }

    if k == "originator" {
        meta.general.originator = Some(v);
        return;
    }

    if k == "owner" {
        meta.general.owner = Some(v);
        return;
    }

    if k == "progid" {
        meta.general.progid = Some(v);
        return;
    }

    if k == "rating" {
        meta.general.rating = Some(v);
        return;
    }

    if k == "refresh" {
        meta.general.refresh = Some(v.clone());
        meta.http_equiv.refresh = Some(v);
        return;
    }

    if k == "resource-type" {
        meta.general.resource_type = Some(v);
        return;
    }

    if k == "revisit-after" {
        meta.general.revisit_after = Some(v);
        return;
    }

    if k == "robots" {
        meta.general.robots = Some(v);
        return;
    }

    // ---------------- DC ----------------
    if k == "contributor" {
        meta.dc.contributor = Some(v);
        return;
    }

    if k == "coverage" {
        meta.dc.coverage = Some(v);
        return;
    }

    if k == "creator" {
        meta.dc.creator = Some(v);
        return;
    }

    if k == "date" {
        meta.dc.date = Some(v);
        return;
    }

    if k == "format" {
        meta.dc.format = Some(v);
        return;
    }

    if k == "identifier" {
        meta.dc.identifier = Some(v);
        return;
    }

    if k == "language" {
        meta.dc.language = Some(v);
        return;
    }

    if k == "publisher" {
        meta.dc.publisher = Some(v);
        return;
    }

    if k == "relation" {
        meta.dc.relation = Some(v);
        return;
    }

    if k == "rights" {
        meta.dc.rights = Some(v);
        return;
    }

    if k == "source" {
        meta.dc.source = Some(v);
        return;
    }

    if k == "subject" {
        meta.dc.subject = Some(v.clone());
        meta.office.subject = Some(v);
        return;
    }

    if k == "title" {
        meta.dc.title = Some(v);
        return;
    }

    if k == "type" {
        meta.dc.type_ = Some(v);
        return;
    }

    // ---------------- NCC ----------------
    if k == "charset" {
        meta.ncc.charset = Some(v);
        return;
    }

    if k == "depth" {
        meta.ncc.depth = Some(v);
        return;
    }

    if k == "files"
        || k == "footnotes"
        || k == "kbytesize"
        || k == "maxpagenormal"
        || k == "multimediatype"
        || k == "narrator"
        || k == "pagefront"
        || k == "pagenormal"
        || k == "pagespecial"
        || k == "prodnotes"
        || k == "produceddate"
        || k == "producer"
        || k == "setinfo"
        || k == "sidebars"
        || k == "sourcedate"
        || k == "sourceedition"
        || k == "sourcepublisher"
        || k == "sourcerights"
        || k == "sourcetitle"
        || k == "tocitems"
    {
        meta.ncc.files = Some(v);
        return;
    }

    if k == "revision" {
        meta.ncc.revision = Some(v.clone());
        meta.office.version = Some(v);
        return;
    }

    if k == "revisiondate" {
        meta.ncc.revision = Some(v);
        return;
    }

    if k == "totaltime" {
        meta.ncc.files = Some(v.clone());
        meta.office.version = Some(v);
        return;
    }

    // ---------------- OFFICE ----------------
    if k == "category"
        || k == "characters"
        || k == "characterswithspaces"
        || k == "lines"
        || k == "paragraphs"
        || k == "words"
    {
        meta.office.category = Some(v);
        return;
    }

    if k == "company" {
        meta.office.company = Some(v);
        return;
    }

    if k == "created" || k == "lastprinted" || k == "lastsaved" {
        meta.office.created = Some(v);
        return;
    }

    if k == "lastauthor" {
        meta.office.last_author = Some(v);
        return;
    }

    if k == "manager" {
        meta.office.manager = Some(v);
        return;
    }

    if k == "pages" {
        meta.office.pages = Some(v);
        return;
    }

    if k == "template" {
        meta.office.template = Some(v);
        return;
    }

    if k == "version" {
        meta.office.version = Some(v);
        return;
    }

    // ---------------- PROD ----------------
    if k == "recengineer" {
        meta.prod.recengineer = Some(v);
        return;
    }

    if k == "reclocation" {
        meta.prod.reclocation = Some(v);
        return;
    }

    // ---------------- VW96 ----------------
    if k == "objecttype" {
        meta.vw96.objecttype = Some(v);
        return;
    }

    // ---------------- FALLBACK ----------------
    meta.other.insert(k, v);
}

fn assign_http_equiv(meta: &mut HtmlMetadata, key: &str, val: &str) {
    let k = key.to_lowercase();
    let v = Some(val.to_string());

    match k.as_str() {
        "cache-control" => meta.http_equiv.cache_control = v,
        "content-disposition" => meta.http_equiv.content_disposition = v,
        "content-language" => meta.http_equiv.content_language = v,
        "content-script-type" => meta.http_equiv.content_script_type = v,
        "content-style-type" => meta.http_equiv.content_style_type = v,
        "content-type" => meta.http_equiv.content_type = v,
        "default-style" => meta.http_equiv.default_style = v,
        "expires" => meta.http_equiv.expires = v,
        "ext-cache" => meta.http_equiv.ext_cache = v,
        "imagetoolbar" => meta.http_equiv.imagetoolbar = v,
        "lotus" => meta.http_equiv.lotus = v,
        "page-enter" => meta.http_equiv.page_enter = v,
        "page-exit" => meta.http_equiv.page_exit = v,
        "pics-label" => meta.http_equiv.pics_label = v,
        "pragma" => meta.http_equiv.pragma = v,
        "refresh" => meta.http_equiv.refresh = v,
        "reply-to" => meta.http_equiv.reply_to = v,
        "set-cookie" => meta.http_equiv.set_cookie = v,
        "site-enter" => meta.http_equiv.site_enter = v,
        "site-exit" => meta.http_equiv.site_exit = v,
        "vary" => meta.http_equiv.vary = v,
        "window-target" => meta.http_equiv.window_target = v,
        _ => {}
    }
}

