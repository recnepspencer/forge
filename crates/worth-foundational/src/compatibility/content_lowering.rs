use serde_json::Value;

use super::JsonCompatibilityLoweringDenial;
use crate::locators::BoundarySourceLocator;
use crate::values::{AspectValue, ContentRefId};

pub(super) fn lower_json_content_ref(
    source: &BoundarySourceLocator,
    value: &Value,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    value
        .as_u64()
        .map(|value| AspectValue::ContentRef(ContentRefId(value)))
        .ok_or_else(|| JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "content reference id",
        })
}
