use topology::facade::TopologySeed;
use worth_spatial::facade::projection_workload::{
    ProjectionWorkload, UnsupportedProjectionReasonCode,
};
use worth_spatial::facade::surface_support::{SurfaceFamily, SurfaceSupportWorkload};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

#[test]
fn projection_workload_blocks_loose_point_operator_inputs() {
    let missing_basis = ProjectionWorkload::for_certified_surface_support(
        certified_surface_support("projection missing basis"),
    )
    .declared("try projection without local frame")
    .project()
    .expect_err("projection requires explicit local frame basis");

    assert_eq!(
        missing_basis.reason_code(),
        UnsupportedProjectionReasonCode::MissingLocalFrameBasis
    );
    assert_eq!(
        missing_basis.human_reason(),
        "Projection workload requires an explicit local frame basis."
    );
    assert!(!missing_basis.can_enter_projection_consumed_planar_facts());
    assert!(!missing_basis.can_enter_operator_execution());

    let missing_declaration = ProjectionWorkload::for_certified_surface_support(
        certified_surface_support("projection missing declaration"),
    )
    .declared("")
    .project()
    .expect_err("projection requires a declaration before receipts");

    assert_eq!(
        missing_declaration.reason_code(),
        UnsupportedProjectionReasonCode::MissingDeclaration
    );
    assert!(missing_declaration
        .human_reason()
        .contains("human-readable declaration"));
}

fn certified_surface_support(
    declaration: &str,
) -> worth_spatial::facade::surface_support::CertifiedSurfaceSupport {
    SurfaceSupportWorkload::for_bound_geometry(bound_cube_geometry(declaration))
        .declared("certify projection support")
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("plane support should certify")
}

fn bound_cube_geometry(declaration: &str) -> BoundGeometryWorkload {
    let topology = TopologySeed::cube()
        .with_declaration(declaration)
        .build()
        .expect("cube topology seed should be admitted");

    GeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("bind {declaration}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar geometry binding should admit")
}
