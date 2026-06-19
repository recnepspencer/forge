use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanLoopDecisionLog,
    PlanarBooleanLoopIdentityMap, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopPersistentNamePropagationMap, PlanarBooleanLoopReconstructionRequest,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanLoopSubshapeSignatureMap,
    PlanarBooleanSourceLoopSplitAttribution,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanLoopReconstructionLedgerInput<'a> {
    request: &'a PlanarBooleanLoopReconstructionRequest,
    decision_log: &'a PlanarBooleanLoopDecisionLog,
    loop_identity_map: &'a PlanarBooleanLoopIdentityMap,
    persistent_name_map: &'a PlanarBooleanLoopPersistentNamePropagationMap,
    subshape_signature_map: &'a PlanarBooleanLoopSubshapeSignatureMap,
    reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: &'a PlanarBooleanBornLoopSet,
    island_partition: &'a PlanarBooleanLoopIslandPartition,
    split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
    degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
}

impl<'a> PlanarBooleanLoopReconstructionLedgerInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_decision_log_and_loop_products(
        request: &'a PlanarBooleanLoopReconstructionRequest,
        decision_log: &'a PlanarBooleanLoopDecisionLog,
        loop_identity_map: &'a PlanarBooleanLoopIdentityMap,
        persistent_name_map: &'a PlanarBooleanLoopPersistentNamePropagationMap,
        subshape_signature_map: &'a PlanarBooleanLoopSubshapeSignatureMap,
        reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
        born_loops: &'a PlanarBooleanBornLoopSet,
        island_partition: &'a PlanarBooleanLoopIslandPartition,
        split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
        role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
        degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
    ) -> Self {
        Self {
            request,
            decision_log,
            loop_identity_map,
            persistent_name_map,
            subshape_signature_map,
            reconstructed_loops,
            born_loops,
            island_partition,
            split_attribution,
            role_outcomes,
            degenerate_outcomes,
        }
    }

    pub fn request(self) -> &'a PlanarBooleanLoopReconstructionRequest {
        self.request
    }

    pub fn decision_log(self) -> &'a PlanarBooleanLoopDecisionLog {
        self.decision_log
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
}
