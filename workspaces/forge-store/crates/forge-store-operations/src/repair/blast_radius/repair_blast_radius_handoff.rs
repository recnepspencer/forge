use forge_store_security::{StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity};

use crate::{RepairBlastRadiusReadiness, RepairPhysicalRegion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10RepairBlastRadiusPermission {
    identity: StoreSecurityScopeIdentity,
}

impl S10RepairBlastRadiusPermission {
    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct S10RepairBlastRadiusHandoff {
    permission: S10RepairBlastRadiusPermission,
    readiness: RepairBlastRadiusReadiness,
}

impl S10RepairBlastRadiusHandoff {
    pub fn from_repair_blast_radius_readiness(readiness: RepairBlastRadiusReadiness) -> Self {
        Self {
            permission: S10RepairBlastRadiusPermission {
                identity: readiness.identity(),
            },
            readiness,
        }
    }

    pub const fn permission(&self) -> S10RepairBlastRadiusPermission {
        self.permission
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.readiness.receipt()
    }

    pub fn physical_region(&self) -> &RepairPhysicalRegion {
        self.readiness.physical_region()
    }

    pub(crate) fn into_readiness(self) -> RepairBlastRadiusReadiness {
        self.readiness
    }
}
