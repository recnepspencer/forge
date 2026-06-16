use worth_spatial::facade::planar_boolean_edge_splitting::{
    AdmittedPointSplitCandidate, PlanarBooleanAdmittedPointSplitCandidateSet,
    PlanarBooleanSplitPointAdmissionCounters,
};

fn main() {
    let _ = PlanarBooleanAdmittedPointSplitCandidateSet {
        point_candidate_set_identity: "raw point candidate set".to_string(),
        admitted_candidates: unavailable_admitted_candidates(),
        counters: PlanarBooleanSplitPointAdmissionCounters::default(),
    };
}

fn unavailable_admitted_candidates() -> Vec<AdmittedPointSplitCandidate> {
    panic!("compile-fail fixture must never construct admitted point split candidates")
}
