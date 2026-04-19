// use std::{fs, path::Path};

// mod html_parser;
// mod ir;
// mod type_parser;
// mod codegen;

// use html_parser::parse_html_schema;
// use codegen::generate_code;

// fn main() {
//     let input_dir = Path::new("schemas");

//     let mut all = String::new();

//     for file in fs::read_dir(input_dir).unwrap() {
//         let file = file.unwrap();
//         let content = fs::read_to_string(file.path()).unwrap();

//         let schema = parse_html_schema(&content);
//         all.push_str(&generate_code(schema));
//     }

//     fs::write("src/generated.rs", all).unwrap();

//     println!("cargo:rerun-if-changed=schemas/");
// }