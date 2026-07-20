use crate::data::aspect::Aspect;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::dependency::DependencyEdge;
use crate::data::dependency::DependencySnapshotId;
use crate::data::error::SignalError;
#[cfg(feature = "parallel")]
use crate::data::graph::ApplyCommitPacket;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::NodeEvaluationResult;
use crate::data::reuse::ReuseBoundaryAuthority;
use crate::data::reuse::ReuseBoundaryEvidence;
#[cfg(feature = "parallel")]
use crate::data::reuse::ReuseBoundaryFailure;
use crate::data::temporal::LoweredTemporalEligibility;
use crate::logic::evaluation::EffectDependencyInputs;
use crate::logic::evaluation::{
    DeferralReason, PreparedApplyResult, PreviousArtifactWarmSnapshot, SuppressionReason,
};
use crate::logic::prepared::PreparedDependencyCapture;
use crate::logic::prepared::{PreparedEvaluation, PreparedEvaluationOutcome};

#[cfg(feature = "parallel")]
use super::apply::build_evaluation_effect;
use super::apply::{apply_effect_with_policy_and_condition, verdict_for_evaluated_result};
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

struct PassivePreparedEffect {
    result: NodeEvaluationResult,
    verdict: crate::logic::evaluation::EvaluationVerdict,
    temporal_eligibility: Option<LoweredTemporalEligibility>,
    reuse_boundary_authority: ReuseBoundaryAuthority,
    reuse_boundary_detail: Option<crate::data::reuse::ReuseBoundaryContext>,
    keyed: Option<crate::logic::prepared::PreparedKeyedContext>,
    causality: Option<crate::data::trace::CausalityMetadata>,
}

