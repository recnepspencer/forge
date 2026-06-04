use forge_foundational::facade::{
    AspectKey, AspectValue, ContractValidatedAspectValueView, EntityId as FoundationalEntityId,
    Generation as FoundationalGeneration, LocalSlot as FoundationalLocalSlot,
    PartitionId as FoundationalPartitionId,
};

use crate::identity::data::EntityId;
use crate::storage::data::RelationReadRecord;

use super::lifecycle_snapshot_values::lifecycle_aspect_value;

pub(crate) fn export_relation_aspect_snapshot_value(
    record: &RelationReadRecord,
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    relation_snapshot_aspect_value(record, aspect_key)
}

fn relation_snapshot_aspect_value(
    record: &RelationReadRecord,
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    match aspect_key.as_str() {
        "source" => Some(endpoint_aspect_value(record.source)),
        "target" => Some(endpoint_aspect_value(record.target)),
        "lifecycle" => Some(lifecycle_aspect_value(record.lifecycle)),
        _ => authoritative_relation_scalar_aspect_value(record, aspect_key),
    }
}

fn authoritative_relation_scalar_aspect_value(
    record: &RelationReadRecord,
    aspect_key: &AspectKey,
) -> Option<AspectValue> {
    let authoritative_state = record.authoritative_aspect_state.as_ref()?;
    let aspect_entry = authoritative_state.get(aspect_key)?;
    match aspect_entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Some(value.clone()),
        ContractValidatedAspectValueView::Struct(_) => None,
    }
}

fn endpoint_aspect_value(entity_id: EntityId) -> AspectValue {
    AspectValue::EntityRef(FoundationalEntityId {
        partition_id: FoundationalPartitionId(entity_id.partition_id.as_u32()),
        local_slot: FoundationalLocalSlot(entity_id.local_slot_value()),
        generation: FoundationalGeneration(entity_id.generation_value()),
    })
}
