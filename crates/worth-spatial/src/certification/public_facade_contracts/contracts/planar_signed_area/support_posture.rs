use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn certified_signed_area_surface_is_registered_with_query_posture() {
    let support = geometry_public_support_matrix();
    let row = support
        .row_for_surface(GeometryPublicSurface::CertifiedSignedArea2D)
        .expect("signed area support row");
    assert_eq!(row.declared_family_key(), Some("CertifiedSignedArea2D"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::CertifiedSignedArea2D,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("routing row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::CertifiedSignedArea2D,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
