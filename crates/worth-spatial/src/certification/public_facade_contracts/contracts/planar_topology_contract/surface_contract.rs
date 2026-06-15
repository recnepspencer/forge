use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessDeclarationFamily,
    PlanarTopologyContractCompletenessQueryDomain,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_readable_topology_contract_surface() {
    let family =
        <PlanarTopologyContractCompletenessDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
            PlanarTopologyContractCompletenessQueryDomain,
        >>::semantic_family_key();
    assert_eq!(family, "PlanarTopologyContractCompleteness");
    assert!(std::any::type_name::<PlanarTopologyContractCompleteness>()
        .contains("PlanarTopologyContractCompleteness"));
}

#[test]
fn planar_topology_contract_completeness_is_registered_with_query_support_posture() {
    let support = geometry_public_support_matrix()
        .row_for_surface(GeometryPublicSurface::PlanarTopologyContractCompleteness)
        .expect("topology contract support row")
        .clone();
    assert_eq!(
        support.declared_family_key(),
        Some("PlanarTopologyContractCompleteness")
    );
    assert!(support.admission_rule().contains("topology-to-spatial"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarTopologyContractCompleteness,
                GeometryRuntimeConcern::ProjectionConsumption,
            )
            .expect("projection consumption row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarTopologyContractCompleteness,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
