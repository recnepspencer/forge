use worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventCoordinateFact;

fn main() {
    let _ = PlanarBooleanPointEventCoordinateFact::new(
        [0.0, 0.0],
        "raw-frame",
        "raw-precision",
    );
}
