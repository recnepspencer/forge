use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitVertexIdentitySet,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopSourceProvenanceBundle,
};

pub struct PlanarBooleanFragmentContinuationIndexInput<'a> {
    request: &'a PlanarBooleanLoopReconstructionRequest,
    source_provenance: &'a PlanarBooleanLoopSourceProvenanceBundle,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
}

impl<'a> PlanarBooleanFragmentContinuationIndexInput<'a> {
    pub fn from_request_and_provenance(
        request: &'a PlanarBooleanLoopReconstructionRequest,
        source_provenance: &'a PlanarBooleanLoopSourceProvenanceBundle,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    ) -> Self {
        Self {
            request,
            source_provenance,
            split_vertices,
            split_fragments,
            overlap_chains,
        }
    }

    pub(crate) fn request(&self) -> &'a PlanarBooleanLoopReconstructionRequest {
        self.request
    }

    pub(crate) fn source_provenance(&self) -> &'a PlanarBooleanLoopSourceProvenanceBundle {
        self.source_provenance
    }

    pub(crate) fn split_vertices(&self) -> &'a PlanarBooleanSplitVertexIdentitySet {
        self.split_vertices
    }

    pub(crate) fn split_fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.split_fragments
    }

    pub(crate) fn overlap_chains(&self) -> &'a PlanarBooleanOverlapEdgeChainSet {
        self.overlap_chains
    }
}
