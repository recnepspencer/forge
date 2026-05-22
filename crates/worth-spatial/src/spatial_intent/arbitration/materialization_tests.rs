use forge_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalDiagnosticOutcomeKind, FoundationalProfileSet,
    FoundationalProfileSetInput, RetentionDeliveryProfile, SupportPostureProfile,
};

use crate::spatial_intent::arbitration::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    materialize_spatial_arbitration_support_report, SpatialArbitrationMaterializationProfilePlan,
    SpatialAuthoredActKind, SpatialIntentCapabilitySet, SpatialObservedRelationFact,
};
use crate::spatial_intent::policy::SpatialIntentPolicyProfile;

fn materialization_plan() -> SpatialArbitrationMaterializationProfilePlan {
    let profile = FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .expect("coherent profile");
    SpatialArbitrationMaterializationProfilePlan {
        requested: profile,
        admitted: profile,
        materialized: profile,
        requested_to_admitted_narrowing: None,
        admitted_to_materialized_narrowing: None,
    }
}

#[test]
fn arbitration_materialization_reports_clarification_required_decision() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );

    let materialized =
        materialize_spatial_arbitration_support_report(analysis, materialization_plan())
            .expect("materialization should succeed");
    let decision_row = materialized
        .support_report()
        .decision_rows()
        .next()
        .expect("decision row");

    assert_eq!(
        decision_row.outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Denied
    );
}

#[test]
fn arbitration_materialization_reports_blocked_capability_support() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );

    let materialized =
        materialize_spatial_arbitration_support_report(analysis, materialization_plan())
            .expect("materialization should succeed");

    assert!(materialized.support_report().support_rows().any(|row| {
        row.semantic_labels()
            .labels()
            .iter()
            .any(|label| label.as_str() == "worth.spatial.arbitration.capability.merge_boolean")
    }));
}
