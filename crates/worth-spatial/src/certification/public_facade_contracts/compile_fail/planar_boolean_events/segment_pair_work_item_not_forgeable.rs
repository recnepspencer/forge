use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCanonicalSegment, PlanarBooleanSegmentPairWorkItem,
};

fn main() {
    let _ = PlanarBooleanSegmentPairWorkItem {
        left: unavailable_segment(),
        right: unavailable_segment(),
        segment_pair_identity: String::from("forged"),
    };
}

fn unavailable_segment() -> PlanarBooleanCanonicalSegment {
    panic!("compile-fail fixture must never construct canonical segments")
}
