use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapChainBoundaryRole, PlanarBooleanOverlapEdgeChainMember,
};
use worth_spatial::facade::planar_boolean_events::PlanarBooleanSourceIntervalSense;

fn main() {
    let _member = PlanarBooleanOverlapEdgeChainMember::new(
        "member".to_string(),
        "fragment".to_string(),
        "subdivision".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        [0.0, 1.0],
        "source interval".to_string(),
        [0.0, 1.0],
        PlanarBooleanSourceIntervalSense::Forward,
        "normalized interval".to_string(),
        [0.0, 1.0],
        PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan,
        "frame".to_string(),
        "precision".to_string(),
        Vec::new(),
        Vec::new(),
    );
}
