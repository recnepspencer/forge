use serde::Serialize;

use super::locality_scope::ReplayUndoSemanticGraphLocalityScope;
use super::prior_proof_identity::ReplayUndoSemanticGraphPriorProofIdentity;
use super::stage_index_identity::ReplayUndoSemanticGraphStageIndexIdentity;
use super::touched_subject::ReplayUndoSemanticGraphTouchedSubject;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReplayUndoSemanticGraphEquivalenceBasis {
    locality_scope: ReplayUndoSemanticGraphLocalityScope,
    touched_subjects: Vec<ReplayUndoSemanticGraphTouchedSubject>,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: Option<ReplayUndoSemanticGraphStageIndexIdentity>,
}

impl ReplayUndoSemanticGraphEquivalenceBasis {
    pub fn new(
        locality_scope: ReplayUndoSemanticGraphLocalityScope,
        mut touched_subjects: Vec<ReplayUndoSemanticGraphTouchedSubject>,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: Option<ReplayUndoSemanticGraphStageIndexIdentity>,
    ) -> Self {
        touched_subjects.sort();
        touched_subjects.dedup();
        Self {
            locality_scope,
            touched_subjects,
            prior_proof_identity,
            stage_index_identity,
        }
    }

    pub const fn locality_scope(&self) -> ReplayUndoSemanticGraphLocalityScope {
        self.locality_scope
    }

    pub fn touched_subjects(&self) -> &[ReplayUndoSemanticGraphTouchedSubject] {
        &self.touched_subjects
    }

    pub const fn prior_proof_identity(&self) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.prior_proof_identity
    }

    pub const fn stage_index_identity(&self) -> Option<&ReplayUndoSemanticGraphStageIndexIdentity> {
        self.stage_index_identity.as_ref()
    }

    pub fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("locality:{}", self.locality_scope.as_str()),
            format!("prior-proof:{}", self.prior_proof_identity.digest_part()),
        ];
        if let Some(stage_index_identity) = self.stage_index_identity() {
            parts.push(stage_index_identity.digest_part());
        }
        parts.extend(
            self.touched_subjects
                .iter()
                .map(ReplayUndoSemanticGraphTouchedSubject::digest_part),
        );
        parts
    }
}