fn lower_passive_prepared_effect<R>(
    graph: &SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
    resolve_reuse_boundary_authority: R,
) -> Result<PassivePreparedEffect, SignalError>
where
    R: FnOnce(
        &SignalGraph,
        NodeId,
        Option<&NodeEvaluationResult>,
        Option<&crate::logic::prepared::PreparedKeyedContext>,
    ) -> Result<ReuseBoundaryAuthority, SignalError>,
{
    let PreparedEvaluation {
        outcome,
        keyed,
        trace_data,
        ..
    } = prepared;
    let temporal_eligibility = trace_data.temporal_eligibility;
    let verdict = match outcome {
        PreparedEvaluationOutcome::ValidatedClean => {
            ensure_temporal_outcome_alignment(outcome, temporal_eligibility.as_ref())?;
            crate::logic::evaluation::EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ValidatedClean,
            }
        }
        PreparedEvaluationOutcome::DeferredByCondition => {
            ensure_temporal_outcome_alignment(outcome, temporal_eligibility.as_ref())?;
            crate::logic::evaluation::EvaluationVerdict::Deferred {
                reason: if temporal_eligibility.is_some() {
                    DeferralReason::TemporalConditionNotMet
                } else {
                    DeferralReason::ConditionNotMet
                },
            }
        }
        PreparedEvaluationOutcome::RevertedCleanByCondition => {
            ensure_temporal_outcome_alignment(outcome, temporal_eligibility.as_ref())?;
            crate::logic::evaluation::EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ConditionRevertedClean,
            }
        }
        PreparedEvaluationOutcome::Evaluate => {
            return Err(SignalError::internal(
                "passive prepared-effect lowering cannot accept evaluate outcomes",
            ));
        }
    };
    let result = NodeEvaluationResult::from_version(graph.node_aspect_version(node)?);

    Ok(PassivePreparedEffect {
        result,
        verdict,
        temporal_eligibility,
        reuse_boundary_authority: resolve_reuse_boundary_authority(
            graph,
            node,
            None,
            keyed.as_ref(),
        )?,
        reuse_boundary_detail: None,
        keyed,
        causality: trace_data.causality,
    })
}

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
    if !matches!(prepared.outcome, PreparedEvaluationOutcome::Evaluate) {
        let passive =
            lower_passive_prepared_effect(graph, node, prepared, |graph, node, result, keyed| {
                crate::logic::evaluation::resolve_reuse_boundary_authority(
                    graph,
                    node,
                    comparator_resolver,
                    result,
                    keyed,
                )
            })?;
        let mut apply_result = apply_effect_with_policy_and_condition(
            graph,
            node,
            passive.result,
            comparator_resolver,
            execution_metadata,
            passive.verdict,
            false,
            passive.reuse_boundary_authority,
            passive.reuse_boundary_detail,
            passive.keyed,
            passive.causality,
            None,
            dependency_inputs,
            defer_snapshot_commit,
            None,
        )?;
        apply_result.dependency_updates = dependency_updates;
        apply_result.report.temporal_eligibility = passive.temporal_eligibility.clone();
        apply_result.temporal_eligibility = passive.temporal_eligibility;
        return Ok(apply_result);
    }

    match prepared.outcome {
        PreparedEvaluationOutcome::Evaluate => {
            ensure_temporal_outcome_alignment(
                prepared.outcome,
                prepared.trace_data.temporal_eligibility.as_ref(),
            )?;
            let mut result = prepared.result;
            result.labels.extend(prepared.trace_data.labels);
            let reuse_decision = crate::logic::evaluation::resolve_prepared_reuse_decision(
                prepared.origin,
                execution_metadata,
            );
            let reuse_contract = graph.node_eval_config(node)?.contract.reuse.clone();
            let previous_artifact_warm = graph
                .node_runtime_artifact_reuse_boundary_snapshot(node)?
                .map(|trace| PreviousArtifactWarmSnapshot {
                    output_identity: trace.output_identity,
                    continuity_token: trace.continuity_token,
                    reuse_boundary_authority: trace.reuse_boundary_authority,
                });
            let previous_reuse_boundary_authority = previous_artifact_warm
                .as_ref()
                .and_then(|trace| trace.reuse_boundary_authority.clone());
            let (current_reuse_boundary_authority, current_reuse_boundary_detail) =
                resolve_effect_reuse_boundary(
                    graph,
                    node,
                    comparator_resolver,
                    Some(&result),
                    prepared.keyed.as_ref(),
                    reuse_decision.strategy,
                    previous_reuse_boundary_authority.as_ref(),
                )?;
            let admission_boundary_evidence =
                hydrate_reuse_boundary_evidence(crate::data::reuse::ReuseBoundaryEvidence {
                    current: current_reuse_boundary_authority.clone(),
                    previous: previous_reuse_boundary_authority.or_else(|| {
                        matches!(
                            reuse_decision.strategy,
                            Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
                                | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
                        )
                        .then_some(current_reuse_boundary_authority.clone())
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
                    crate::data::reuse::ReuseBasis::from_boundary_authority(
                        strategy,
                        reuse_decision.source,
                        reuse_decision.crossing,
                        &current_reuse_boundary_authority,
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
            let verdict = verdict_for_evaluated_result(
                previous_artifact_warm.as_ref(),
                &result,
                meaningful_output_change,
            )?;
            let mut apply_result = apply_effect_with_policy_and_condition(
                graph,
                node,
                result,
                comparator_resolver,
                Some(metadata),
                verdict,
                reuse_decision.recomputed,
                current_reuse_boundary_authority,
                current_reuse_boundary_detail,
                prepared.keyed,
                prepared.trace_data.causality,
                reuse_certification,
                dependency_inputs,
                defer_snapshot_commit,
                previous_artifact_warm,
            )?;
            apply_result.dependency_updates = dependency_updates;
            apply_result.report.temporal_eligibility =
                prepared.trace_data.temporal_eligibility.clone();
            apply_result.temporal_eligibility = prepared.trace_data.temporal_eligibility;
            Ok(apply_result)
        }
        PreparedEvaluationOutcome::ValidatedClean
        | PreparedEvaluationOutcome::DeferredByCondition
        | PreparedEvaluationOutcome::RevertedCleanByCondition => Err(SignalError::internal(
            "passive prepared outcomes must be lowered before entering the evaluate branch",
        )),
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
    if !matches!(prepared.outcome, PreparedEvaluationOutcome::Evaluate) {
        let passive =
            lower_passive_prepared_effect(graph, node, prepared, |graph, node, result, keyed| {
                crate::logic::evaluation::resolve_reuse_boundary_authority_with_policy(
                    graph,
                    node,
                    comparator_policy.clone(),
                    result,
                    keyed,
                )
            })?;
        let effect = build_evaluation_effect(
            node,
            passive.result,
            execution_metadata,
            passive.verdict,
            false,
            passive.reuse_boundary_authority,
            passive.reuse_boundary_detail,
            passive.keyed,
            passive.causality,
            None,
            dependency_inputs,
            None,
        );
        return graph
            .build_apply_commit_packet(effect, comparator_policy, defer_snapshot_commit)
            .map_err(ApplyCommitBuildError::Signal);
    }

    let packet = match prepared.outcome {
        PreparedEvaluationOutcome::Evaluate => {
            ensure_temporal_outcome_alignment(
                prepared.outcome,
                prepared.trace_data.temporal_eligibility.as_ref(),
            )?;
            let mut result = prepared.result;
            result.labels.extend(prepared.trace_data.labels);
            let reuse_decision = crate::logic::evaluation::resolve_prepared_reuse_decision(
                prepared.origin,
                execution_metadata,
            );
            let reuse_contract = graph.node_eval_config(node)?.contract.reuse.clone();
            let previous_artifact_warm = graph
                .node_runtime_artifact_reuse_boundary_snapshot(node)?
                .map(|trace| PreviousArtifactWarmSnapshot {
                    output_identity: trace.output_identity,
                    continuity_token: trace.continuity_token,
                    reuse_boundary_authority: trace.reuse_boundary_authority,
                });
            let previous_reuse_boundary_authority = previous_artifact_warm
                .as_ref()
                .and_then(|trace| trace.reuse_boundary_authority.clone());
            let (current_reuse_boundary_authority, current_reuse_boundary_detail) =
                resolve_effect_reuse_boundary_with_policy(
                    graph,
                    node,
                    comparator_policy.clone(),
                    Some(&result),
                    prepared.keyed.as_ref(),
                    reuse_decision.strategy,
                    previous_reuse_boundary_authority.as_ref(),
                )?;
            let admission_boundary_evidence =
                hydrate_reuse_boundary_evidence(crate::data::reuse::ReuseBoundaryEvidence {
                    current: current_reuse_boundary_authority.clone(),
                    previous: previous_reuse_boundary_authority.or_else(|| {
                        matches!(
                            reuse_decision.strategy,
                            Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
                                | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
                        )
                        .then_some(current_reuse_boundary_authority.clone())
                    }),
                });
            let reuse_certification = crate::logic::evaluation::certify_reuse_decision(
                &reuse_contract,
                &reuse_decision,
                &admission_boundary_evidence,
            )
            .map_err(|failure| ApplyCommitBuildError::ReuseBoundary {
                error: SignalError::invalid_input(format!(
                    "reuse certification failed for {node}: {:?}; {}",
                    failure.failure,
                    format_reuse_boundary_evidence(&admission_boundary_evidence)
                )),
                failure: failure.failure,
            })?;
            let lowered_reuse_basis = reuse_decision
                .strategy
                .map(|strategy| {
                    crate::data::reuse::ReuseBasis::from_boundary_authority(
                        strategy,
                        reuse_decision.source,
                        reuse_decision.crossing,
                        &current_reuse_boundary_authority,
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
            let verdict = verdict_for_evaluated_result(
                previous_artifact_warm.as_ref(),
                &result,
                meaningful_output_change,
            )?;
            let effect = build_evaluation_effect(
                node,
                result,
                Some(metadata),
                verdict,
                reuse_decision.recomputed,
                current_reuse_boundary_authority,
                current_reuse_boundary_detail,
                prepared.keyed,
                prepared.trace_data.causality,
                reuse_certification,
                dependency_inputs,
                previous_artifact_warm,
            );
            graph
                .build_apply_commit_packet(effect, comparator_policy, defer_snapshot_commit)
                .map_err(ApplyCommitBuildError::Signal)?
        }
        PreparedEvaluationOutcome::ValidatedClean
        | PreparedEvaluationOutcome::DeferredByCondition
        | PreparedEvaluationOutcome::RevertedCleanByCondition => {
            return Err(ApplyCommitBuildError::Signal(SignalError::internal(
                "passive prepared outcomes must be lowered before entering the evaluate branch",
            )))
        }
    };

    Ok(packet)
}

fn ensure_temporal_outcome_alignment(
    outcome: PreparedEvaluationOutcome,
    temporal_eligibility: Option<&LoweredTemporalEligibility>,
) -> Result<(), SignalError> {
    match (outcome, temporal_eligibility) {
        (PreparedEvaluationOutcome::Evaluate, Some(LoweredTemporalEligibility::Deferred(_))) => {
            Err(SignalError::internal(
                "evaluate outcome cannot carry deferred temporal eligibility proof",
            ))
        }
        (
            PreparedEvaluationOutcome::DeferredByCondition,
            Some(LoweredTemporalEligibility::Ready(_)),
        ) => Err(SignalError::internal(
            "deferred outcome cannot carry ready temporal eligibility proof",
        )),
        (
            PreparedEvaluationOutcome::ValidatedClean
            | PreparedEvaluationOutcome::RevertedCleanByCondition,
            Some(_),
        ) => Err(SignalError::internal(
            "passive non-temporal suppression cannot carry temporal eligibility proof",
        )),
        _ => Ok(()),
    }
}

fn node_output_change_is_meaningful(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let comparator = comparator_resolver
        .policy_for_node(node, graph.node_eval_config(node)?.comparator.as_ref());
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
    let previous = graph.node_aspect_version(node)?;
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
    let previous = graph.node_aspect_version(node)?;
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
                context.partition_region_basis_count
            )
        })
        .unwrap_or_else(|| "prev[none]".to_string());
    let current = format!(
        "curr[topology={}, structural={:?}, family={:?}, partition_regions={}]",
        evidence.current.topology_regime,
        evidence.current.structural_dependency_basis,
        evidence.current.artifact_family,
        evidence.current.partition_region_basis_count
    );
    format!("{previous}; {current}")
}

fn resolve_effect_reuse_boundary(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &impl ComparatorPolicyResolver,
    result: Option<&NodeEvaluationResult>,
    keyed: Option<&crate::logic::prepared::PreparedKeyedContext>,
    strategy: Option<crate::data::reuse::ReuseStrategy>,
    previous: Option<&crate::data::reuse::ReuseBoundaryAuthority>,
) -> Result<
    (
        ReuseBoundaryAuthority,
        Option<crate::data::reuse::ReuseBoundaryContext>,
    ),
    SignalError,
> {
    if retains_reuse_boundary_detail(graph, strategy) {
        let detail = hydrate_reuse_topology_boundary_from_previous(
            graph,
            node,
            crate::logic::evaluation::resolve_reuse_boundary_context(
                graph,
                node,
                comparator_resolver,
                result,
                keyed,
            )?,
            previous,
        )?;
        let authority = detail.authority();
        return Ok((authority, Some(detail)));
    }

    let authority = hydrate_reuse_topology_boundary_authority_from_previous(
        graph,
        node,
        crate::logic::evaluation::resolve_reuse_boundary_authority(
            graph,
            node,
            comparator_resolver,
            result,
            keyed,
        )?,
        previous,
    )?;
    Ok((authority, None))
}

#[cfg(feature = "parallel")]
fn resolve_effect_reuse_boundary_with_policy(
    graph: &SignalGraph,
    node: NodeId,
    comparator_policy: VersionComparatorPolicy,
    result: Option<&NodeEvaluationResult>,
    keyed: Option<&crate::logic::prepared::PreparedKeyedContext>,
    strategy: Option<crate::data::reuse::ReuseStrategy>,
    previous: Option<&crate::data::reuse::ReuseBoundaryAuthority>,
) -> Result<
    (
        ReuseBoundaryAuthority,
        Option<crate::data::reuse::ReuseBoundaryContext>,
    ),
    SignalError,
> {
    if retains_reuse_boundary_detail(graph, strategy) {
        let detail = hydrate_reuse_topology_boundary_from_previous(
            graph,
            node,
            crate::logic::evaluation::resolve_reuse_boundary_context_with_policy(
                graph,
                node,
                comparator_policy,
                result,
                keyed,
            )?,
            previous,
        )?;
        let authority = detail.authority();
        return Ok((authority, Some(detail)));
    }

    let authority = hydrate_reuse_topology_boundary_authority_from_previous(
        graph,
        node,
        crate::logic::evaluation::resolve_reuse_boundary_authority_with_policy(
            graph,
            node,
            comparator_policy,
            result,
            keyed,
        )?,
        previous,
    )?;
    Ok((authority, None))
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
    previous: Option<&crate::data::reuse::ReuseBoundaryAuthority>,
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

fn hydrate_reuse_topology_boundary_authority_from_previous(
    graph: &SignalGraph,
    node: NodeId,
    mut current: crate::data::reuse::ReuseBoundaryAuthority,
    previous: Option<&crate::data::reuse::ReuseBoundaryAuthority>,
) -> Result<crate::data::reuse::ReuseBoundaryAuthority, SignalError> {
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

fn retains_reuse_boundary_detail(
    graph: &SignalGraph,
    strategy: Option<crate::data::reuse::ReuseStrategy>,
) -> bool {
    let retention = graph.runtime_policy().retention_budget;
    let cold_retention_active = matches!(
        retention.explanation_retention,
        crate::diagnostics::policy::ArtifactRetentionPolicy::Retain
    ) || matches!(
        retention.provenance_retention,
        crate::diagnostics::policy::ArtifactRetentionPolicy::Retain
    );
    cold_retention_active
        && matches!(
            strategy,
            Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
                | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
        )
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
    use super::{
        record_reuse_rejection_telemetry, resolve_effect_reuse_boundary,
        retains_reuse_boundary_detail,
    };
    use crate::data::comparator::DefaultComparatorPolicyResolver;
    use crate::data::graph::SignalGraph;
    use crate::data::output::NodeEvaluationResult;
    use crate::data::proof::PartitionScopeSet;
    use crate::data::reuse::{
        ArtifactSemanticBoundary, PersistentCorrespondenceEvidence, ReuseBoundaryFailure,
        ReuseStrategy,
    };
    use crate::diagnostics::policy::ArtifactRetentionPolicy;
    use crate::logic::prepared::PreparedKeyedContext;
    use crate::tests::support::version_ab;

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

    #[test]
    fn memoized_reuse_resolves_authority_without_rich_detail() {
        let mut graph = SignalGraph::new();
        let node = graph.node().build();
        graph.set_runtime_policy(
            crate::facade::SignalRuntimePolicy::development()
                .with_explanation_retention(ArtifactRetentionPolicy::Retain)
                .with_provenance_retention(ArtifactRetentionPolicy::Retain),
        );

        let mut comparator_resolver = DefaultComparatorPolicyResolver::default();
        let (authority, detail) = resolve_effect_reuse_boundary(
            &graph,
            node,
            &mut comparator_resolver,
            Some(&NodeEvaluationResult::from_version(version_ab(1, 0))),
            None,
            Some(ReuseStrategy::MemoizedArtifactReuse),
            None,
        )
        .expect("authority-only resolution should succeed");

        assert!(detail.is_none());
        assert_eq!(authority.topology_regime, 0);
    }

    #[test]
    fn cross_identity_reuse_retains_rich_boundary_detail_when_policy_requires_it() {
        let mut graph = SignalGraph::new();
        let node = graph.node().build();
        graph.set_runtime_policy(crate::facade::SignalRuntimePolicy::development());
        let mut comparator_resolver = DefaultComparatorPolicyResolver::default();
        let keyed = PreparedKeyedContext {
            persistent_correspondence: Some(
                PersistentCorrespondenceEvidence::lineage_backed_mapping("lineage-map:left->right"),
            ),
            composition_regions: PartitionScopeSet::default(),
            ..PreparedKeyedContext::default()
        };

        let (authority, detail) = resolve_effect_reuse_boundary(
            &graph,
            node,
            &mut comparator_resolver,
            Some(&NodeEvaluationResult::from_version(version_ab(1, 0))),
            Some(&keyed),
            Some(ReuseStrategy::CrossIdentityPersistentMatch),
            None,
        )
        .expect("cross-identity boundary resolution should succeed");

        assert!(detail.is_some());
        assert!(authority.persistent_correspondence_kind().is_some());
    }

    #[test]
    fn reuse_boundary_detail_retention_is_strategy_and_policy_gated() {
        let mut graph = SignalGraph::new();
        graph.set_runtime_policy(
            crate::facade::SignalRuntimePolicy::operational()
                .with_explanation_retention(ArtifactRetentionPolicy::Omit)
                .with_provenance_retention(ArtifactRetentionPolicy::Omit),
        );

        assert!(!retains_reuse_boundary_detail(
            &graph,
            Some(ReuseStrategy::CrossIdentityPersistentMatch)
        ));

        graph.set_runtime_policy(crate::facade::SignalRuntimePolicy::development());
        assert!(retains_reuse_boundary_detail(
            &graph,
            Some(ReuseStrategy::PartialArtifactSplicing)
        ));
        assert!(!retains_reuse_boundary_detail(
            &graph,
            Some(ReuseStrategy::MemoizedArtifactReuse)
        ));
    }
}

fn apply_prepared_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
    capture: &PreparedDependencyCapture,
) -> Result<u32, SignalError> {
    let desired = build_prepared_dependency_edges(graph, capture);
    let report = graph.reconcile_dependencies(node, &desired)?;
    Ok(report.added + report.removed)
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
