use crate::runtime::source_ingress::digest::fold_texts;
use crate::runtime::source_ingress::revision::WorthUiSourcePackageRevision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateOrderingReceipt {
    provider_id: String,
    source_revision_digest: u64,
    event_burst_digest: u64,
    debounce_policy_digest: u64,
    sequence: u64,
    receipt_digest: u64,
}

impl WorthUiCandidateOrderingReceipt {
    pub(crate) fn from_revision(
        revision: &WorthUiSourcePackageRevision,
        debounce_policy_digest: u64,
    ) -> Self {
        let basis = [
            format!("provider:{}", revision.provider_id()),
            format!("source:{}", revision.final_package_digest()),
            format!("burst:{}", revision.event_burst_digest()),
            format!("debounce:{debounce_policy_digest}"),
            format!("sequence:{}", revision.sequence()),
        ];
        Self {
            provider_id: revision.provider_id().to_owned(),
            source_revision_digest: revision.final_package_digest(),
            event_burst_digest: revision.event_burst_digest(),
            debounce_policy_digest,
            sequence: revision.sequence(),
            receipt_digest: fold_texts(basis),
        }
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn source_revision_digest(&self) -> u64 {
        self.source_revision_digest
    }

    pub fn event_burst_digest(&self) -> u64 {
        self.event_burst_digest
    }

    pub fn debounce_policy_digest(&self) -> u64 {
        self.debounce_policy_digest
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }

    pub(crate) fn matches_revision(&self, revision: &WorthUiSourcePackageRevision) -> bool {
        self.provider_id == revision.provider_id()
            && self.source_revision_digest == revision.final_package_digest()
            && self.event_burst_digest == revision.event_burst_digest()
            && self.sequence == revision.sequence()
    }

    #[cfg(test)]
    pub(crate) fn with_sequence_for_test(mut self, sequence: u64) -> Self {
        self.sequence = sequence;
        self
    }
}
