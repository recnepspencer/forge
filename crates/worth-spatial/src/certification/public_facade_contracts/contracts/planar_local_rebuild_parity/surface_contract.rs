use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityDeclarationFamily,
    PlanarLocalRebuildParityQueryDomain, PlanarLocalRebuildScope,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn planar_local_rebuild_parity_public_surface_is_readable_and_registered() {
    let family = <PlanarLocalRebuildParityDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        PlanarLocalRebuildParityQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "PlanarLocalRebuildParity");

    let _ = PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "surface-local-rebuild-scope",
    ))
    .certify_same_planar_basis_across_views();

    let support_matrix = geometry_public_support_matrix();
    let support = support_matrix
        .row_for_surface(GeometryPublicSurface::PlanarLocalRebuildParity)
        .expect("local rebuild parity support row");
    assert_eq!(
        support.declared_family_key(),
        Some("PlanarLocalRebuildParity")
    );
    assert_eq!(
        support.admission_rule(),
        "support comes from admitted local planar rebuild parity certification consuming grouped neighborhood replacement, rebinding continuity, retained/projection-consumed facts, motion, recovery, and diagnostics without broad search"
    );

    let applicability = geometry_applicability_matrix();
    for concern in [
        GeometryRuntimeConcern::GroupedNeighborhoodWorkflow,
        GeometryRuntimeConcern::ContributionComposition,
        GeometryRuntimeConcern::LowerRuntimeRouting,
        GeometryRuntimeConcern::ProjectionConsumption,
        GeometryRuntimeConcern::SignalContinuation,
        GeometryRuntimeConcern::HistoricalInspection,
        GeometryRuntimeConcern::BranchLocalInspection,
        GeometryRuntimeConcern::ReplayParity,
        GeometryRuntimeConcern::BooleanReadinessCertification,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarLocalRebuildParity, concern)
                .expect("required local rebuild parity row")
                .status(),
            GeometryApplicabilityStatus::RequiredNow
        );
    }
}
