use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentEndpointRef,
};

fn main() {
    let start = PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_start(
        "source edge",
        "carrier",
        "frame",
        "precision",
    );
    let end = PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_end(
        "source edge",
        "carrier",
        "frame",
        "precision",
    );
    let _ = PlanarBooleanSplitEdgeFragment::new(
        "fragment".to_string(),
        "source edge".to_string(),
        "carrier".to_string(),
        start,
        end,
        [0.0, 1.0],
        [0.0f64.to_bits(), 1.0f64.to_bits()],
        "frame".to_string(),
        "precision".to_string(),
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    );
}
