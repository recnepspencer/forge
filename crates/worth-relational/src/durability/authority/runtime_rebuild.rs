mod checkpoint_restore;
mod envelope_replay;
mod history_parity;
mod merge_plan;

use std::collections::BTreeSet;

use crate::durability::data::{DurabilityError, DurabilityMode};
use crate::runtime::RelationalRuntime;

use checkpoint_restore::{
    clear_recovery_partition_pins, finalize_restored_runtime, refresh_recovered_history_counters,
    restore_checkpoint_state,
};
use envelope_replay::replay_durable_envelope;

pub(super) fn rebuild_runtime_from_plan(
    admitted: super::recovery::admission::AdmittedRecoveryPlan,
) -> Result<RelationalRuntime, DurabilityError> {
    let plan = admitted.into_plan();
    let mut restored = RelationalRuntime::new(plan.config.clone());
    let original_durability_mode = restored.config.durability.policy.mode;
    restored.config.durability.policy.mode = DurabilityMode::InMemoryCanonical;
    restored.durability.store = None;
    restored.commit_strategies.executors = plan.commit_strategy_executors.clone();

    if let Some(checkpoint) = &plan.checkpoint {
        restore_checkpoint_state(&mut restored, checkpoint)?;
    }

    refresh_recovered_history_counters(&mut restored);
    clear_recovery_partition_pins(&mut restored);

    let available_commit_ids = restored
        .history
        .commit_envelopes
        .keys()
        .copied()
        .chain(plan.tail_log.iter().map(|entry| entry.commit.commit_id))
        .collect::<BTreeSet<_>>();

    for envelope in &plan.tail_log {
        replay_durable_envelope(&mut restored, envelope, &available_commit_ids, &plan)?;
    }

    finalize_restored_runtime(&mut restored, original_durability_mode);
    Ok(restored)
}
