mod branch_readmission;
mod checkpoint_restore;
mod envelope_replay;
mod history_parity;
mod merge_plan;
mod recovered_counter_capacity;
mod recovered_lineage_artifacts;
mod recovered_schema_basis;
mod root_inventory;

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::durability::data::{DurabilityError, DurabilityMode};
use crate::runtime::RelationalRuntime;

use checkpoint_restore::{
    clear_recovery_partition_pins, finalize_restored_runtime, restore_checkpoint_state,
};
use envelope_replay::replay_readmitted_envelope;
use recovered_counter_capacity::{
    refresh_checkpoint_counters, validate_tail_lineage_allocator_capacity,
};
use recovered_lineage_artifacts::{
    reconcile_recovered_lineage_artifacts, validate_recovered_lineage_artifacts,
};
use root_inventory::RecoveredRootInventory;

pub(super) fn rebuild_runtime_from_plan(
    admitted: super::recovery::admission::AdmittedRecoveryPlan,
) -> Result<(RelationalRuntime, crate::durability::data::RecoveryPlan), DurabilityError> {
    let plan = admitted.into_plan();
    let tail_envelopes = plan.tail_envelopes_in_stream_order();
    validate_tail_lineage_allocator_capacity(tail_envelopes.iter().copied())?;
    validate_recovered_lineage_artifacts(plan.checkpoint.as_ref(), tail_envelopes.iter().copied())?;
    drop(tail_envelopes);
    let mut restored = RelationalRuntime::new(plan.config.clone());
    let original_durability_mode = restored.config.durability.policy.mode;
    restored.config.durability.policy.mode = DurabilityMode::InMemoryCanonical;
    restored.durability.store = None;
    restored.commit_strategies.executors = plan.commit_strategy_executors.clone();

    if let Some(checkpoint) = &plan.checkpoint {
        restore_checkpoint_state(&mut restored, checkpoint)?;
    }
    // Tail replay performs ordinary compare-and-publish cutovers. Seed its
    // fixed-depth current-head index from the exact restored branch cells
    // before the first replayed movement tries to retire a prior head.
    restored.history.rebuild_branch_head_version_index();

    refresh_checkpoint_counters(&mut restored)?;
    clear_recovery_partition_pins(&mut restored);

    let mut pending_tail = plan.tail_log.clone();
    pending_tail.sort_by_key(|commit| commit.position());
    let available_commit_ids = restored
        .history
        .commit_envelopes
        .keys()
        .copied()
        .chain(
            pending_tail
                .iter()
                .map(|commit| commit.envelope().commit.commit_id),
        )
        .collect::<BTreeSet<_>>();
    let mut recovered_roots = RecoveredRootInventory::capture(&restored)?;

    let mut finalized_tail = Vec::with_capacity(pending_tail.len());
    for pending in pending_tail {
        let commit_id = pending.envelope().commit.commit_id;
        let restore_authoritative_envelope = plan.should_restore_authoritative_envelope(commit_id);
        let positioned = replay_readmitted_envelope(
            &mut restored,
            pending,
            &available_commit_ids,
            restore_authoritative_envelope,
            &mut recovered_roots,
        )?;
        finalized_tail.push(positioned);
    }
    restored
        .record_identity
        .release_unconsumed_restored_reservations();
    let mut recovered_canonical_commits = plan
        .checkpoint
        .iter()
        .flat_map(|checkpoint| checkpoint.envelopes.iter())
        .cloned()
        .map(Arc::new)
        .collect::<Vec<_>>();
    for durable in &finalized_tail {
        let selected = if plan.should_restore_authoritative_envelope(durable.commit.commit_id) {
            Arc::new(durable.clone())
        } else {
            restored
                .history
                .positioned_canonical_commit(durable.commit.commit_id)
                .ok_or_else(|| {
                    DurabilityError::new(
                        crate::durability::data::RecoveryFailureClass::ReplayFailure,
                        "replayed commit has no positioned canonical route",
                    )
                })?
        };
        recovered_canonical_commits.push(selected);
    }
    restored
        .history
        .install_recovered_canonical_inventory(recovered_canonical_commits)
        .map_err(|detail| {
            DurabilityError::new(
                crate::durability::data::RecoveryFailureClass::CorruptCheckpoint,
                detail,
            )
        })?;
    reconcile_recovered_lineage_artifacts(&mut restored, &finalized_tail)?;
    refresh_checkpoint_counters(&mut restored)?;
    restored.durability.set_log(finalized_tail);

    finalize_restored_runtime(&mut restored, original_durability_mode)?;
    recovered_roots.finish();
    Ok((restored, plan))
}
