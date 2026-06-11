use topology::facade::TopologySeed;
use worth_spatial::facade::projection_workload::{LocalFrameBasis, ProjectionWorkload};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};
use worth_spatial::facade::workload_vocabulary::SpatialWorkloadStage;

#[test]
fn projection_workload_preserves_topology_binding_and_plane_basis() {
    let bound_geometry = bound_cube_geometry("projection workload preserves basis");
    let expected_faces = bound_geometry.planar_faces().len();
    let expected_edges = bound_geometry.planar_edges().len();
    let expected_loops = bound_geometry.planar_loops().len();

    let surface_support = certified_surface_support(bound_geometry);
    let surface_support_identity = surface_support
        .receipts()
        .stage_identity()
        .receipt_identity();
    let certified_plane_support_identity = surface_support
        .certified_plane_support()
        .upstream_geometry_binding_identity()
        .to_string();
    let topology_query_surface = surface_support
        .certified_plane_support()
        .topology_query_surface()
        .to_string();
    let first_face_topology = surface_support.geometry_snapshot().face_rows()[0]
        .topology_entity_identity()
        .to_string();
    let first_face_carrier = surface_support.geometry_snapshot().face_rows()[0]
        .geometry_carrier_identity()
        .to_string();
    let first_edge_topology = surface_support.geometry_snapshot().edge_rows()[0]
        .topology_entity_identity()
        .to_string();
    let first_edge_carrier = surface_support.geometry_snapshot().edge_rows()[0]
        .geometry_carrier_identity()
        .to_string();
    let first_loop_topology = surface_support.geometry_snapshot().loop_rows()[0]
        .topology_entity_identity()
        .to_string();
    let first_loop_carrier = surface_support.geometry_snapshot().loop_rows()[0]
        .geometry_carrier_identity()
        .to_string();

    let projected = ProjectionWorkload::for_certified_surface_support(surface_support)
        .declared("project cube workload")
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("certified surface support should project");

    assert_eq!(
        projected.receipts().stage_identity().stage(),
        SpatialWorkloadStage::Projection
    );
    assert_eq!(
        projected
            .receipts()
            .stage_receipt()
            .identity()
            .upstream_receipt(),
        surface_support_identity
    );
    assert_eq!(
        projected.receipts().upstream_surface_support_identity(),
        surface_support_identity
    );
    assert_eq!(
        projected.receipts().certified_plane_support_identity(),
        certified_plane_support_identity
    );
    assert_eq!(
        projected.receipts().topology_query_surface(),
        topology_query_surface
    );
    assert_eq!(
        projected
            .receipts()
            .local_frame_receipt()
            .surface_support_identity(),
        surface_support_identity
    );
    assert_eq!(
        projected
            .receipts()
            .projection_consumption_receipt()
            .projection_stage_identity(),
        projected.receipts().stage_identity()
    );
    assert!(projected.can_enter_projection_consumed_planar_facts());
    assert!(!projected.can_enter_operator_execution());

    assert_eq!(projected.projected_faces().len(), expected_faces);
    assert_eq!(projected.projected_edges().edges().len(), expected_edges);
    assert_eq!(projected.projected_loops().len(), expected_loops);
    assert_eq!(
        projected.receipts().counters().projected_faces(),
        expected_faces
    );
    assert_eq!(
        projected.receipts().counters().projected_edges(),
        expected_edges
    );
    assert_eq!(
        projected.receipts().counters().projected_loops(),
        expected_loops
    );
    assert_eq!(projected.receipts().counters().local_basis_parts(), 4);
    assert_eq!(
        projected
            .receipts()
            .projection_consumption_receipt()
            .projected_entity_count(),
        expected_faces + expected_edges + expected_loops
    );

    let first_projected_face = projected.projected_faces()[0].identity();
    assert_eq!(
        first_projected_face.topology_entity_identity(),
        first_face_topology
    );
    assert_eq!(
        first_projected_face.geometry_carrier_identity(),
        first_face_carrier
    );
    assert_eq!(
        first_projected_face.surface_support_identity(),
        surface_support_identity
    );
    assert_eq!(
        first_projected_face.local_basis_identity(),
        projected
            .receipts()
            .local_frame_receipt()
            .local_basis_identity()
    );
    assert_eq!(
        first_projected_face.projected_fact_identity(),
        expected_projected_fact_identity(
            &first_face_topology,
            &first_face_carrier,
            &surface_support_identity,
            projected
                .receipts()
                .local_frame_receipt()
                .local_basis_identity(),
        )
    );

    let first_projected_edge = projected.projected_edges().edges()[0].identity();
    assert_eq!(
        first_projected_edge.topology_entity_identity(),
        first_edge_topology
    );
    assert_eq!(
        first_projected_edge.geometry_carrier_identity(),
        first_edge_carrier
    );
    assert_eq!(
        first_projected_edge.surface_support_identity(),
        surface_support_identity
    );
    assert_eq!(
        first_projected_edge.local_basis_identity(),
        projected
            .receipts()
            .local_frame_receipt()
            .local_basis_identity()
    );
    assert_eq!(
        first_projected_edge.projected_fact_identity(),
        expected_projected_fact_identity(
            &first_edge_topology,
            &first_edge_carrier,
            &surface_support_identity,
            projected
                .receipts()
                .local_frame_receipt()
                .local_basis_identity(),
        )
    );

    let first_projected_loop = projected.projected_loops()[0].identity();
    assert_eq!(
        first_projected_loop.topology_entity_identity(),
        first_loop_topology
    );
    assert_eq!(
        first_projected_loop.geometry_carrier_identity(),
        first_loop_carrier
    );
    assert_eq!(
        first_projected_loop.surface_support_identity(),
        surface_support_identity
    );
    assert_eq!(
        first_projected_loop.local_basis_identity(),
        projected
            .receipts()
            .local_frame_receipt()
            .local_basis_identity()
    );
    assert_eq!(
        first_projected_loop.projected_fact_identity(),
        expected_projected_fact_identity(
            &first_loop_topology,
            &first_loop_carrier,
            &surface_support_identity,
            projected
                .receipts()
                .local_frame_receipt()
                .local_basis_identity(),
        )
    );
}

