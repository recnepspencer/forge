use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn coplanar_overlap_extractor_is_registered_with_non_mutating_support_posture() {
    let support = geometry_public_support_matrix();
    let row = support
        .row_for_surface(GeometryPublicSurface::CoplanarOverlapContractExtractor)
        .expect("overlap support row");
    assert_eq!(
        row.declared_family_key(),
        Some("CoplanarOverlapContractExtractor")
    );
    assert!(row.admission_rule().contains("without imprinting"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::CoplanarOverlapContractExtractor,
                GeometryRuntimeConcern::ProjectionConsumption
            )
            .expect("projection consumption row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::CoplanarOverlapContractExtractor,
                GeometryRuntimeConcern::MutationEvidence
            )
            .expect("mutation row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
