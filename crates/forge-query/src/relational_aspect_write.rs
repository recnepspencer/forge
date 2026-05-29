use std::collections::BTreeMap;

use forge_foundational::facade::{
    aspects, AspectIdentity, AspectKey, AspectValue, CanonicalF64, FieldKey, ScalarAspectType,
};
use forge_relational::facade::schema::{AspectBinding, DeclaredAspect};
use forge_relational::facade::transactions::{AspectFieldPatch, AspectFieldPatchTarget};
use serde_json::Value;

pub(crate) fn entity_string_field_aspect(
    aspect_label: &str,
    field_label: &str,
) -> Result<DeclaredAspect, String> {
    Ok(DeclaredAspect {
        binding: AspectBinding::EntityField {
            field: field_key(field_label)?,
        },
        contract: scalar_string_contract(aspect_label)?,
    })
}

#[cfg(test)]
pub(crate) fn relation_string_field_aspect(
    aspect_label: &str,
    field_label: &str,
) -> Result<DeclaredAspect, String> {
    Ok(DeclaredAspect {
        binding: AspectBinding::RelationField {
            field: field_key(field_label)?,
        },
        contract: scalar_string_contract(aspect_label)?,
    })
}

pub(crate) fn lifecycle_string_aspect(aspect_label: &str) -> Result<DeclaredAspect, String> {
    Ok(DeclaredAspect {
        binding: AspectBinding::LifecycleTransition,
        contract: scalar_string_contract(aspect_label)?,
    })
}

#[cfg(test)]
pub(crate) fn relation_source_endpoint_aspect(
    aspect_label: &str,
) -> Result<DeclaredAspect, String> {
    Ok(DeclaredAspect {
        binding: AspectBinding::RelationSourceEndpoint,
        contract: entity_reference_contract(aspect_label)?,
    })
}

#[cfg(test)]
pub(crate) fn relation_target_endpoint_aspect(
    aspect_label: &str,
) -> Result<DeclaredAspect, String> {
    Ok(DeclaredAspect {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect_label)?,
    })
}

pub(crate) fn single_field_patch(
    aspect_label: &str,
    field_label: &str,
    value: Value,
) -> Result<AspectFieldPatch, String> {
    field_patch_from_values([(aspect_label, field_label, value)])
}

pub(crate) fn field_patch_from_values<'a>(
    values: impl IntoIterator<Item = (&'a str, &'a str, Value)>,
) -> Result<AspectFieldPatch, String> {
    let mut targets = BTreeMap::new();
    for (aspect_label, field_label, value) in values {
        targets.insert(
            AspectFieldPatchTarget::single(aspect_key(aspect_label)?, field_key(field_label)?),
            json_scalar_to_aspect_value(value)?,
        );
    }
    Ok(AspectFieldPatch::from(targets))
}

pub(crate) fn json_scalar_to_aspect_value(value: Value) -> Result<AspectValue, String> {
    match value {
        Value::Null => Ok(AspectValue::Null),
        Value::Bool(value) => Ok(AspectValue::Bool(value)),
        Value::Number(value) => json_number_to_aspect_value(value),
        Value::String(value) => Ok(AspectValue::String(value.into())),
        Value::Array(_) | Value::Object(_) => Err(
            "relational aspect field patches only admit scalar JSON values at this boundary"
                .to_string(),
        ),
    }
}

pub(crate) fn aspect_value_to_json(value: &AspectValue) -> Value {
    match value {
        AspectValue::Null => Value::Null,
        AspectValue::Bool(value) => Value::Bool(*value),
        AspectValue::Int8(value) => Value::from(*value),
        AspectValue::Int16(value) => Value::from(*value),
        AspectValue::Int32(value) => Value::from(*value),
        AspectValue::Int64(value) => Value::from(*value),
        AspectValue::UInt8(value) => Value::from(*value),
        AspectValue::UInt16(value) => Value::from(*value),
        AspectValue::UInt32(value) => Value::from(*value),
        AspectValue::UInt64(value) => Value::from(*value),
        AspectValue::Float32(value) => {
            serde_json::Number::from_f64(f32::from_bits(value.bits()) as f64)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        AspectValue::Float64(value) => serde_json::Number::from_f64(f64::from_bits(value.bits()))
            .map(Value::Number)
            .unwrap_or(Value::Null),
        AspectValue::String(value) => Value::String(interned_string_text(value)),
        other => Value::String(format!("{other:?}")),
    }
}

pub(crate) fn aspect_key(label: &str) -> Result<AspectKey, String> {
    AspectKey::new(label).ok_or_else(|| format!("`{label}` is not a foundational aspect key"))
}

pub(crate) fn field_key(label: &str) -> Result<FieldKey, String> {
    FieldKey::new(label).ok_or_else(|| format!("`{label}` is not a foundational field key"))
}

pub(crate) fn terminal_field_label(path: &str) -> Result<&str, String> {
    path.split('.')
        .next_back()
        .filter(|segment| !segment.trim().is_empty())
        .ok_or_else(|| format!("`{path}` does not contain a field segment"))
}

fn scalar_string_contract(
    aspect_label: &str,
) -> Result<forge_foundational::AspectContract, String> {
    Ok(aspects()
        .contract()
        .for_key(aspect_key(aspect_label)?)
        .identified_by(AspectIdentity(stable_contract_identity(aspect_label)))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String))
}

#[cfg(test)]
fn entity_reference_contract(
    aspect_label: &str,
) -> Result<forge_foundational::AspectContract, String> {
    Ok(aspects()
        .contract()
        .for_key(aspect_key(aspect_label)?)
        .identified_by(AspectIdentity(stable_contract_identity(aspect_label)))
        .at_revision(aspects().vocabulary().revision(1))
        .reference_entity())
}

fn json_number_to_aspect_value(value: serde_json::Number) -> Result<AspectValue, String> {
    if let Some(value) = value.as_i64() {
        return Ok(AspectValue::Int64(value));
    }
    if let Some(value) = value.as_u64() {
        return Ok(AspectValue::UInt64(value));
    }
    let value = value
        .as_f64()
        .ok_or_else(|| "JSON number could not lower into an aspect value".to_string())?;
    Ok(AspectValue::Float64(CanonicalF64::from_f64(value)))
}

fn interned_string_text(value: &forge_foundational::facade::InternedString) -> String {
    match value {
        forge_foundational::facade::InternedString::Raw(value) => value.clone(),
        forge_foundational::facade::InternedString::Symbol(symbol) => {
            format!("symbol:{}", symbol.0)
        }
    }
}

fn stable_contract_identity(label: &str) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in label.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211_u64);
    }
    hash
}
