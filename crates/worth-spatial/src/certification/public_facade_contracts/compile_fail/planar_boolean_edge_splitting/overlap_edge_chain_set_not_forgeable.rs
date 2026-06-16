use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainCounters, PlanarBooleanOverlapEdgeChainSet,
};

fn main() {
    let _set = PlanarBooleanOverlapEdgeChainSet::new(
        "set".to_string(),
        "interval schedules".to_string(),
        "fragments".to_string(),
        Vec::new(),
        PlanarBooleanOverlapEdgeChainCounters::default(),
    );
}
