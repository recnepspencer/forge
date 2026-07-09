use worth_foundational::{
    materialize_diagnostic_explanation_bundle, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticExplanationInput, FoundationalProfileSet,
};

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
use super::artifacts::WorthQueryDomainCapabilityExplanationBundle;

pub type WorthQueryAdmissionContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQuerySupportContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryInvariantCapabilityContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryWorkflowContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryContinuityContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryAftermathContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryExplanationContributionExplanationBundle<T> =
    WorthQueryDomainCapabilityExplanationBundle<WorthQueryExplanationContributionPayload, T>;

pub fn materialize_domain_capability_explanation_bundle<P, T>(
    contribution: WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryDomainCapabilityExplanationBundle<P, T>,
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
        WorthQueryDomainCapabilityDescriptiveArtifactKind::ExplanationBundle,
    )?;
    let provenance = build_provenance(
        &contribution,
        &profile_progression,
        &rows,
        WorthQueryDomainCapabilityDescriptiveArtifactKind::ExplanationBundle,
        WorthQueryDomainCapabilityProvenanceFreshnessPolicy::SupportSurface(delivery_class),
    )?;
    let bundle = materialize_diagnostic_explanation_bundle(
        FoundationalDiagnosticExplanationInput::new(
            rows.subject.clone(),
            rows.outcome_kind,
            rows.required_rows,
            rows.standard_rows,
            rows.forensic_rows,
            worth_foundational::FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            worth_foundational::FoundationalDiagnosticPartiality::Complete,
            worth_foundational::FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        *profile_progression.materialized().payload().materialized(),
        delivery_class,
    )
    .map_err(|denial| {
        WorthQueryDomainCapabilityDescriptiveMaterializationDenial::ExplanationBundle {
            category: contribution.payload().category(),
            denial,
        }
    })?;

    Ok(WorthQueryDomainCapabilityExplanationBundle::new(
        contribution,
        profile_progression,
        provenance,
        bundle,
    ))
}

pub fn materialize_admission_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadyAdmissionContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryAdmissionContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_support_traceability_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadySupportContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQuerySupportContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_invariant_capability_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadyInvariantCapabilityContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryInvariantCapabilityContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_workflow_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadyWorkflowContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryWorkflowContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_continuity_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadyContinuityContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryContinuityContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_aftermath_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadyAftermathContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryAftermathContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}

pub fn materialize_explanation_explanation_bundle<T>(
    contribution: WorthQueryMaterializationReadyExplanationContribution<T>,
    requested_profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    WorthQueryExplanationContributionExplanationBundle<T>,
    WorthQueryDomainCapabilityDescriptiveMaterializationDenial,
>
where
    T: WorthQueryDomainCapabilityTargetBinding,
{
    materialize_domain_capability_explanation_bundle(
        contribution,
        requested_profile,
        delivery_class,
    )
}
