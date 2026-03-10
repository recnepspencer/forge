use crate::facade::{
    CompactionOutcome, DurableCheckpoint, DurabilityError, RecoveryPlan, RelationalRuntime,
};
use crate::logic::runtime::RecoveryOutcome;

use super::super::fixture::FintechWorld;

pub(crate) fn checkpoint_world(
    world: &mut FintechWorld,
) -> Result<DurableCheckpoint, DurabilityError> {
    world.runtime.checkpoint()
}

pub(crate) fn recover_persisted_world(
    world: &FintechWorld,
) -> Result<(RelationalRuntime, RecoveryOutcome), String> {
    recover_runtime_from_plan(world.runtime.recovery_plan())
}

pub(crate) fn recover_runtime_from_plan(
    plan: RecoveryPlan,
) -> Result<(RelationalRuntime, RecoveryOutcome), String> {
    let mut recovered = RelationalRuntime::new(plan.config.clone());
    let outcome = recovered
        .recover(plan)
        .map_err(|error| format!("failed to recover persisted fintech world: {error:?}"))?;
    Ok((recovered, outcome))
}

pub(crate) fn compact_world_store(
    world: &mut FintechWorld,
) -> Result<CompactionOutcome, DurabilityError> {
    world.runtime.compact_store()
}
