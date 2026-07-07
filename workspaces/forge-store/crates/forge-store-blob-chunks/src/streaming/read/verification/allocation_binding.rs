use forge_store_budgets::{AllocationEnvelopeSet, AllocationScope};
use forge_store_buffer_pool::{AllocationReceipt, AllocationRequestKind};

use crate::BlobStreamingReadDenial;

pub(crate) fn require_streaming_allocation(
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
) -> Result<(), BlobStreamingReadDenial> {
    if allocation.scope() != AllocationScope::Streaming {
        return Err(BlobStreamingReadDenial::AllocationScopeMismatch);
    }
    if allocation.kind() != AllocationRequestKind::StreamingWindow {
        return Err(BlobStreamingReadDenial::AllocationKindMismatch);
    }
    let envelope_bytes = envelopes.budget(AllocationScope::Streaming).as_bytes();
    if allocation.bytes() > envelope_bytes {
        return Err(BlobStreamingReadDenial::ResidentEnvelopeExceeded {
            peak_resident_bytes: allocation.bytes(),
            envelope_bytes,
        });
    }
    let streaming_counters = allocation.counters().scope(AllocationScope::Streaming);
    if streaming_counters.allocated_bytes() == 0 {
        return Err(BlobStreamingReadDenial::AllocationCountersHidden);
    }
    Ok(())
}
