use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateCounters,
    PlanarBooleanIntervalSplitCandidateSet,
};

fn main() {
    let _ = PlanarBooleanIntervalSplitCandidateSet {
        candidate_set_identity: "raw interval candidate set".to_string(),
        participation_index_identity: "raw participation index".to_string(),
        candidates: unavailable_candidates(),
        counters: PlanarBooleanIntervalSplitCandidateCounters::default(),
    };
}

fn unavailable_candidates() -> Vec<PlanarBooleanIntervalSplitCandidate> {
    panic!("compile-fail fixture must never construct interval split candidates")
}
