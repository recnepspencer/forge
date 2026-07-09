use crate::evidence::AbsentModeLaneEvidence;
use worth_relational::facade::{
    history::CommitId, replay::CanonicalCommitEnvelope, runtime::RelationalRuntime,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsentModeSemanticEvidence {
    latest_commit_id: Option<CommitId>,
    latest_commit_envelope: Option<CanonicalCommitEnvelope>,
}

impl AbsentModeSemanticEvidence {
    pub fn latest_commit_id(&self) -> Option<CommitId> {
        self.latest_commit_id
    }

    pub fn latest_commit_envelope(&self) -> Option<&CanonicalCommitEnvelope> {
        self.latest_commit_envelope.as_ref()
    }
}

#[derive(Debug)]
pub struct AbsentRuntimeWitness {
    runtime: RelationalRuntime,
}

impl AbsentRuntimeWitness {
    pub fn new(runtime: RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn runtime(&self) -> &RelationalRuntime {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut RelationalRuntime {
        &mut self.runtime
    }

    pub fn semantic_evidence(&self) -> AbsentModeSemanticEvidence {
        let latest_commit_id = self
            .runtime
            .history()
            .latest_commit()
            .map(|commit| commit.commit_id);
        let latest_commit_envelope = latest_commit_id.and_then(|commit_id| {
            self.runtime
                .replay()
                .canonical_commit_envelope(commit_id)
                .cloned()
        });
        AbsentModeSemanticEvidence {
            latest_commit_id,
            latest_commit_envelope,
        }
    }

    pub fn milestone_2_lane_evidence(&self) -> AbsentModeLaneEvidence {
        AbsentModeLaneEvidence::from_semantic_evidence(&self.semantic_evidence())
    }

    pub fn into_runtime(self) -> RelationalRuntime {
        self.runtime
    }
}
