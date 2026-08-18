use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::CommitId;
use crate::runtime::RelationalRuntime;

pub(crate) trait CommitEnvelopeSource {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope>;
}

impl CommitEnvelopeSource for RelationalRuntime {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| artifact.envelope().as_ref())
    }
}
