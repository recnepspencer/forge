use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayScopeIdentity, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphPriorProofIdentity,
};

use super::TopologyReplayScopeProductCounters;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::replay_family_catalog::TopologyReplayFamilyIdentity;
use crate::replay_undo_semantic_graph::{
    TopologyReplaySemanticGraphSelectedPlanIdentity, TopologyReplaySemanticGraphStageIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplayScopeProduct<'a> {
    family_identity: TopologyReplayFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    selected_plan_identity: TopologyReplaySemanticGraphSelectedPlanIdentity,
    stage_identity: TopologyReplaySemanticGraphStageIdentity,
    counters: TopologyReplayScopeProductCounters,
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
    scope_identity: ReplayScopeIdentity,
}

impl<'a> TopologyReplayScopeProduct<'a> {
    pub(crate) fn new(
        family_identity: TopologyReplayFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        selected_plan_identity: TopologyReplaySemanticGraphSelectedPlanIdentity,
        stage_identity: TopologyReplaySemanticGraphStageIdentity,
        counters: TopologyReplayScopeProductCounters,
        equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
        scope_identity: ReplayScopeIdentity,
    ) -> Self {
        Self {
            family_identity,
            touched_closure,
            prior_proof_identity,
            selected_plan_identity,
            stage_identity,
            counters,
            equivalence_basis,
            scope_identity,
        }
    }

    pub const fn family_identity(&self) -> TopologyReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn prior_proof_identity(&self) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.prior_proof_identity
    }

    pub const fn selected_plan_identity(&self) -> &TopologyReplaySemanticGraphSelectedPlanIdentity {
        &self.selected_plan_identity
    }

    pub const fn stage_identity(&self) -> &TopologyReplaySemanticGraphStageIdentity {
        &self.stage_identity
    }

    pub fn stage_index_identity(
        &self,
    ) -> &schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphStageIndexIdentity{
        self.stage_identity.stage_index_identity()
    }

    pub const fn counters(&self) -> &TopologyReplayScopeProductCounters {
        &self.counters
    }

    pub const fn equivalence_basis(&self) -> &ReplayUndoSemanticGraphEquivalenceBasis {
        &self.equivalence_basis
    }

    pub const fn scope_identity(&self) -> &ReplayScopeIdentity {
        &self.scope_identity
    }
}
