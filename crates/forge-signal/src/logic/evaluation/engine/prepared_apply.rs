use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult};
use crate::logic::evaluation::{DeferralReason, PreparedApplyResult, SuppressionReason};
use crate::logic::evaluation::EffectDependencyInputs;
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluation, PreparedEvaluationOrigin,
    PreparedEvaluationOutcome,
};

use super::apply::{
    apply_effect_with_policy_and_condition, verdict_for_evaluated_result,
};
use super::metadata::EvaluationExecutionMetadata;

#[cfg(any(test, feature = "parallel"))]
pub(crate) fn apply_prepared_evaluation_with_policy(
    graph: &mut SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<PreparedApplyResult, SignalError> {
    let dependency_updates = apply_prepared_dependencies(graph, node, &prepared.dependencies)?;
    apply_prepared_evaluation_after_dependencies_with_policy(
        graph,
        node,
        prepared,
        comparator_resolver,
        execution_metadata,
        dependency_updates,
        None,
        false,
    )
}

pub(crate) fn apply_prepared_dependency_batch(
    graph: &mut SignalGraph,
    captures: &[(NodeId, &PreparedDependencyCapture)],
) -> Result<Vec<u32>, SignalError> {
    let desired = captures
        .iter()
        .map(|(node, capture)| (*node, build_prepared_dependency_edges(graph, capture)))
        .collect::<Vec<_>>();
    let reports = graph.reconcile_dependencies_batch(&desired)?;
    Ok(reports.into_iter().map(|report| report.update_count()).collect())
}

pub(crate) fn apply_prepared_evaluation_after_dependencies_with_policy(
    graph: &mut SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    dependency_updates: u32,
    dependency_inputs: Option<EffectDependencyInputs>,
    defer_snapshot_commit: bool,
) -> Result<PreparedApplyResult, SignalError> {
    match prepared.outcome {
        PreparedEvaluationOutcome::Evaluate => {
            let mut result = prepared.result;
            result.labels.extend(prepared.trace_data.labels);
            let metadata = match (execution_metadata, prepared.origin) {
                (Some(metadata), _) => Some(metadata),
                (None, PreparedEvaluationOrigin::MemoizedReuse) => {
                    let synthesized = EvaluationExecutionMetadata {
                        keyed: None,
                        memoized_origin: MemoizedResultOrigin::MemoizedFromCache,
                    };
                    return apply_prepared_with_synthesized_metadata(
                        graph,
                        node,
                        result,
                        comparator_resolver,
                        synthesized,
                        dependency_updates,
                        prepared.keyed,
                        prepared.trace_data.causality,
                    );
                }
                _ => None,
            };
            let recomputed = !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse);
            let verdict = verdict_for_evaluated_result(graph, node, &result, recomputed)?;
            let mut apply_result = apply_effect_with_policy_and_condition(
                graph,
                node,
                result,
                comparator_resolver,
                metadata,
                verdict,
                recomputed,
                prepared.keyed,
                prepared.trace_data.causality,
                dependency_inputs,
                defer_snapshot_commit,
            )?;
            apply_result.dependency_updates = dependency_updates;
            Ok(apply_result)
        }
        PreparedEvaluationOutcome::ValidatedClean => {
            let current_version = graph.get_entry(node)?.get_aspect_version();
            let mut apply_result = apply_effect_with_policy_and_condition(
                graph,
                node,
                NodeEvaluationResult::from_version(current_version),
                comparator_resolver,
                execution_metadata,
                crate::logic::evaluation::EvaluationVerdict::Suppressed {
                    reason: SuppressionReason::ValidatedClean,
                },
                false,
                prepared.keyed,
                prepared.trace_data.causality,
                dependency_inputs,
                defer_snapshot_commit,
            )?;
            apply_result.dependency_updates = dependency_updates;
            Ok(apply_result)
        }
        PreparedEvaluationOutcome::DeferredByCondition => {
            let current_version = graph.get_entry(node)?.get_aspect_version();
            let mut apply_result = apply_effect_with_policy_and_condition(
                graph,
                node,
                NodeEvaluationResult::from_version(current_version),
                comparator_resolver,
                execution_metadata,
                crate::logic::evaluation::EvaluationVerdict::Deferred {
                    reason: DeferralReason::ConditionNotMet,
                },
                false,
                prepared.keyed,
                prepared.trace_data.causality,
                dependency_inputs,
                defer_snapshot_commit,
            )?;
            apply_result.dependency_updates = dependency_updates;
            Ok(apply_result)
        }
        PreparedEvaluationOutcome::RevertedCleanByCondition => {
            let current_version = graph.get_entry(node)?.get_aspect_version();
            let mut apply_result = apply_effect_with_policy_and_condition(
                graph,
                node,
                NodeEvaluationResult::from_version(current_version),
                comparator_resolver,
                execution_metadata,
                crate::logic::evaluation::EvaluationVerdict::Suppressed {
                    reason: SuppressionReason::ConditionRevertedClean,
                },
                false,
                prepared.keyed,
                prepared.trace_data.causality,
                dependency_inputs,
                defer_snapshot_commit,
            )?;
            apply_result.dependency_updates = dependency_updates;
            Ok(apply_result)
        }
    }
}

fn apply_prepared_with_synthesized_metadata(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    metadata: EvaluationExecutionMetadata,
    dependency_updates: u32,
    keyed_context: Option<crate::logic::prepared::PreparedKeyedContext>,
    causality: Option<crate::data::trace::CausalityMetadata>,
) -> Result<PreparedApplyResult, SignalError> {
    let verdict = verdict_for_evaluated_result(graph, node, &result, false)?;
    let mut apply_result = apply_effect_with_policy_and_condition(
        graph,
        node,
        result,
        comparator_resolver,
        Some(&metadata),
        verdict,
        false,
        keyed_context,
        causality,
        None,
        false,
    )?;
    apply_result.dependency_updates = dependency_updates;
    Ok(apply_result)
}

#[cfg(any(test, feature = "parallel"))]
fn apply_prepared_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
    capture: &PreparedDependencyCapture,
) -> Result<u32, SignalError> {
    let desired = build_prepared_dependency_edges(graph, capture);
    let report = graph.reconcile_dependencies(node, &desired)?;
    Ok(report.update_count())
}

fn build_prepared_dependency_edges(
    graph: &mut SignalGraph,
    capture: &PreparedDependencyCapture,
) -> Vec<DependencyEdge> {
    capture
        .as_slice()
        .iter()
        .map(|dependency| {
            graph.build_dependency_edge(
                dependency.source,
                dependency.aspect,
                dependency.scope.clone(),
            )
        })
        .collect()
}
