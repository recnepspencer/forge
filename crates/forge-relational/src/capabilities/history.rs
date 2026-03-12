use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::CanonicalCommitEnvelope;

pub(crate) trait CommitEnvelopeSource {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope>;
}

impl CommitEnvelopeSource for RelationalRuntime {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.history.commit_envelopes.get(&commit_id)
    }
}

pub(crate) trait HistorySource: CommitEnvelopeSource {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&CommitReference>;
}

impl HistorySource for RelationalRuntime {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&CommitReference> {
        self.history
            .branch_heads
            .get(branch_id)
            .and_then(|head| head.as_ref())
    }
}
