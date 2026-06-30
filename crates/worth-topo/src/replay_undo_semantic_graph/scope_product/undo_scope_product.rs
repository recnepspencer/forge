use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoSemanticGraphStageIndexIdentity, UndoScopeIdentity,
};

use super::TopologyUndoScopeProductCounters;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::undo_family_catalog::TopologyUndoFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyUndoScopeProduct<'a> {
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    admitted_input_semantic_graph_identity: String,
    counters: TopologyUndoScopeProductCounters,
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
    scope_identity: UndoScopeIdentity,
}

impl<'a> TopologyUndoScopeProduct<'a> {
    pub(crate) fn new(
        family_identity: TopologyUndoFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
        admitted_input_semantic_graph_identity: String,
        counters: TopologyUndoScopeProductCounters,
        equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
        scope_identity: UndoScopeIdentity,
    ) -> Self {
        Self {
            family_identity,
            touched_closure,
            prior_proof_identity,
            stage_index_identity,
            admitted_input_semantic_graph_identity,
            counters,
            equivalence_basis,
            scope_identity,
        }
    }

    pub const fn family_identity(&self) -> TopologyUndoFamilyIdentity {
        self.family_identity
    }

    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn prior_proof_identity(&self) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.prior_proof_identity
    }

    pub const fn stage_index_identity(&self) -> &ReplayUndoSemanticGraphStageIndexIdentity {
        &self.stage_index_identity
    }

    pub fn admitted_input_semantic_graph_identity(&self) -> &str {
        &self.admitted_input_semantic_graph_identity
    }

    pub const fn counters(&self) -> &TopologyUndoScopeProductCounters {
        &self.counters
    }

    pub const fn equivalence_basis(&self) -> &ReplayUndoSemanticGraphEquivalenceBasis {
        &self.equivalence_basis
    }

    pub const fn scope_identity(&self) -> &UndoScopeIdentity {
        &self.scope_identity
    }
}
