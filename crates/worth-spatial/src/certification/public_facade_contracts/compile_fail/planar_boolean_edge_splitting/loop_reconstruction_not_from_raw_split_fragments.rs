use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanLoopReconstructionSplitConsumptionInput, PlanarBooleanSplitEdgeFragmentSet,
};

fn reject_loop_reconstruction_from_raw_fragments(fragments: &PlanarBooleanSplitEdgeFragmentSet) {
    let _ = PlanarBooleanLoopReconstructionSplitConsumptionInput::from_raw_fragments(fragments);
}

fn main() {}
