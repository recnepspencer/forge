use worth_foundational::facade::{AspectKey, AspectValue, ContractValidatedAspectValueView};

use crate::storage::data::EntityReadRecord;

use super::lifecycle_snapshot_values::lifecycle_aspect_value;

pub(crate) fn export_entity_aspect_snapshot_value(
    record: &EntityReadRecord,
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    Some(if aspect_key.as_str() == "lifecycle" {
        lifecycle_aspect_value(record.lifecycle)
    } else {
        authoritative_entity_scalar_aspect_value(record, aspect_key)?
    })
}

fn authoritative_entity_scalar_aspect_value(
    record: &EntityReadRecord,
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    let authoritative_state = record.authoritative_aspect_state.as_ref()?;
    let aspect_entry = authoritative_state.get(aspect_key)?;
    match aspect_entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Some(value.clone()),
        ContractValidatedAspectValueView::Struct(_) => None,
    }
}
