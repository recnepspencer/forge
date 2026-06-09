use worth_kernel::facade::authoring::intents::{
    MoveSpatialIntent, ReorientSpatialIntent,
};
use worth_spatial::facade::refs::{SpatialAnchorRef, SpatialPointWitnessRef};
use worth_spatial::facade::refs::SpatialWitnessCatalog;

fn demo(catalog: &impl SpatialWitnessCatalog) {
    let _ = MoveSpatialIntent::shape("shape-1")
        .to_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]))
        .admit_with_catalog(catalog);
    let _ = ReorientSpatialIntent::shape("shape-2")
        .so(SpatialAnchorRef::shape_origin())
        .points_toward([1.0, 2.0, 3.0])
        .admit_with_catalog(catalog);
}

fn main() {}
