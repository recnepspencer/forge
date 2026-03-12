use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult};
use crate::logic::evaluation::{
    EvaluationEffect, EvaluationVerdict, PreparedApplyResult, SuppressionReason,
};
use crate::logic::prepared::PreparedKeyedContext;

use super::metadata::EvaluationExecutionMetadata;

pub(crate) fn apply_effect_with_policy_and_condition(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    verdict: EvaluationVerdict,
    recomputed: bool,
    keyed_context: Option<PreparedKeyedContext>,
    causality: Option<crate::data::trace::CausalityMetadata>,
) -> Result<PreparedApplyResult, SignalError> {
    let meaningful_input_changes = count_meaningful_input_changes(graph, node)?;
    let effect = EvaluationEffect {
        node,
        verdict,
        aspect_version: result.aspect_version,
        output_change: result.output_change,
        output_identity: result.output_identity,
        continuity_token: result.continuity_token,
        changed_regions: result.changed_regions,
        labels: result.labels,
        dependency_snapshot: build_dep_snapshot(graph, node)?,
        meaningful_input_changes,
        recomputed,
        memoized_origin: execution_metadata
            .map(|metadata| metadata.memoized_origin)
            .unwrap_or(MemoizedResultOrigin::DirectCompute),
        keyed_context,
        causality,
    };
    let comparator = {
        let entry = graph.get_entry(node)?;
        comparator_resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref())
    };
    let report = graph.apply_effect(effect, comparator, comparator_resolver)?;
    Ok(PreparedApplyResult {
        dependency_updates: 0,
        report,
    })
}

pub(crate) fn verdict_for_evaluated_result(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    recomputed: bool,
) -> Result<EvaluationVerdict, SignalError> {
    let previous_trace = graph.get_entry(node)?.get_trace_summary();
    let previous_output_identity = previous_trace.and_then(|trace| trace.output_identity.as_ref());
    let previous_continuity_token = previous_trace.and_then(|trace| trace.continuity_token.as_ref());

    let output_identity_unchanged = matches!(
        (previous_output_identity, result.output_identity.as_ref()),
        (Some(previous), Some(current)) if previous == current
    );
    let continuity_token_unchanged = matches!(
        (previous_continuity_token, result.continuity_token.as_ref()),
        (Some(previous), Some(current)) if previous == current
    );

    let verdict = if output_identity_unchanged {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::OutputIdentityUnchanged,
        }
    } else if continuity_token_unchanged {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::ContinuityTokenUnchanged,
        }
    } else if recomputed {
        EvaluationVerdict::Recomputed
    } else {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::ComparatorMatch,
        }
    };

    Ok(verdict)
}

fn build_dep_snapshot(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<DependencySnapshot, SignalError> {
    let mut snapshot = DependencySnapshot::empty();
    for dep in graph.runtime_dependencies_of(node)?.to_vec() {
        let source = dep.source();
        let aspect = dep.aspect();
        if graph.is_alive(source) {
            let entry = graph.get_entry(source)?;
            let ver = entry.version_for_scope(aspect, dep.scope_ref());
            snapshot.record(source, aspect, ver, dep.scope_ref().cloned());
        }
    }
    Ok(snapshot)
}

fn count_meaningful_input_changes(graph: &mut SignalGraph, node: NodeId) -> Result<u32, SignalError> {
    let dependencies = graph.runtime_dependencies_of(node)?.to_vec();
    let snapshot_entries = graph.get_dep_snapshot(node)?.entries();
    let mut dep_index = 0usize;
    let mut snapshot_index = 0usize;
    let mut changes = 0_u32;
    while dep_index < dependencies.len() && snapshot_index < snapshot_entries.len() {
        let dependency = &dependencies[dep_index];
        let snapshot = &snapshot_entries[snapshot_index];
        match dependency.sort_key().cmp(&snapshot.sort_key()) {
            std::cmp::Ordering::Less => dep_index += 1,
            std::cmp::Ordering::Greater => snapshot_index += 1,
            std::cmp::Ordering::Equal => {
                let cached = snapshot.cached_version;
                if !graph.is_alive(dependency.source()) {
                    changes += 1;
                } else {
                    let current = graph.get_entry(dependency.source())?.version_for_scope(
                        dependency.aspect(),
                        dependency.scope_ref(),
                    );
                    if current != cached {
                        changes += 1;
                    }
                }
                dep_index += 1;
                snapshot_index += 1;
            }
        }
    }
    Ok(changes)
}
