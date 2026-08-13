use std::collections::BTreeSet;

use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;

pub(crate) trait HistorySource: super::CommitEnvelopeSource {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&CommitReference>;
    fn authoritative_commit_envelopes(&self) -> Vec<&CanonicalCommitEnvelope>;

    fn has_committed_version_at_or_before_outside_closure(
        &self,
        version_id: VersionId,
        closure: &BTreeSet<CommitId>,
    ) -> bool {
        self.authoritative_commit_envelopes()
            .into_iter()
            .any(|candidate| {
                candidate.commit.version_id <= version_id
                    && !closure.contains(&candidate.commit.commit_id)
            })
    }
}

impl HistorySource for RelationalRuntime {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&CommitReference> {
        self.history
            .branch_heads
            .get(branch_id)
            .and_then(|head| head.as_ref())
    }

    fn authoritative_commit_envelopes(&self) -> Vec<&CanonicalCommitEnvelope> {
        self.history
            .commit_envelopes
            .values()
            .map(|envelope| envelope.as_ref())
            .collect()
    }
}
