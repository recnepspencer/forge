use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumptionInput, PlanarBooleanSplitEdgeFragmentSet,
};

fn reject_raw_fragments(fragments: &PlanarBooleanSplitEdgeFragmentSet) {
    let _ = PlanarBooleanDownstreamSplitConsumptionInput::from_raw_fragments(fragments);
}

fn main() {}
