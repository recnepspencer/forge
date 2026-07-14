use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};
use crate::failure::StoreError;
use crate::live_query::{ContinuationBatchResult, CursorContinuationPlan};
use crate::tiering::InterleavedContinuationParityReport;

use super::shared::{observation_for_artifacts, record_interleaving_observation};

pub(crate) fn observe_continuation_interleaving<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    plan: &CursorContinuationPlan,
    result: Option<&ContinuationBatchResult>,
) -> Result<InterleavedContinuationParityReport, StoreError> {
    let observation = observation_for_artifacts(
        backend.state(),
        vec![
            format!(
                "authoritative_branch_head:{}",
                plan.stable_basis().branch_id().0
            ),
            format!(
                "stable_basis:{}",
                plan.stable_basis().stable_basis_id().as_str()
            ),
        ],
    );
    let strategy = result.map_or(plan.strategy(), ContinuationBatchResult::resolved_strategy);
    let report = InterleavedContinuationParityReport::new(
        observation,
        strategy,
        plan.foreground_isolation().clone(),
        true,
    );
    record_interleaving_observation(backend.counters(), report.observation(), true, true);
    Ok(report)
}
