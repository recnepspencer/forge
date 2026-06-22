use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsDeclarationFamily,
    ProjectionConsumedPlanarFactsQueryDomain,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_readable_projection_consumed_planar_facts_surface() {
    let family =
        <ProjectionConsumedPlanarFactsDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
            ProjectionConsumedPlanarFactsQueryDomain,
        >>::semantic_family_key();
    assert_eq!(family, "ProjectionConsumedPlanarFacts");
    assert!(std::any::type_name::<ProjectionConsumedPlanarFacts>()
        .contains("ProjectionConsumedPlanarFacts"));
}

#[test]
fn projection_consumed_planar_facts_are_registered_with_query_support_posture() {
    let support = geometry_public_support_matrix()
        .row_for_surface(GeometryPublicSurface::ProjectionConsumedPlanarFacts)
        .expect("projection-consumed planar facts support row")
        .clone();
    assert_eq!(
        support.declared_family_key(),
        Some("ProjectionConsumedPlanarFacts")
    );
    assert_eq!(
        support.admission_rule(),
        "support comes from admitted projection-consumed planar fact certification consuming retained planar facts and exact bundle projection receipts for downstream boolean-readiness without payload spelunking or recomputation"
    );

    let applicability = geometry_applicability_matrix();
    for concern in [
        GeometryRuntimeConcern::LowerRuntimeRouting,
        GeometryRuntimeConcern::HistoricalInspection,
        GeometryRuntimeConcern::BranchLocalInspection,
        GeometryRuntimeConcern::ProjectionConsumption,
        GeometryRuntimeConcern::BooleanReadinessCertification,
    ] {
        assert_eq!(
            applicability
                .row(
                    GeometryPublicSurface::ProjectionConsumedPlanarFacts,
                    concern
                )
                .expect("required projection-consumed applicability row")
                .status(),
            GeometryApplicabilityStatus::RequiredNow
        );
    }
    for concern in [
        GeometryRuntimeConcern::MutationEvidence,
        GeometryRuntimeConcern::RecoveryAction,
    ] {
        assert_eq!(
            applicability
                .row(
                    GeometryPublicSurface::ProjectionConsumedPlanarFacts,
                    concern
                )
                .expect("denied projection-consumed applicability row")
                .status(),
            GeometryApplicabilityStatus::DeniedForThisRuntime
        );
    }
}
