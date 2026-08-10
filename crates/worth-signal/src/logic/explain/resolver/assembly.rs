use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::policy::DiagnosticsAvailability;

use super::super::analysis::classify_condition_decision;
use super::super::types::NodeExplanation;
use super::causes::resolve_upstream_causes;
use super::lineage::{ExplanationLineage, ExplanationTraversalCost};
use super::rendering::build_causal_link_with_graph;

pub(super) struct ExplanationResolution {
    pub(super) explanation: NodeExplanation,
    pub(super) traversal_cost: ExplanationTraversalCost,
}

pub(super) fn assemble(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &impl ComparatorPolicyResolver,
) -> Result<ExplanationResolution, SignalError> {
    let entry = graph.get_entry(node)?;
    let state = *entry.get_state();
    let dirty_aspects = entry.get_dirty_aspects();
    let contract = graph.get_contract(node)?.clone();
    let condition = entry.get_eval_config().condition.clone();
    let historical_artifact_record = entry.historical_artifact_record(node);
    let trace_summary = entry.trace_summary();
    let output_identity = trace_summary
        .as_ref()
        .and_then(|trace| trace.output_identity.clone());
    let execution_record_id = trace_summary
        .as_ref()
        .and_then(|trace| trace.execution_record_id);
    let semantic_segment_id = trace_summary
        .as_ref()
        .and_then(|trace| trace.semantic_segment_id);
    let output_change = trace_summary.as_ref().map(|trace| trace.output_change);
    let changed_regions = trace_summary
        .as_ref()
        .map(|trace| trace.changed_regions.clone())
        .unwrap_or_default();
    let propagation_suppressed = trace_summary
        .as_ref()
        .map(|trace| trace.propagation_suppressed)
        .unwrap_or(false);
    let memoized_origin = trace_summary.as_ref().map(|trace| trace.memoized_origin);
    let reuse_basis = trace_summary
        .as_ref()
        .map(|trace| trace.reuse_basis.clone());
    let reuse_origin = trace_summary.as_ref().map(|trace| trace.reuse_origin);
    let reuse_certification = historical_artifact_record
        .as_ref()
        .and_then(|record| record.retained.as_ref())
        .and_then(|retained| retained.reuse_certification.clone());
    let causality = entry.get_causality().cloned();
    let comparator_override = entry.get_eval_config().comparator.clone();
    let condition_decision = classify_condition_decision(graph, node, &condition);

    let mut lineage = ExplanationLineage::collect(graph, node)?;
    let upstream = resolve_upstream_causes(
        graph,
        node,
        comparator_resolver,
        comparator_override.as_ref(),
        comparator_override.is_some(),
        &condition,
        condition_decision,
        &mut lineage,
    )?;
    let mut traversal_cost = lineage.traversal_cost().clone();
    let causal_links = upstream
        .iter()
        .map(|cause| build_causal_link_with_graph(graph, cause, &mut traversal_cost))
        .collect();

    Ok(ExplanationResolution {
        explanation: NodeExplanation {
            node,
            materialization_mode: DiagnosticsAvailability::ReconstructedAvailable,
            state,
            dirty_aspects,
            contract_reads: contract.semantics.reads,
            contract_produces: contract.semantics.produces,
            contract_partition_scope: contract.semantics.partition_scope.clone(),
            required_context: contract.semantics.required_context,
            condition,
            historical_artifact_record,
            execution_record_id,
            semantic_segment_id,
            output_identity,
            output_change,
            changed_regions,
            propagation_suppressed,
            memoized_origin,
            reuse_basis,
            reuse_origin,
            reuse_certification,
            causal_links,
            rewiring: lineage.rewiring(),
            upstream,
            causality,
        },
        traversal_cost,
    })
}
