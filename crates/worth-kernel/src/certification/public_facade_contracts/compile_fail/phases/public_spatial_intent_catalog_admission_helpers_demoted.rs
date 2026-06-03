use worth_kernel::facade::authoring::intents::{
    MoveSpatialIntent, ReorientSpatialIntent,
};
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialPointWitnessRef};

fn main() {
    let _ = MoveSpatialIntent::shape("shape-1")
        .to_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]))
        .admit_with_catalog(&());
    let _ = ReorientSpatialIntent::shape("shape-2")
        .so(SpatialAnchorRef::shape_origin())
        .points_toward([1.0, 2.0, 3.0])
        .admit_with_catalog(&());
}
