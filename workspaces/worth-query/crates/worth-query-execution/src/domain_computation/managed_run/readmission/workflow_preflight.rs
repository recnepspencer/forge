use super::super::WorthQueryYieldedWorkflowRun;
use super::workflow_outcome::WorthQueryWorkflowReadmissionDenialKind;
use crate::domain_computation::WorthQueryExecutionRuntime;

pub(super) fn query_preflight_denial(
    yielded: &WorthQueryYieldedWorkflowRun,
    runtime: &WorthQueryExecutionRuntime,
) -> Option<(WorthQueryWorkflowReadmissionDenialKind, &'static str)> {
    let operation = yielded.resource_attempt.binding_authority();
    if !operation.belongs_to(runtime) {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::ForeignQueryRuntime,
            "yielded workflow belongs to a different Query execution runtime",
        ));
    }
    if !operation.belongs_to_current_installation(runtime) {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::StaleInstallationGeneration,
            "yielded workflow belongs to a stale installed-operation generation",
        ));
    }
    if yielded
        .resource_attempt
        .retained_capacity_reservation_count()
        == 0
    {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::RetainedCapacityMismatch,
            "yielded workflow no longer owns its capacity-reservation package",
        ));
    }
    if !yielded.relational_basis.is_live() {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::RelationalLeaseNotLive,
            "yielded workflow Relational basis lease is no longer live",
        ));
    }
    if !yielded.execution.provider_generation_matches_anchor() {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::ProviderCheckpointMismatch,
            "workflow checkpoint generation no longer matches its provider anchor",
        ));
    }
    if !yielded.artifacts.registry_is_frozen_at_owned_generation()
        || yielded.artifacts.production_generation().ordinal()
            != yielded.artifact_evidence.production_generation()
    {
        return Some((
            WorthQueryWorkflowReadmissionDenialKind::ArtifactGenerationMismatch,
            "workflow artifact registry is not frozen at the yielded production generation",
        ));
    }
    None
}
