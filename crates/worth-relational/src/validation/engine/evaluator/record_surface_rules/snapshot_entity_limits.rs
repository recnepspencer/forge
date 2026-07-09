use crate::diagnostics::data::DiagnosticCode;
use crate::validation::data::{
    InvariantClass, InvariantViolation, InvariantViolationFields, StorageInconsistencyScan,
};

use super::super::super::context::InvariantExecutionContext;
use super::super::common::{storage_inconsistency_violation, StorageInconsistencyContext};

pub(super) fn evaluate_max_snapshot_entities(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    limit: usize,
) -> Option<InvariantViolation> {
    let state_view = context.state_view();
    let mut visible_entities = 0;
    if state_view.version_id() == context.current_version_id() {
        for partition_id in state_view.state().partition_ids() {
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!(
                        "partition {:?} missing during snapshot entity count",
                        partition_id
                    ),
                    StorageInconsistencyContext::default()
                        .with_partition_id(partition_id)
                        .with_scan(StorageInconsistencyScan::MaxSnapshotEntities),
                ));
            };
            visible_entities += partition.entity_arena.live_bitset.count_ones();
        }
    } else {
        for partition_id in state_view.state().partition_ids() {
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!(
                        "partition {:?} missing during historical entity scan",
                        partition_id
                    ),
                    StorageInconsistencyContext::default()
                        .with_partition_id(partition_id)
                        .with_scan(StorageInconsistencyScan::HistoricalMaxSnapshotEntities),
                ));
            };
            context
                .metrics()
                .count_entity_slot_scans(partition.entity_arena.slot_count());
            visible_entities += (0..partition.entity_arena.slot_count())
                .filter(|slot| state_view.entity_visible_at_version(&partition.entity_arena, *slot))
                .count();
        }
    }
    if visible_entities > limit {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::InvariantViolation,
            detail: format!(
                "snapshot at version {} has {} entities, limit is {}",
                state_view.version_id().as_u64(),
                visible_entities,
                limit
            ),
            fields: InvariantViolationFields::SnapshotEntityLimit {
                version_id: state_view.version_id(),
                visible_entities,
                limit,
            },
        });
    }
    None
}
