use worth_spatial::facade::planar_precision::{
    PlanarPrecisionBasis, PlanarPrecisionBasisDenialKind,
};

use super::proof_fixture::predicate_receipt;

#[test]
fn planar_precision_escalation_denies_when_required_basis_is_missing() {
    let predicate = predicate_receipt(
        "movement:identity",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );

    assert_basis_denial(
        PlanarPrecisionBasis::builder()
            .local_frame_identity("frame:micro-feature-local-xy")
            .topology_basis_identity("topology:thin-slot-loop")
            .movement_rotation_posture_identity("movement:identity")
            .tolerance_policy_identity("tolerance:micro-feature-exact")
            .local_feature_scale_order(-9)
            .world_magnitude_order(12)
            .normalization_scale(1.0e-9)
            .build(),
        PlanarPrecisionBasisDenialKind::MissingPredicateReceipt,
    );
    assert_basis_denial(
        PlanarPrecisionBasis::builder()
            .local_frame_identity("frame:micro-feature-local-xy")
            .topology_basis_identity("topology:thin-slot-loop")
            .movement_rotation_posture_identity("movement:identity")
            .tolerance_policy_identity("tolerance:micro-feature-exact")
            .world_magnitude_order(12)
            .normalization_scale(1.0e-9)
            .predicate_receipt(&predicate)
            .build(),
        PlanarPrecisionBasisDenialKind::MissingLocalFeatureScaleOrder,
    );
    assert_basis_denial(
        PlanarPrecisionBasis::builder()
            .local_frame_identity("frame:micro-feature-local-xy")
            .topology_basis_identity("topology:thin-slot-loop")
            .movement_rotation_posture_identity("movement:identity")
            .tolerance_policy_identity("tolerance:micro-feature-exact")
            .local_feature_scale_order(-9)
            .normalization_scale(1.0e-9)
            .predicate_receipt(&predicate)
            .build(),
        PlanarPrecisionBasisDenialKind::MissingWorldMagnitudeOrder,
    );
    assert_basis_denial(
        PlanarPrecisionBasis::builder()
            .local_frame_identity("frame:micro-feature-local-xy")
            .topology_basis_identity("topology:thin-slot-loop")
            .movement_rotation_posture_identity("movement:identity")
            .tolerance_policy_identity("tolerance:micro-feature-exact")
            .local_feature_scale_order(12)
            .world_magnitude_order(-9)
            .normalization_scale(1.0e12)
            .predicate_receipt(&predicate)
            .build(),
        PlanarPrecisionBasisDenialKind::ContradictoryScaleSeparation,
    );
    assert_basis_denial(
        PlanarPrecisionBasis::builder()
            .local_frame_identity("frame:micro-feature-local-xy")
            .topology_basis_identity("topology:thin-slot-loop")
            .movement_rotation_posture_identity("movement:identity")
            .tolerance_policy_identity("tolerance:micro-feature-exact")
            .local_feature_scale_order(-9)
            .world_magnitude_order(12)
            .normalization_scale(f64::NAN)
            .predicate_receipt(&predicate)
            .build(),
        PlanarPrecisionBasisDenialKind::InvalidNormalizationScale,
    );
    assert_basis_denial(
        PlanarPrecisionBasis::builder()
            .local_frame_identity("frame:micro-feature-local-xy")
            .topology_basis_identity("topology:thin-slot-loop")
            .movement_rotation_posture_identity("movement:identity")
            .tolerance_policy_identity("tolerance:micro-feature-exact")
            .local_feature_scale_order(-9)
            .world_magnitude_order(12)
            .normalization_scale(1.0)
            .predicate_receipt(&predicate)
            .build(),
        PlanarPrecisionBasisDenialKind::NormalizationScaleLocalFeatureMismatch,
    );
}

fn assert_basis_denial(
    result: Result<
        PlanarPrecisionBasis,
        worth_spatial::facade::planar_precision::PlanarPrecisionBasisDenial,
    >,
    expected: PlanarPrecisionBasisDenialKind,
) {
    let denial = result.expect_err("basis must be denied");
    assert_eq!(denial.kind(), expected);
}
