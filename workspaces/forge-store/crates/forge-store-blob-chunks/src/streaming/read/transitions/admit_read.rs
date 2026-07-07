use forge_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};

use super::super::verification::{allocation_binding, stable_read_bytes};
use crate::{
    BlobStreamingReadAdmission, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial,
    BlobStreamingReadRequest,
};

pub(crate) fn admit_read(
    admission: BlobStreamingReadAdmission,
    request: &BlobStreamingReadRequest,
    allocation: forge_store_buffer_pool::AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    counter_strength: CounterEvidenceStrength,
) -> Result<BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial> {
    allocation_binding::require_streaming_allocation(allocation, envelopes)?;
    let counters = admission
        .seed_counters(BlobStreamingReadCounterSnapshot::start(counter_strength))
        .record_allocation();
    stable_read_bytes::require_stable_read_bytes(
        admission,
        request.frontier().proof_frontier().total_bytes(),
        counters,
    )?;
    Ok(counters)
}
