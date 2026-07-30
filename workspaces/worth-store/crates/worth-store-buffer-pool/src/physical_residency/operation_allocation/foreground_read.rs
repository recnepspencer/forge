use std::{num::NonZeroU64, ops::Deref};

use super::OperationAllocationGrant;
use crate::physical_residency::{
    PhysicalOperationAllocationScope, PhysicalResidencyDenial, PhysicalResidencyPool,
};

/// Pool-issued operation allocation authority for foreground reads.
///
/// Speculative read admission consumes this concrete authority so a generic
/// operation grant or a differently scoped grant cannot authorize prefetch or
/// read-ahead work.
#[derive(Debug)]
pub struct ForegroundReadAllocationGrant {
    operation: OperationAllocationGrant,
}

impl PhysicalResidencyPool {
    pub fn begin_foreground_read_operation(
        &self,
        bytes: NonZeroU64,
    ) -> Result<ForegroundReadAllocationGrant, PhysicalResidencyDenial> {
        let operation =
            self.begin_operation(PhysicalOperationAllocationScope::ForegroundRead, bytes)?;
        Ok(ForegroundReadAllocationGrant { operation })
    }
}

impl Deref for ForegroundReadAllocationGrant {
    type Target = OperationAllocationGrant;

    fn deref(&self) -> &Self::Target {
        &self.operation
    }
}
