use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopCandidateSet, PlanarBooleanLoopSourceProvenanceBundle,
};

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanReconstructedLoopBoundaryInput<'a> {
    loop_candidates: &'a PlanarBooleanLoopCandidateSet,
    source_provenance: &'a PlanarBooleanLoopSourceProvenanceBundle,
}

impl<'a> PlanarBooleanReconstructedLoopBoundaryInput<'a> {
    pub fn from_loop_candidates_and_provenance(
        loop_candidates: &'a PlanarBooleanLoopCandidateSet,
        source_provenance: &'a PlanarBooleanLoopSourceProvenanceBundle,
    ) -> Self {
        Self {
            loop_candidates,
            source_provenance,
        }
    }

    pub fn loop_candidates(self) -> &'a PlanarBooleanLoopCandidateSet {
        self.loop_candidates
    }

    pub fn source_provenance(self) -> &'a PlanarBooleanLoopSourceProvenanceBundle {
        self.source_provenance
    }
}
