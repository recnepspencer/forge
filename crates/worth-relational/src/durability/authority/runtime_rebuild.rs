mod checkpoint_restore;
mod envelope_replay;
mod history_parity;
mod merge_plan;
mod recovered_counter_capacity;
mod recovered_lineage_artifacts;
mod recovered_schema_basis;
mod root_inventory;

use std::collections::BTreeSet;

use crate::durability::data::{DurabilityError, DurabilityMode};
use crate::runtime::RelationalRuntime;

use checkpoint_restore::{
    clear_recovery_partition_pins, finalize_restored_runtime, restore_checkpoint_state,
};
use envelope_replay::replay_durable_envelope;
use recovered_counter_capacity::{
    refresh_checkpoint_counters, validate_tail_lineage_allocator_capacity,
};
use recovered_lineage_artifacts::{
    reconcile_recovered_lineage_artifacts, validate_recovered_lineage_artifacts,
};
use root_inventory::RecoveredRootInventory;

pub(super) fn rebuild_runtime_from_plan(
    admitted: super::recovery::admission::AdmittedRecoveryPlan,
) -> Result<RelationalRuntime, DurabilityError> {
    let plan = admitted.into_plan();
    validate_tail_lineage_allocator_capacity(&plan.tail_log)?;
    validate_recovered_lineage_artifacts(plan.checkpoint.as_ref(), &plan.tail_log)?;
    let mut restored = RelationalRuntime::new(plan.config.clone());
    let original_durability_mode = restored.config.durability.policy.mode;
    restored.config.durability.policy.mode = DurabilityMode::InMemoryCanonical;
    restored.durability.store = None;
    restored.commit_strategies.executors = plan.commit_strategy_executors.clone();

    if let Some(checkpoint) = &plan.checkpoint {
        restore_checkpoint_state(&mut restored, checkpoint)?;
    }

    refresh_checkpoint_counters(&mut restored)?;
    clear_recovery_partition_pins(&mut restored);

    let available_commit_ids = restored
        .history
        .commit_envelopes
        .keys()
        .copied()
        .chain(plan.tail_log.iter().map(|entry| entry.commit.commit_id))
        .collect::<BTreeSet<_>>();
    let mut recovered_roots = RecoveredRootInventory::capture(&restored)?;

    for envelope in &plan.tail_log {
        replay_durable_envelope(
            &mut restored,
            envelope,
            &available_commit_ids,
            &plan,
            &mut recovered_roots,
        )?;
    }
    reconcile_recovered_lineage_artifacts(&mut restored, &plan.tail_log)?;

    finalize_restored_runtime(&mut restored, original_durability_mode);
    recovered_roots.finish();
    Ok(restored)
}
