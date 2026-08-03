use std::{num::NonZeroU64, ops::Deref};

use super::OperationAllocationGrant;
use crate::physical_residency::{
    PhysicalOperationAllocationScope, PhysicalResidencyDenial, PhysicalResidencyPool,
};

/// Pool-issued operation allocation authority for foreground mutation.
///
/// The inner generic grant remains borrowable for append planning and reads,
/// but only this stronger type can authorize candidate or copy-on-write
/// allocation.
#[derive(Debug)]
pub struct ForegroundWriteAllocationGrant {
    operation: OperationAllocationGrant,
}

impl PhysicalResidencyPool {
    pub fn begin_foreground_write_operation(
        &self,
        bytes: NonZeroU64,
    ) -> Result<ForegroundWriteAllocationGrant, PhysicalResidencyDenial> {
        let operation =
            self.begin_operation(PhysicalOperationAllocationScope::ForegroundWrite, bytes)?;
        Ok(ForegroundWriteAllocationGrant { operation })
    }
}

impl Deref for ForegroundWriteAllocationGrant {
    type Target = OperationAllocationGrant;

    fn deref(&self) -> &Self::Target {
        &self.operation
    }
}
