use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanDeniedLoopCandidateSet,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanLoopCandidateSet,
    PlanarBooleanLoopIdentityMap, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopPersistentNamePropagationMap, PlanarBooleanLoopReconstructionRequest,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanLoopSubshapeSignatureMap,
    PlanarBooleanSourceLoopSplitAttribution, PlanarBooleanWalkOutcomeSet,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanLoopDecisionLogInput<'a> {
    request: &'a PlanarBooleanLoopReconstructionRequest,
    continuation_index: &'a PlanarBooleanFragmentContinuationIndex,
    walk_outcomes: &'a PlanarBooleanWalkOutcomeSet,
    loop_candidates: &'a PlanarBooleanLoopCandidateSet,
    denied_loop_candidates: &'a PlanarBooleanDeniedLoopCandidateSet,
    reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: &'a PlanarBooleanBornLoopSet,
    island_partition: &'a PlanarBooleanLoopIslandPartition,
    split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
    degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
    loop_identity_map: &'a PlanarBooleanLoopIdentityMap,
    persistent_name_map: &'a PlanarBooleanLoopPersistentNamePropagationMap,
    subshape_signature_map: &'a PlanarBooleanLoopSubshapeSignatureMap,
}

impl<'a> PlanarBooleanLoopDecisionLogInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_phase_thirteen_products(
        request: &'a PlanarBooleanLoopReconstructionRequest,
        continuation_index: &'a PlanarBooleanFragmentContinuationIndex,
        walk_outcomes: &'a PlanarBooleanWalkOutcomeSet,
        loop_candidates: &'a PlanarBooleanLoopCandidateSet,
        denied_loop_candidates: &'a PlanarBooleanDeniedLoopCandidateSet,
        reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
        born_loops: &'a PlanarBooleanBornLoopSet,
        island_partition: &'a PlanarBooleanLoopIslandPartition,
        split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
        role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
        degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
        loop_identity_map: &'a PlanarBooleanLoopIdentityMap,
        persistent_name_map: &'a PlanarBooleanLoopPersistentNamePropagationMap,
        subshape_signature_map: &'a PlanarBooleanLoopSubshapeSignatureMap,
    ) -> Self {
        Self {
            request,
            continuation_index,
            walk_outcomes,
            loop_candidates,
            denied_loop_candidates,
            reconstructed_loops,
            born_loops,
            island_partition,
            split_attribution,
            role_outcomes,
            degenerate_outcomes,
            loop_identity_map,
            persistent_name_map,
            subshape_signature_map,
        }
    }

    pub fn request(self) -> &'a PlanarBooleanLoopReconstructionRequest {
        self.request
    }

    pub fn continuation_index(self) -> &'a PlanarBooleanFragmentContinuationIndex {
        self.continuation_index
    }

    pub fn walk_outcomes(self) -> &'a PlanarBooleanWalkOutcomeSet {
        self.walk_outcomes
    }

    pub fn loop_candidates(self) -> &'a PlanarBooleanLoopCandidateSet {
        self.loop_candidates
    }

    pub fn denied_loop_candidates(self) -> &'a PlanarBooleanDeniedLoopCandidateSet {
        self.denied_loop_candidates
    }

    pub fn reconstructed_loops(self) -> &'a PlanarBooleanAdmittedReconstructedLoopSet {
        self.reconstructed_loops
    }

    pub fn born_loops(self) -> &'a PlanarBooleanBornLoopSet {
        self.born_loops
    }

    pub fn island_partition(self) -> &'a PlanarBooleanLoopIslandPartition {
        self.island_partition
    }

    pub fn split_attribution(self) -> &'a PlanarBooleanSourceLoopSplitAttribution {
        self.split_attribution
    }

    pub fn role_outcomes(self) -> &'a PlanarBooleanLoopRoleOutcomeSet {
        self.role_outcomes
    }

    pub fn degenerate_outcomes(self) -> &'a PlanarBooleanDegenerateLoopOutcomeSet {
        self.degenerate_outcomes
    }

    pub fn loop_identity_map(self) -> &'a PlanarBooleanLoopIdentityMap {
        self.loop_identity_map
    }

    pub fn persistent_name_map(self) -> &'a PlanarBooleanLoopPersistentNamePropagationMap {
        self.persistent_name_map
    }

    pub fn subshape_signature_map(self) -> &'a PlanarBooleanLoopSubshapeSignatureMap {
        self.subshape_signature_map
    }
}
