use std::collections::BTreeSet;

use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::{BranchId, CommitId, RelationalCommitReceipt};
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;

pub(crate) trait HistorySource: super::CommitEnvelopeSource {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&RelationalCommitReceipt>;
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
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&RelationalCommitReceipt> {
        let cell = self.history.branch_cell(branch_id)?;
        let commit_id = match cell.observation().target() {
            worth_foundational::FoundationalBranchTarget::Empty => return None,
            worth_foundational::FoundationalBranchTarget::Basis(target) => {
                CommitId(target.commit_id())
            }
        };
        self.history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| &artifact.envelope().commit)
    }

    fn authoritative_commit_envelopes(&self) -> Vec<&CanonicalCommitEnvelope> {
        self.history.commit_catalog.envelope_refs()
    }
}
