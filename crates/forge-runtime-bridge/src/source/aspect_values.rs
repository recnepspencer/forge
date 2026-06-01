use forge_foundational::facade::{AspectValue, CanonicalF64, InternedString};
use serde_json::Value;

pub(super) fn decode_snapshot_aspect_bytes(
    aspect_bytes: &[u8],
) -> Result<AspectValue, SnapshotAspectBytesDecodeError> {
    if let Ok(value) = serde_json::from_slice::<Value>(aspect_bytes) {
        return scalar_aspect_value_from_json(value);
    }
    let text = std::str::from_utf8(aspect_bytes)
        .map_err(|_| SnapshotAspectBytesDecodeError::InvalidUtf8AspectBytes)?;
    Ok(AspectValue::String(text.to_string().into()))
}

pub(super) fn canonical_aspect_value_text(value: &AspectValue) -> String {
    match value {
        AspectValue::Null => "null".to_string(),
        AspectValue::Bool(value) => format!("bool:{value}"),
        AspectValue::Int8(value) => format!("i8:{value}"),
        AspectValue::Int16(value) => format!("i16:{value}"),
        AspectValue::Int32(value) => format!("i32:{value}"),
        AspectValue::Int64(value) => format!("i64:{value}"),
        AspectValue::UInt8(value) => format!("u8:{value}"),
        AspectValue::UInt16(value) => format!("u16:{value}"),
        AspectValue::UInt32(value) => format!("u32:{value}"),
        AspectValue::UInt64(value) => format!("u64:{value}"),
        AspectValue::Float32(value) => format!("f32-bits:{}", value.bits()),
        AspectValue::Float64(value) => format!("f64-bits:{}", value.bits()),
        AspectValue::Decimal(value) => format!("decimal:{}", value.as_str()),
        AspectValue::BigInt(value) => format!("bigint:{}", value.as_str()),
        AspectValue::Rational(value) => format!(
            "rational:{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        ),
        AspectValue::String(value) => format!("string:{}", interned_string_text(value)),
        AspectValue::Bytes(value) => format!("bytes-ref:{}", value.0),
        AspectValue::Uuid(value) => format!("uuid:{}", hex_bytes(value)),
        AspectValue::Date(value) => format!("date-days:{}", value.days_from_unix_epoch),
        AspectValue::Time(value) => format!("time-nanos:{}", value.nanos_since_midnight),
        AspectValue::Timestamp(value) => {
            format!("timestamp-micros:{}", value.micros_since_unix_epoch)
        }
        AspectValue::TimestampTz(value) => format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        ),
        AspectValue::EntityRef(value) => format!(
            "entity-ref:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        ),
        AspectValue::ContentRef(value) => format!("content-ref:{}", value.0),
    }
}

#[cfg(test)]
pub(super) fn aspect_value_to_json(value: &AspectValue) -> Value {
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
        AspectValue::Float32(value) => finite_float_json_or_tag(
            f32::from_bits(value.bits()) as f64,
            "f32-bits",
            value.bits() as u64,
        ),
        AspectValue::Float64(value) => {
            finite_float_json_or_tag(f64::from_bits(value.bits()), "f64-bits", value.bits())
        }
        AspectValue::Decimal(value) => Value::String(value.as_str().to_string()),
        AspectValue::BigInt(value) => Value::String(value.as_str().to_string()),
        AspectValue::Rational(value) => Value::String(format!(
            "{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        )),
        AspectValue::String(value) => Value::String(interned_string_text(value)),
        AspectValue::Bytes(value) => Value::String(format!("bytes-ref:{}", value.0)),
        AspectValue::Uuid(value) => Value::String(format!("uuid:{}", hex_bytes(value))),
        AspectValue::Date(value) => {
            Value::String(format!("date-days:{}", value.days_from_unix_epoch))
        }
        AspectValue::Time(value) => {
            Value::String(format!("time-nanos:{}", value.nanos_since_midnight))
        }
        AspectValue::Timestamp(value) => Value::String(format!(
            "timestamp-micros:{}",
            value.micros_since_unix_epoch
        )),
        AspectValue::TimestampTz(value) => Value::String(format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        )),
        AspectValue::EntityRef(value) => Value::String(format!(
            "entity-ref:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        )),
        AspectValue::ContentRef(value) => Value::String(format!("content-ref:{}", value.0)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SnapshotAspectBytesDecodeError {
    UnsupportedJsonShape,
    InvalidUtf8AspectBytes,
}

fn scalar_aspect_value_from_json(
    value: Value,
) -> Result<AspectValue, SnapshotAspectBytesDecodeError> {
    match value {
        Value::Null => Ok(AspectValue::Null),
        Value::Bool(value) => Ok(AspectValue::Bool(value)),
        Value::Number(value) => scalar_number_aspect_value(value)
            .ok_or(SnapshotAspectBytesDecodeError::UnsupportedJsonShape),
        Value::String(value) => Ok(AspectValue::String(value.into())),
        Value::Array(_) | Value::Object(_) => {
            Err(SnapshotAspectBytesDecodeError::UnsupportedJsonShape)
        }
    }
}

fn scalar_number_aspect_value(number: serde_json::Number) -> Option<AspectValue> {
    if let Some(value) = number.as_i64() {
        return Some(AspectValue::Int64(value));
    }
    if let Some(value) = number.as_u64() {
        return Some(AspectValue::UInt64(value));
    }
    number
        .as_f64()
        .map(|value| AspectValue::Float64(CanonicalF64::from_f64(value)))
}

#[cfg(test)]
fn finite_float_json_or_tag(value: f64, label: &str, bits: u64) -> Value {
    serde_json::Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or_else(|| Value::String(format!("{label}:{bits}")))
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
