use crate::ir::{Schema, FieldType};

pub fn generate_code(schema: Schema) -> String {
    let mut out = String::new();

    for ns in schema.namespaces {
        out.push_str(&format!("namespace_gen!({} {{\n", ns.name));

        for f in ns.fields {
            let ty = format_type(&f.ty);

            out.push_str(&format!(
                "    {}: \"{}\" => {},\n",
                f.rust_name, f.xmp_key, ty
            ));
        }

        out.push_str("});\n\n");
    }

    out
}

fn format_type(ty: &FieldType) -> String {
    match ty {
        FieldType::String => "String".into(),
        FieldType::Integer => "Integer".into(),
        FieldType::Real => "Real".into(),
        FieldType::Bool => "Bool".into(),
        FieldType::Struct(name) => format!("Struct({})", name),
        FieldType::Vec(inner) => format!("Vec<{}>", format_type(inner)),
    }
}