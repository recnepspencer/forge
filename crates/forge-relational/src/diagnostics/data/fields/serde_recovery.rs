use serde_json::{Map, Value};

use super::RelationalDiagnosticValue;

pub(super) fn diagnostic_value_from_serde_value(value: &Value) -> RelationalDiagnosticValue {
    match value {
        Value::Null => RelationalDiagnosticValue::Null,
        Value::Bool(boolean) => RelationalDiagnosticValue::Bool(*boolean),
        Value::Number(number) => diagnostic_number_value(number),
        Value::String(text) => RelationalDiagnosticValue::String(text.clone()),
        Value::Array(values) => {
            RelationalDiagnosticValue::array(values.iter().map(diagnostic_value_from_serde_value))
        }
        Value::Object(entries) => RelationalDiagnosticValue::object(
            entries
                .iter()
                .map(|(key, value)| (key.clone(), diagnostic_value_from_serde_value(value))),
        ),
    }
}

pub(super) fn canonicalize_serde_value(value: &Value) -> Value {
    match value {
        Value::Null => Value::Null,
        Value::Bool(boolean) => Value::Bool(*boolean),
        Value::Number(number) => Value::Number(number.clone()),
        Value::String(text) => Value::String(text.clone()),
        Value::Array(values) => Value::Array(values.iter().map(canonicalize_serde_value).collect()),
        Value::Object(entries) => {
            let mut ordered = Map::new();
            let mut keys = entries.keys().cloned().collect::<Vec<_>>();
            keys.sort();
            for key in keys {
                let value = entries
                    .get(&key)
                    .expect("object key collected from map must exist");
                ordered.insert(key, canonicalize_serde_value(value));
            }
            Value::Object(ordered)
        }
    }
}

fn diagnostic_number_value(number: &serde_json::Number) -> RelationalDiagnosticValue {
    if let Some(value) = number.as_u64() {
        return RelationalDiagnosticValue::Unsigned(value);
    }
    if let Some(value) = number.as_i64() {
        return RelationalDiagnosticValue::Signed(value);
    }
    RelationalDiagnosticValue::String(number.to_string())
}
