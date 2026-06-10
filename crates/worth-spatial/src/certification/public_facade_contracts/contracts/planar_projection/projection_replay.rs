use super::proof_fixture::{certified_frame, projection_basis, projection_receipt};

#[test]
fn certified_plane_projection_replays_to_identical_2d_coordinates_and_basis_digest() {
    let frame = certified_frame(
        "projection-replay-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let basis = projection_basis(&frame, "point:thin-slot-corner-a", [1.0e-9, 0.0, 0.0]);
    let first = projection_receipt("projection-replay", basis.clone());
    let second = projection_receipt("projection-replay", basis);

    assert_eq!(first.point_2d(), [0.0, -1.0e-9]);
    assert_eq!(second.point_2d(), first.point_2d());
    assert_eq!(first.fact_digest(), second.fact_digest());
    assert_eq!(
        first.mutation_evidence().evidence_digest(),
        second.mutation_evidence().evidence_digest()
    );
    assert_eq!(first.local_frame_fact_digest(), frame.fact_digest());
    assert_eq!(
        first.mutation_evidence().source_point_identity(),
        "point:thin-slot-corner-a"
    );
    assert_eq!(
        first.mutation_evidence().local_frame_fact_digest(),
        frame.fact_digest()
    );
    assert_eq!(first.mutation_evidence().fact_digest(), first.fact_digest());
    assert_eq!(
        first.mutation_evidence().declaration_digest(),
        first.declaration_digest()
    );
    assert!(!first.mutation_evidence().evidence_digest().is_empty());
    assert_eq!(first.signed_distance_to_plane_bits(), 0.0f64.to_bits());
    assert_eq!(first.counters().projection_derivations(), 1);
    assert_eq!(first.counters().local_frame_receipts_consumed(), 1);
    assert_eq!(first.counters().local_delta_basis_reads(), 1);
    assert_eq!(first.counters().plane_distance_checks(), 1);
}

#[test]
fn certified_plane_projection_mutation_evidence_changes_with_source_identity() {
    let frame = certified_frame(
        "projection-evidence-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let first = projection_receipt(
        "projection-evidence-a",
        projection_basis(&frame, "point:thin-slot-corner-a", [1.0e-9, 0.0, 0.0]),
    );
    let second = projection_receipt(
        "projection-evidence-b",
        projection_basis(&frame, "point:thin-slot-corner-b", [1.0e-9, 0.0, 0.0]),
    );

    assert_eq!(first.point_2d(), second.point_2d());
    assert_ne!(
        first.mutation_evidence().source_point_identity(),
        second.mutation_evidence().source_point_identity()
    );
    assert_ne!(
        first.mutation_evidence().evidence_digest(),
        second.mutation_evidence().evidence_digest()
    );
    assert_ne!(first.fact_digest(), second.fact_digest());
}

#[test]
fn certified_plane_projection_digest_changes_when_frame_digest_changes() {
    let base_frame = certified_frame(
        "projection-base-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let alternate_frame = certified_frame(
        "projection-alternate-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-alternate",
    );
    let base = projection_receipt(
        "projection-base",
        projection_basis(&base_frame, "point:thin-slot-corner-a", [1.0e-9, 0.0, 0.0]),
    );
    let alternate = projection_receipt(
        "projection-alternate",
        projection_basis(
            &alternate_frame,
            "point:thin-slot-corner-a",
            [1.0e-9, 0.0, 0.0],
        ),
    );

    assert_eq!(base.point_2d(), alternate.point_2d());
    assert_ne!(
        base.local_frame_fact_digest(),
        alternate.local_frame_fact_digest()
    );
    assert_ne!(base.fact_digest(), alternate.fact_digest());
}
