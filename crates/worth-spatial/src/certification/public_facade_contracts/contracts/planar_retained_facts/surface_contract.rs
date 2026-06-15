use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsDeclarationFamily, RetainedPlanarFactsQueryDomain,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_readable_retained_planar_facts_surface() {
    let family = <RetainedPlanarFactsDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        RetainedPlanarFactsQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "RetainedPlanarFacts");
    assert!(std::any::type_name::<RetainedPlanarFacts>().contains("RetainedPlanarFacts"));
}

#[test]
fn retained_planar_facts_are_registered_with_query_support_posture() {
    let support = geometry_public_support_matrix()
        .row_for_surface(GeometryPublicSurface::RetainedPlanarFacts)
        .expect("retained planar facts support row")
        .clone();
    assert_eq!(support.declared_family_key(), Some("RetainedPlanarFacts"));
    assert!(support.admission_rule().contains("movement/rotation"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::RetainedPlanarFacts,
                GeometryRuntimeConcern::HistoricalInspection,
            )
            .expect("historical inspection row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::RetainedPlanarFacts,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
