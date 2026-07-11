use forge_store_security::{S51AdmittedSecurityScopeReadiness, StoreSecurityScopeIdentity};

use crate::{BackupExportCustodyCounterSnapshot, BackupExportCustodyMode};

#[derive(Debug, PartialEq, Eq)]
pub struct BackupExportCustodyAdmission {
    mode: Option<BackupExportCustodyMode>,
    readiness: S51AdmittedSecurityScopeReadiness,
    counters: BackupExportCustodyCounterSnapshot,
}

impl BackupExportCustodyAdmission {
    pub(crate) const fn from_outbound_declaration(
        mode: BackupExportCustodyMode,
        readiness: S51AdmittedSecurityScopeReadiness,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Self {
        Self {
            mode: Some(mode),
            readiness,
            counters,
        }
    }

    pub(crate) const fn from_trust_boundary_readmission(
        readiness: S51AdmittedSecurityScopeReadiness,
        counters: BackupExportCustodyCounterSnapshot,
    ) -> Self {
        Self {
            mode: None,
            readiness,
            counters,
        }
    }

    pub const fn mode(&self) -> Option<BackupExportCustodyMode> {
        self.mode
    }

    pub const fn identity(&self) -> StoreSecurityScopeIdentity {
        self.readiness.receipt().identity()
    }

    pub const fn readiness(&self) -> &S51AdmittedSecurityScopeReadiness {
        &self.readiness
    }

    pub const fn counters(&self) -> BackupExportCustodyCounterSnapshot {
        self.counters
    }

    pub fn into_readiness(self) -> S51AdmittedSecurityScopeReadiness {
        self.readiness
    }
}
