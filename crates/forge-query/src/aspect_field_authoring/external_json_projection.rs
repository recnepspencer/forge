use forge_foundational::facade::{AspectValue, InternedString};
use serde_json::Value;

pub(crate) fn project_aspect_value_to_workspace_json(value: &AspectValue) -> Value {
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

fn interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(value) => value.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}
