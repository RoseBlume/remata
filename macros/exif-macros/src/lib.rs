use proc_macro::TokenStream;
use quote::quote;

use syn::{
    parse_macro_input, Ident, LitInt,
    Token, parse::Parse, Result, LitStr
};

use scraper::{Html, Selector};
use std::collections::HashSet;

/* =========================================================
   INPUT SYNTAX
   =========================================================
   MAKE => 0x010F, Str => make;
*/
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Ident, LitInt, Token, parse::Parse, Result};

struct Tag {
    name: Ident,
    _arrow1: Token![=>],
    id: LitInt,
    _comma: Token![,],
    kind: Ident,
    _arrow2: Token![=>],
    field: Ident,
    _semicolon: Token![;],
}

struct Tags(Vec<Tag>);

impl Parse for Tags {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut tags = Vec::new();

        while !input.is_empty() {
            tags.push(Tag {
                name: input.parse()?,
                _arrow1: input.parse()?,
                id: input.parse()?,
                _comma: input.parse()?,
                kind: input.parse()?,
                _arrow2: input.parse()?,
                field: input.parse()?,
                _semicolon: input.parse()?,
            });
        }

        Ok(Tags(tags))
    }
}

#[proc_macro]
pub fn exif_tags(input: TokenStream) -> TokenStream {
    let Tags(tags) = parse_macro_input!(input as Tags);

    let mut consts = Vec::new();
    let mut registry = Vec::new();

    for t in tags {
        let name = t.name;
        let id = t.id;
        let field = t.field;
        let kind = t.kind.to_string();

        // const
        consts.push(quote! {
            pub const #name: u16 = #id;
        });

        // generate per-tag decoder
        let decode = match kind.as_str() {
            "Str" => quote! { read_string(v) },
            "U16" => quote! { read_u16(v) },
            "Rational" => quote! { read_rational(v) },
            "SRational" => quote! { read_srational(v) },
            "U16Vec" => quote! { v.chunks_exact(2).map(read_u16).collect::<Vec<u16>>() },
            _ => panic!("Unknown kind"),
        };

        // TYPE-SAFE assignment (IMPORTANT PART)
        let assign = match kind.as_str() {
            "Str" => quote! {
                exif.#field = Some(#decode);
            },
            "U16" => quote! {
                exif.#field = Some(#decode);
            },
            "Rational" => quote! {
                exif.#field = Some(#decode);
            },
            "SRational" => quote! {
                exif.#field = Some(#decode);
            },
            "U16Vec" => quote! {
                exif.#field = Some(#decode);
            },
            _ => unreachable!(),
        };

        registry.push(quote! {
            (#id, |exif: &mut crate::Exif, v: &[u8]| {
                #assign
            }),
        });
    }

    quote! {
        pub mod generated {
            use super::*;

            #(#consts)*

            pub fn registry() -> &'static [(u16, fn(&mut Exif, &[u8]))] {
                &[
                    #(#registry)*
                ]
            }
        }
    }
    .into()
}




#[proc_macro]
pub fn exif_from_html(input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(input as LitStr);
    let path = path_lit.value();

    let html = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            return syn::Error::new_spanned(
                path_lit,
                format!("failed to read '{}': {}", path, err),
            )
            .to_compile_error()
            .into();
        }
    };

    let doc = Html::parse_document(&html);
    let row_sel = Selector::parse("tr").unwrap();
    let cell_sel = Selector::parse("td").unwrap();

    let mut fields = Vec::new();
    let mut consts = Vec::new();
    let mut registry = Vec::new();

    let mut seen_consts = HashSet::new();
    let mut seen_fields = HashSet::new();

    for row in doc.select(&row_sel) {
        let cells: Vec<String> = row
            .select(&cell_sel)
            .map(|c| c.text().collect::<String>().trim().to_string())
            .collect();

        if cells.len() < 3 {
            continue;
        }

        let tag_hex = &cells[0];
        let name = &cells[1];
        let ty = &cells[2];

        let tag_id = match u16::from_str_radix(tag_hex.trim_start_matches("0x"), 16) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let field_name = to_field(name);
        let const_name = to_const(name);

        // ✅ dedupe
        if !seen_consts.insert(const_name.to_string()) {
            continue;
        }
        if !seen_fields.insert(field_name.to_string()) {
            continue;
        }

        let (decode, rust_type, scale, is_array) = match parse_type(ty) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let field_ty = if is_array {
            quote! { Option<Vec<#rust_type>> }
        } else {
            quote! { Option<#rust_type> }
        };

        fields.push(quote! {
            pub #field_name: #field_ty,
        });

        consts.push(quote! {
            pub const #const_name: u16 = #tag_id;
        });

        let decode_logic = build_decode(&decode);

        let assign = if decode == "string" || decode == "string!" {
            if is_array {
                quote! {
                    exif.#field_name = Some(vec![
                        {
                            let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
                            String::from_utf8_lossy(&v[..end]).to_string()
                        }
                    ]);
                }
            } else {
                quote! {
                    exif.#field_name = Some({
                        let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
                        String::from_utf8_lossy(&v[..end]).to_string()
                    });
                }
            }
        } else {
            let scale = scale.unwrap_or(1.0);

            if is_array {
                quote! {
                    exif.#field_name = Some(
                        v.chunks(std::mem::size_of::<#rust_type>())
                            .map(|v| {
                                let val = #decode_logic;
                                ((val as f64) * #scale) as #rust_type
                            })
                            .collect::<Vec<#rust_type>>()
                    );
                }
            } else {
                quote! {
                    {
                        let val = #decode_logic;
                        exif.#field_name = Some(((val as f64) * #scale) as #rust_type);
                    }
                }
            }
        };

        registry.push(quote! {
            (#tag_id, |exif: &mut Exif, v: &[u8]| {
                #assign
            }),
        });
    }

    let expanded = quote! {
        pub mod generated {

            #[derive(Debug, Default)]
            pub struct Exif {
                #(#fields)*
            }

            #(#consts)*

            pub fn registry() -> &'static [(u16, fn(&mut Exif, &[u8]))] {
                &[
                    #(#registry)*
                ]
            }
        }
    };

    expanded.into()
}

