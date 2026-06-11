use topology::facade::TopologySeed;
use worth_spatial::facade::projection_workload::{LocalFrameBasis, ProjectionWorkload};
use worth_spatial::facade::surface_support::{SurfaceFamily, SurfaceSupportWorkload};
use worth_spatial::facade::transform_workload::{
    RotationTurn, TransformReorientation, TransformSequence, VectorDelta,
};
use worth_spatial::facade::workload_binding::{
    GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet, PlanarLoopCarrierSet,
};

pub fn projected_cube_workload(
    declaration: &str,
) -> worth_spatial::facade::projection_workload::ProjectedPlanarWorkload {
    let topology = TopologySeed::cube()
        .with_declaration(declaration)
        .build()
        .expect("cube topology seed should admit");
    let bound_geometry = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("bind {declaration}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar geometry binding should admit");
    let surface_support = SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("certify support for {declaration}"))
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("plane support should certify");

    ProjectionWorkload::for_certified_surface_support(surface_support)
        .declared(format!("project {declaration}"))
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .expect("projection workload should admit")
}

pub fn acceptance_transform_sequence() -> TransformSequence {
    TransformSequence::new()
        .translate(VectorDelta::xy(10, 0))
        .rotate(RotationTurn::quarter_turn_clockwise())
        .reorient(TransformReorientation::preserves_handedness())
        .cancel_with_exact_replay(16)
}
