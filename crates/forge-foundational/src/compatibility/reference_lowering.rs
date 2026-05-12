use serde_json::Value;

use super::scalar_lowering::required_u64;
use super::JsonCompatibilityLoweringDenial;
use crate::locators::BoundarySourceLocator;
use crate::values::{AspectValue, ContentRefId, EntityId, PartitionId, ScalarAspectType};

pub(super) fn lower_json_entity_ref(
    source: &BoundarySourceLocator,
    value: &Value,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    let Value::Object(object) = value else {
        return Err(JsonCompatibilityLoweringDenial::JsonShapeNotAdmitted {
            source: source.clone(),
            expected: "entity reference object",
        });
    };

    let partition = required_u64(
        object.get("partition_id"),
        source,
        ScalarAspectType::EntityRef,
    )?;
    let local_slot = required_u64(
        object.get("local_slot"),
        source,
        ScalarAspectType::EntityRef,
    )?;
    let generation = required_u64(
        object.get("generation"),
        source,
        ScalarAspectType::EntityRef,
    )?;

    if partition > u32::MAX as u64 || generation > u32::MAX as u64 {
        return Err(JsonCompatibilityLoweringDenial::AmbiguousNumericWidth {
            source: source.clone(),
            expected: ScalarAspectType::EntityRef,
        });
    }

    Ok(AspectValue::EntityRef(EntityId::new(
        PartitionId(partition as u32),
        local_slot,
        generation as u32,
    )))
}

pub(super) fn lower_json_content_ref(
    source: &BoundarySourceLocator,
    value: &Value,
) -> Result<AspectValue, JsonCompatibilityLoweringDenial> {
    required_u64(Some(value), source, ScalarAspectType::ContentRef)
        .map(|value| AspectValue::ContentRef(ContentRefId(value)))
}
