use super::proof_fixture::{certified_frame, projection_basis, projection_receipt};

#[test]
fn thin_feature_projection_uses_local_delta_not_world_coordinate_subtraction() {
    let frame = certified_frame(
        "projection-scale-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let receipt = projection_receipt(
        "projection-scale",
        projection_basis(&frame, "point:thin-slot-corner-a", [1.0e-9, 0.0, 0.0]),
    );

    assert_eq!(receipt.basis().source_point(), [1.0e12, 0.0, 0.0]);
    assert_eq!(
        receipt.basis().local_delta_from_frame_origin(),
        [1.0e-9, 0.0, 0.0]
    );
    assert_eq!(receipt.point_2d(), [0.0, -1.0e-9]);
}

#[test]
fn mb_m6_1_projection_basis_survives_coplanar_overlap_storm() {
    let frame = certified_frame(
        "projection-overlap-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let points = [
        ("point:overlap-a", [1.0e-9, 0.0, 0.0], [0.0, -1.0e-9]),
        ("point:overlap-b", [0.0, 1.0e-9, 0.0], [1.0e-9, 0.0]),
        ("point:overlap-c", [1.0e-9, 1.0e-9, 0.0], [1.0e-9, -1.0e-9]),
    ];

    for (identity, delta, expected_2d) in points {
        let receipt = projection_receipt(
            "projection-overlap",
            projection_basis(&frame, identity, delta),
        );
        assert_eq!(receipt.point_2d(), expected_2d);
        assert_eq!(receipt.local_frame_fact_digest(), frame.fact_digest());
        assert_eq!(receipt.signed_distance_to_plane_bits(), 0.0f64.to_bits());
        assert_eq!(receipt.counters().basis_digest_part_count(), 19);
    }
}
