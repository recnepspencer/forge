use worth_spatial::facade::planar_boolean_edge_splitting::{
    AdmittedPointSplitCandidate, PlanarBooleanPointSplitCandidate,
    PlanarBooleanSplitPointEndpointPosture,
};

fn main() {
    let _ = AdmittedPointSplitCandidate {
        candidate: unavailable_candidate(),
        endpoint_posture: PlanarBooleanSplitPointEndpointPosture::Interior,
        exact_endpoint_source_identity: None,
        exact_projected_endpoint_fact_identity: None,
    };
}

fn unavailable_candidate() -> PlanarBooleanPointSplitCandidate {
    panic!("compile-fail fixture must never construct point split candidate")
}
