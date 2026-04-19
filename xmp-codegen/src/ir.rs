#[derive(Debug, Clone)]
pub struct Schema {
    pub namespaces: Vec<Namespace>,
    pub structs: Vec<StructDef>,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub rust_name: String,
    pub xmp_key: String,
    pub ty: FieldType,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    String,
    Integer,
    Real,
    Bool,
    Struct(String),
    Vec(Box<FieldType>),
}