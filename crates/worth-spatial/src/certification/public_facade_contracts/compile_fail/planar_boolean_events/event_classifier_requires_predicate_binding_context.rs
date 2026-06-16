use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventClassifierInput;

fn main() {
    let _ = PlanarBooleanEventClassifierInput::from_raw_segment_pair(
        "raw-left-segment",
        "raw-right-segment",
        "raw-predicate-context",
    );
}
