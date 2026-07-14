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
use super::artifacts::WorthQueryDomainCapabilityDescriptiveSummary;

pub type WorthQueryAdmissionContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQuerySupportContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryInvariantCapabilityContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryWorkflowContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryContinuityContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryAftermathContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryExplanationContributionSummary<T> =
    WorthQueryDomainCapabilityDescriptiveSummary<WorthQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_summary<P, T>(
    contribution: WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryDomainCapabilityDescriptiveSummary<P, T>,
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
        WorthQueryDomainCapabilityDescriptiveArtifactKind::Summary,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        WorthQueryDomainCapabilityDescriptiveArtifactKind::Summary,
        WorthQueryDomainCapabilityProvenanceFreshnessPolicy::SummaryReduction,
    )?;

    Ok(WorthQueryDomainCapabilityDescriptiveSummary::new(
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
    contribution: WorthQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryAdmissionContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_support_traceability_summary<T>(
    contribution: WorthQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQuerySupportContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_invariant_capability_summary<T>(
    contribution: WorthQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryInvariantCapabilityContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_workflow_summary<T>(
    contribution: WorthQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryWorkflowContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_continuity_summary<T>(
    contribution: WorthQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryContinuityContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_aftermath_summary<T>(
    contribution: WorthQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryAftermathContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}

pub fn materialize_explanation_summary<T>(
    contribution: WorthQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
) -> Result<
    WorthQueryExplanationContributionSummary<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_summary(contribution, requested_profile)
}
