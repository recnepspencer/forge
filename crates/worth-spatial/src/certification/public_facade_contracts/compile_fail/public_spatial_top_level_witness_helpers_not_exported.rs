use worth_spatial::facade::{
    refs::{SpatialDirectionWitnessRef, SpatialPointWitnessRef},
    resolve_spatial_direction_witness, resolve_spatial_point_witness,
};

fn main() {
    let _ = resolve_spatial_point_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]));
    let _ =
        resolve_spatial_direction_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]));
}
