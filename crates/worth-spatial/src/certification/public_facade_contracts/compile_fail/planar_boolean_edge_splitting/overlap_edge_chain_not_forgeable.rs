use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapChainPosture, PlanarBooleanOverlapEdgeChain,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

fn main() {
    let _chain = PlanarBooleanOverlapEdgeChain::new(
        "chain".to_string(),
        "event".to_string(),
        PlanarBooleanIntervalEventKind::PartialOverlap,
        PlanarBooleanOverlapChainPosture::PartialOverlap,
        vec!["source interval".to_string()],
        vec!["normalized interval".to_string()],
        vec![PlanarBooleanSourceIntervalSense::Forward],
        vec!["event group".to_string()],
        Vec::new(),
    );
}
