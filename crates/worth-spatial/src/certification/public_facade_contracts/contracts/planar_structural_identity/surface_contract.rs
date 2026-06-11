use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
    PlanarStructuralIdentityDeclarationFamily, PlanarStructuralIdentityQueryDomain,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

use super::contract_subject::{boolean_readiness_receipt, bundle_transform_basis};
use super::runtime_handles::structural_identity_handle;

#[test]
fn spatial_public_facade_exports_readable_structural_identity_surface() {
    let family = <PlanarStructuralIdentityDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        PlanarStructuralIdentityQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "PlanarStructuralIdentity");
    assert!(std::any::type_name::<PlanarStructuralIdentity>().contains("PlanarStructuralIdentity"));
}

#[test]
fn planar_structural_identity_is_registered_with_query_support_posture() {
    let support_matrix = geometry_public_support_matrix();
    let support = support_matrix
        .row_for_surface(GeometryPublicSurface::PlanarStructuralIdentity)
        .expect("structural identity support row");
    assert_eq!(
        support.declared_family_key(),
        Some("PlanarStructuralIdentity")
    );
    assert!(support
        .admission_rule()
        .contains("canonical transform basis"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarStructuralIdentity,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("structural identity routing row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarStructuralIdentity,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("structural identity mutation row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}

#[test]
fn planar_structural_identity_plan_exposes_retained_identity_inspection_breadth() {
    let contracts = PlanarStructuralIdentityContracts::new(structural_identity_handle(
        "structural-plan-inspection",
    ));
    let plan = PlanarStructuralIdentity::from_boolean_readiness(boolean_readiness_receipt(
        "structural-plan-inspection",
    ))
    .with_topology_identity("topology:plan-inspection")
    .with_persistent_name("name:plan-inspection")
    .with_binding_identity("binding:plan-inspection")
    .with_lineage_identity("lineage:plan-inspection")
    .with_canonical_transform_basis(bundle_transform_basis())
    .compile(&contracts)
    .expect("structural identity plan");

    assert_eq!(plan.inspected_identity_rows(), 9);
}
