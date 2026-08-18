use std::sync::Arc;

use crate::durability::data::{DurabilityError, DurabilityMode, DurableCheckpoint};
use crate::history::data::VersionNode;
use crate::runtime::RelationalRuntime;

use super::super::super::derived_index_artifacts::restore_checkpoint_derived_index_artifacts;
use super::super::authority_continuity::validate_checkpoint_lineage_artifact;

pub(super) fn restore_checkpoint_state(
    restored: &mut RelationalRuntime,
    checkpoint: &DurableCheckpoint,
) -> Result<(), DurabilityError> {
    validate_checkpoint_lineage_artifact(checkpoint)?;
    let aspect_contracts =
        crate::durability::checkpoints::aspect_state_images::CheckpointAspectContractCatalog::readmit(
            &checkpoint.aspect_contracts,
        )?;
    restored.partitions = checkpoint
        .partition_images
        .iter()
        .cloned()
        .map(|image| {
            let partition_id = image.partition_id;
            crate::durability::checkpoints::images::partition_from_image(
                image,
                &restored.schema_contract_runtime.aspect_contract_plans,
                &aspect_contracts,
            )
            .map(|partition| (partition_id, partition))
        })
        .collect::<Result<_, _>>()?;
    crate::storage::partition::rebuild_adjacency_kind_buckets(&mut restored.partitions).map_err(
        |detail| {
            DurabilityError::new(
                crate::durability::data::RecoveryFailureClass::CorruptCheckpoint,
                detail,
            )
        },
    )?;
    restored.history.commit_envelopes = checkpoint
        .envelopes
        .iter()
        .cloned()
        .map(|envelope| (envelope.commit.commit_id, Arc::new(envelope)))
        .collect();
    restored.history.patch_stream_index = checkpoint
        .envelopes
        .iter()
        .map(|envelope| (envelope.patch.position, envelope.commit.commit_id))
        .collect();
    restored.history.commit_graph = checkpoint
        .envelopes
        .iter()
        .cloned()
        .map(|envelope| {
            (
                envelope.commit.commit_id,
                VersionNode {
                    commit: envelope.commit,
                },
            )
        })
        .collect();
    restored.history.rebuild_phase4_registry();
    restored
        .history
        .restore_branch_cells(&checkpoint.branch_cells)
        .map_err(|detail| {
            DurabilityError::new(
                crate::durability::data::RecoveryFailureClass::CorruptCheckpoint,
                detail,
            )
        })?;
    restored.lineage.nodes = checkpoint
        .lineage
        .nodes()
        .iter()
        .cloned()
        .map(|node| (node.lineage_id, node))
        .collect();
    restored.lineage.events = checkpoint
        .envelopes
        .iter()
        .flat_map(|envelope| envelope.lineage_events().iter().cloned())
        .collect();
    restored.lineage.events.sort_by_key(|event| event.event_id);
    restored.lineage.rebuild_branch_event_positions();
    restored.lineage.correspondence_candidates =
        checkpoint.lineage.correspondence_candidates().to_vec();
    restored.lineage.rejected_decisions = checkpoint.lineage.rejected_decisions().to_vec();
    restored.indexes.definitions = checkpoint
        .index_definitions
        .iter()
        .cloned()
        .map(|definition| (definition.index_id, definition))
        .collect();
    restore_checkpoint_derived_index_artifacts(restored, &checkpoint.derived_index_artifacts);
    restored
        .services
        .symbols
        .restore_snapshot(checkpoint.symbol_table.clone());
    restored.durability.push_checkpoint(checkpoint.clone());
    Ok(())
}

pub(super) fn refresh_recovered_history_counters(restored: &mut RelationalRuntime) {
    restored.history.next_commit_id = restored
        .history
        .commit_envelopes
        .keys()
        .map(|id| id.0)
        .max()
        .unwrap_or(0)
        + 1;
    restored.history.next_version_id = restored
        .history
        .commit_envelopes
        .values()
        .map(|envelope| envelope.commit.version_id.0)
        .max()
        .unwrap_or(0)
        + 1;
}

pub(super) fn clear_recovery_partition_pins(restored: &mut RelationalRuntime) {
    for partition in restored.partitions.values_mut() {
        partition.entity_arena.clear_all_pins();
        partition.relation_arena.clear_all_pins();
    }
}

pub(super) fn finalize_restored_runtime(
    restored: &mut RelationalRuntime,
    original_durability_mode: DurabilityMode,
) {
    restored.lineage.next_event_id = restored
        .lineage
        .events
        .iter()
        .map(|event| event.event_id)
        .max()
        .unwrap_or(0)
        + 1;
    restored.lineage.next_candidate_id = restored
        .lineage
        .correspondence_candidates
        .iter()
        .map(|candidate| candidate.candidate_id.0)
        .max()
        .unwrap_or(0)
        + 1;
    restored.config.durability.policy.mode = original_durability_mode;
    restored
        .index_authority()
        .rebuild_unique_entity_aspect_field_indexes();
    restored.visibility_pins().rebuild_branch_pins_from_heads();
    restored.visibility.cache.clear();
    restored
        .visibility_pins()
        .rebuild_branch_head_visibility_residency();
}
