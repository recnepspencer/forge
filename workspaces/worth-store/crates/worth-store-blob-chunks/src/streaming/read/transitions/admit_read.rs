use worth_store_budgets::CounterEvidenceStrength;

use super::super::verification::stable_read_bytes;
use crate::{
    BlobStreamingReadAdmission, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial,
    BlobStreamingReadRequest,
};

pub(crate) fn admit_read(
    admission: BlobStreamingReadAdmission,
    request: &BlobStreamingReadRequest,
    counter_strength: CounterEvidenceStrength,
) -> Result<BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial> {
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
