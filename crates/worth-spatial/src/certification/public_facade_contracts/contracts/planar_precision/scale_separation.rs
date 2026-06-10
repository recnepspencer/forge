use worth_math::arithmetic::precision::PrecisionMode;
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionCertificationCase,
};

use super::proof_fixture::{
    precision_basis, precision_handle, precision_receipt_for, predicate_receipt,
};

#[test]
fn planar_precision_escalation_uses_local_feature_scale_not_world_magnitude() {
    let predicate = predicate_receipt(
        "movement:rotation-cancelled",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let basis = precision_basis(&predicate);
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
    );
    let receipt = planar_precision_certification_facts(&entry, &precision_handle("scale"))
        .expect("precision certificate");

    assert_eq!(receipt.scale_separation_orders(), 21);
    assert_eq!(receipt.basis().local_feature_scale_order(), -9);
    assert_eq!(receipt.basis().world_magnitude_order(), 12);
    assert_eq!(receipt.basis().normalization_scale(), 1.0e-9);
    assert_eq!(receipt.predicate_fact_digest(), predicate.fact_digest());
    assert_eq!(receipt.counters().predicate_precision_rows_consumed(), 1);
    assert_eq!(
        receipt.counters().precision_escalation_breadth(),
        predicate
            .precision_escalation()
            .get_expansion_length()
            .unwrap_or(0)
    );
    assert_eq!(receipt.counters().local_coordinate_normalizations(), 1);
    assert_eq!(receipt.counters().scale_separation_calculations(), 1);
    assert!(receipt.counters().basis_digest_part_count() >= 17);
}

#[test]
fn planar_precision_counters_preserve_nonzero_worth_math_escalation_breadth() {
    let predicate = predicate_receipt(
        "movement:rotation-cancelled",
        [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0 + 1.0e-15]],
    );
    let basis = precision_basis(&predicate);
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
    );
    let receipt = planar_precision_certification_facts(&entry, &precision_handle("adaptive"))
        .expect("adaptive precision certificate");

    assert_eq!(
        predicate.precision_escalation().get_resolved_at(),
        PrecisionMode::ExpansionB
    );
    assert_eq!(
        receipt.counters().precision_escalation_breadth(),
        predicate
            .precision_escalation()
            .get_expansion_length()
            .expect("adaptive predicate must expose expansion length")
    );
    assert!(receipt.counters().precision_escalation_breadth() > 0);
}

#[test]
fn mb_m6_3_thin_feature_scale_separation_contract() {
    let cancelled = predicate_receipt(
        "movement:rotation-cancelled",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let repeated_cancel = predicate_receipt(
        "movement:rotation-cancelled",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let translated_world = predicate_receipt(
        "movement:translated-large-world",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let handle = precision_handle("mb-m6-3");

    let cancelled_receipt = precision_receipt_for(&handle, cancelled);
    let repeated_receipt = precision_receipt_for(&handle, repeated_cancel);
    let translated_receipt = precision_receipt_for(&handle, translated_world);

    assert_eq!(cancelled_receipt.scale_separation_orders(), 21);
    assert_eq!(
        cancelled_receipt.fact_digest(),
        repeated_receipt.fact_digest()
    );
    assert_ne!(
        cancelled_receipt.fact_digest(),
        translated_receipt.fact_digest(),
        "movement/rotation posture must participate in retained precision identity"
    );
}
