use serde_json::Value;

use super::JsonCompatibilityLoweringDenial;
use crate::locators::BoundarySourceLocator;
use crate::values::{
    AspectValue, CanonicalDate, CanonicalF32, CanonicalF64, CanonicalTime, CanonicalTimestamp,
    ScalarAspectType,
};

pub(super) fn lower_numeric_scalar(
    source: &BoundarySourceLocator,
    value: &Value,
    expected: ScalarAspectType,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    match expected {
        ScalarAspectType::Int8 => {
            signed_integer(value, source, expected, i8::MIN as i64, i8::MAX as i64)
                .map(|value| AspectValue::Int8(value as i8))
        }
        ScalarAspectType::Int16 => {
            signed_integer(value, source, expected, i16::MIN as i64, i16::MAX as i64)
                .map(|value| AspectValue::Int16(value as i16))
        }
        ScalarAspectType::Int32 => {
            signed_integer(value, source, expected, i32::MIN as i64, i32::MAX as i64)
                .map(|value| AspectValue::Int32(value as i32))
        }
        ScalarAspectType::Int64 => {
            signed_integer(value, source, expected, i64::MIN, i64::MAX).map(AspectValue::Int64)
        }
        ScalarAspectType::UInt8 => unsigned_integer(value, source, expected, u8::MAX as u64)
            .map(|value| AspectValue::UInt8(value as u8)),
        ScalarAspectType::UInt16 => unsigned_integer(value, source, expected, u16::MAX as u64)
            .map(|value| AspectValue::UInt16(value as u16)),
        ScalarAspectType::UInt32 => unsigned_integer(value, source, expected, u32::MAX as u64)
            .map(|value| AspectValue::UInt32(value as u32)),
        ScalarAspectType::UInt64 => {
            unsigned_integer(value, source, expected, u64::MAX).map(AspectValue::UInt64)
        }
        ScalarAspectType::Float32 => value
            .as_f64()
            .map(|value| AspectValue::Float32(CanonicalF32::from_f32(value as f32)))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "number",
            }),
        ScalarAspectType::Float64 => value
            .as_f64()
            .map(|value| AspectValue::Float64(CanonicalF64::from_f64(value)))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "number",
            }),
        ScalarAspectType::Date => {
            signed_integer(value, source, expected, i32::MIN as i64, i32::MAX as i64).map(|value| {
                AspectValue::Date(CanonicalDate {
                    days_from_unix_epoch: value as i32,
                })
            })
        }
        ScalarAspectType::Time => {
            let nanos_since_midnight =
                unsigned_integer(value, source, expected, CanonicalTime::NANOS_PER_DAY - 1)?;
            Ok(AspectValue::Time(
                CanonicalTime::new(nanos_since_midnight).expect("bounded above"),
            ))
        }
        ScalarAspectType::Timestamp => signed_integer(value, source, expected, i64::MIN, i64::MAX)
            .map(|value| {
                AspectValue::Timestamp(CanonicalTimestamp {
                    micros_since_unix_epoch: value,
                })
            }),
        _ => Err(JsonCompatibilityLoweringDenial::UnsupportedScalarFamily {
            source: source.clone(),
            expected,
        }),
    }
}

pub(crate) fn required_u64(
    value: Option<&Value>,
    source: &BoundarySourceLocator,
    expected: ScalarAspectType,
) -> Result<u64, JsonCompatibilityLoweringDenial> {
    value.and_then(Value::as_u64).ok_or_else(|| {
        JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
            source: source.clone(),
            expected,
        }
    })
}

fn signed_integer(
    value: &Value,
    source: &BoundarySourceLocator,
    expected: ScalarAspectType,
    min: i64,
    max: i64,
) -> Result<i64, JsonCompatibilityLoweringDenial> {
    let Some(value) = value.as_i64() else {
        return Err(JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
            source: source.clone(),
            expected,
        });
    };
    if value < min || value > max {
        Err(JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
            source: source.clone(),
            expected,
        })
    } else {
        Ok(value)
    }
}

fn unsigned_integer(
    value: &Value,
    source: &BoundarySourceLocator,
    expected: ScalarAspectType,
    max: u64,
) -> Result<u64, JsonCompatibilityLoweringDenial> {
    let value = required_u64(Some(value), source, expected)?;
    if value > max {
        Err(JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
            source: source.clone(),
            expected,
        })
    } else {
        Ok(value)
    }
}
