use worth_foundational::facade::{AspectValue, InternedString};

pub(super) fn retained_scalar_value_digest_text(value: &AspectValue) -> String {
    match value {
        AspectValue::Null => "null".to_string(),
        AspectValue::Bool(value) => format!("bool:{value}"),
        AspectValue::Int8(value) => format!("int8:{value}"),
        AspectValue::Int16(value) => format!("int16:{value}"),
        AspectValue::Int32(value) => format!("int32:{value}"),
        AspectValue::Int64(value) => format!("int64:{value}"),
        AspectValue::UInt8(value) => format!("uint8:{value}"),
        AspectValue::UInt16(value) => format!("uint16:{value}"),
        AspectValue::UInt32(value) => format!("uint32:{value}"),
        AspectValue::UInt64(value) => format!("uint64:{value}"),
        AspectValue::Float32(value) => format!("float32_bits:{}", value.bits()),
        AspectValue::Float64(value) => format!("float64_bits:{}", value.bits()),
        AspectValue::Decimal(value) => format!("decimal:{}", value.as_str()),
        AspectValue::BigInt(value) => format!("bigint:{}", value.as_str()),
        AspectValue::Rational(value) => format!(
            "rational:{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        ),
        AspectValue::String(value) => format!("string:{}", retained_interned_string_text(value)),
        AspectValue::Bytes(value) => format!("bytes:{value:?}"),
        AspectValue::Uuid(value) => format!("uuid:{value:02x?}"),
        AspectValue::Date(value) => format!("date_days:{}", value.days_from_unix_epoch),
        AspectValue::Time(value) => format!("time_nanos:{}", value.nanos_since_midnight),
        AspectValue::Timestamp(value) => {
            format!("timestamp_micros:{}", value.micros_since_unix_epoch)
        }
        AspectValue::TimestampTz(value) => format!(
            "timestamp_tz:{}@{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        ),
        AspectValue::EntityRef(value) => format!("entity_ref:{value:?}"),
        AspectValue::ContentRef(value) => format!("content_ref:{value:?}"),
    }
}

fn retained_interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(value) => value.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}
