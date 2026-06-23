use worth_spatial::facade::planar_boolean_edge_splitting::{
    AdmittedIntervalSplitCandidate, PlanarBooleanAdmittedIntervalSplitCandidateSet,
    PlanarBooleanSplitIntervalAdmissionCounters,
};

fn main() {
    let _ = PlanarBooleanAdmittedIntervalSplitCandidateSet {
        interval_candidate_set_identity: "forged interval candidate set".to_string(),
        admitted_candidates: Vec::<AdmittedIntervalSplitCandidate>::new(),
        counters: PlanarBooleanSplitIntervalAdmissionCounters::default(),
    };
}
