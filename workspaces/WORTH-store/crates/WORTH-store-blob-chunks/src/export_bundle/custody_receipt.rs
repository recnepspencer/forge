use worth_store_operations::BackupExportCustodyMode;
use worth_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobExportCustodyEvidence {
    identity: StoreSecurityScopeIdentity,
    mode: Option<BackupExportCustodyMode>,
}

impl BlobExportCustodyEvidence {
    pub(crate) const fn new(
        identity: StoreSecurityScopeIdentity,
        mode: Option<BackupExportCustodyMode>,
    ) -> Self {
        Self { identity, mode }
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.identity
    }

    pub const fn mode(&self) -> Option<BackupExportCustodyMode> {
        self.mode
    }
}
