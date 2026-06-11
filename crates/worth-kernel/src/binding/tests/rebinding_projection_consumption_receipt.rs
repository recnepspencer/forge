use crate::binding::tests::support::{admitted_rebinding_handle, face_surface_rebinding_fixture};
use worth_spatial::facade::bindings::primitive_rebinding_retained_fact_source;
use worth_spatial::facade::projection::{
    geometry_projection_consumption_entry, primitive_rebinding_geometry_projection_consumption,
    GeometryProjectedFactKind, GeometryProjectionConsumptionDeclarationFamily,
};

#[test]
fn rebinding_geometry_projection_consumption_receipt_preserves_family_owned_projection_truth() {
    let fixture = face_surface_rebinding_fixture();
    let handle = admitted_rebinding_handle("rebinding-geometry-projection-consumption");
    let retained_source = primitive_rebinding_retained_fact_source(&fixture.declaration, &handle)
        .expect("retained fact source");
    let projection_entry = geometry_projection_consumption_entry(retained_source.clone());
    let receipt = primitive_rebinding_geometry_projection_consumption(&projection_entry, &handle)
        .expect("projection consumption receipt");
    let rebinding_receipt = retained_source.receipt().clone();
    let _: GeometryProjectionConsumptionDeclarationFamily =
        GeometryProjectionConsumptionDeclarationFamily;

    assert_eq!(
        receipt.projected_fact_kind(),
        GeometryProjectedFactKind::PrimitiveRebindingProjectionFact
    );
    assert_eq!(
        receipt.source_family(),
        rebinding_receipt.neighborhood_family()
    );
    assert_eq!(
        receipt.projection_contract_identity(),
        "worth.spatial.rebinding.geometry_projection_consumption"
    );
    assert!(receipt.materialization_basis_digest().is_none());
    assert!(!receipt.source_receipt_digest().is_empty());
    assert!(!receipt.projection_digest().is_empty());
}
