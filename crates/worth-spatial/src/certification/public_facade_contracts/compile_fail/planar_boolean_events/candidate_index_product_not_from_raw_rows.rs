use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanSegmentCandidateIndexProduct,
    PlanarBooleanSegmentCandidateRowReceipt, PlanarBooleanSegmentPairEnumerationCounters,
};

fn main() {
    let _ = PlanarBooleanSegmentCandidateIndexProduct {
        product_identity: String::from("forged"),
        canonical_segment_set_identity: String::from("synthetic"),
        declaration_digest: String::from("fake-declaration"),
        plan_digest: String::from("fake-plan"),
        envelope_digest: String::from("fake-envelope"),
        strategy: PlanarBooleanCandidateIndexStrategy::AabbSweep,
        fallback_posture: PlanarBooleanCandidateIndexFallbackPosture::NotUsed,
        lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome::Bound,
        counters: unavailable_counters(),
        rows: Vec::<PlanarBooleanSegmentCandidateRowReceipt>::new(),
    };
}

fn unavailable_counters() -> PlanarBooleanSegmentPairEnumerationCounters {
    panic!("compile-fail fixture must never construct candidate-index counters")
}
