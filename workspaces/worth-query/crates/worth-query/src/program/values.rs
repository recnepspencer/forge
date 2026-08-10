use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalF64, CanonicalFieldPath, InternedString};

use crate::memory_workspace::WorthQueryEntity;

use super::error::WorthQueryProgramError;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryProgramValue {
    value: WorthQueryProgramValueTree,
}

#[derive(Clone, Debug, PartialEq)]
enum WorthQueryProgramValueTree {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<WorthQueryProgramValueTree>),
    Object(BTreeMap<String, WorthQueryProgramValueTree>),
    NativeScalar(AspectValue),
}

impl WorthQueryProgramValue {
    pub fn null() -> Self {
        Self {
            value: WorthQueryProgramValueTree::Null,
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Bool(value),
        }
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: WorthQueryProgramValueTree::String(value.into()),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Number(value.to_string()),
        }
    }

    pub fn unsigned_integer(value: u64) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Number(value.to_string()),
        }
    }

    pub fn decimal_text(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !is_canonical_number_text(&value) {
            return Err(format!(
                "program value number `{value}` is not valid canonical number text"
            ));
        }
        Ok(Self {
            value: WorthQueryProgramValueTree::Number(value),
        })
    }

    pub fn array(values: impl IntoIterator<Item = WorthQueryProgramValue>) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Array(
                values.into_iter().map(|value| value.value).collect(),
            ),
        }
    }

    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, WorthQueryProgramValue)>,
    ) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.value))
                    .collect(),
            ),
        }
    }

    pub(super) fn from_live_read_entities(
        rows: impl IntoIterator<Item = WorthQueryEntity>,
    ) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Array(
                rows.into_iter()
                    .map(|row| program_value_tree_from_live_read_entity(&row))
                    .collect(),
            ),
        }
    }

    pub(crate) fn foundational_scalar_value(&self) -> Result<AspectValue, WorthQueryProgramError> {
        foundational_scalar_value_from_program_value_tree(&self.value)
    }

    pub fn array_len(&self) -> Option<usize> {
        let WorthQueryProgramValueTree::Array(values) = &self.value else {
            return None;
        };
        Some(values.len())
    }

    pub fn field_path_value(
        &self,
        field_path: &CanonicalFieldPath,
    ) -> Option<WorthQueryProgramValueField<'_>> {
        let value = program_value_tree_at_field_path(&self.value, field_path)?;
        Some(WorthQueryProgramValueField { value })
    }

    pub fn field_path_string_value(&self, field_path: &CanonicalFieldPath) -> Option<&str> {
        program_tree_string_value(program_value_tree_at_field_path(&self.value, field_path)?)
    }

    pub fn array_field_path_string_value(
        &self,
        index: usize,
        field_path: &CanonicalFieldPath,
    ) -> Option<&str> {
        let WorthQueryProgramValueTree::Array(values) = &self.value else {
            return None;
        };
        program_tree_string_value(program_value_tree_at_field_path(
            values.get(index)?,
            field_path,
        )?)
    }

    pub fn string_value(&self) -> Option<&str> {
        let WorthQueryProgramValueTree::String(value) = &self.value else {
            return None;
        };
        Some(value)
    }

    pub fn is_string(&self) -> bool {
        matches!(self.value, WorthQueryProgramValueTree::String(_))
    }

    pub fn is_integer(&self) -> bool {
        match &self.value {
            WorthQueryProgramValueTree::Number(value) => {
                value.parse::<i64>().is_ok() || value.parse::<u64>().is_ok()
            }
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self.value, WorthQueryProgramValueTree::Bool(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthQueryProgramValueField<'a> {
    value: &'a WorthQueryProgramValueTree,
}

impl WorthQueryProgramValueField<'_> {
    pub fn string_value(&self) -> Option<&str> {
        program_tree_string_value(self.value)
    }

    pub fn foundational_scalar_value(&self) -> Result<AspectValue, WorthQueryProgramError> {
        foundational_scalar_value_from_program_value_tree(self.value)
    }
}

fn program_value_tree_from_live_read_entity(row: &WorthQueryEntity) -> WorthQueryProgramValueTree {
    let mut fields = BTreeMap::new();
    for (field_path, value) in row.native_field_values() {
        insert_program_field_path(
            &mut fields,
            field_path,
            program_value_tree_from_aspect_value(value),
        );
    }
    WorthQueryProgramValueTree::Object(fields)
}

fn insert_program_field_path(
    target: &mut BTreeMap<String, WorthQueryProgramValueTree>,
    field_path: &CanonicalFieldPath,
    value: WorthQueryProgramValueTree,
) {
    let segments = field_path
        .fields()
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect::<Vec<_>>();
    insert_program_path_segments(target, &segments, value);
}

fn insert_program_path_segments(
    target: &mut BTreeMap<String, WorthQueryProgramValueTree>,
    segments: &[String],
    value: WorthQueryProgramValueTree,
) {
    let Some((head, tail)) = segments.split_first() else {
        return;
    };
    if tail.is_empty() {
        target.insert(head.clone(), value);
        return;
    }

    let entry = target
        .entry(head.clone())
        .or_insert_with(|| WorthQueryProgramValueTree::Object(BTreeMap::new()));
    let WorthQueryProgramValueTree::Object(fields) = entry else {
        *entry = WorthQueryProgramValueTree::Object(BTreeMap::new());
        let WorthQueryProgramValueTree::Object(fields) = entry else {
            unreachable!("program path segment was just replaced with an object");
        };
        insert_program_path_segments(fields, tail, value);
        return;
    };
    insert_program_path_segments(fields, tail, value);
}

fn program_value_tree_at_field_path<'a>(
    value: &'a WorthQueryProgramValueTree,
    field_path: &CanonicalFieldPath,
) -> Option<&'a WorthQueryProgramValueTree> {
    let mut current = value;
    for field in field_path.fields() {
        let WorthQueryProgramValueTree::Object(fields) = current else {
            return None;
        };
        current = fields.get(field.as_str())?;
    }
    Some(current)
}

