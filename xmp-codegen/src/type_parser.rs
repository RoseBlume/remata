use crate::ir::FieldType;

pub fn parse_type(raw: &str) -> FieldType {
    let raw = raw.to_lowercase();

    if raw.contains("struct") && raw.contains("+") {
        return FieldType::Vec(Box::new(FieldType::Struct("unknown".into())));
    }

    if raw.starts_with("struct") {
        return FieldType::Struct("unknown".into());
    }

    if raw.starts_with("real") {
        return FieldType::Real;
    }

    if raw.starts_with("integer") {
        return FieldType::Integer;
    }

    if raw.starts_with("bool") {
        return FieldType::Bool;
    }

    FieldType::String
}