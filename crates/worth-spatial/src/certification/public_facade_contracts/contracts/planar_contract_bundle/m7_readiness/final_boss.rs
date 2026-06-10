use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessFamily, PlanarM7ReadinessSupportPosture,
};

use super::fixture::{bundle_contracts, m7_readiness_parts};

#[test]
fn mb_m6_8_boolean_readiness_final_boss() {
    let world = "m7-readiness-final-boss";
    let parts = m7_readiness_parts(world);
    let motion_digest = parts.motion.retained_motion_digest().to_string();
    let projection_digest = parts.projected.projection_consumption_digest().to_string();
    let retained_digest = parts.retained.retained_fact_digest().to_string();
    let readiness_declaration_digest = parts.readiness.declaration_digest().to_string();
    let readiness_envelope_digest = parts.readiness.envelope_digest().to_string();
    let contracts = bundle_contracts(world);
    let receipt = PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
        .with_structural_identity(parts.structural)
        .with_motion_posture(parts.motion)
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(parts.projected)
        .with_recovery_posture(parts.recovery)
        .with_diagnostics(parts.diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "M6 stops before split/classify/assemble; M7 must opt into boolean execution lanes",
        ))
        .compile(&contracts)
        .expect("final-boss M7 readiness plan")
        .certify()
        .expect("final-boss M7 readiness receipt");

    assert!(receipt.is_acceptable_m7_input());
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert!(!receipt.readiness_digest().is_empty());
    assert_eq!(receipt.declaration_digest(), readiness_declaration_digest);
    assert_eq!(receipt.envelope_digest(), readiness_envelope_digest);
    assert!(receipt.family_rows().iter().any(|row| {
        row.family() == PlanarM7ReadinessFamily::MotionPosture
            && row.receipt_digest() == motion_digest
    }));
    assert!(receipt.family_rows().iter().any(|row| {
        row.family() == PlanarM7ReadinessFamily::RetainedPlanarFacts
            && row.receipt_digest() == retained_digest
    }));
    assert!(receipt.family_rows().iter().any(|row| {
        row.family() == PlanarM7ReadinessFamily::ProjectionConsumedFacts
            && row.receipt_digest() == projection_digest
    }));
    assert!(receipt.family_rows().iter().any(|row| {
        row.family() == PlanarM7ReadinessFamily::SupportPosture
            && row.receipt_digest().contains("support-gated")
    }));
    assert!(!receipt
        .family_rows()
        .iter()
        .any(|row| row.family() == PlanarM7ReadinessFamily::CleanFailBoundary));
    assert_eq!(receipt.counters().support_posture_rows(), 1);
}
