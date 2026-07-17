use worth_store_security::StoreSecurityScopeIdentity;

use super::BlobCustodyPurpose;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportCustodyEvidence {
    identity: StoreSecurityScopeIdentity,
    purpose: BlobCustodyPurpose,
}

impl BlobExportCustodyEvidence {
    pub(crate) const fn new(
        identity: StoreSecurityScopeIdentity,
        purpose: BlobCustodyPurpose,
    ) -> Self {
        Self { identity, purpose }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn purpose(&self) -> BlobCustodyPurpose {
        self.purpose
    }
}
