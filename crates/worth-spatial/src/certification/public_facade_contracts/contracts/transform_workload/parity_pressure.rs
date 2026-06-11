use super::contract_subject::{acceptance_transform_sequence, projected_cube_workload};
use worth_spatial::facade::transform_workload::{
    RotationTurn, TransformEvidenceKind, TransformParityKind, TransformReorientation,
    TransformSequence, TransformWorkload, VectorDelta,
};

#[test]
fn transform_parity_separates_cancellation_convergence_from_semantic_divergence() {
    let transformed = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform parity cancellation",
    ))
    .declared("cancellation parity")
    .with_transform_sequence(acceptance_transform_sequence())
    .transform()
    .expect("accepted cancellation should transform");

    assert!(transformed.parity_report().has_equivalent_convergence());
    assert!(transformed.parity_report().has_semantic_divergence());
    assert_eq!(transformed.receipts().counters().parity_rows(), 2);
    assert!(transformed
        .parity_report()
        .rows()
        .iter()
        .any(
            |row| row.kind() == TransformParityKind::EquivalentConvergence
                && row.reason()
                    == "exact cancellation replay converges when transform basis is replayed"
        ));
    assert!(transformed
        .parity_report()
        .rows()
        .iter()
        .any(|row| row.kind() == TransformParityKind::SemanticDivergence
            && row.reason() == "non-identity transform changes coordinate or posture evidence"));
}

#[test]
fn transform_parity_reports_divergence_without_cancellation_convergence() {
    let transformed = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform parity divergence only",
    ))
    .declared("ordinary transform divergence")
    .with_transform_sequence(
        TransformSequence::new()
            .translate(VectorDelta::xy(2, 3))
            .rotate(RotationTurn::HalfTurn),
    )
    .transform()
    .expect("ordinary non-identity transform should admit");

    assert!(!transformed.parity_report().has_equivalent_convergence());
    assert!(transformed.parity_report().has_semantic_divergence());
    assert_eq!(transformed.receipts().counters().parity_rows(), 1);
    assert_eq!(transformed.receipts().counters().cancellation_steps(), 0);
}

#[test]
fn transform_parity_accepts_hostile_catalog_cancellation_profile() {
    let transformed = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform parity hostile cancellation",
    ))
    .declared("hostile cancellation parity")
    .with_transform_sequence(
        TransformSequence::new()
            .translate(VectorDelta::xy(-5, 8))
            .rotate(RotationTurn::QuarterTurnCounterClockwise)
            .cancel_with_exact_replay(64),
    )
    .transform()
    .expect("hostile catalog cancellation should admit");

    assert!(transformed.parity_report().has_equivalent_convergence());
    assert!(transformed.parity_report().has_semantic_divergence());
    assert_eq!(transformed.receipts().counters().cancellation_steps(), 64);
    assert_eq!(transformed.receipts().counters().evidence_rows(), 3);
}

#[test]
fn transform_parity_treats_reorientation_as_posture_not_coordinate_motion() {
    let transformed = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform parity reorientation only",
    ))
    .declared("reorientation posture divergence")
    .with_transform_sequence(
        TransformSequence::new().reorient(TransformReorientation::preserves_handedness()),
    )
    .transform()
    .expect("reorientation posture evidence should admit");

    assert_eq!(transformed.evidence().changed_coordinate_rows(), 0);
    assert_eq!(
        transformed.evidence().rows()[0].kind(),
        TransformEvidenceKind::PostureChange
    );
    assert!(!transformed.parity_report().has_equivalent_convergence());
    assert!(transformed.parity_report().has_semantic_divergence());
    assert_eq!(
        transformed.receipts().counters().changed_coordinate_rows(),
        0
    );
    assert_eq!(transformed.receipts().counters().evidence_rows(), 1);
}
