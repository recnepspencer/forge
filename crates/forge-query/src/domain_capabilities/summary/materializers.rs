use forge_foundational::FoundationalProfileSet;

use super::super::foundational_integration::{
    build_provenance, build_rows, materialize_profile_progression,
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
use super::artifacts::ForgeQueryDomainCapabilityDescriptiveSummary;

pub type ForgeQueryAdmissionContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQuerySupportContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryInvariantCapabilityContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<
        ForgeQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type ForgeQueryWorkflowContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryContinuityContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryAftermathContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryExplanationContributionSummary<T> =
    ForgeQueryDomainCapabilityDescriptiveSummary<ForgeQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_summary<P, T>(
    contribution: ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryDomainCapabilityDescriptiveSummary<P, T>,
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
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::Summary,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::Summary,
    )?;

    Ok(ForgeQueryDomainCapabilityDescriptiveSummary::new(
        contribution,
        profile_progression,
        provenance,
        rows.subject,
        rows.primary_code,
        rows.outcome_kind,
        rows.required_rows.len(),
        rows.standard_rows.len(),
        rows.forensic_rows.len(),
    ))
}

pub fn materialize_admission_summary<T>(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryAdmissionContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_support_traceability_summary<T>(
    contribution: ForgeQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQuerySupportContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_invariant_capability_summary<T>(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryInvariantCapabilityContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_workflow_summary<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryWorkflowContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_continuity_summary<T>(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryContinuityContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_aftermath_summary<T>(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryAftermathContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_explanation_summary<T>(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    ForgeQueryExplanationContributionSummary<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}
