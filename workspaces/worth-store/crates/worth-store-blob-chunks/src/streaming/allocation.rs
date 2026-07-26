use worth_store_buffer_pool::{
    OperationAllocationGrant, OperationAllocationObservation, PhysicalOperationAllocationScope,
};

#[derive(Debug)]
pub(crate) struct AdmittedBlobStreamingAllocation {
    grant: OperationAllocationGrant,
    observation: BlobStreamingAllocationObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingAllocationObservation {
    allocation: OperationAllocationObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlobStreamingAllocationDenial {
    WrongScope {
        actual: PhysicalOperationAllocationScope,
    },
    WindowExceedsAllocation {
        window_bytes: u64,
        allocation_bytes: u64,
    },
    CountersUnavailable,
}

impl AdmittedBlobStreamingAllocation {
    pub(crate) fn admit(
        grant: OperationAllocationGrant,
        window_bytes: u64,
    ) -> Result<Self, BlobStreamingAllocationDenial> {
        let allocation = grant.observation();
        if allocation.scope() != PhysicalOperationAllocationScope::Blob {
            return Err(BlobStreamingAllocationDenial::WrongScope {
                actual: allocation.scope(),
            });
        }
        if allocation.bytes() < window_bytes {
            return Err(BlobStreamingAllocationDenial::WindowExceedsAllocation {
                window_bytes,
                allocation_bytes: allocation.bytes(),
            });
        }
        if allocation
            .counters()
            .active_operation_bytes_for(PhysicalOperationAllocationScope::Blob)
            < allocation.bytes()
        {
            return Err(BlobStreamingAllocationDenial::CountersUnavailable);
        }
        Ok(Self {
            grant,
            observation: BlobStreamingAllocationObservation { allocation },
        })
    }

    pub(crate) const fn observation(&self) -> BlobStreamingAllocationObservation {
        self.observation
    }

    pub(crate) const fn bytes(&self) -> u64 {
        self.grant.bytes()
    }
}

impl BlobStreamingAllocationObservation {
    pub const fn allocation(self) -> OperationAllocationObservation {
        self.allocation
    }
}
