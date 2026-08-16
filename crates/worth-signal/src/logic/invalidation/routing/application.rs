use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::{FrontierDiagnosticsProjection, FrontierDiagnosticsSidecar, FrontierPlan};

use super::counters::record_diagnostic_projection;

pub(super) fn execute_invalidation_frontier(
    graph: &mut SignalGraph,
    plan: &FrontierPlan,
) -> Result<FrontierDiagnosticsSidecar, SignalError> {
    for seed in plan.seed_batch.as_slice() {
        super::mark_source_seed(graph, seed)?;
    }

    let counters = FrontierDiagnosticsProjection {
        frontier_seed_count: plan.seed_batch.as_slice().len() as u64,
        ..FrontierDiagnosticsProjection::default()
    };
    record_diagnostic_projection(graph, &counters);

    Ok(FrontierDiagnosticsSidecar::new(
        plan.seed_batch.as_slice().len() as u64,
        Vec::new(),
        Vec::new(),
        plan.touched_scope_summary.clone(),
        counters,
    ))
}
