use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessFamily, PlanarM7ReadinessSupportPosture,
};

use super::fixture::{bundle_contracts, m7_readiness_parts};

#[test]
fn boolean_readiness_bundle_contains_all_required_planar_fact_families() {
    let world = "m7-readiness-complete";
    let parts = m7_readiness_parts(world);
    let readiness_declaration_digest = parts.readiness.declaration_digest().to_string();
    let readiness_envelope_digest = parts.readiness.envelope_digest().to_string();
    let contracts = bundle_contracts(world);
    let plan = PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
        .with_structural_identity(parts.structural)
        .with_motion_posture(parts.motion)
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(parts.projected)
        .with_recovery_posture(parts.recovery)
        .with_diagnostics(parts.diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "M7 boolean split/classify/assemble is support-gated until Milestone 7",
        ))
        .compile(&contracts)
        .expect("M7 readiness plan");

    assert_eq!(plan.inspected_closeout_rows(), 12);
    let receipt = plan.certify().expect("M7 readiness receipt");

    assert!(receipt.is_acceptable_m7_input());
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(receipt.family_rows().len(), 12);
    assert_eq!(receipt.counters().closeout_rows(), 12);
    assert_eq!(receipt.counters().support_posture_rows(), 1);
    assert!(receipt.counters().retained_fact_rows() > 0);
    assert!(receipt.counters().projection_consumed_rows() > 0);
    assert_eq!(receipt.counters().rejected_rows(), 0);
    assert_eq!(receipt.declaration_digest(), readiness_declaration_digest);
    assert_eq!(receipt.envelope_digest(), readiness_envelope_digest);

    let families = receipt
        .family_rows()
        .iter()
        .map(|row| row.family())
        .collect::<Vec<_>>();
    assert_eq!(
        families,
        vec![
            PlanarM7ReadinessFamily::BooleanReadinessBundle,
            PlanarM7ReadinessFamily::PredicateAuthority,
            PlanarM7ReadinessFamily::StructuralIdentity,
            PlanarM7ReadinessFamily::MotionPosture,
            PlanarM7ReadinessFamily::TopologyCompleteness,
            PlanarM7ReadinessFamily::Precision,
            PlanarM7ReadinessFamily::Transform,
            PlanarM7ReadinessFamily::RetainedPlanarFacts,
            PlanarM7ReadinessFamily::ProjectionConsumedFacts,
            PlanarM7ReadinessFamily::RecoveryPosture,
            PlanarM7ReadinessFamily::Diagnostics,
            PlanarM7ReadinessFamily::SupportPosture,
        ]
    );
    assert!(receipt
        .family_rows()
        .iter()
        .all(|row| !row.receipt_digest().is_empty()));
    assert!(!receipt.readiness_digest().is_empty());
}
