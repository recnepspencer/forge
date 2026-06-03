use forge_foundational::facade::{
    AspectValue, ContractValidatedAspectArtifact, ContractValidatedAspectValueView, InternedString,
    StructAspectValue,
};

pub(super) fn project_validated_aspect_value_for_consumption_json(
    value: &ContractValidatedAspectArtifact,
) -> serde_json::Value {
    match value.payload().view() {
        ContractValidatedAspectValueView::Scalar(value) => {
            project_aspect_value_for_consumption_json(value)
        }
        ContractValidatedAspectValueView::Struct(value) => {
            project_struct_aspect_value_for_consumption_json(value)
        }
    }
}

pub(super) fn project_aspect_value_for_consumption_json(value: &AspectValue) -> serde_json::Value {
    match value {
        AspectValue::Null => serde_json::Value::Null,
        AspectValue::Bool(value) => serde_json::Value::Bool(*value),
        AspectValue::Int8(value) => serde_json::Value::from(*value),
        AspectValue::Int16(value) => serde_json::Value::from(*value),
        AspectValue::Int32(value) => serde_json::Value::from(*value),
        AspectValue::Int64(value) => serde_json::Value::from(*value),
        AspectValue::UInt8(value) => serde_json::Value::from(*value),
        AspectValue::UInt16(value) => serde_json::Value::from(*value),
        AspectValue::UInt32(value) => serde_json::Value::from(*value),
        AspectValue::UInt64(value) => serde_json::Value::from(*value),
        AspectValue::Float32(value) => float_value_to_json(
            f32::from_bits(value.bits()) as f64,
            "f32-bits",
            value.bits() as u64,
        ),
        AspectValue::Float64(value) => {
            float_value_to_json(f64::from_bits(value.bits()), "f64-bits", value.bits())
        }
        AspectValue::Decimal(value) => serde_json::Value::String(value.as_str().to_string()),
        AspectValue::BigInt(value) => serde_json::Value::String(value.as_str().to_string()),
        AspectValue::Rational(value) => serde_json::Value::String(format!(
            "{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        )),
        AspectValue::String(value) => serde_json::Value::String(interned_string_text(value)),
        AspectValue::Bytes(value) => serde_json::Value::String(format!("bytes-ref:{}", value.0)),
        AspectValue::Uuid(value) => serde_json::Value::String(format!("uuid:{}", hex_bytes(value))),
        AspectValue::Date(value) => {
            serde_json::Value::String(format!("date-days:{}", value.days_from_unix_epoch))
        }
        AspectValue::Time(value) => {
            serde_json::Value::String(format!("time-nanos:{}", value.nanos_since_midnight))
        }
        AspectValue::Timestamp(value) => serde_json::Value::String(format!(
            "timestamp-micros:{}",
            value.micros_since_unix_epoch
        )),
        AspectValue::TimestampTz(value) => serde_json::Value::String(format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        )),
        AspectValue::EntityRef(value) => serde_json::Value::String(format!(
            "entity-ref:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        )),
        AspectValue::ContentRef(value) => {
            serde_json::Value::String(format!("content-ref:{}", value.0))
        }
    }
}

fn project_struct_aspect_value_for_consumption_json(
    value: &StructAspectValue,
) -> serde_json::Value {
    serde_json::Value::Object(
        value
            .fields()
            .map(|(field, value)| {
                (
                    field.as_str().to_string(),
                    project_aspect_value_for_consumption_json(value),
                )
            })
            .collect(),
    )
}

fn float_value_to_json(value: f64, label: &str, bits: u64) -> serde_json::Value {
    serde_json::Number::from_f64(value)
        .map(serde_json::Value::Number)
        .unwrap_or_else(|| serde_json::Value::String(format!("{label}:{bits}")))
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(text) => text.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}
