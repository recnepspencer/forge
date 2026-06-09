use worth_spatial::certification::support::placement::{
    admit_spatial_placement, SpatialPlacementSpec,
};

fn main() {
    let _ = admit_spatial_placement(SpatialPlacementSpec::world());
}