/* =========================================================
   HELPERS
   ========================================================= */

fn to_const(name: &str) -> proc_macro2::Ident {
    let s = sanitize(name).to_uppercase();
    proc_macro2::Ident::new(&s, proc_macro2::Span::call_site())
}

fn to_field(name: &str) -> proc_macro2::Ident {
    let s = sanitize(name).to_lowercase();
    proc_macro2::Ident::new(&s, proc_macro2::Span::call_site())
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/* =========================================================
   TYPE PARSER
   ========================================================= */

fn parse_type(
    t: &str,
) -> syn::Result<(String, proc_macro2::TokenStream, Option<f64>, bool)> {
    let t = t.trim().trim_end_matches([':', '!', '/', '~']);

    // let is_array = t.contains('[');

    if let Some((base, meta)) = t.split_once('[') {
        let meta = meta.trim_end_matches(']');

        if let Ok(scale) = meta.parse::<f64>() {
            return Ok((base.into(), rust_type(base)?, Some(scale), true));
        }

        if meta == "n" || meta.is_empty() {
            return Ok((base.into(), rust_type(base)?, None, true));
        }

        return Err(parse_err("invalid array spec"));
    }

    if matches!(t, "undef" | "no" | "-no" | "-" | "") {
        return Err(parse_err("ignored type"));
    }

    Ok((t.into(), rust_type(t)?, None, false))
}

fn rust_type(base: &str) -> syn::Result<proc_macro2::TokenStream> {
    Ok(match base {
        "int8u" => quote! { u8 },
        "int16u" => quote! { u16 },
        "int32u" => quote! { u32 },
        "int16s" => quote! { i16 },
        "int32s" => quote! { i32 },
        "float" => quote! { f32 },
        "double" => quote! { f64 },
        "string" | "string!" => quote! { String },
        "rational64u" | "rational64s" => quote! { f64 },
        _ => return Err(parse_err(format!("unknown type '{}'", base))),
    })
}

fn parse_err(msg: impl std::fmt::Display) -> syn::Error {
    syn::Error::new(proc_macro2::Span::call_site(), msg.to_string())
}

/* =========================================================
   DECODE LOGIC
   ========================================================= */

fn build_decode(decode: &str) -> proc_macro2::TokenStream {
    match decode {
        "int8u" => quote! { v[0] as f64 },

        "int16u" => quote! {
            u16::from_le_bytes([v[0], v[1]]) as f64
        },

        "int32u" => quote! {
            u32::from_le_bytes(v[..4].try_into().unwrap()) as f64
        },

        "int16s" => quote! {
            i16::from_le_bytes([v[0], v[1]]) as f64
        },

        "int32s" => quote! {
            i32::from_le_bytes(v[..4].try_into().unwrap()) as f64
        },

        "float" => quote! {
            f32::from_le_bytes(v[..4].try_into().unwrap()) as f64
        },

        "double" => quote! {
            f64::from_le_bytes(v[..8].try_into().unwrap())
        },

        "string" | "string!" => quote! {
            {
                let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
                String::from_utf8_lossy(&v[..end]).to_string()
            }
        },

        "rational64u" => quote! {
            {
                let num = u32::from_le_bytes(v[..4].try_into().unwrap()) as f64;
                let den = u32::from_le_bytes(v[4..8].try_into().unwrap()) as f64;
                if den == 0.0 { 0.0 } else { num / den }
            }
        },

        "rational64s" => quote! {
            {
                let num = i32::from_le_bytes(v[..4].try_into().unwrap()) as f64;
                let den = i32::from_le_bytes(v[4..8].try_into().unwrap()) as f64;
                if den == 0.0 { 0.0 } else { num / den }
            }
        },

        _ => unreachable!(),
    }
}