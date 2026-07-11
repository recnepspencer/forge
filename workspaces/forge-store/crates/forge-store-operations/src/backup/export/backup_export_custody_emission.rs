use forge_store_security::StoreSecurityScopeIdentity;

use crate::{BackupExportCustodyCounterSnapshot, BackupExportCustodyReadiness};

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportCapsuleEmission {
    security_scope: StoreSecurityScopeIdentity,
    counters: BackupExportCustodyCounterSnapshot,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportTerminalProjectionPreparation {
    security_scope: StoreSecurityScopeIdentity,
    counters: BackupExportCustodyCounterSnapshot,
}

impl BackupExportCapsuleEmission {
    pub fn prepare(readiness: BackupExportCustodyReadiness) -> Self {
        Self {
            security_scope: readiness.identity(),
            counters: readiness.counters().prepared_emission(),
        }
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }
}

impl BackupExportTerminalProjectionPreparation {
    pub(crate) fn prepare(readiness: BackupExportCustodyReadiness) -> Self {
        Self {
            security_scope: readiness.identity(),
            counters: readiness.counters().prepared_terminal_projection(),
        }
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }
}
