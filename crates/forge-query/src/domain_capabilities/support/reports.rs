use forge_foundational::{
    materialize_diagnostic_support_report, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticSupportClaimStrength, FoundationalDiagnosticSupportInput,
    FoundationalDiagnosticSurfaceAvailability, FoundationalProfileSet,
};

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
use super::artifacts::ForgeQueryDomainCapabilitySupportReport;

pub type ForgeQueryAdmissionContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQuerySupportContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryInvariantCapabilityContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQueryInvariantCapabilityContributionPayload, T>;
pub type ForgeQueryWorkflowContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryContinuityContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryAftermathContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryExplanationContributionSupportReport<T> =
    ForgeQueryDomainCapabilitySupportReport<ForgeQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_support_report<P, T>(
    contribution: ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryDomainCapabilitySupportReport<P, T>,
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
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::SupportReport,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::SupportReport,
        ForgeQueryDomainCapabilityProvenanceFreshnessPolicy::SupportSurface(delivery_class),
    )?;
    let report = materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            rows.subject.clone(),
            rows.outcome_kind,
            rows.required_rows,
            rows.standard_rows,
            rows.forensic_rows,
            availability_for(&contribution),
            FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
            forge_foundational::FoundationalDiagnosticPartiality::Complete,
            counters_for(&contribution),
            Vec::new(),
        ),
        *profile_progression.materialized().payload().materialized(),
        delivery_class,
    )
    .map_err(|denial| {
        ForgeQueryDomainCapabilityDescriptiveMaterializationDenial::SupportReport {
            category: contribution.payload().category(),
            denial,
        }
    })?;

    Ok(ForgeQueryDomainCapabilitySupportReport::new(
        contribution,
        profile_progression,
        provenance,
        report,
    ))
}

pub fn materialize_admission_support_report<T>(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryAdmissionContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

pub fn materialize_support_traceability_support_report<T>(
    contribution: ForgeQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQuerySupportContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

pub fn materialize_invariant_capability_support_report<T>(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryInvariantCapabilityContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

pub fn materialize_workflow_support_report<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryWorkflowContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

pub fn materialize_continuity_support_report<T>(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryContinuityContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

pub fn materialize_aftermath_support_report<T>(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryAftermathContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

pub fn materialize_explanation_support_report<T>(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryExplanationContributionSupportReport<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_support_report(contribution, requested_profile, delivery_class)
}

fn availability_for<P, T>(
    contribution: &ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
) -> FoundationalDiagnosticSurfaceAvailability
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    match contribution.payload().target().kind() {
        super::super::ForgeQueryDomainCapabilityTargetKind::IntentDeclaration
        | super::super::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan => {
            FoundationalDiagnosticSurfaceAvailability::retained_hot()
        }
        super::super::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope => {
            FoundationalDiagnosticSurfaceAvailability::reconstructable()
        }
    }
}

fn counters_for<P, T>(
    contribution: &ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
) -> forge_foundational::FoundationalDiagnosticCounterSnapshot
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    match contribution.payload().target().kind() {
        super::super::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope => {
            forge_foundational::FoundationalDiagnosticCounterSnapshot::new(0, 1, 0, 0, 0, 1)
        }
        _ => forge_foundational::FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
    }
}
