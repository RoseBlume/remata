//! XMP schema codegen core library
//!
//! Parses ExifTool-style HTML documentation into an intermediate schema
//! and generates Rust code (namespace_gen! or structs).

pub mod ir;
pub mod html_parser;
pub mod type_parser;
pub mod codegen;

/// High-level API: parse a single HTML schema page into IR
pub fn parse_schema_html(input: &str) -> ir::Schema {
    html_parser::parse_html_schema(input)
}

/// High-level API: generate Rust code from IR
pub fn generate_rust(schema: ir::Schema) -> String {
    codegen::generate_code(schema)
}

/// Convenience: HTML → Rust in one step
pub fn compile_html_to_rust(input: &str) -> String {
    let schema = parse_schema_html(input);
    generate_rust(schema)
}

/// Batch processing for multiple schema pages
pub fn compile_many_html(inputs: &[String]) -> String {
    let mut out = String::new();

    for page in inputs {
        let schema = parse_schema_html(page);
        out.push_str(&generate_rust(schema));
        out.push('\n');
    }

    out
}

/// Optional: debug helper to inspect parsed IR
pub fn debug_schema(input: &str) -> ir::Schema {
    let schema = parse_schema_html(input);

    #[cfg(debug_assertions)]
    {
        println!("{:#?}", schema);
    }

    schema
}