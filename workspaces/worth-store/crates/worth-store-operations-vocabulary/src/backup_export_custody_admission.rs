use worth_store_security::{StoreAdmittedSecurityScope, StoreSecurityScopeIdentity};

use crate::{BackupExportCustodyCounterSnapshot, BackupExportCustodyMode};

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportCustodyAdmission {
    mode: Option<BackupExportCustodyMode>,
    security_scope: StoreAdmittedSecurityScope,
    counters: BackupExportCustodyCounterSnapshot,
}

impl BackupExportCustodyAdmission {
    pub const fn from_outbound_declaration(
        mode: BackupExportCustodyMode,
        security_scope: StoreAdmittedSecurityScope,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Self {
        Self {
            mode: Some(mode),
            security_scope,
            counters,
        }
    }

    pub const fn from_trust_boundary_readmission(
        security_scope: StoreAdmittedSecurityScope,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Self {
        Self {
            mode: None,
            security_scope,
            counters,
        }
    }

    pub const fn mode(&self) -> Option<BackupExportCustodyMode> {
        self.mode
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.security_scope.identity()
    }

    pub const fn security_scope(&self) -> &StoreAdmittedSecurityScope {
        &self.security_scope
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }

    pub fn into_security_scope(self) -> StoreAdmittedSecurityScope {
        self.security_scope
    }
}
