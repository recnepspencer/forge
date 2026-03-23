use crate::data::aspect::Aspect;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::comparator::VersionComparatorPolicy;
#[cfg(test)]
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
#[cfg(feature = "parallel")]
use crate::data::graph::ApplyCommitPacket;
use crate::data::handle::NodeId;
use crate::data::output::NodeEvaluationResult;
#[cfg(feature = "parallel")]
use crate::data::reuse::ReuseBoundaryFailure;
use crate::data::reuse::ReuseBoundaryEvidence;
use crate::data::dependency::DependencySnapshotId;
use crate::logic::evaluation::EffectDependencyInputs;
use crate::logic::evaluation::{DeferralReason, PreparedApplyResult, SuppressionReason};
#[cfg(test)]
use crate::logic::prepared::PreparedDependencyCapture;
use crate::logic::prepared::{PreparedEvaluation, PreparedEvaluationOutcome};

use super::apply::{apply_effect_with_policy_and_condition, verdict_for_evaluated_result};
#[cfg(feature = "parallel")]
use super::apply::build_evaluation_effect;
use super::metadata::EvaluationExecutionMetadata;

#[cfg(feature = "parallel")]
#[derive(Debug)]
pub(crate) enum ApplyCommitBuildError {
    Signal(SignalError),
    ReuseBoundary {
        error: SignalError,
        failure: ReuseBoundaryFailure,
    },
}

#[cfg(feature = "parallel")]
impl ApplyCommitBuildError {
    pub(crate) fn into_signal(self) -> SignalError {
        match self {
            Self::Signal(error) => error,
            Self::ReuseBoundary { error, .. } => error,
        }
    }

    pub(crate) fn reuse_failure(&self) -> Option<ReuseBoundaryFailure> {
        match self {
            Self::Signal(_) => None,
            Self::ReuseBoundary { failure, .. } => Some(*failure),
        }
    }
}

#[cfg(feature = "parallel")]
impl From<SignalError> for ApplyCommitBuildError {
    fn from(error: SignalError) -> Self {
        Self::Signal(error)
    }
}

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
                    Some(&result),
                    prepared.keyed.as_ref(),
                )?;
            let previous_reuse_boundary_context = graph
                .get_entry(node)?
                .get_runtime_artifact_state()
                .and_then(|trace| trace.reuse_boundary_context.clone());
            let current_reuse_boundary_context = hydrate_reuse_topology_boundary_from_previous(
                graph,
                node,
                current_reuse_boundary_context,
                previous_reuse_boundary_context.as_ref(),
            )?;
            let admission_boundary_evidence = hydrate_reuse_boundary_evidence(
                crate::data::reuse::ReuseBoundaryEvidence {
                current: current_reuse_boundary_context.clone(),
                previous: previous_reuse_boundary_context.or_else(|| {
                    matches!(
                        reuse_decision.strategy,
                        Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
                            | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
                    )
                    .then_some(current_reuse_boundary_context.clone())
                }),
            });
            let reuse_certification = crate::logic::evaluation::certify_reuse_decision(
                &reuse_contract,
                &reuse_decision,
                &admission_boundary_evidence,
            )
            .map_err(|failure| {
                record_reuse_rejection_telemetry(graph, &failure.failure);
                SignalError::invalid_input(format!(
                    "reuse certification failed for {node}: {:?}; {}",
                    failure.failure,
                    format_reuse_boundary_evidence(&admission_boundary_evidence)
                ))
            })?;
            let lowered_reuse_basis = reuse_decision
                .strategy
                .map(|strategy| {
                    crate::data::reuse::ReuseBasis::from_boundary_context(
                        strategy,
                        reuse_decision.source,
                        reuse_decision.crossing,
                        &current_reuse_boundary_context,
                    )
                })
                .unwrap_or_else(crate::data::reuse::ReuseBasis::fresh_compute);
            let synthesized_metadata = EvaluationExecutionMetadata {
                keyed: None,
                memoized_origin: reuse_decision.memoized_origin,
                reuse_basis: lowered_reuse_basis,
                reuse_origin: reuse_decision.origin,
            };
            let metadata = execution_metadata.unwrap_or(&synthesized_metadata);
            let meaningful_output_change =
                node_output_change_is_meaningful(graph, node, &result, comparator_resolver)?;
            let verdict =
                verdict_for_evaluated_result(graph, node, &result, meaningful_output_change)?;
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
                    None,
                    prepared.keyed.as_ref(),
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
                    None,
                    prepared.keyed.as_ref(),
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
                    None,
                    prepared.keyed.as_ref(),
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

