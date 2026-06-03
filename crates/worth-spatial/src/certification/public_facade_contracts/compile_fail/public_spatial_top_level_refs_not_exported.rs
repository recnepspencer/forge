use worth_spatial::facade::{
    EmptySpatialWitnessCatalog, SpatialAnchorRef, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialPointWitnessRef, SpatialWitnessCatalog,
};

fn require_catalog(_catalog: &impl SpatialWitnessCatalog) {}

fn main() {
    let catalog = EmptySpatialWitnessCatalog;
    require_catalog(&catalog);
    let _ = SpatialAnchorRef::shape_origin();
    let _ = SpatialFrameRef::world();
    let _ = SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]);
    let _ = SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]);
}
