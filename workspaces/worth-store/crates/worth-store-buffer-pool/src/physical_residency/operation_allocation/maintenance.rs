use std::{num::NonZeroU64, ops::Deref};

use super::OperationAllocationGrant;
use crate::physical_residency::{
    PhysicalOperationAllocationScope, PhysicalResidencyDenial, PhysicalResidencyPool,
};

/// Pool-issued allocation authority for maintenance work.
#[derive(Debug)]
pub struct MaintenanceAllocationGrant {
    operation: OperationAllocationGrant,
}

impl PhysicalResidencyPool {
    pub fn begin_maintenance_operation(
        &self,
        bytes: NonZeroU64,
    ) -> Result<MaintenanceAllocationGrant, PhysicalResidencyDenial> {
        let operation =
            self.begin_operation(PhysicalOperationAllocationScope::Maintenance, bytes)?;
        Ok(MaintenanceAllocationGrant { operation })
    }
}

impl MaintenanceAllocationGrant {
    pub(crate) fn into_operation(self) -> OperationAllocationGrant {
        self.operation
    }
}

impl Deref for MaintenanceAllocationGrant {
    type Target = OperationAllocationGrant;

    fn deref(&self) -> &Self::Target {
        &self.operation
    }
}
