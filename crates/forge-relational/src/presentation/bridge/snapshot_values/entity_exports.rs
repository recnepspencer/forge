use forge_foundational::facade::{AspectKey, AspectValue, ContractValidatedAspectValueView};

use crate::storage::data::EntityReadRecord;

use super::aspect_encoding::encode_snapshot_aspect_value;
use super::lifecycle_snapshot_values::lifecycle_aspect_value;

pub(crate) fn export_entity_aspect_snapshot_bytes(
    record: &EntityReadRecord,
    aspect_label: &str,
) -> Option<Vec<u8>> {
    let value = if aspect_label == "lifecycle" {
        lifecycle_aspect_value(record.lifecycle)
    } else {
        authoritative_entity_scalar_aspect_value(record, aspect_label)?
    };
    encode_snapshot_aspect_value(&value)
}

fn authoritative_entity_scalar_aspect_value(
    record: &EntityReadRecord,
    aspect_label: &str,
) -> Option<AspectValue> {
    let aspect_key = AspectKey::new(aspect_label)?;
    let authoritative_state = record.authoritative_aspect_state.as_ref()?;
    let aspect_entry = authoritative_state.get(&aspect_key)?;
    match aspect_entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Some(value.clone()),
        ContractValidatedAspectValueView::Struct(_) => None,
    }
}
