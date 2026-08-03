use worth_store_budgets::CounterEvidenceStrength;

use super::super::super::allocation::{
    AdmittedBlobStreamingAllocation, BlobStreamingAllocationObservation,
};
use crate::{BlobStreamingIngestDenial, BlobStreamingWindow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingResidencyProof {
    allocation: BlobStreamingAllocationObservation,
    peak_resident_bytes: u64,
    counter_strength: CounterEvidenceStrength,
}

impl BlobStreamingResidencyProof {
    pub(crate) fn from_executed_streaming_session(
        allocation: &AdmittedBlobStreamingAllocation<'_>,
        observed_peak_resident_bytes: u64,
        window: BlobStreamingWindow,
        counter_strength: CounterEvidenceStrength,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        if !counter_strength.satisfies(CounterEvidenceStrength::Exact) {
            return Err(BlobStreamingIngestDenial::MissingExactCounters {
                actual: counter_strength,
            });
        }
        let peak = observed_peak_resident_bytes;
        if peak == 0 || peak > window.max_resident_bytes() || peak > allocation.bytes() {
            return Err(BlobStreamingIngestDenial::AllocationWindowExceeded {
                window_bytes: peak,
                allocation_bytes: allocation.bytes(),
            });
        }
        Ok(Self {
            allocation: allocation.observation(),
            peak_resident_bytes: peak,
            counter_strength,
        })
    }

    pub const fn allocation(self) -> BlobStreamingAllocationObservation {
        self.allocation
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation.allocation_bytes()
    }

    pub const fn peak_resident_bytes(self) -> u64 {
        self.peak_resident_bytes
    }

    pub const fn counter_strength(self) -> CounterEvidenceStrength {
        self.counter_strength
    }
}
