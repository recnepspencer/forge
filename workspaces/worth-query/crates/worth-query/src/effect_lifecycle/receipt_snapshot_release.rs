use super::batch_execution::ExecutedEffectBatchPlan;
use super::execution::{EffectExecutionAuthority, ExecutedEffectPlan};

pub(super) fn release_scalar(
    authority: &mut EffectExecutionAuthority<'_>,
    executed: &ExecutedEffectPlan,
) {
    let Some(snapshot) = executed.published_snapshot() else {
        return;
    };
    super::exact_snapshot_closeout::release_exact_execution_snapshot(
        authority
            .relational_runtime()
            .expect("Relational execution preserves its admitted authority"),
        snapshot,
    );
}

pub(super) fn release_batch(
    authority: &mut EffectExecutionAuthority<'_>,
    executed: &ExecutedEffectBatchPlan,
) {
    let Some(snapshot) = executed.published_snapshot() else {
        return;
    };
    super::exact_snapshot_closeout::release_exact_execution_snapshot(
        authority
            .relational_runtime()
            .expect("Relational batch execution preserves its admitted authority"),
        snapshot,
    );
}
