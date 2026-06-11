use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn certified_plane_projection_is_admitted_in_geometry_support_matrix() {
    let matrix = geometry_public_support_matrix();
    let row = matrix
        .row_for_surface(GeometryPublicSurface::ProjectPointToCertifiedPlane2D)
        .expect("projection support row");

    assert_eq!(
        row.declared_family_key(),
        Some("ProjectPointToCertifiedPlane2D")
    );
    assert!(row
        .admission_rule()
        .contains("retained local-frame certificates"));
}

#[test]
fn certified_plane_projection_support_requires_query_and_gates_consumption() {
    let matrix = geometry_applicability_matrix();

    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::ProjectPointToCertifiedPlane2D,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("route row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::ProjectPointToCertifiedPlane2D,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("mutation row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::ProjectPointToCertifiedPlane2D,
                GeometryRuntimeConcern::ProjectionConsumption,
            )
            .expect("consumption row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::ProjectPointToCertifiedPlane2D,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
