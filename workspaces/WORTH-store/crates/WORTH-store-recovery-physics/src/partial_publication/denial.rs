use crate::{LogSequenceNumber, UnadmittedDirtyPagePublicationDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TornPublicationDenial {
    torn_lsn: Option<LogSequenceNumber>,
    reason: String,
}

impl TornPublicationDenial {
    pub fn new(torn_lsn: Option<LogSequenceNumber>, reason: impl Into<String>) -> Self {
        Self {
            torn_lsn,
            reason: reason.into(),
        }
    }

    pub const fn torn_lsn(&self) -> Option<LogSequenceNumber> {
        self.torn_lsn
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonAuthoritativePublicationSource {
    BackendResidue,
    LiveAcknowledgmentMemory,
    LogOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonAuthoritativePublicationDenial {
    source: NonAuthoritativePublicationSource,
    persisted_digest: String,
}

impl NonAuthoritativePublicationDenial {
    pub fn new(
        source: NonAuthoritativePublicationSource,
        persisted_digest: impl Into<String>,
    ) -> Self {
        Self {
            source,
            persisted_digest: persisted_digest.into(),
        }
    }

    pub const fn source(&self) -> NonAuthoritativePublicationSource {
        self.source
    }

    pub fn persisted_digest(&self) -> &str {
        &self.persisted_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnadmittedDurablePageMutationDenial {
    denial: UnadmittedDirtyPagePublicationDenial,
}

impl UnadmittedDurablePageMutationDenial {
    pub const fn from_page_publication_denial(
        denial: UnadmittedDirtyPagePublicationDenial,
    ) -> Self {
        Self { denial }
    }

    pub const fn page_denial(&self) -> &UnadmittedDirtyPagePublicationDenial {
        &self.denial
    }
}
