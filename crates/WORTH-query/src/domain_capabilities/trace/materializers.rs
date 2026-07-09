use worth_foundational::FoundationalProfileSet;

use super::super::foundational_integration::{
    build_provenance, build_rows, materialize_profile_progression,
    WorthQueryDomainCapabilityProvenanceFreshnessPolicy,
};
use super::super::materialization::{
    WorthQueryDomainCapabilityDescriptiveArtifactKind,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
};
use super::super::payloads::{
    WorthQueryAdmissionContributionPayload, WorthQueryAftermathContributionPayload,
    WorthQueryContinuityContributionPayload, WorthQueryDomainCapabilityPayload,
    WorthQueryExplanationContributionPayload, WorthQueryInvariantCapabilityContributionPayload,
    WorthQuerySupportContributionPayload, WorthQueryWorkflowContributionPayload,
};
use super::super::targets::WorthQueryDomainCapabilityTargetBinding;
use super::super::{
    WorthQueryMaterializationReadyAdmissionContribution,
    WorthQueryMaterializationReadyAftermathContribution,
    WorthQueryMaterializationReadyContinuityContribution,
    WorthQueryMaterializationReadyDomainCapabilityContribution,
    WorthQueryMaterializationReadyExplanationContribution,
    WorthQueryMaterializationReadyInvariantCapabilityContribution,
    WorthQueryMaterializationReadySupportContribution,
    WorthQueryMaterializationReadyWorkflowContribution,
};
use super::artifacts::WorthQueryDomainCapabilityTraceArtifact;

pub type WorthQueryAdmissionContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQuerySupportContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryInvariantCapabilityContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQueryInvariantCapabilityContributionPayload, T>;
pub type WorthQueryWorkflowContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryContinuityContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryAftermathContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryExplanationContributionTraceArtifact<T> =
    WorthQueryDomainCapabilityTraceArtifact<WorthQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_trace_artifact<P, T>(
    contribution: WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryDomainCapabilityTraceArtifact<P, T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    let rows = build_rows(&contribution);
    let profile_progression = materialize_profile_progression(
        &contribution,
        requested_profile,
        WorthQueryDomainCapabilityDescriptiveArtifactKind::TraceArtifact,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        WorthQueryDomainCapabilityDescriptiveArtifactKind::TraceArtifact,
        WorthQueryDomainCapabilityProvenanceFreshnessPolicy::TraceRetention,
    )?;

    Ok(WorthQueryDomainCapabilityTraceArtifact::new(
        contribution,
        profile_progression,
        provenance,
        rows.subject,
        rows.primary_code,
        rows.outcome_kind,
        rows.required_rows,
        rows.standard_rows,
        rows.forensic_rows,
    ))
}

pub fn materialize_admission_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryAdmissionContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_support_traceability_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQuerySupportContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_invariant_capability_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryInvariantCapabilityContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_workflow_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryWorkflowContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_continuity_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryContinuityContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_aftermath_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryAftermathContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_explanation_trace_artifact<T>(
    contribution: WorthQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryExplanationContributionTraceArtifact<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}
