use serde_json::Value;

use super::JsonCompatibilityLoweringDenial;
use crate::locators::BoundarySourceLocator;
use crate::values::{AspectValue, ContentRefId, ScalarAspectType};

pub(super) fn lower_primitive_scalar(
    source: &BoundarySourceLocator,
    value: &Value,
    expected: ScalarAspectType,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    match expected {
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
        ScalarAspectType::Bytes => value
            .as_u64()
            .map(|value| AspectValue::Bytes(ContentRefId(value)))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "bytes reference id",
            }),
        ScalarAspectType::ContentRef => value
            .as_u64()
            .map(|value| AspectValue::ContentRef(ContentRefId(value)))
            .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
                source: source.clone(),
                expected: "content reference id",
            }),
        _ => Err(JsonCompatibilityLoweringDenial::UnsupportedScalarFamily {
            source: source.clone(),
            expected,
        }),
    }
}
