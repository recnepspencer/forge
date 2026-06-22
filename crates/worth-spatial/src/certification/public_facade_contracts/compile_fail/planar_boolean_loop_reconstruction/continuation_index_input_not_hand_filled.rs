use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitVertexIdentitySet,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanFragmentContinuationIndexInput, PlanarBooleanLoopReconstructionRequest,
    PlanarBooleanLoopSourceProvenanceBundle,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanFragmentContinuationIndexInput {
        request: bogus::<&PlanarBooleanLoopReconstructionRequest>(),
        source_provenance: bogus::<&PlanarBooleanLoopSourceProvenanceBundle>(),
        split_vertices: bogus::<&PlanarBooleanSplitVertexIdentitySet>(),
        split_fragments: bogus::<&PlanarBooleanSplitEdgeFragmentSet>(),
        overlap_chains: bogus::<&PlanarBooleanOverlapEdgeChainSet>(),
    };
}
