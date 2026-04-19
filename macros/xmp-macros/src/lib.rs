use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitStr, Token,
};

/// Field:
/// name: "Key" => Kind
struct Field {
    name: Ident,
    _colon: Token![:],
    key: LitStr,
    _arrow: Token![=>],
    kind: Ident,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            key: input.parse()?,
            _arrow: input.parse()?,
            kind: input.parse()?,
        })
    }
}

struct MacroInput {
    struct_name: Ident,
    fields: Vec<Field>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;
        let content;
        syn::braced!(content in input);

        let fields = content.parse_terminated(Field::parse, Token![,])?;

        Ok(Self {
            struct_name,
            fields: fields.into_iter().collect(),
        })
    }
}

#[proc_macro]
pub fn namespace_gen(input: TokenStream) -> TokenStream {
    let MacroInput {
        struct_name,
        fields,
    } = parse_macro_input!(input as MacroInput);

    let field_names: Vec<_> = fields.iter().map(|f| &f.name).collect();
    let keys: Vec<_> = fields.iter().map(|f| &f.key).collect();
    let kinds: Vec<_> = fields.iter().map(|f| &f.kind).collect();

    let expanded = quote! {
        #[derive(Debug, Clone, Default)]
        pub struct #struct_name {
            #( pub #field_names: Option<XmpValue>, )*
        }


        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #( #field_names: None, )*
                }
            }

            fn map_kind(kind: &str, value: &str) -> Option<XmpValue> {
                match kind {
                    "String" => Some(XmpValue::String(value.to_string())),
                    "Real" => value.parse().ok().map(XmpValue::Real),
                    "Integer" => value.parse().ok().map(XmpValue::Integer),
                    "Bool" => match value.to_lowercase().as_str() {
                        "true" => Some(XmpValue::Bool(true)),
                        "false" => Some(XmpValue::Bool(false)),
                        _ => None,
                    },

                    // arrays
                    "StringArray" => {
                        Some(XmpValue::Array(vec![XmpValue::String(value.to_string())]))
                    }
                    "RealArray" => {
                        value.parse().ok().map(|v| XmpValue::Array(vec![XmpValue::Real(v)]))
                    }
                    "BoolArray" => {
                        let b = matches!(value.to_lowercase().as_str(), "true");
                        Some(XmpValue::Array(vec![XmpValue::Bool(b)]))
                    }

                    // structs
                    "Struct" => {
                        Some(XmpValue::Struct(XmpStructValue {
                            fields: std::collections::HashMap::new(),
                        }))
                    }
                    "StructArray" => {
                        Some(XmpValue::Array(vec![
                            XmpValue::Struct(XmpStructValue {
                                fields: std::collections::HashMap::new(),
                            })
                        ]))
                    }

                    _ => None,
                }
            }

            pub fn insert(&mut self, key: &str, value: &str, kind: &str) {
                match key {
                    #(
                        #keys => {
                            self.#field_names = Self::map_kind(stringify!(#kinds), value);
                        }
                    )*
                    _ => {}
                }
            }
        }
    };

    TokenStream::from(expanded)
}