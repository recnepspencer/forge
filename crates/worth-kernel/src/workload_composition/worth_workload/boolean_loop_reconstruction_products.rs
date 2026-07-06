use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanSplitEdgeFragmentSet,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanDegenerateLoopOutcomeBoundary,
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopContainmentEvidencePostureSet,
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopIdentityMap, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopPersistentNamePropagationMap, PlanarBooleanLoopReconstructionLedger,
    PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopRoleOutcomeSet,
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanLoopSubshapeSignatureMap,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanWalkOutcomeSet,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CompletedBooleanLoopReconstructionProducts {
    downstream_split_consumption: PlanarBooleanDownstreamSplitConsumption,
    loop_split_consumption: PlanarBooleanLoopReconstructionSplitConsumption,
    request: PlanarBooleanLoopReconstructionRequest,
    source_provenance: PlanarBooleanLoopSourceProvenanceBundle,
    split_fragments: PlanarBooleanSplitEdgeFragmentSet,
    continuation_index: PlanarBooleanFragmentContinuationIndex,
    walk_candidate_assembly: PlanarBooleanClosedWalkCandidateAssembly,
    walk_outcomes: PlanarBooleanWalkOutcomeSet,
    candidate_boundary: PlanarBooleanLoopCandidateBoundary,
    reconstructed_boundary: PlanarBooleanReconstructedLoopBoundary,
    island_partition: PlanarBooleanLoopIslandPartition,
    split_attribution: PlanarBooleanSourceLoopSplitAttribution,
    role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
    containment_postures: PlanarBooleanLoopContainmentEvidencePostureSet,
    degenerate_boundary: PlanarBooleanDegenerateLoopOutcomeBoundary,
    degenerate_outcomes: PlanarBooleanDegenerateLoopOutcomeSet,
    loop_identity_map: PlanarBooleanLoopIdentityMap,
    persistent_name_propagation_map: PlanarBooleanLoopPersistentNamePropagationMap,
    subshape_signature_map: PlanarBooleanLoopSubshapeSignatureMap,
    decision_log: PlanarBooleanLoopDecisionLog,
    loop_ledger: PlanarBooleanLoopReconstructionLedger,
}

impl CompletedBooleanLoopReconstructionProducts {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        downstream_split_consumption: PlanarBooleanDownstreamSplitConsumption,
        loop_split_consumption: PlanarBooleanLoopReconstructionSplitConsumption,
        request: PlanarBooleanLoopReconstructionRequest,
        source_provenance: PlanarBooleanLoopSourceProvenanceBundle,
        split_fragments: PlanarBooleanSplitEdgeFragmentSet,
        continuation_index: PlanarBooleanFragmentContinuationIndex,
        walk_candidate_assembly: PlanarBooleanClosedWalkCandidateAssembly,
        walk_outcomes: PlanarBooleanWalkOutcomeSet,
        candidate_boundary: PlanarBooleanLoopCandidateBoundary,
        reconstructed_boundary: PlanarBooleanReconstructedLoopBoundary,
        island_partition: PlanarBooleanLoopIslandPartition,
        split_attribution: PlanarBooleanSourceLoopSplitAttribution,
        role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
        containment_postures: PlanarBooleanLoopContainmentEvidencePostureSet,
        degenerate_boundary: PlanarBooleanDegenerateLoopOutcomeBoundary,
        degenerate_outcomes: PlanarBooleanDegenerateLoopOutcomeSet,
        loop_identity_map: PlanarBooleanLoopIdentityMap,
        persistent_name_propagation_map: PlanarBooleanLoopPersistentNamePropagationMap,
        subshape_signature_map: PlanarBooleanLoopSubshapeSignatureMap,
        decision_log: PlanarBooleanLoopDecisionLog,
        loop_ledger: PlanarBooleanLoopReconstructionLedger,
    ) -> Self {
        Self {
            downstream_split_consumption,
            loop_split_consumption,
            request,
            source_provenance,
            split_fragments,
            continuation_index,
            walk_candidate_assembly,
            walk_outcomes,
            candidate_boundary,
            reconstructed_boundary,
            island_partition,
            split_attribution,
            role_outcomes,
            containment_postures,
            degenerate_boundary,
            degenerate_outcomes,
            loop_identity_map,
            persistent_name_propagation_map,
            subshape_signature_map,
            decision_log,
            loop_ledger,
        }
    }

    pub fn downstream_split_consumption(&self) -> &PlanarBooleanDownstreamSplitConsumption {
        &self.downstream_split_consumption
    }

    pub fn loop_split_consumption(&self) -> &PlanarBooleanLoopReconstructionSplitConsumption {
        &self.loop_split_consumption
    }

    pub fn request(&self) -> &PlanarBooleanLoopReconstructionRequest {
        &self.request
    }

    pub fn source_provenance(&self) -> &PlanarBooleanLoopSourceProvenanceBundle {
        &self.source_provenance
    }

    pub fn split_fragments(&self) -> &PlanarBooleanSplitEdgeFragmentSet {
        &self.split_fragments
    }

    pub fn continuation_index(&self) -> &PlanarBooleanFragmentContinuationIndex {
        &self.continuation_index
    }

    pub fn walk_candidate_assembly(&self) -> &PlanarBooleanClosedWalkCandidateAssembly {
        &self.walk_candidate_assembly
    }

    pub fn walk_outcomes(&self) -> &PlanarBooleanWalkOutcomeSet {
        &self.walk_outcomes
    }

    pub fn candidate_boundary(&self) -> &PlanarBooleanLoopCandidateBoundary {
        &self.candidate_boundary
    }

    pub fn reconstructed_boundary(&self) -> &PlanarBooleanReconstructedLoopBoundary {
        &self.reconstructed_boundary
    }

    pub fn island_partition(&self) -> &PlanarBooleanLoopIslandPartition {
        &self.island_partition
    }

    pub fn split_attribution(&self) -> &PlanarBooleanSourceLoopSplitAttribution {
        &self.split_attribution
    }

    pub fn role_outcomes(&self) -> &PlanarBooleanLoopRoleOutcomeSet {
        &self.role_outcomes
    }

    pub fn containment_postures(&self) -> &PlanarBooleanLoopContainmentEvidencePostureSet {
        &self.containment_postures
    }

    pub fn degenerate_boundary(&self) -> &PlanarBooleanDegenerateLoopOutcomeBoundary {
        &self.degenerate_boundary
    }

    pub fn degenerate_outcomes(&self) -> &PlanarBooleanDegenerateLoopOutcomeSet {
        &self.degenerate_outcomes
    }

    pub fn loop_identity_map(&self) -> &PlanarBooleanLoopIdentityMap {
        &self.loop_identity_map
    }

    pub fn persistent_name_propagation_map(
        &self,
    ) -> &PlanarBooleanLoopPersistentNamePropagationMap {
        &self.persistent_name_propagation_map
    }

    pub fn subshape_signature_map(&self) -> &PlanarBooleanLoopSubshapeSignatureMap {
        &self.subshape_signature_map
    }

    pub fn decision_log(&self) -> &PlanarBooleanLoopDecisionLog {
        &self.decision_log
    }

    pub fn loop_ledger(&self) -> &PlanarBooleanLoopReconstructionLedger {
        &self.loop_ledger
    }
}