#[cfg(feature = "parallel")]
pub(crate) fn build_prepared_apply_commit_packet(
    graph: &SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
    comparator_policy: VersionComparatorPolicy,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    _dependency_updates: u32,
    dependency_inputs: EffectDependencyInputs,
    defer_snapshot_commit: bool,
) -> Result<ApplyCommitPacket, ApplyCommitBuildError> {
    let packet = match prepared.outcome {
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
                crate::logic::evaluation::resolve_reuse_boundary_context_with_policy(
                    graph,
                    node,
                    comparator_policy.clone(),
                    Some(&result),
                    prepared.keyed.as_ref(),
                )?;
            let previous_reuse_boundary_context = graph
                .get_entry(node)?
                .get_runtime_artifact_state()
                .and_then(|trace| trace.reuse_boundary_context.clone());
            let current_reuse_boundary_context = hydrate_reuse_topology_boundary_from_previous(
                graph,
                node,
                current_reuse_boundary_context,
                previous_reuse_boundary_context.as_ref(),
            )?;
            let admission_boundary_evidence = hydrate_reuse_boundary_evidence(
                crate::data::reuse::ReuseBoundaryEvidence {
                current: current_reuse_boundary_context.clone(),
                previous: previous_reuse_boundary_context.or_else(|| {
                    matches!(
                        reuse_decision.strategy,
                        Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
                            | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
                    )
                    .then_some(current_reuse_boundary_context.clone())
                }),
            });
            let reuse_certification = crate::logic::evaluation::certify_reuse_decision(
                &reuse_contract,
                &reuse_decision,
                &admission_boundary_evidence,
            )
            .map_err(|failure| {
                ApplyCommitBuildError::ReuseBoundary {
                    error: SignalError::invalid_input(format!(
                        "reuse certification failed for {node}: {:?}; {}",
                        failure.failure,
                        format_reuse_boundary_evidence(&admission_boundary_evidence)
                    )),
                    failure: failure.failure,
                }
            })?;
            let lowered_reuse_basis = reuse_decision
                .strategy
                .map(|strategy| {
                    crate::data::reuse::ReuseBasis::from_boundary_context(
                        strategy,
                        reuse_decision.source,
                        reuse_decision.crossing,
                        &current_reuse_boundary_context,
                    )
                })
                .unwrap_or_else(crate::data::reuse::ReuseBasis::fresh_compute);
            let synthesized_metadata = EvaluationExecutionMetadata {
                keyed: None,
                memoized_origin: reuse_decision.memoized_origin,
                reuse_basis: lowered_reuse_basis,
                reuse_origin: reuse_decision.origin,
            };
            let metadata = execution_metadata.unwrap_or(&synthesized_metadata);
            let meaningful_output_change = node_output_change_is_meaningful_with_lowered_policy(
                graph,
                node,
                &result,
                &comparator_policy,
            )?;
            let verdict =
                verdict_for_evaluated_result(graph, node, &result, meaningful_output_change)?;
            let effect = build_evaluation_effect(
                node,
                result,
                Some(metadata),
                verdict,
                reuse_decision.recomputed,
                current_reuse_boundary_context,
                prepared.keyed,
                prepared.trace_data.causality,
                reuse_certification,
                dependency_inputs,
            );
            graph
                .build_apply_commit_packet(effect, comparator_policy, defer_snapshot_commit)
                .map_err(ApplyCommitBuildError::Signal)?
        }
        PreparedEvaluationOutcome::ValidatedClean => {
            let current_version = graph.get_entry(node)?.get_aspect_version();
            let effect = build_evaluation_effect(
                node,
                NodeEvaluationResult::from_version(current_version),
                execution_metadata,
                crate::logic::evaluation::EvaluationVerdict::Suppressed {
                    reason: SuppressionReason::ValidatedClean,
                },
                false,
                crate::logic::evaluation::resolve_reuse_boundary_context_with_policy(
                    graph,
                    node,
                    comparator_policy.clone(),
                    None,
                    prepared.keyed.as_ref(),
                )?,
                prepared.keyed,
                prepared.trace_data.causality,
                None,
                dependency_inputs,
            );
            graph
                .build_apply_commit_packet(effect, comparator_policy, defer_snapshot_commit)
                .map_err(ApplyCommitBuildError::Signal)?
        }
        PreparedEvaluationOutcome::DeferredByCondition => {
            let current_version = graph.get_entry(node)?.get_aspect_version();
            let effect = build_evaluation_effect(
                node,
                NodeEvaluationResult::from_version(current_version),
                execution_metadata,
                crate::logic::evaluation::EvaluationVerdict::Deferred {
                    reason: DeferralReason::ConditionNotMet,
                },
                false,
                crate::logic::evaluation::resolve_reuse_boundary_context_with_policy(
                    graph,
                    node,
                    comparator_policy.clone(),
                    None,
                    prepared.keyed.as_ref(),
                )?,
                prepared.keyed,
                prepared.trace_data.causality,
                None,
                dependency_inputs,
            );
            graph
                .build_apply_commit_packet(effect, comparator_policy, defer_snapshot_commit)
                .map_err(ApplyCommitBuildError::Signal)?
        }
        PreparedEvaluationOutcome::RevertedCleanByCondition => {
            let current_version = graph.get_entry(node)?.get_aspect_version();
            let effect = build_evaluation_effect(
                node,
                NodeEvaluationResult::from_version(current_version),
                execution_metadata,
                crate::logic::evaluation::EvaluationVerdict::Suppressed {
                    reason: SuppressionReason::ConditionRevertedClean,
                },
                false,
                crate::logic::evaluation::resolve_reuse_boundary_context_with_policy(
                    graph,
                    node,
                    comparator_policy.clone(),
                    None,
                    prepared.keyed.as_ref(),
                )?,
                prepared.keyed,
                prepared.trace_data.causality,
                None,
                dependency_inputs,
            );
            graph
                .build_apply_commit_packet(effect, comparator_policy, defer_snapshot_commit)
                .map_err(ApplyCommitBuildError::Signal)?
        }
    };

    Ok(packet)
}

