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
    pub fn from_s10_permission(
        readiness: BackupExportCustodyReadiness,
        permission: S10BackupExportCustodyPermission,
    ) -> Option<Self> {
        if !matches!(
            readiness.mode(),
            Some(crate::BackupExportCustodyMode::Backup)
                | Some(crate::BackupExportCustodyMode::PointInTimeRecovery)
        ) || readiness.identity() != permission.identity()
        {
            return None;
        }
        Some(Self {
            permission,
            readiness,
        })
    }

    pub const fn permission(&self) -> S10BackupExportCustodyPermission {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.readiness.receipt()
    }

    pub const fn readiness(&self) -> &BackupExportCustodyReadiness {
        &self.readiness
    }

    pub(crate) fn into_readiness(self) -> BackupExportCustodyReadiness {
        self.readiness
    }
}
