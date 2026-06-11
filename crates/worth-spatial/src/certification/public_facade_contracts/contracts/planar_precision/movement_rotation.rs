use worth_spatial::facade::planar_precision::{
    PlanarPrecisionBasis, PlanarPrecisionBasisDenialKind,
};

use super::proof_fixture::predicate_receipt;

#[test]
fn planar_precision_basis_must_match_predicate_receipt_movement_and_rotation() {
    let predicate = predicate_receipt(
        "movement:rotation-cancelled",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let denial = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:micro-feature-local-xy")
        .topology_basis_identity("topology:thin-slot-loop")
        .movement_rotation_posture_identity("movement:tiny-rotation-invalidated")
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect_err("movement/rotation mismatch must deny");

    assert_eq!(
        denial.kind(),
        PlanarPrecisionBasisDenialKind::PredicateBasisMismatch
    );
}
