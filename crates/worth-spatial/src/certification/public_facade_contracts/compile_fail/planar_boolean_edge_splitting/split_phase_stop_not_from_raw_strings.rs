use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitPhaseStop, PlanarBooleanSplitDecisionPhase,
};

fn main() {
    let _ = PlanarBooleanEdgeSplitPhaseStop::typed_denial(
        PlanarBooleanSplitDecisionPhase::SplitVertexIdentity,
        "source edge",
        "carrier",
        "evidence",
        vec!["event".to_string()],
        vec!["event-group".to_string()],
        "CoordinateOnlySplitVertexIdentity",
        "raw strings should not construct production phase stops",
    );
}
