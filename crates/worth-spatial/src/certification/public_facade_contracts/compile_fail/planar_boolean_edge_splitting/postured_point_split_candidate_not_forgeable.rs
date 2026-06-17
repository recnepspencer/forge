use worth_spatial::facade::planar_boolean_edge_splitting::{
    AdmittedPointSplitCandidate, PlanarBooleanPointSplitPosture, PosturedPointSplitCandidate,
};

fn main() {
    let _ = PosturedPointSplitCandidate {
        postured_candidate_identity: "forged".to_string(),
        admitted_candidate: unavailable_admitted_candidate(),
        posture: PlanarBooleanPointSplitPosture::InteriorSplit,
    };
}

fn unavailable_admitted_candidate() -> AdmittedPointSplitCandidate {
    panic!("compile-fail fixture must never construct admitted point split candidate")
}
