use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::CommitId;
use crate::runtime::RelationalRuntime;

pub(crate) trait CommitEnvelopeSource {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<CanonicalCommitEnvelope>;

    fn canonical_envelope_owned(&self, commit_id: CommitId) -> Option<CanonicalCommitEnvelope> {
        self.commit_envelope(commit_id)
    }
}

impl CommitEnvelopeSource for RelationalRuntime {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<CanonicalCommitEnvelope> {
        self.history
            .canonical_envelope(commit_id)
            .map(|envelope| envelope.as_ref().clone())
    }

    fn canonical_envelope_owned(&self, commit_id: CommitId) -> Option<CanonicalCommitEnvelope> {
        self.history
            .commit_catalog
            .get(commit_id)
            .map(|artifact| artifact.envelope().as_ref().clone())
            .or_else(|| {
                self.history
                    .canonical_envelope(commit_id)
                    .map(|envelope| envelope.as_ref().clone())
            })
    }
}