#[test]
fn projection_workload_separates_same_binding_by_surface_support_identity() {
    let bound_geometry = bound_cube_geometry("projection workload shared binding source");
    let first_projected = project_certified_surface_support(certified_surface_support_named(
        bound_geometry.clone(),
        "first certified surface support",
    ));
    let second_projected = project_certified_surface_support(certified_surface_support_named(
        bound_geometry,
        "second certified surface support",
    ));

    let first_face = first_projected.projected_faces()[0].identity();
    let second_face = second_projected.projected_faces()[0].identity();

    assert_eq!(
        first_face.topology_entity_identity(),
        second_face.topology_entity_identity()
    );
    assert_eq!(
        first_face.geometry_carrier_identity(),
        second_face.geometry_carrier_identity()
    );
    assert_eq!(
        first_face.local_basis_identity(),
        second_face.local_basis_identity()
    );
    assert_ne!(
        first_face.surface_support_identity(),
        second_face.surface_support_identity()
    );
    assert_ne!(
        first_face.projected_fact_identity(),
        second_face.projected_fact_identity()
    );
}

fn expected_projected_fact_identity(
    topology_entity_identity: &str,
    geometry_carrier_identity: &str,
    surface_support_identity: &str,
    local_basis_identity: &str,
) -> String {
    format!(
        "projected-entity:{topology_entity_identity}:{geometry_carrier_identity}:{surface_support_identity}:{local_basis_identity}"
    )
}

fn certified_surface_support(bound_geometry: BoundGeometryWorkload) -> CertifiedSurfaceSupport {
    certified_surface_support_named(bound_geometry, "certify projection plane support")
}

fn certified_surface_support_named(
    bound_geometry: BoundGeometryWorkload,
    declaration: &str,
) -> CertifiedSurfaceSupport {
    SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(declaration)
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("plane support should certify")
}

fn project_certified_surface_support(
    surface_support: CertifiedSurfaceSupport,
) -> worth_spatial::facade::projection_workload::ProjectedPlanarWorkload {
    ProjectionWorkload::for_certified_surface_support(surface_support)
        .declared("project shared binding support")
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("certified surface support should project")
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
