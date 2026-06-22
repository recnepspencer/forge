use topology::facade::TopologySeed;
use worth_spatial::facade::surface_support::{
    SurfaceFamily, SurfaceSupportStatus, SurfaceSupportWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};
use worth_spatial::facade::workload_vocabulary::SpatialWorkloadStage;

#[test]
fn surface_support_workload_certifies_planes_and_preserves_binding_receipts() {
    let bound_geometry = bound_cube_geometry("surface support plane certification");
    let geometry_receipt_identity = bound_geometry
        .receipts()
        .stage_identity()
        .receipt_identity();
    let topology_query_surface = bound_geometry
        .receipts()
        .topology_query_surface()
        .to_string();
    let upstream_carriers = bound_geometry.receipts().counters().geometry_carriers();

    let support = SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared("certify cube plane support")
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("plane support should certify");

    assert_eq!(
        support.receipts().stage_identity().stage(),
        SpatialWorkloadStage::SurfaceSupport
    );
    assert_eq!(
        support
            .receipts()
            .stage_receipt()
            .identity()
            .upstream_receipt(),
        geometry_receipt_identity
    );
    assert_eq!(
        support.receipts().upstream_geometry_binding_identity(),
        geometry_receipt_identity
    );
    assert_eq!(
        support
            .certified_plane_support()
            .upstream_geometry_binding_identity(),
        geometry_receipt_identity
    );
    assert_eq!(
        support.certified_plane_support().topology_query_surface(),
        topology_query_surface
    );
    assert_eq!(
        support.certified_plane_support().family(),
        SurfaceFamily::Plane
    );
    assert!(support.can_enter_local_frame_workload());
    assert!(support.can_enter_projection_workload());
    assert!(!support.can_enter_operator_execution());

    assert_eq!(support.receipts().counters().classified_families(), 5);
    assert_eq!(support.receipts().counters().certified_planes(), 1);
    assert_eq!(support.receipts().counters().unsupported_families(), 4);
    assert_eq!(
        support.receipts().counters().upstream_geometry_carriers(),
        upstream_carriers
    );
    assert!(support.receipts().matrix_rows().iter().any(|row| {
        row.family() == SurfaceFamily::Plane && row.status() == SurfaceSupportStatus::Certified
    }));
}

#[test]
fn surface_support_workload_rejects_missing_declaration_and_family() {
    let missing_declaration = SurfaceSupportWorkload::for_bound_geometry(bound_cube_geometry(
        "surface support missing declaration",
    ))
    .declared("")
    .with_surface_family(SurfaceFamily::Plane)
    .certify()
    .expect_err("blank declaration cannot certify surface support");

    assert_eq!(
        missing_declaration.reason_code(),
        worth_spatial::facade::surface_support::UnsupportedSurfaceSupportReasonCode::MissingDeclaration
    );
    assert!(missing_declaration
        .human_reason()
        .contains("human-readable declaration"));
    assert!(missing_declaration.receipt().is_none());
    assert!(!missing_declaration.can_enter_projection_workload());

    let missing_family = SurfaceSupportWorkload::for_bound_geometry(bound_cube_geometry(
        "surface support missing family",
    ))
    .declared("try missing family")
    .certify()
    .expect_err("surface support requires explicit family classification");

    assert_eq!(
        missing_family.reason_code(),
        worth_spatial::facade::surface_support::UnsupportedSurfaceSupportReasonCode::MissingSurfaceFamily
    );
    assert_eq!(
        missing_family.human_reason(),
        "Surface support requires an explicit surface family."
    );
    let receipt = missing_family
        .receipt()
        .expect("missing family should still preserve the surface support declaration receipt");
    assert_eq!(
        receipt.stage_identity().stage(),
        SpatialWorkloadStage::SurfaceSupport
    );
    assert_eq!(receipt.stage_identity().declaration(), "try missing family");
    assert_eq!(receipt.family(), None);
    assert_eq!(
        receipt.envelope().posture().reason(),
        "Surface support requires an explicit surface family."
    );
    assert!(!missing_family.can_enter_local_frame_workload());
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
