use crate::history::data::CommitId;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::CanonicalCommitEnvelope;

pub(crate) trait CommitEnvelopeSource {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope>;
}

impl CommitEnvelopeSource for RelationalRuntime {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.history
            .commit_envelopes
            .get(&commit_id)
            .map(|envelope| envelope.as_ref())
    }
}
