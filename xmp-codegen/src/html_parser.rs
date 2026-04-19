use scraper::{Html, Selector};

use crate::ir::{Schema, Namespace, Field};
use crate::type_parser::parse_type;

pub fn parse_html_schema(input: &str) -> Schema {
    let doc = Html::parse_document(input);

    let mut fields = Vec::new();

    let row_sel = Selector::parse("tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    for row in doc.select(&row_sel) {
        let cols: Vec<_> = row.select(&td_sel).collect();
        if cols.len() < 2 {
            continue;
        }

        let name = cols[0].text().collect::<String>().trim().to_string();
        let typ = cols[1].text().collect::<String>();

        if name.is_empty() {
            continue;
        }

        let (rust_name, xmp_key) = normalize_name(&name);

        fields.push(Field {
            rust_name,
            xmp_key,
            ty: parse_type(&typ),
        });
    }

    Schema {
        namespaces: vec![Namespace {
            name: "ACDSeeRegions".into(),
            fields,
        }],
        structs: vec![],
    }
}

fn normalize_name(name: &str) -> (String, String) {
    let rust = name
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_ascii_lowercase()
            } else if c.is_uppercase() {
                format!("_{}", c.to_ascii_lowercase()).chars().next().unwrap()
            } else {
                c
            }
        })
        .collect();

    (rust, name.to_string())
}