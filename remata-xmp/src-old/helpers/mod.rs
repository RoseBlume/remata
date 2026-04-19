use super::{
    XmpValue,
    Rational
};
pub fn split_ns(tag: &str) -> Option<(&str, &str)> {
    let mut parts = tag.splitn(2, ':');
    Some((parts.next()?, parts.next()?))
}

pub fn as_i64(v: &XmpValue) -> Option<i64> {
    match v {
        XmpValue::Integer(i) => Some(*i),
        XmpValue::String(s) => s.parse().ok(),
        _ => None,
    }
}

pub fn as_i32(v: &XmpValue) -> Option<i32> {
    as_i64(v).map(|v| v as i32)
}

pub fn as_string(v: &XmpValue) -> Option<String> {
    match v {
        XmpValue::String(s) => Some(s.clone()),
        _ => None,
    }
}

pub fn as_rational(v: &XmpValue) -> Option<Rational> {
    match v {
        XmpValue::String(s) => Rational::from_str(s),
        _ => None,
    }
}