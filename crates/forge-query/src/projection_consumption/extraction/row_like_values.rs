use forge_foundational::facade::{AspectValue, InternedString};

pub(super) fn consumed_aspect_value_as_str(value: &AspectValue) -> Option<&str> {
    match value {
        AspectValue::String(value) => match value {
            InternedString::Raw(value) => Some(value.as_str()),
            InternedString::Symbol(_) => None,
        },
        _ => None,
    }
}
