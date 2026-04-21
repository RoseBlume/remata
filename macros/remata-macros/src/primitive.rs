use proc_macro::{TokenStream, TokenTree, Delimiter};


pub fn expand(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    // --- Find enum name
    let enum_name = loop {
        match tokens.next() {
            Some(TokenTree::Ident(id)) if id.to_string() == "enum" => {
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
        let mut value_attr = None;

        // Parse attributes
        while let TokenTree::Punct(p) = &token {
            if p.as_char() == '#' {
                if let Some(TokenTree::Group(g)) = iter.next() {
                    let attr = g.stream().to_string();
                    if attr.contains("value") {
                        if let Some(eq_pos) = attr.find('=') {
                            let val = attr[eq_pos + 1..]
                                .replace(']', "")
                                .trim()
                                .to_string();
                            value_attr = Some(val);
                        }
                    }
                }
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

        // Detect tuple variant (Unknown)
        let is_tuple = matches!(
            iter.peek(),
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis
        );

        if is_tuple {
            iter.next(); // consume tuple
            unknown_variant = Some(name);
        } else if let Some(val) = value_attr {
            variants.push((name, val));
        }
    }

    // --- Generate match arms
    let mut match_arms_try = String::new();
    let mut match_arms_from = String::new();

    for (name, val) in &variants {
        match_arms_try.push_str(&format!(
            "{} => Ok(Self::{}),",
            val, name
        ));

        match_arms_from.push_str(&format!(
            "{} => Self::{},",
            val, name
        ));
    }

    // --- Fallback behavior
    // let _fallback = if let Some(unknown) = &unknown_variant {
    //     format!("_ => Ok(Self::{}(v as _)),", unknown)
    // } else {
    //     "_ => Err(()),".to_string()
    // };

    // --- Supported primitives
    let primitives = [
        "u8", "u16", "u32", "u64", "usize",
        "i8", "i16", "i32", "i64", "isize",
    ];

    let mut impls = String::new();

    for prim in primitives {
        if let Some(unknown) = &unknown_variant {
            // --- Generate From (infallible)
            impls.push_str(&format!(
                "impl core::convert::From<{prim}> for {enum_name} {{
                    fn from(v: {prim}) -> Self {{
                        match v as i128 {{
                            {match_arms_from}
                            _ => Self::{unknown}(v as _),
                        }}
                    }}
                }}"
            ));
        } else {
            // --- Generate TryFrom (fallible)
            impls.push_str(&format!(
                "impl core::convert::TryFrom<{prim}> for {enum_name} {{
                    type Error = ();

                    fn try_from(v: {prim}) -> Result<Self, Self::Error> {{
                        match v as i128 {{
                            {match_arms_try}
                            _ => Err(()),
                        }}
                    }}
                }}"
            ));
        }
    }

    impls.parse().unwrap()
}