use worth_spatial::facade::planar_local_frame::{
    PlanarLocalFrameBasis, PlanarLocalFrameDenialKind,
};

use super::proof_fixture::{precision_handle, precision_receipt};

fn valid_builder(
    precision: &worth_spatial::facade::planar_precision::PlanarPrecisionCertificateReceipt,
    movement_rotation: &'static str,
) -> worth_spatial::facade::planar_local_frame::PlanarLocalFrameBasisBuilder {
    PlanarLocalFrameBasis::builder()
        .frame_identity("frame:micro-feature-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:move-rotate-cancelled")
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .precision_receipt(precision)
}

#[test]
fn planar_local_frame_denies_non_finite_origin() {
    let precision = precision_receipt(&precision_handle("origin"), "movement:rotation-cancelled");
    let denial = valid_builder(&precision, "movement:rotation-cancelled")
        .origin([f64::INFINITY, 0.0, 0.0])
        .build()
        .expect_err("non-finite origin must deny");

    assert_eq!(denial.kind(), PlanarLocalFrameDenialKind::NonFiniteOrigin);
}

#[test]
fn planar_local_frame_denies_zero_and_non_finite_normal() {
    let precision = precision_receipt(&precision_handle("normal"), "movement:rotation-cancelled");
    let zero = valid_builder(&precision, "movement:rotation-cancelled")
        .normal([0.0, 0.0, 0.0])
        .build()
        .expect_err("zero normal must deny");
    let non_finite = valid_builder(&precision, "movement:rotation-cancelled")
        .normal([0.0, f64::NAN, 1.0])
        .build()
        .expect_err("non-finite normal must deny");

    assert_eq!(zero.kind(), PlanarLocalFrameDenialKind::InvalidNormal);
    assert_eq!(non_finite.kind(), PlanarLocalFrameDenialKind::InvalidNormal);
}

#[test]
fn planar_local_frame_denies_invalid_normal_before_precision_mismatch() {
    let precision = precision_receipt(
        &precision_handle("normal-before-mismatch"),
        "movement:rotation-cancelled",
    );
    let denial = valid_builder(&precision, "movement:drifted")
        .normal([0.0, 0.0, 0.0])
        .build()
        .expect_err("invalid local-frame normal must deny before receipt alignment");

    assert_eq!(denial.kind(), PlanarLocalFrameDenialKind::InvalidNormal);
}

#[test]
fn planar_local_frame_denies_missing_transform_and_precision_receipt() {
    let precision = precision_receipt(&precision_handle("missing"), "movement:rotation-cancelled");
    let missing_transform = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:micro-feature-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .movement_rotation_posture_identity("movement:rotation-cancelled")
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .precision_receipt(&precision)
        .build()
        .expect_err("missing transform-chain digest must deny");
    let missing_precision = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:micro-feature-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:move-rotate-cancelled")
        .movement_rotation_posture_identity("movement:rotation-cancelled")
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .build()
        .expect_err("missing precision receipt must deny");

    assert_eq!(
        missing_transform.kind(),
        PlanarLocalFrameDenialKind::MissingTransformChainDigest
    );
    assert_eq!(
        missing_precision.kind(),
        PlanarLocalFrameDenialKind::MissingPrecisionReceipt
    );
}

#[test]
fn planar_local_frame_denies_precision_receipt_mismatch() {
    let precision = precision_receipt(&precision_handle("mismatch"), "movement:rotation-cancelled");
    let denial = valid_builder(&precision, "movement:drifted")
        .build()
        .expect_err("movement mismatch must deny");

    assert_eq!(
        denial.kind(),
        PlanarLocalFrameDenialKind::PrecisionBasisMismatch
    );
}

#[test]
fn planar_local_frame_certificate_changes_when_semantic_rotation_exits_planar_class() {
    let precision = precision_receipt(
        &precision_handle("invalidated"),
        "movement:semantic-rotation-invalidated",
    );
    let denial = valid_builder(&precision, "movement:semantic-rotation-invalidated")
        .build()
        .expect_err("invalidated movement posture must deny");

    assert_eq!(
        denial.kind(),
        PlanarLocalFrameDenialKind::SemanticRotationInvalidatedPlanarClass
    );
}
