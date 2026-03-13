use crate::data::comparator::ComparatorPolicyResolver;
#[cfg(test)]
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::NodeEvaluationResult;
use crate::logic::evaluation::EffectDependencyInputs;
use crate::logic::evaluation::{DeferralReason, PreparedApplyResult, SuppressionReason};
#[cfg(test)]
use crate::logic::prepared::PreparedDependencyCapture;
use crate::logic::prepared::{PreparedEvaluation, PreparedEvaluationOutcome};

use super::apply::{apply_effect_with_policy_and_condition, verdict_for_evaluated_result};
use super::metadata::EvaluationExecutionMetadata;

#[cfg(test)]
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
            let reuse_decision = crate::logic::evaluation::resolve_prepared_reuse_decision(
                prepared.origin,
                execution_metadata,
            );
            let reuse_contract = graph
                .get_entry(node)?
                .get_eval_config()
                .contract
                .reuse
                .clone();
            let current_reuse_boundary_context =
                crate::logic::evaluation::resolve_reuse_boundary_context(
                    graph,
                    node,
                    comparator_resolver,
                )?;
            let previous_reuse_boundary_context = graph
                .get_entry(node)?
                .get_runtime_artifact_state()
                .and_then(|trace| trace.reuse_boundary_context.clone());
            let reuse_certification = crate::logic::evaluation::certify_reuse_decision(
                &reuse_contract,
                reuse_decision,
                &crate::data::reuse::ReuseBoundaryEvidence {
                    current: current_reuse_boundary_context.clone(),
                    previous: previous_reuse_boundary_context,
                },
            )
            .map_err(|failure| {
                SignalError::invalid_input(format!(
                    "reuse certification failed for {node}: {:?}",
                    failure.failure
                ))
            })?;
            let synthesized_metadata = EvaluationExecutionMetadata {
                keyed: None,
                memoized_origin: reuse_decision.memoized_origin,
                reuse_basis: reuse_decision.basis,
            };
            let metadata = execution_metadata.unwrap_or(&synthesized_metadata);
            let verdict =
                verdict_for_evaluated_result(graph, node, &result, reuse_decision.recomputed)?;
            let mut apply_result = apply_effect_with_policy_and_condition(
                graph,
                node,
                result,
                comparator_resolver,
                Some(metadata),
                verdict,
                reuse_decision.recomputed,
                current_reuse_boundary_context,
                prepared.keyed,
                prepared.trace_data.causality,
                reuse_certification,
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
                crate::logic::evaluation::resolve_reuse_boundary_context(
                    graph,
                    node,
                    comparator_resolver,
                )?,
                prepared.keyed,
                prepared.trace_data.causality,
                None,
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
                crate::logic::evaluation::resolve_reuse_boundary_context(
                    graph,
                    node,
                    comparator_resolver,
                )?,
                prepared.keyed,
                prepared.trace_data.causality,
                None,
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
                crate::logic::evaluation::resolve_reuse_boundary_context(
                    graph,
                    node,
                    comparator_resolver,
                )?,
                prepared.keyed,
                prepared.trace_data.causality,
                None,
                dependency_inputs,
                defer_snapshot_commit,
            )?;
            apply_result.dependency_updates = dependency_updates;
            Ok(apply_result)
        }
    }
}

#[cfg(test)]
fn apply_prepared_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
    capture: &PreparedDependencyCapture,
) -> Result<u32, SignalError> {
    let desired = build_prepared_dependency_edges(graph, capture);
    let report = graph.reconcile_dependencies(node, &desired)?;
    Ok(report.added + report.removed)
}

#[cfg(test)]
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
