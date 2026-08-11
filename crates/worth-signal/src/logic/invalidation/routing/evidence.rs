use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::{FrontierPlan, InvalidationTraceRecord};
use crate::diagnostics::policy::FrontierTracingPolicy;

pub(super) fn retained_trace_records(
    graph: &SignalGraph,
    plan: &FrontierPlan,
) -> Result<Vec<InvalidationTraceRecord>, SignalError> {
    let tracing_policy = graph.runtime_policy().frontier_tracing_policy;
    if matches!(tracing_policy, FrontierTracingPolicy::SummaryOnly) {
        return Ok(Vec::new());
    }

    let mut records = Vec::new();
    for wave in &plan.direct_waves {
        for entry in &wave.entries {
            records.push(InvalidationTraceRecord::new(
                entry.node,
                wave.aspect,
                wave.wave_index,
                entry.classification,
                entry.inclusion_basis,
            ));
        }
    }
    Ok(records)
}
