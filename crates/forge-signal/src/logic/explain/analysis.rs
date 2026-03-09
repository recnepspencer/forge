use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::EvaluationCondition;
use crate::data::output::{ChangedRegion, OutputChange, PartitionMatchMode, PartitionSubscription};
use crate::data::trace::TraceSummary;

use super::types::ConditionDecision;

pub(super) fn classify_condition_decision(
    graph: &SignalGraph,
    node: NodeId,
    condition: &EvaluationCondition,
) -> Option<ConditionDecision> {
    let entry = graph.get_entry(node).ok()?;
    let dirty_aspects = entry.get_dirty_aspects();
    let max_delta = max_dependency_delta(graph, node).ok()?;

    match condition {
        EvaluationCondition::AspectFilter(mask)
            if !dirty_aspects.is_empty() && !dirty_aspects.intersects(*mask) =>
        {
            Some(ConditionDecision::Deferred)
        }
        EvaluationCondition::OnDemand => Some(ConditionDecision::Deferred),
        EvaluationCondition::DeltaThreshold(threshold)
            if !dirty_aspects.is_empty() && (max_delta as f64) <= *threshold =>
        {
            Some(ConditionDecision::RevertedClean)
        }
        EvaluationCondition::Debounce(_) => Some(ConditionDecision::Deferred),
        EvaluationCondition::Custom(_) => Some(ConditionDecision::Deferred),
        _ => None,
    }
}

fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for (source, aspect, cached_version, _) in graph.get_dep_snapshot(node)?.entries() {
        if !graph.is_alive(*source) {
            continue;
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        max_delta = max_delta.max(current_version.abs_diff(*cached_version));
    }
    Ok(max_delta)
}

pub(super) fn partition_scope_untouched(
    trace_summary: Option<&TraceSummary>,
    scope: &PartitionSubscription,
) -> bool {
    let Some(trace_summary) = trace_summary else {
        return false;
    };
    if trace_summary.output_change == OutputChange::Unchanged {
        return true;
    }
    if trace_summary.changed_regions.is_empty() {
        return false;
    }
    !trace_summary
        .changed_regions
        .iter()
        .any(|region| partition_subscription_matches(scope, region))
}

pub(super) fn partition_subscription_matches(
    subscription: &PartitionSubscription,
    region: &ChangedRegion,
) -> bool {
    if subscription.partition != region.partition {
        return false;
    }
    match subscription.match_mode {
        PartitionMatchMode::WholePartition => true,
        PartitionMatchMode::PartitionAndDetail => subscription.detail == region.detail,
    }
}
