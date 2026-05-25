use super::materialization::{
    materialize_explanation_explanation_bundle, materialize_support_traceability_support_report,
};
use super::payloads::{
    ForgeQueryExplanationContributionPayload, ForgeQueryExplanationContributionPosture,
    ForgeQuerySupportContributionPayload, ForgeQuerySupportContributionPosture,
};
use super::targets::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget;
use super::test_support::{lower_runtime_target, ready_payload};
use forge_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalDiagnosticDeliveryClass, FoundationalProfileSet,
    FoundationalProfileSetInput, RetentionDeliveryProfile, SupportPostureProfile,
};

#[test]
fn lower_runtime_descriptive_outputs_do_not_claim_current_fresh_provenance() {
    let support = materialize_support_traceability_support_report(
        lower_runtime_ready_contribution(ForgeQuerySupportContributionPayload::new(
            ForgeQuerySupportContributionPosture::DeclarationTraceability,
            "runtime.boundary.traceability.replay",
            "lower runtime support remains replay-derived through replay-capable delivery",
        )),
        forensic_support_profile(),
        FoundationalDiagnosticDeliveryClass::ReconstructableFromReplay,
    )
    .expect("support report should materialize");
    let explanation = materialize_explanation_explanation_bundle(
        lower_runtime_ready_contribution(ForgeQueryExplanationContributionPayload::new(
            ForgeQueryExplanationContributionPosture::ExplainsFallback,
            "runtime.boundary.explanation.hot",
            "lower runtime explanation remains replay-derived even when callers request hot delivery",
        )),
        forensic_support_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .expect("explanation bundle should materialize");

    for provenance in [support.provenance(), explanation.provenance()] {
        assert_eq!(
            provenance.locality(),
            forge_foundational::FoundationalBoundaryEvidenceLocality::ReplayDerived
        );
        assert_eq!(
            provenance.freshness_posture(),
            forge_foundational::FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay
        );
    }
}

fn lower_runtime_ready_contribution<P>(
    payload: P,
) -> super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
    P,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
>
where
    P: super::ForgeQueryDomainCapabilityPayload,
    (P, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget):
        super::proof_integration::AllowedContributionBinding<
            P,
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
{
    ready_payload(lower_runtime_target("boundary-provenance"), payload)
}

fn forensic_support_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Forensic,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .expect("valid profile")
}
