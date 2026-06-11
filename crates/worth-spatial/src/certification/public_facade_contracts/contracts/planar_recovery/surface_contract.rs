use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureDeclarationFamily,
    PlanarRecoveryPostureQueryDomain, PlanarRecoverySource,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_readable_planar_recovery_surface() {
    let family = <PlanarRecoveryPostureDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        PlanarRecoveryPostureQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "PlanarRecoveryPosture");
    let surface_plan = PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::dirty_input("surface:dirty-planar-input"),
    )
    .prepare_next_step();
    let _ = surface_plan;
}

#[test]
fn planar_recovery_posture_is_registered_with_query_support_posture() {
    let support = geometry_public_support_matrix()
        .row_for_surface(GeometryPublicSurface::PlanarRecoveryPosture)
        .expect("planar recovery support row")
        .clone();
    assert_eq!(support.declared_family_key(), Some("PlanarRecoveryPosture"));
    assert_eq!(
        support.admission_rule(),
        "support comes from admitted planar recovery posture certification consuming typed planar blockers and basis receipts to produce next-step recovery without changing planar truth"
    );

    let applicability = geometry_applicability_matrix();
    for concern in [
        GeometryRuntimeConcern::LowerRuntimeRouting,
        GeometryRuntimeConcern::RecoveryAction,
        GeometryRuntimeConcern::HistoricalInspection,
        GeometryRuntimeConcern::BranchLocalInspection,
        GeometryRuntimeConcern::BooleanReadinessCertification,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarRecoveryPosture, concern)
                .expect("required planar recovery applicability row")
                .status(),
            GeometryApplicabilityStatus::RequiredNow
        );
    }
    for concern in [
        GeometryRuntimeConcern::MutationEvidence,
        GeometryRuntimeConcern::ReplayParity,
        GeometryRuntimeConcern::ProjectionConsumption,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarRecoveryPosture, concern)
                .expect("denied planar recovery applicability row")
                .status(),
            GeometryApplicabilityStatus::DeniedForThisRuntime
        );
    }
}
