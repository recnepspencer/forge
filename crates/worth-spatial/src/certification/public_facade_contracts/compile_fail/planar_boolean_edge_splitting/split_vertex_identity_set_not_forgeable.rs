use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitVertexIdentityCounters, PlanarBooleanSplitVertexIdentitySet,
};

fn main() {
    let _ = PlanarBooleanSplitVertexIdentitySet::new(
        "set".to_string(),
        "interval subdivision set".to_string(),
        vec![],
        PlanarBooleanSplitVertexIdentityCounters::default(),
    );
}
