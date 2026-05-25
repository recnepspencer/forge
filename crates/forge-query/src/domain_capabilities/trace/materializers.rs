use forge_foundational::FoundationalProfileSet;

use super::super::foundational_integration::{
    build_provenance, build_rows, materialize_profile_progression,
    ForgeQueryDomainCapabilityProvenanceFreshnessPolicy,
};
use super::super::materialization::{
    ForgeQueryDomainCapabilityDescriptiveArtifactKind,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
};
use super::super::payloads::{
    ForgeQueryAdmissionContributionPayload, ForgeQueryAftermathContributionPayload,
    ForgeQueryContinuityContributionPayload, ForgeQueryDomainCapabilityPayload,
    ForgeQueryExplanationContributionPayload, ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQuerySupportContributionPayload, ForgeQueryWorkflowContributionPayload,
};
use super::super::targets::ForgeQueryDomainCapabilityTargetBinding;
use super::super::{
    ForgeQueryMaterializationReadyAdmissionContribution,
    ForgeQueryMaterializationReadyAftermathContribution,
    ForgeQueryMaterializationReadyContinuityContribution,
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
    ForgeQueryMaterializationReadyExplanationContribution,
    ForgeQueryMaterializationReadyInvariantCapabilityContribution,
    ForgeQueryMaterializationReadySupportContribution,
    ForgeQueryMaterializationReadyWorkflowContribution,
};
use super::artifacts::ForgeQueryDomainCapabilityTraceArtifact;

pub type ForgeQueryAdmissionContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQuerySupportContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryInvariantCapabilityContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQueryInvariantCapabilityContributionPayload, T>;
pub type ForgeQueryWorkflowContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryContinuityContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryAftermathContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryExplanationContributionTraceArtifact<T> =
    ForgeQueryDomainCapabilityTraceArtifact<ForgeQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_trace_artifact<P, T>(
    contribution: ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryDomainCapabilityTraceArtifact<P, T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let rows = build_rows(&contribution);
    let profile_progression = materialize_profile_progression(
        &contribution,
        requested_profile,
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::TraceArtifact,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::TraceArtifact,
        ForgeQueryDomainCapabilityProvenanceFreshnessPolicy::TraceRetention,
    )?;

    Ok(ForgeQueryDomainCapabilityTraceArtifact::new(
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
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryAdmissionContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_support_traceability_trace_artifact<T>(
    contribution: ForgeQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQuerySupportContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_invariant_capability_trace_artifact<T>(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryInvariantCapabilityContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_workflow_trace_artifact<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryWorkflowContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_continuity_trace_artifact<T>(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryContinuityContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_aftermath_trace_artifact<T>(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryAftermathContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}

pub fn materialize_explanation_trace_artifact<T>(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryExplanationContributionTraceArtifact<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_trace_artifact(contribution, requested_profile)
}
