use forge_foundational::facade::{AspectValue, InternedString};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryDesiredAspectOperation {
    Set,
    Clear,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDesiredAspectValue {
    operation: ForgeQueryDesiredAspectOperation,
    value: Option<AspectValue>,
}

impl ForgeQueryDesiredAspectValue {
    pub(crate) fn set_native(value: AspectValue) -> Self {
        Self {
            operation: ForgeQueryDesiredAspectOperation::Set,
            value: Some(value),
        }
    }

    pub(crate) fn clear() -> Self {
        Self {
            operation: ForgeQueryDesiredAspectOperation::Clear,
            value: None,
        }
    }

    pub fn value(&self) -> Option<&AspectValue> {
        self.value.as_ref()
    }

    pub fn clears_existing_value(&self) -> bool {
        self.operation == ForgeQueryDesiredAspectOperation::Clear
    }

    pub(crate) fn terminal_digest_material(&self) -> String {
        match (self.operation, self.value.as_ref()) {
            (ForgeQueryDesiredAspectOperation::Clear, _) => "clear".to_string(),
            (ForgeQueryDesiredAspectOperation::Set, Some(value)) => {
                format!("set:{}", terminal_aspect_value_digest_text(value))
            }
            (ForgeQueryDesiredAspectOperation::Set, None) => "set:<missing>".to_string(),
        }
    }
}

pub(crate) fn terminal_aspect_value_digest_text(value: &AspectValue) -> String {
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
        AspectValue::String(value) => match value {
            InternedString::Raw(value) => format!("string:{}:{value}", value.len()),
            InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
        },
        AspectValue::Bytes(value) => format!("bytes-ref:{}", value.0),
        AspectValue::Uuid(value) => value.iter().map(|byte| format!("{byte:02x}")).collect(),
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
            "entity:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        ),
        AspectValue::ContentRef(value) => format!("content-ref:{}", value.0),
    }
}
