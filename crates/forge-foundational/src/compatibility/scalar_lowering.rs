use serde_json::Value;

use super::JsonCompatibilityLoweringDenial;
use crate::locators::BoundarySourceLocator;
use crate::values::{
    AspectValue, CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32, CanonicalF64,
    CanonicalRational, CanonicalTime, CanonicalTimestamp, ScalarAspectType,
};

pub(super) fn lower_json_scalar(
    source: &BoundarySourceLocator,
    value: &Value,
    expected: ScalarAspectType,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    match expected {
        ScalarAspectType::Uuid => lower_uuid_array(source, value),
        _ if value.is_array() || value.is_object() => Err(
            JsonCompatibilityLoweringDenial::UnsupportedRecursiveDocument {
                source: source.clone(),
                expected,
            },
        ),
        ScalarAspectType::Null if value.is_null() => Ok(AspectValue::Null),
        ScalarAspectType::Bool => value.as_bool().map(AspectValue::Bool).ok_or_else(|| {
            JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "boolean",
            }
        }),
        ScalarAspectType::String => value
            .as_str()
            .map(|value| AspectValue::String(value.into()))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "string",
            }),
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
        ScalarAspectType::Decimal => value
            .as_str()
            .map(|value| AspectValue::Decimal(CanonicalDecimal::new(value)))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "decimal string",
            }),
        ScalarAspectType::BigInt => value
            .as_str()
            .map(|value| AspectValue::BigInt(CanonicalBigInt::new(value)))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "big-int string",
            }),
        ScalarAspectType::Rational => lower_rational_string(source, value),
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
        ScalarAspectType::Bytes | ScalarAspectType::ContentRef => {
            unsigned_integer(value, source, expected, u64::MAX).map(|value| match expected {
                ScalarAspectType::Bytes => AspectValue::Bytes(crate::values::ContentRefId(value)),
                ScalarAspectType::ContentRef => {
                    AspectValue::ContentRef(crate::values::ContentRefId(value))
                }
                _ => unreachable!("matched above"),
            })
        }
        _ => Err(JsonCompatibilityLoweringDenial::UnsupportedScalarFamily {
            source: source.clone(),
            expected,
        }),
    }
}

fn lower_rational_string(
    source: &BoundarySourceLocator,
    value: &Value,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    let Some(value) = value.as_str() else {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "rational string",
        });
    };
    let Some((numerator, denominator)) = value.split_once('/') else {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "rational string",
        });
    };

    CanonicalRational::new(
        CanonicalBigInt::new(numerator),
        CanonicalBigInt::new(denominator),
    )
    .map(AspectValue::Rational)
    .ok_or_else(|| JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
        source: source.clone(),
        expected: ScalarAspectType::Rational,
    })
}

fn lower_uuid_array(
    source: &BoundarySourceLocator,
    value: &Value,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    let Value::Array(bytes) = value else {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "16-byte UUID array",
        });
    };
    if bytes.len() != 16 {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "16-byte UUID array",
        });
    }

    let mut uuid = [0_u8; 16];
    for (index, byte) in bytes.iter().enumerate() {
        uuid[index] = unsigned_integer(byte, source, ScalarAspectType::Uuid, u8::MAX as u64)? as u8;
    }
    Ok(AspectValue::Uuid(uuid))
}

pub(super) fn required_u64(
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
