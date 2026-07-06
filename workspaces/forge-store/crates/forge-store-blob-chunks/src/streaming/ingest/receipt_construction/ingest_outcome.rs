use forge_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};
use forge_store_buffer_pool::AllocationReceipt;

use super::performance::counter_backed_streaming_performance_receipt;
use super::residency::BlobStreamingResidencyProof;
use super::super::frontier::BlobStreamingContentFrontier;
use super::super::types::BlobStreamingIngest;
use crate::{
    AdmittedBlobChunkSequence, BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial,
    BlobStreamingResumePosture, BlobStreamingWindow,
};

pub(crate) fn emit_ingest_receipt(
    sequence: AdmittedBlobChunkSequence,
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    window: BlobStreamingWindow,
    counter_strength: CounterEvidenceStrength,
    counters: BlobStreamingIngestCounterSnapshot,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    let frontier = BlobStreamingContentFrontier::from_sequence(&sequence);
    let resumability = BlobStreamingResumePosture::from_frontier(&frontier);
    let residency = BlobStreamingResidencyProof::from_executed_streaming_session(
        allocation,
        envelopes,
        counters.peak_resident_bytes(),
        window,
        counter_strength,
    )?;
    let performance = counter_backed_streaming_performance_receipt(counters);
    Ok(BlobStreamingIngest::from_bounded_parts(
        sequence,
        frontier,
        resumability,
        residency,
        counters,
        performance,
    ))
}