fn program_value_tree_from_aspect_value(value: &AspectValue) -> WorthQueryProgramValueTree {
    WorthQueryProgramValueTree::NativeScalar(value.clone())
}

fn program_tree_string_value(value: &WorthQueryProgramValueTree) -> Option<&str> {
    match value {
        WorthQueryProgramValueTree::String(value) => Some(value),
        WorthQueryProgramValueTree::NativeScalar(AspectValue::String(InternedString::Raw(
            value,
        ))) => Some(value),
        _ => None,
    }
}

fn is_canonical_number_text(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    if bytes.is_empty() {
        return false;
    }
    if bytes[index] == b'-' {
        index += 1;
        if index == bytes.len() {
            return false;
        }
    }
    match bytes[index] {
        b'0' => {
            index += 1;
            if index < bytes.len() && bytes[index].is_ascii_digit() {
                return false;
            }
        }
        b'1'..=b'9' => {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        _ => return false,
    }
    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        if index == bytes.len() || !bytes[index].is_ascii_digit() {
            return false;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
    }
    if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
        index += 1;
        if index < bytes.len() && matches!(bytes[index], b'+' | b'-') {
            index += 1;
        }
        if index == bytes.len() || !bytes[index].is_ascii_digit() {
            return false;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
    }
    index == bytes.len()
}

fn foundational_scalar_value_from_program_value_tree(
    value: &WorthQueryProgramValueTree,
) -> Result<AspectValue, WorthQueryProgramError> {
    match value {
        WorthQueryProgramValueTree::Null => Ok(AspectValue::Null),
        WorthQueryProgramValueTree::Bool(value) => Ok(AspectValue::Bool(*value)),
        WorthQueryProgramValueTree::Number(value) => {
            if let Ok(value) = value.parse::<i64>() {
                Ok(AspectValue::Int64(value))
            } else if let Ok(value) = value.parse::<u64>() {
                Ok(AspectValue::UInt64(value))
            } else if let Ok(value) = value.parse::<f64>() {
                if !value.is_finite() {
                    return Err(WorthQueryProgramError::new(
                        "program scalar aspect value number must be finite",
                    ));
                }
                Ok(AspectValue::Float64(CanonicalF64::from_f64(value)))
            } else {
                Err(WorthQueryProgramError::new(format!(
                    "program scalar aspect value number `{value}` is invalid"
                )))
            }
        }
        WorthQueryProgramValueTree::String(value) => Ok(
            crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(value.clone()),
        ),
        WorthQueryProgramValueTree::NativeScalar(value) => Ok(value.clone()),
        WorthQueryProgramValueTree::Array(_) => Err(WorthQueryProgramError::new(
            "program scalar aspect value cannot be an array",
        )),
        WorthQueryProgramValueTree::Object(_) => Err(WorthQueryProgramError::new(
            "program scalar aspect value cannot be an object",
        )),
    }
}
