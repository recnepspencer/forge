use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn planar_local_frame_certificate_is_admitted_in_geometry_support_matrix() {
    let matrix = geometry_public_support_matrix();
    let row = matrix
        .row_for_surface(GeometryPublicSurface::PlanarLocalFrameCertificate)
        .expect("local-frame support row");

    assert_eq!(
        row.declared_family_key(),
        Some("PlanarLocalFrameCertificate")
    );
    assert!(row
        .admission_rule()
        .contains("retained precision basis and transform posture"));
}

#[test]
fn planar_local_frame_certificate_support_is_gated_for_recovery_and_mutation() {
    let matrix = geometry_applicability_matrix();

    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PlanarLocalFrameCertificate,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("route row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PlanarLocalFrameCertificate,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PlanarLocalFrameCertificate,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("mutation row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
