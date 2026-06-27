use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayScopeIdentity, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphPriorProofIdentity, ReplayUndoSemanticGraphStageIndexIdentity,
};

use super::{SpatialReplayScopeProductCounters, SpatialReplayScopeProductIdentity};
use crate::replay_family_catalog::{
    SpatialReplayFamilyCoveredLookupIdentity, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyWorkloadDependencyPosture,
};
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialReplayScopeProduct<'a> {
    family_identity: SpatialReplayFamilyIdentity,
    covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
    workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
    admitted_input_semantic_graph_identity: String,
    selected_plan_identity: String,
    lookup_consumed_workload_handoff_identity: String,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
    retained_replay_receipt: Option<&'a RetainedReplayWorkloadReceipt>,
    counters: SpatialReplayScopeProductCounters,
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
    scope_identity: ReplayScopeIdentity,
    scope_product_identity: SpatialReplayScopeProductIdentity,
}

impl<'a> SpatialReplayScopeProduct<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family_identity: SpatialReplayFamilyIdentity,
        covered_lookup_identity: SpatialReplayFamilyCoveredLookupIdentity,
        workload_dependency_posture: SpatialReplayFamilyWorkloadDependencyPosture,
        admitted_input_semantic_graph_identity: String,
        selected_plan_identity: String,
        lookup_consumed_workload_handoff_identity: String,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
        lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
        retained_replay_receipt: Option<&'a RetainedReplayWorkloadReceipt>,
        counters: SpatialReplayScopeProductCounters,
        equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
        scope_identity: ReplayScopeIdentity,
        scope_product_identity: SpatialReplayScopeProductIdentity,
    ) -> Self {
        Self {
            family_identity,
            covered_lookup_identity,
            workload_dependency_posture,
            admitted_input_semantic_graph_identity,
            selected_plan_identity,
            lookup_consumed_workload_handoff_identity,
            prior_proof_identity,
            stage_index_identity,
            lookup_consumed_workload_handoff,
            retained_replay_receipt,
            counters,
            equivalence_basis,
            scope_identity,
            scope_product_identity,
        }
    }

    pub const fn family_identity(&self) -> SpatialReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn covered_lookup_identity(&self) -> SpatialReplayFamilyCoveredLookupIdentity {
        self.covered_lookup_identity
    }

    pub const fn workload_dependency_posture(
        &self,
    ) -> SpatialReplayFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub fn admitted_input_semantic_graph_identity(&self) -> &str {
        &self.admitted_input_semantic_graph_identity
    }

    pub fn selected_plan_identity(&self) -> &str {
        &self.selected_plan_identity
    }

    pub fn lookup_consumed_workload_handoff_identity(&self) -> &str {
        &self.lookup_consumed_workload_handoff_identity
    }

    pub const fn prior_proof_identity(&self) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.prior_proof_identity
    }

    pub const fn stage_index_identity(&self) -> &ReplayUndoSemanticGraphStageIndexIdentity {
        &self.stage_index_identity
    }

    pub const fn lookup_consumed_workload_handoff(
        &self,
    ) -> &'a EvidenceLookupConsumedWorkloadHandoff {
        self.lookup_consumed_workload_handoff
    }

    pub const fn retained_replay_receipt(&self) -> Option<&'a RetainedReplayWorkloadReceipt> {
        self.retained_replay_receipt
    }

    pub const fn counters(&self) -> &SpatialReplayScopeProductCounters {
        &self.counters
    }

    pub const fn equivalence_basis(&self) -> &ReplayUndoSemanticGraphEquivalenceBasis {
        &self.equivalence_basis
    }

    pub const fn scope_identity(&self) -> &ReplayScopeIdentity {
        &self.scope_identity
    }

    pub const fn scope_product_identity(&self) -> &SpatialReplayScopeProductIdentity {
        &self.scope_product_identity
    }
}
