use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundary, PlanarCleanFailBoundaryDeclarationFamily,
    PlanarCleanFailBoundaryQueryDomain, PlanarCleanFailInput,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_readable_planar_clean_fail_boundary_surface() {
    let family = <PlanarCleanFailBoundaryDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        PlanarCleanFailBoundaryQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "PlanarCleanFailBoundary");
    let boundary = PlanarCleanFailBoundary::from_planar_input(
        PlanarCleanFailInput::dirty_planar_loop("surface:dirty"),
    );
    let _ = boundary;
}

#[test]
fn planar_clean_fail_boundary_is_registered_with_query_support_posture() {
    let support = geometry_public_support_matrix()
        .row_for_surface(GeometryPublicSurface::PlanarCleanFailBoundary)
        .expect("clean-fail boundary support row")
        .clone();
    assert_eq!(
        support.declared_family_key(),
        Some("PlanarCleanFailBoundary")
    );
    assert_eq!(
        support.admission_rule(),
        "support comes from admitted planar clean-fail boundary certification consuming admission, movement/rotation, recovery, and diagnostics while proving no repair or bounded conversion was attempted"
    );

    let applicability = geometry_applicability_matrix();
    for concern in [
        GeometryRuntimeConcern::LowerRuntimeRouting,
        GeometryRuntimeConcern::RecoveryAction,
        GeometryRuntimeConcern::ProjectionConsumption,
        GeometryRuntimeConcern::HistoricalInspection,
        GeometryRuntimeConcern::BranchLocalInspection,
        GeometryRuntimeConcern::BooleanReadinessCertification,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarCleanFailBoundary, concern)
                .expect("required clean-fail applicability row")
                .status(),
            GeometryApplicabilityStatus::RequiredNow
        );
    }
    for concern in [
        GeometryRuntimeConcern::GroupedNeighborhoodWorkflow,
        GeometryRuntimeConcern::ContributionComposition,
        GeometryRuntimeConcern::MutationEvidence,
        GeometryRuntimeConcern::SignalContinuation,
        GeometryRuntimeConcern::ReplayParity,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarCleanFailBoundary, concern)
                .expect("denied clean-fail applicability row")
                .status(),
            GeometryApplicabilityStatus::DeniedForThisRuntime
        );
    }
}
