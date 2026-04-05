use crate::capabilities::CommitEnvelopeSource;
use crate::history::data::CommitId;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{CanonicalCommitEnvelope, RelationalReplayOutcome};

pub struct ReplayAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> ReplayAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn canonical_commit_envelope(
        &self,
        commit_id: CommitId,
    ) -> Option<&CanonicalCommitEnvelope> {
        self.runtime.commit_envelope(commit_id)
    }

    pub fn compare_outcome(&self, outcome: &RelationalReplayOutcome) -> bool {
        outcome.failure.is_none() && outcome.mismatches.is_empty()
    }
}

impl RelationalRuntime {
    pub(crate) fn replay_access(&self) -> ReplayAccess<'_> {
        ReplayAccess::new(self)
    }
}
