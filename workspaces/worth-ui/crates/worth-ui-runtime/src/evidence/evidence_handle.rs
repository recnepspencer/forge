use super::{UiEvidenceFamily, UiEvidenceIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UiEvidenceHandle {
    family: UiEvidenceFamily,
    identity: UiEvidenceIdentity,
    handle_digest: u64,
}

impl UiEvidenceHandle {
    pub(crate) fn new(
        family: UiEvidenceFamily,
        identity: UiEvidenceIdentity,
        handle_digest: u64,
    ) -> Self {
        Self {
            family,
            identity,
            handle_digest,
        }
    }

    pub fn family(&self) -> UiEvidenceFamily {
        self.family
    }

    pub fn identity(&self) -> UiEvidenceIdentity {
        self.identity
    }

    pub fn handle_digest(&self) -> u64 {
        self.handle_digest
    }
}
