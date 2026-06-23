use worth_spatial::facade::planar_boolean_edge_splitting::{
    AdmittedIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidate,
};

fn main() {
    let _ = AdmittedIntervalSplitCandidate {
        candidate: unavailable_candidate(),
        admitted_parameter_range: [0.2, 0.7],
    };
}

fn unavailable_candidate() -> PlanarBooleanIntervalSplitCandidate {
    panic!("compile-fail fixture must never construct interval split candidate")
}
