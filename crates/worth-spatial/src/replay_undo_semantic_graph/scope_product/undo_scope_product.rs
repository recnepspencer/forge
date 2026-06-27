use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoSemanticGraphStageIndexIdentity, UndoScopeIdentity,
};

use crate::replay_undo_semantic_graph::SpatialUndoScopeProductCounters;
use crate::undo_family_catalog::{
    SpatialUndoFamilyIdentity, SpatialUndoFamilyWorkloadDependencyPosture,
};
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialUndoScopeProduct<'a> {
    family_identity: SpatialUndoFamilyIdentity,
    workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
    admitted_input_semantic_graph_identity: String,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    lookup_consumed_workload_handoff: Option<&'a EvidenceLookupConsumedWorkloadHandoff>,
    counters: SpatialUndoScopeProductCounters,
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
    scope_identity: UndoScopeIdentity,
}

impl<'a> SpatialUndoScopeProduct<'a> {
    pub(crate) fn new(
        family_identity: SpatialUndoFamilyIdentity,
        workload_dependency_posture: SpatialUndoFamilyWorkloadDependencyPosture,
        admitted_input_semantic_graph_identity: String,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
        lookup_consumed_workload_handoff: Option<&'a EvidenceLookupConsumedWorkloadHandoff>,
        counters: SpatialUndoScopeProductCounters,
        equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
        scope_identity: UndoScopeIdentity,
    ) -> Self {
        Self {
            family_identity,
            workload_dependency_posture,
            admitted_input_semantic_graph_identity,
            prior_proof_identity,
            stage_index_identity,
            lookup_consumed_workload_handoff,
            counters,
            equivalence_basis,
            scope_identity,
        }
    }

    pub const fn family_identity(&self) -> SpatialUndoFamilyIdentity {
        self.family_identity
    }

    pub const fn workload_dependency_posture(&self) -> SpatialUndoFamilyWorkloadDependencyPosture {
        self.workload_dependency_posture
    }

    pub fn admitted_input_semantic_graph_identity(&self) -> &str {
        &self.admitted_input_semantic_graph_identity
    }

    pub const fn prior_proof_identity(&self) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.prior_proof_identity
    }

    pub const fn stage_index_identity(&self) -> &ReplayUndoSemanticGraphStageIndexIdentity {
        &self.stage_index_identity
    }

    pub const fn lookup_consumed_workload_handoff(
        &self,
    ) -> Option<&'a EvidenceLookupConsumedWorkloadHandoff> {
        self.lookup_consumed_workload_handoff
    }

    pub const fn counters(&self) -> &SpatialUndoScopeProductCounters {
        &self.counters
    }

    pub const fn equivalence_basis(&self) -> &ReplayUndoSemanticGraphEquivalenceBasis {
        &self.equivalence_basis
    }

    pub const fn scope_identity(&self) -> &UndoScopeIdentity {
        &self.scope_identity
    }
}
