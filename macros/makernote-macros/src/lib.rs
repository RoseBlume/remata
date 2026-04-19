
use proc_macro::{TokenStream, TokenTree, Delimiter};

#[proc_macro_derive(FromPrimitive, attributes(value))]
pub fn derive_from_primitive(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    // --- Find enum name
    let enum_name = loop {
        match tokens.next() {
            Some(TokenTree::Ident(id)) => {
                if id.to_string() != "enum" {
                    continue;
                }
                if let Some(TokenTree::Ident(name)) = tokens.next() {
                    break name.to_string();
                }
            }
            _ => {}
        }
    };

    // --- Find enum body
    let body = tokens.find_map(|t| {
        if let TokenTree::Group(g) = t {
            if g.delimiter() == Delimiter::Brace {
                return Some(g.stream());
            }
        }
        None
    }).expect("Expected enum body");

    // --- Parse variants
    let mut variants = Vec::new();
    let mut unknown_variant = None;

    let mut iter = body.into_iter().peekable();

    while let Some(mut token) = iter.next() {
        // Capture attributes
        let mut value_attr = None;

        while let TokenTree::Punct(p) = &token {
            if p.as_char() == '#' {
                if let Some(TokenTree::Group(g)) = iter.next() {
                    let attr = g.stream().to_string();
                    if attr.contains("value") {
                        // crude parse: value = X
                        if let Some(eq_pos) = attr.find('=') {
                            let val = attr[eq_pos + 1..]
                                .replace(']', "")
                                .trim()
                                .to_string();
                            value_attr = Some(val);
                        }
                    }
                }
                // move to next token after attribute
                if let Some(next) = iter.next() {
                    token = next;
                }
            } else {
                break;
            }
        }

        // Variant name
        let name = match token {
            TokenTree::Ident(id) => id.to_string(),
            _ => continue,
        };

        // Check if tuple variant (Unknown)
        let mut is_unknown = false;
        if let Some(TokenTree::Group(g)) = iter.peek() {
            if g.delimiter() == Delimiter::Parenthesis {
                is_unknown = true;
                iter.next(); // consume it
            }
        }

        if is_unknown {
            unknown_variant = Some(name);
        } else if let Some(val) = value_attr {
            variants.push((name, val));
        }

        // skip commas
    }

    let unknown = unknown_variant.expect("Must have Unknown(T) variant");

    // --- Generate match arms
    let mut match_arms = String::new();
    for (name, val) in &variants {
        match_arms.push_str(&format!(
            "{} => Self::{},",
            val, name
        ));
    }

    // --- Supported primitives
    let primitives = [
        "u8", "u16", "u32", "u64", "usize",
        "i8", "i16", "i32", "i64", "isize",
    ];

    let mut impls = String::new();

    for prim in primitives {
        impls.push_str(&format!(
            "impl From<{prim}> for {enum_name} {{
                fn from(v: {prim}) -> Self {{
                    match v as i128 {{
                        {match_arms}
                        _ => Self::{unknown}(v as _),
                    }}
                }}
            }}"
        ));
    }

    impls.parse().unwrap()
}