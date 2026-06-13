#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourcePackageRevision {
    provider_id: String,
    final_package_digest: u64,
    event_burst_digest: u64,
    sequence: u64,
}

impl WorthUiSourcePackageRevision {
    pub(crate) fn new(
        provider_id: impl Into<String>,
        final_package_digest: u64,
        event_burst_digest: u64,
        sequence: u64,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            final_package_digest,
            event_burst_digest,
            sequence,
        }
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn final_package_digest(&self) -> u64 {
        self.final_package_digest
    }

    pub fn event_burst_digest(&self) -> u64 {
        self.event_burst_digest
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }
}
