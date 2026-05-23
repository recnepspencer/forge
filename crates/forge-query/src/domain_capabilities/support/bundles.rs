use forge_foundational::{
    materialize_diagnostic_explanation_bundle, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticExplanationInput, FoundationalProfileSet,
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
use super::artifacts::ForgeQueryDomainCapabilityExplanationBundle;

pub type ForgeQueryAdmissionContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQuerySupportContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryInvariantCapabilityContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<
        ForgeQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type ForgeQueryWorkflowContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryContinuityContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryAftermathContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryExplanationContributionExplanationBundle<T> =
    ForgeQueryDomainCapabilityExplanationBundle<ForgeQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_explanation_bundle<P, T>(
    contribution: ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryDomainCapabilityExplanationBundle<P, T>,
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
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::ExplanationBundle,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        ForgeQueryDomainCapabilityDescriptiveArtifactKind::ExplanationBundle,
        ForgeQueryDomainCapabilityProvenanceFreshnessPolicy::SupportSurface(delivery_class),
    )?;
    let bundle = materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            rows.subject.clone(),
            rows.outcome_kind,
            rows.required_rows,
            rows.standard_rows,
            rows.forensic_rows,
            forge_foundational::FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            forge_foundational::FoundationalDiagnosticPartiality::Complete,
            forge_foundational::FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        *profile_progression.materialized().payload().materialized(),
        delivery_class,
    )
    .map_err(|denial| {
        ForgeQueryDomainCapabilityDescriptiveMaterializationDenial::ExplanationBundle {
            category: contribution.payload().category(),
            denial,
        }
    })?;

    Ok(ForgeQueryDomainCapabilityExplanationBundle::new(
        contribution,
        profile_progression,
        provenance,
        bundle,
    ))
}

pub fn materialize_admission_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryAdmissionContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_support_traceability_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQuerySupportContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_invariant_capability_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryInvariantCapabilityContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_workflow_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryWorkflowContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_continuity_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryContinuityContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_aftermath_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryAftermathContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_explanation_explanation_bundle<T>(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    ForgeQueryExplanationContributionExplanationBundle<T>,
    ForgeQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}
