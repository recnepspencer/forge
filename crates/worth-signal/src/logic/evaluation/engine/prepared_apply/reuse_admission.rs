use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::NodeEvaluationResult;
use crate::data::reuse::{
    ReuseBoundaryAuthority, ReuseBoundaryContext, ReuseBoundaryEvidence, ReuseCertificationRecord,
    ReuseStrategy,
};
use crate::logic::evaluation::reuse::ResolvedReuseDecision;
use crate::logic::evaluation::{EvaluationExecutionMetadata, PreviousArtifactWarmSnapshot};
use crate::logic::prepared::{PreparedEvaluationOrigin, PreparedKeyedContext};

use super::admission::{
    format_reuse_boundary_evidence, hydrate_reuse_boundary_evidence, resolve_effect_reuse_boundary,
};
use super::telemetry::record_reuse_rejection_telemetry;

pub(super) struct EvaluatedReuseAdmission {
    pub(super) decision: ResolvedReuseDecision,
    pub(super) current_boundary_authority: ReuseBoundaryAuthority,
    pub(super) current_boundary_detail: Option<ReuseBoundaryContext>,
    pub(super) certification: Option<ReuseCertificationRecord>,
    pub(super) previous_artifact_warm: Option<PreviousArtifactWarmSnapshot>,
}

pub(super) fn resolve_evaluated_reuse_admission(
    graph: &mut SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    keyed: Option<&PreparedKeyedContext>,
    origin: PreparedEvaluationOrigin,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    comparator_resolver: &mut impl crate::data::comparator::ComparatorPolicyResolver,
) -> Result<EvaluatedReuseAdmission, SignalError> {
    let decision =
        crate::logic::evaluation::resolve_prepared_reuse_decision(origin, execution_metadata);
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
    let (current_boundary_authority, current_boundary_detail) = resolve_effect_reuse_boundary(
        graph,
        node,
        comparator_resolver,
        Some(result),
        keyed,
        decision.strategy,
        previous_reuse_boundary_authority.as_ref(),
    )?;
    let certification = certify_evaluated_reuse_boundary(
        graph,
        node,
        &reuse_contract,
        &decision,
        &current_boundary_authority,
        previous_reuse_boundary_authority,
    )?;
    Ok(EvaluatedReuseAdmission {
        decision,
        current_boundary_authority,
        current_boundary_detail,
        certification,
        previous_artifact_warm,
    })
}

fn certify_evaluated_reuse_boundary(
    graph: &mut SignalGraph,
    node: NodeId,
    reuse_contract: &crate::data::reuse::NodeReuseContract,
    decision: &ResolvedReuseDecision,
    current_boundary_authority: &ReuseBoundaryAuthority,
    previous_reuse_boundary_authority: Option<ReuseBoundaryAuthority>,
) -> Result<Option<ReuseCertificationRecord>, SignalError> {
    let admission_boundary_evidence = hydrate_reuse_boundary_evidence(ReuseBoundaryEvidence {
        current: current_boundary_authority.clone(),
        previous: previous_reuse_boundary_authority.or_else(|| {
            matches!(
                decision.strategy,
                Some(ReuseStrategy::CrossIdentityPersistentMatch)
                    | Some(ReuseStrategy::PartialArtifactSplicing)
            )
            .then_some(current_boundary_authority.clone())
        }),
    });
    crate::logic::evaluation::certify_reuse_decision(
        reuse_contract,
        decision,
        &admission_boundary_evidence,
    )
    .map_err(|failure| {
        record_reuse_rejection_telemetry(graph, &failure.failure);
        SignalError::invalid_input(format!(
            "reuse certification failed for {node}: {:?}; {}",
            failure.failure,
            format_reuse_boundary_evidence(&admission_boundary_evidence)
        ))
    })
}
