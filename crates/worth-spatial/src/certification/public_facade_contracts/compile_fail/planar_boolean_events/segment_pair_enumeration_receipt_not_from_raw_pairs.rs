use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanSegmentCandidateIndexProduct, PlanarBooleanSegmentPairEnumerationReceipt,
    PlanarBooleanSegmentPairWorkItem,
};

fn main() {
    let _ = PlanarBooleanSegmentPairEnumerationReceipt {
        segment_pair_enumeration_identity: String::from("forged"),
        candidate_index_product: unavailable_candidate_index_product(),
        work_items: Vec::<PlanarBooleanSegmentPairWorkItem>::new(),
    };
}

fn unavailable_candidate_index_product() -> PlanarBooleanSegmentCandidateIndexProduct {
    panic!("compile-fail fixture must never construct candidate-index product")
}
