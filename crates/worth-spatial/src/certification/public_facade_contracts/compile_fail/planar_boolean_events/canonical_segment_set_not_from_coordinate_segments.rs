use worth_spatial::facade::planar_boolean_events::PlanarBooleanCanonicalSegmentSet;

fn main() {
    let _ = PlanarBooleanCanonicalSegmentSet::from_coordinate_segments(vec![
        ([0.0, 0.0], [1.0, 0.0]),
    ]);
}
