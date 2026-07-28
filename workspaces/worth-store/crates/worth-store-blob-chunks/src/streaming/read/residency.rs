use super::super::allocation::{
    AdmittedBlobStreamingAllocation, BlobStreamingAllocationObservation,
};
use crate::{BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingReadResidencyProof {
    allocation: BlobStreamingAllocationObservation,
    peak_resident_bytes: u64,
}

impl BlobStreamingReadResidencyProof {
    pub(crate) fn from_executed_streaming_session(
        allocation: &AdmittedBlobStreamingAllocation<'_>,
        counters: BlobStreamingReadCounterSnapshot,
    ) -> Result<Self, BlobStreamingReadDenial> {
        let peak_resident_bytes = counters.peak_resident_bytes();
        if peak_resident_bytes == 0 || peak_resident_bytes > allocation.bytes() {
            return Err(BlobStreamingReadDenial::AllocationWindowExceeded {
                window_bytes: peak_resident_bytes,
                allocation_bytes: allocation.bytes(),
            });
        }
        Ok(Self {
            allocation: allocation.observation(),
            peak_resident_bytes,
        })
    }

    pub const fn allocation(self) -> BlobStreamingAllocationObservation {
        self.allocation
    }

    pub const fn peak_resident_bytes(self) -> u64 {
        self.peak_resident_bytes
    }
}
