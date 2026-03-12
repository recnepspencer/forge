use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult};
use crate::logic::evaluation::{
    EffectDependencyInputs, EvaluationEffect, EvaluationVerdict, PreparedApplyResult,
    SuppressionReason,
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
    dependency_inputs: Option<EffectDependencyInputs>,
    defer_snapshot_commit: bool,
) -> Result<PreparedApplyResult, SignalError> {
    let dependency_inputs = match dependency_inputs {
        Some(inputs) => inputs,
        None => build_effect_dependency_inputs(graph, node)?,
    };
    let effect = EvaluationEffect {
        node,
        verdict,
        aspect_version: result.aspect_version,
        output_change: result.output_change,
        output_identity: result.output_identity,
        continuity_token: result.continuity_token,
        changed_regions: result.changed_regions,
        labels: result.labels,
        dependency_snapshot: dependency_inputs.dependency_snapshot,
        meaningful_input_changes: dependency_inputs.meaningful_input_changes,
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
    let pending_snapshot = if defer_snapshot_commit {
        Some(crate::logic::evaluation::PendingDependencySnapshot {
            node,
            snapshot: effect.dependency_snapshot.clone(),
        })
    } else {
        None
    };
    let report = graph.apply_effect(effect, comparator, comparator_resolver, defer_snapshot_commit)?;
    Ok(PreparedApplyResult {
        dependency_updates: 0,
        report,
        pending_snapshot,
    })
}

pub(crate) fn collect_effect_dependency_inputs_batch(
    graph: &mut SignalGraph,
    nodes: &[NodeId],
) -> Result<Vec<EffectDependencyInputs>, SignalError> {
    nodes.iter()
        .map(|&node| build_effect_dependency_inputs(graph, node))
        .collect()
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

fn build_effect_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<EffectDependencyInputs, SignalError> {
    let mut snapshot = DependencySnapshot::empty();
    let dependencies = graph.runtime_dependencies_of(node)?.to_vec();
    let snapshot_entries = graph.get_dep_snapshot(node)?.entries();
    let mut snapshot_index = 0usize;
    let mut changes = 0_u32;

    for dep in dependencies {
        let source = dep.source();
        let aspect = dep.aspect();
        if graph.is_alive(source) {
            let entry = graph.get_entry(source)?;
            let ver = entry.version_for_scope(aspect, dep.scope_ref());
            snapshot.record(source, aspect, ver, dep.scope_ref().cloned());
            while snapshot_index < snapshot_entries.len()
                && snapshot_entries[snapshot_index].sort_key() < dep.sort_key()
            {
                snapshot_index += 1;
            }
            if snapshot_index < snapshot_entries.len()
                && snapshot_entries[snapshot_index].sort_key() == dep.sort_key()
            {
                if snapshot_entries[snapshot_index].cached_version != ver {
                    changes += 1;
                }
                snapshot_index += 1;
            }
        } else {
            changes += 1;
        }
    }

    Ok(EffectDependencyInputs {
        dependency_snapshot: snapshot,
        meaningful_input_changes: changes,
    })
}
