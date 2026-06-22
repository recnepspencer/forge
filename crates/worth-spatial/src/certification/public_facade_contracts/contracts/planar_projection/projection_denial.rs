use worth_spatial::facade::planar_projection::{
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DDenialBasisLocus,
    ProjectPointToCertifiedPlane2DDenialKind,
};

use super::proof_fixture::certified_frame;

#[test]
fn certified_plane_projection_denies_off_plane_or_missing_basis_before_predicates() {
    let missing_receipt = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("point:missing-frame")
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:thin-slot-local-normalized")
        .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
        .movement_rotation_posture_identity("movement:rotation-cancelled")
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .build()
        .expect_err("projection cannot synthesize a missing local-frame receipt");
    assert_eq!(
        missing_receipt.kind(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingLocalFrameReceipt
    );
    assert_eq!(
        missing_receipt.basis_locus(),
        ProjectPointToCertifiedPlane2DDenialBasisLocus::LocalFrameReceipt
    );

    let frame = certified_frame(
        "projection-denial-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let off_plane = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("point:off-plane")
        .source_point([1.0e12, 0.0, 1.0e-12])
        .source_point_basis_digest("point-basis:thin-slot-local-normalized")
        .local_delta_from_frame_origin([1.0e-9, 0.0, 1.0e-12])
        .local_frame_receipt(&frame)
        .build()
        .expect_err("projection must deny closest-point repair");
    assert_eq!(
        off_plane.kind(),
        ProjectPointToCertifiedPlane2DDenialKind::OffPlanePoint
    );
    assert_eq!(
        off_plane.basis_locus(),
        ProjectPointToCertifiedPlane2DDenialBasisLocus::PlaneDistance
    );
}

#[test]
fn certified_plane_projection_denies_nonfinite_and_mismatched_frame_basis() {
    let frame = certified_frame(
        "projection-nonfinite-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let nonfinite_source = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("point:nonfinite")
        .source_point([f64::NAN, 0.0, 0.0])
        .source_point_basis_digest("point-basis:thin-slot-local-normalized")
        .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
        .local_frame_receipt(&frame)
        .build()
        .expect_err("nonfinite source point denied");
    assert_eq!(
        nonfinite_source.kind(),
        ProjectPointToCertifiedPlane2DDenialKind::NonFiniteSourcePoint
    );
    assert_eq!(
        nonfinite_source.basis_locus(),
        ProjectPointToCertifiedPlane2DDenialBasisLocus::SourcePoint
    );

    let nonfinite_delta = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("point:nonfinite-delta")
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:thin-slot-local-normalized")
        .local_delta_from_frame_origin([f64::INFINITY, 0.0, 0.0])
        .local_frame_receipt(&frame)
        .build()
        .expect_err("nonfinite local delta denied");
    assert_eq!(
        nonfinite_delta.kind(),
        ProjectPointToCertifiedPlane2DDenialKind::NonFiniteLocalDelta
    );
    assert_eq!(
        nonfinite_delta.basis_locus(),
        ProjectPointToCertifiedPlane2DDenialBasisLocus::LocalDelta
    );

    let mismatched_transform = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("point:mismatched-transform")
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:thin-slot-local-normalized")
        .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
        .local_frame_receipt(&frame)
        .transform_chain_digest("transform:forged-after-receipt")
        .build()
        .expect_err("frame snapshot mismatch denied");
    assert_eq!(
        mismatched_transform.kind(),
        ProjectPointToCertifiedPlane2DDenialKind::FrameBasisMismatch
    );
    assert_eq!(
        mismatched_transform.basis_locus(),
        ProjectPointToCertifiedPlane2DDenialBasisLocus::FrameBasis
    );
}

#[test]
fn certified_plane_projection_denial_basis_loci_cover_missing_semantic_inputs() {
    let frame = certified_frame(
        "projection-missing-input-frame",
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );

    assert_missing_input_locus(
        ProjectPointToCertifiedPlane2DBasis::builder()
            .source_point([1.0e12, 0.0, 0.0])
            .source_point_basis_digest("point-basis:thin-slot-local-normalized")
            .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
            .local_frame_receipt(&frame)
            .build(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingSourcePointIdentity,
        ProjectPointToCertifiedPlane2DDenialBasisLocus::SourcePointIdentity,
    );
    assert_missing_input_locus(
        ProjectPointToCertifiedPlane2DBasis::builder()
            .source_point_identity("point:missing-basis")
            .source_point([1.0e12, 0.0, 0.0])
            .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
            .local_frame_receipt(&frame)
            .build(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingSourcePointBasisDigest,
        ProjectPointToCertifiedPlane2DDenialBasisLocus::SourcePointBasisDigest,
    );
    assert_missing_input_locus(
        ProjectPointToCertifiedPlane2DBasis::builder()
            .source_point_identity("point:missing-movement")
            .source_point([1.0e12, 0.0, 0.0])
            .source_point_basis_digest("point-basis:thin-slot-local-normalized")
            .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
            .local_frame_receipt(&frame)
            .movement_rotation_posture_identity("")
            .build(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingMovementRotationPostureIdentity,
        ProjectPointToCertifiedPlane2DDenialBasisLocus::MovementRotationPosture,
    );
    assert_missing_input_locus(
        ProjectPointToCertifiedPlane2DBasis::builder()
            .source_point_identity("point:missing-tolerance")
            .source_point([1.0e12, 0.0, 0.0])
            .source_point_basis_digest("point-basis:thin-slot-local-normalized")
            .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
            .local_frame_receipt(&frame)
            .tolerance_policy_identity("")
            .build(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingTolerancePolicyIdentity,
        ProjectPointToCertifiedPlane2DDenialBasisLocus::TolerancePolicy,
    );
}

fn assert_missing_input_locus(
    result: Result<
        ProjectPointToCertifiedPlane2DBasis,
        worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DDenial,
    >,
    expected_kind: ProjectPointToCertifiedPlane2DDenialKind,
    expected_locus: ProjectPointToCertifiedPlane2DDenialBasisLocus,
) {
    let denial = result.expect_err("missing projection input must deny");
    assert_eq!(denial.kind(), expected_kind);
    assert_eq!(denial.basis_locus(), expected_locus);
}
