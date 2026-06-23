use worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentCarrierSet;

fn main() {
    let _ = PlanarBooleanSegmentCarrierSet::from_coordinate_segments(vec![
        ([0.0, 0.0], [1.0, 0.0]),
    ]);
}
