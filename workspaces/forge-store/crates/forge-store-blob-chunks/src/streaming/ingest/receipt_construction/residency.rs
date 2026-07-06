use forge_store_budgets::{AllocationEnvelopeSet, AllocationScope, CounterEvidenceStrength};
use forge_store_buffer_pool::{AllocationReceipt, AllocationRequestKind};

use crate::{BlobStreamingIngestDenial, BlobStreamingWindow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingResidencyProof {
    allocation_bytes: u64,
    peak_resident_bytes: u64,
    envelope_bytes: u64,
    counter_strength: CounterEvidenceStrength,
}

impl BlobStreamingResidencyProof {
    pub(crate) fn from_executed_streaming_session(
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        observed_peak_resident_bytes: u64,
        window: BlobStreamingWindow,
        counter_strength: CounterEvidenceStrength,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        if !counter_strength.satisfies(CounterEvidenceStrength::Exact) {
            return Err(BlobStreamingIngestDenial::MissingExactCounters {
                actual: counter_strength,
            });
        }
        if allocation.scope() != AllocationScope::Streaming {
            return Err(BlobStreamingIngestDenial::AllocationScopeMismatch);
        }
        if allocation.kind() != AllocationRequestKind::StreamingWindow {
            return Err(BlobStreamingIngestDenial::AllocationKindMismatch);
        }
        let allocated = allocation.bytes();
        let envelope_bytes = envelopes.budget(AllocationScope::Streaming).as_bytes();
        if allocated > envelope_bytes {
            return Err(BlobStreamingIngestDenial::ResidentEnvelopeExceeded {
                peak_resident_bytes: allocated,
                envelope_bytes,
            });
        }
        let peak = observed_peak_resident_bytes;
        if peak == 0 || peak > window.max_resident_bytes() || peak > envelope_bytes {
            return Err(BlobStreamingIngestDenial::ResidentEnvelopeExceeded {
                peak_resident_bytes: peak,
                envelope_bytes,
            });
        }
        let streaming_counters = allocation.counters().scope(AllocationScope::Streaming);
        if streaming_counters.allocated_bytes() == 0 {
            return Err(BlobStreamingIngestDenial::AllocationCountersHidden);
        }
        Ok(Self {
            allocation_bytes: allocated,
            peak_resident_bytes: peak,
            envelope_bytes,
            counter_strength,
        })
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn peak_resident_bytes(self) -> u64 {
        self.peak_resident_bytes
    }

    pub const fn envelope_bytes(self) -> u64 {
        self.envelope_bytes
    }

    pub const fn counter_strength(self) -> CounterEvidenceStrength {
        self.counter_strength
    }
}