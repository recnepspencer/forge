use forge_store_security::{StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity};

use crate::BackupExportCustodyReadiness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10BackupExportCustodyPermission {
    identity: StoreSecurityScopeIdentity,
}

impl S10BackupExportCustodyPermission {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct S10BackupExportCustodyHandoff {
    permission: S10BackupExportCustodyPermission,
    readiness: BackupExportCustodyReadiness,
}

impl S10BackupExportCustodyHandoff {
    pub fn from_backup_export_readiness(readiness: BackupExportCustodyReadiness) -> Self {
        Self {
            permission: S10BackupExportCustodyPermission {
                identity: readiness.identity(),
            },
            readiness,
        }
    }

    pub const fn permission(&self) -> S10BackupExportCustodyPermission {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.readiness.receipt()
    }

    pub(crate) fn into_readiness(self) -> BackupExportCustodyReadiness {
        self.readiness
    }
}
