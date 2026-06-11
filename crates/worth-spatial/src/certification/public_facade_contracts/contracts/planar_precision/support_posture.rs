use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn planar_precision_certification_support_posture_is_explicit() {
    let matrix = geometry_applicability_matrix();

    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PlanarPrecisionCertification,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("route row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PlanarPrecisionCertification,
                GeometryRuntimeConcern::HistoricalInspection,
            )
            .expect("historical row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        matrix
            .row(
                GeometryPublicSurface::PlanarPrecisionCertification,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
