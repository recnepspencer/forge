use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphPriorProofIdentity;
use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphStageIndexIdentity;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::replay_family_catalog::TopologyReplayFamilyIdentity;
use crate::undo_family_catalog::TopologyUndoFamilyIdentity;

use super::{
    selected_plan_identity::TopologyReplaySemanticGraphSelectedPlanIdentity,
    stage_identity::TopologyReplaySemanticGraphStageIdentity,
};

#[derive(Debug)]
pub struct TopologyReplaySemanticGraphAdmittedInput<'a> {
    family_identity: TopologyReplayFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    selected_plan_identity: TopologyReplaySemanticGraphSelectedPlanIdentity,
    stage_identity: TopologyReplaySemanticGraphStageIdentity,
    admission_digest: String,
}

#[derive(Debug)]
pub struct TopologyUndoSemanticGraphAdmittedInput<'a> {
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    admission_digest: String,
}

impl<'a> TopologyReplaySemanticGraphAdmittedInput<'a> {
    pub(crate) fn new(
        family_identity: TopologyReplayFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        selected_plan_identity: TopologyReplaySemanticGraphSelectedPlanIdentity,
        stage_identity: TopologyReplaySemanticGraphStageIdentity,
    ) -> Self {
        let admission_digest = super::replay_admission::replay_admission_digest(
            family_identity,
            touched_closure,
            &prior_proof_identity,
            stage_identity.stage_index_identity(),
        );
        Self {
            family_identity,
            touched_closure,
            prior_proof_identity,
            selected_plan_identity,
            stage_identity,
            admission_digest,
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

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
}

impl<'a> TopologyUndoSemanticGraphAdmittedInput<'a> {
    pub(crate) fn new(
        family_identity: TopologyUndoFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    ) -> Self {
        let admission_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:replay-undo-semantic-graph:undo-admitted-input:v1".to_string(),
                format!("family:{}", family_identity.as_str()),
                format!("touched-closure:{}", touched_closure.closure_digest()),
                format!("prior-proof:{}", prior_proof_identity.digest()),
                format!("stage-index:{}", stage_index_identity.digest()),
            ],
        );
        Self {
            family_identity,
            touched_closure,
            prior_proof_identity,
            stage_index_identity,
            admission_digest,
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

    pub fn semantic_graph_identity(&self) -> &str {
        &self.admission_digest
    }
}
