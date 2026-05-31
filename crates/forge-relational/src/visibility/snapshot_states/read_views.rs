use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RelationalReadView;
use crate::storage::logic::state::SnapshotState;

pub(crate) fn read_view_from_snapshot_state(
    runtime: &RelationalRuntime,
    state: &SnapshotState,
) -> RelationalReadView {
    let current_state = runtime.storage_access().current_state();
    let reader = runtime.read_truth();
    let mut entities = Vec::with_capacity(state.pinned_entity_count);
    let mut relations = Vec::with_capacity(state.pinned_relation_count);
    for (partition_id, pins) in &state.pinned_partitions {
        for slot in pins.entity_slots.iter_set_slots() {
            let entity_id = crate::identity::data::EntityId::new(*partition_id, slot as u64, 0);
            if let Some(record) = reader.unmasked_entity_record_for_id_at_version(
                &current_state,
                entity_id,
                state.handle.version_id,
            ) {
                entities.push(record);
            }
        }
        for slot in pins.relation_slots.iter_set_slots() {
            let relation_id = crate::identity::data::RelationId::new(*partition_id, slot as u64, 0);
            if let Some(record) = reader.unmasked_relation_record_for_id_at_version(
                &current_state,
                relation_id,
                state.handle.version_id,
            ) {
                relations.push(if pins.retained_relation_slots.count_ones_in_range(slot, slot + 1)
                    == 1
                {
                    crate::storage::data::RelationReadRecord {
                        lifecycle: crate::storage::data::RecordLifecycleState::RetainedDanglingForAudit,
                        ..record
                    }
                } else {
                    record
                });
            }
        }
    }
    runtime.services.instrumentation.count(|counters| {
        counters.visible_unmasked_entity_records_materialized += entities.len();
        counters.visible_unmasked_relation_records_materialized += relations.len();
    });
    RelationalReadView {
        snapshot: state.handle.clone(),
        entities,
        relations,
    }
}
