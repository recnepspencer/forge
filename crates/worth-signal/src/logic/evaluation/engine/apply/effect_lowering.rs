use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult};
use crate::data::reuse::{
    ReuseBoundaryAuthority, ReuseBoundaryContext, ReuseCertificationRecord, ReuseOrigin,
};
use crate::logic::evaluation::{
    DiagnosticEnvelope, EffectDependencyInputs, EffectRuntimeMetadata, EvaluationEffect,
    EvaluationVerdict, OperationalEffect, PreviousArtifactWarmSnapshot,
};
use crate::logic::prepared::PreparedKeyedContext;

use super::super::metadata::EvaluationExecutionMetadata;

pub(crate) fn build_evaluation_effect(
    node: NodeId,
    result: NodeEvaluationResult,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    verdict: EvaluationVerdict,
    recomputed: bool,
    reuse_boundary_authority: ReuseBoundaryAuthority,
    reuse_boundary_detail: Option<ReuseBoundaryContext>,
    keyed_context: Option<PreparedKeyedContext>,
    causality: Option<crate::data::trace::CausalityMetadata>,
    reuse_certification: Option<ReuseCertificationRecord>,
    dependency_inputs: EffectDependencyInputs,
    previous_artifact_warm: Option<PreviousArtifactWarmSnapshot>,
) -> EvaluationEffect {
    let memoized_origin = execution_metadata
        .map(|metadata| metadata.memoized_origin)
        .unwrap_or(MemoizedResultOrigin::DirectCompute);
    let reuse_basis = execution_metadata
        .map(|metadata| metadata.reuse_basis.clone())
        .unwrap_or_else(crate::data::reuse::ReuseBasis::fresh_compute);
    let reuse_origin = execution_metadata
        .map(|metadata| metadata.reuse_origin)
        .unwrap_or(ReuseOrigin::FreshCompute);
    EvaluationEffect {
        operational: OperationalEffect {
            node,
            verdict,
            aspect_version: result.aspect_version,
            changed_aspect_regions: result.changed_aspect_regions,
            output_change: result.output_change,
            reuse_basis,
            reuse_origin,
            reuse_boundary_authority,
            dependency_snapshot_update: dependency_inputs.dependency_snapshot_update,
            snapshot_delta: dependency_inputs.snapshot_delta,
            meaningful_input_changes: dependency_inputs.meaningful_input_changes,
        },
        diagnostics: DiagnosticEnvelope::from_parts(
            result.output_identity,
            result.continuity_token,
            result.changed_regions,
            result.labels,
        ),
        runtime_metadata: EffectRuntimeMetadata {
            memoized_origin,
            recomputed,
            keyed_context,
            causality,
            reuse_certification,
            reuse_boundary_detail,
            previous_artifact_warm,
        },
    }
}

pub(super) fn resolve_output_equivalence(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<crate::data::output_equivalence::OutputEquivalencePolicy, crate::data::error::SignalError>
{
    let config = graph.node_eval_config(node)?;
    Ok(config.output_equivalence.clone())
}
