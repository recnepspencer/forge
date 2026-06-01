use serde_json::Value;

use super::JsonCompatibilityLoweringDenial;
use crate::locators::BoundarySourceLocator;
use crate::values::{
    AspectValue, CanonicalBigInt, CanonicalDecimal, CanonicalRational, ScalarAspectType,
};

pub(super) fn lower_wrapper_scalar(
    source: &BoundarySourceLocator,
    value: &Value,
    expected: ScalarAspectType,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    match expected {
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
        ScalarAspectType::Uuid => lower_uuid_array(source, value),
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
        uuid[index] = byte
            .as_u64()
            .filter(|value| *value <= u8::MAX as u64)
            .ok_or_else(|| JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
                source: source.clone(),
                expected: ScalarAspectType::Uuid,
            })? as u8;
    }
    Ok(AspectValue::Uuid(uuid))
}