fn node_output_change_is_meaningful(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let comparator = comparator_resolver.policy_for_node(
        node,
        graph.get_entry(node)?.get_eval_config().comparator.as_ref(),
    );
    node_output_change_is_meaningful_with_policy(
        graph,
        node,
        result,
        &comparator,
        comparator_resolver,
    )
}

fn node_output_change_is_meaningful_with_policy(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    comparator_policy: &VersionComparatorPolicy,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let previous = graph.get_entry(node)?.get_aspect_version();
    for (index, (&cached, &current)) in previous
        .slots()
        .iter()
        .zip(result.aspect_version.slots().iter())
        .enumerate()
    {
        if cached == current {
            continue;
        }
        if comparator_policy.has_meaningful_change(
            Aspect::new(index as u8),
            cached,
            current,
            comparator_resolver,
        )? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(feature = "parallel")]
fn node_output_change_is_meaningful_with_lowered_policy(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    comparator_policy: &VersionComparatorPolicy,
) -> Result<bool, SignalError> {
    let previous = graph.get_entry(node)?.get_aspect_version();
    for (&cached, &current) in previous
        .slots()
        .iter()
        .zip(result.aspect_version.slots().iter())
    {
        if cached == current {
            continue;
        }
        let meaningful = match comparator_policy {
            VersionComparatorPolicy::Exact | VersionComparatorPolicy::OutputIdentity => {
                current != cached
            }
            VersionComparatorPolicy::Tolerance { epsilon } => current.abs_diff(cached) > *epsilon,
            VersionComparatorPolicy::Custom { key } => {
                return Err(SignalError::invalid_input(format!(
                    "custom comparator '{key}' requires serial comparator resolution"
                )));
            }
        };
        if meaningful {
            return Ok(true);
        }
    }
    Ok(false)
}

fn format_reuse_boundary_evidence(evidence: &ReuseBoundaryEvidence) -> String {
    let previous = evidence
        .previous
        .as_ref()
        .map(|context| {
            format!(
                "prev[topology={}, structural={:?}, family={:?}, partition_regions={}]",
                context.topology_regime,
                context.structural_dependency_basis,
                context.artifact_family,
                context.partition_region_basis.len()
            )
        })
        .unwrap_or_else(|| "prev[none]".to_string());
    let current = format!(
        "curr[topology={}, structural={:?}, family={:?}, partition_regions={}]",
        evidence.current.topology_regime,
        evidence.current.structural_dependency_basis,
        evidence.current.artifact_family,
        evidence.current.partition_region_basis.len()
    );
    format!("{previous}; {current}")
}

fn hydrate_reuse_boundary_evidence(mut evidence: ReuseBoundaryEvidence) -> ReuseBoundaryEvidence {
    if let Some(previous) = evidence.previous.as_mut() {
        if previous.structural_dependency_basis == DependencySnapshotId::EMPTY
            && evidence.current.structural_dependency_basis != DependencySnapshotId::EMPTY
        {
            previous.structural_dependency_basis = evidence.current.structural_dependency_basis;
        }
        if evidence.current.structural_dependency_basis == DependencySnapshotId::EMPTY
            && previous.structural_dependency_basis != DependencySnapshotId::EMPTY
        {
            evidence.current.structural_dependency_basis = previous.structural_dependency_basis;
        }
    }
    evidence
}

fn hydrate_reuse_topology_boundary_from_previous(
    graph: &SignalGraph,
    node: NodeId,
    mut current: crate::data::reuse::ReuseBoundaryContext,
    previous: Option<&crate::data::reuse::ReuseBoundaryContext>,
) -> Result<crate::data::reuse::ReuseBoundaryContext, SignalError> {
    let Some(previous) = previous else {
        return Ok(current);
    };
    if current.topology_regime != 0 {
        return Ok(current);
    }
    if !graph.dependencies_of(node)?.is_empty() {
        return Ok(current);
    }
    if previous.topology_regime == 0 {
        return Ok(current);
    }
    current.topology_regime = previous.topology_regime;
    Ok(current)
}

pub(crate) fn record_reuse_rejection_telemetry(
    graph: &mut SignalGraph,
    failure: &crate::data::reuse::ReuseBoundaryFailure,
) {
    let evaluation = &mut graph.telemetry_mut().evaluation;
    match failure {
        crate::data::reuse::ReuseBoundaryFailure::UnsupportedStrategyFamily(_) => {
            evaluation.reuse_rejected_unsupported_strategy_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::ContractStrategyDisallowed(_) => {
            evaluation.reuse_rejected_contract_strategy_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::BoundaryMismatch(_)
        | crate::data::reuse::ReuseBoundaryFailure::SnapshotReuseNotAllowed
        | crate::data::reuse::ReuseBoundaryFailure::AuthorityReuseNotAllowed => {
            evaluation.reuse_rejected_boundary_mismatch_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::BoundaryContextUnavailable(_) => {
            evaluation.reuse_rejected_missing_prior_context_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing => {
            evaluation.reuse_rejected_persistent_correspondence_missing_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::PersistentCorrespondenceEvidenceInvalid => {
            evaluation.reuse_rejected_persistent_correspondence_invalid_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::CompositionRegionLegalityFailure => {
            evaluation.reuse_rejected_composition_region_count += 1;
        }
        crate::data::reuse::ReuseBoundaryFailure::MixedBasisInsufficiency => {
            evaluation.reuse_rejected_mixed_basis_insufficiency_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::record_reuse_rejection_telemetry;
    use crate::data::graph::SignalGraph;
    use crate::data::reuse::{ArtifactSemanticBoundary, ReuseBoundaryFailure};

    #[test]
    fn typed_reuse_rejection_telemetry_maps_to_canonical_counters() {
        let mut graph = SignalGraph::new();

        record_reuse_rejection_telemetry(
            &mut graph,
            &ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing,
        );
        record_reuse_rejection_telemetry(
            &mut graph,
            &ReuseBoundaryFailure::CompositionRegionLegalityFailure,
        );
        record_reuse_rejection_telemetry(
            &mut graph,
            &ReuseBoundaryFailure::BoundaryContextUnavailable(
                ArtifactSemanticBoundary::TopologyRegime,
            ),
        );

        let evaluation = &graph.telemetry().evaluation;
        assert_eq!(
            evaluation.reuse_rejected_persistent_correspondence_missing_count,
            1
        );
        assert_eq!(evaluation.reuse_rejected_composition_region_count, 1);
        assert_eq!(evaluation.reuse_rejected_missing_prior_context_count, 1);
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
