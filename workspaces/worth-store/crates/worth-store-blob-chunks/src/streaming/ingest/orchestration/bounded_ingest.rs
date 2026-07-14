use worth_store_budgets::{AllocationEnvelopeSet, CounterEvidenceStrength};
use worth_store_buffer_pool::AllocationReceipt;

use super::super::transitions::{
    admit_stream, advance_frontier, emit_ingest_receipt, finalize_sequence,
};
use super::super::types::BlobStreamingIngest;
use super::super::verification::counter_strength;
use crate::{
    BlobStreamingChunkWriter, BlobStreamingIngestDenial, BlobStreamingIngestRequest,
    BlobStreamingPressureAdmission, BlobStreamingSourceFrame, BlobStreamingWindow,
};

impl BlobStreamingIngest {
    pub(crate) fn run_bounded<W>(
        request: BlobStreamingIngestRequest,
        window: BlobStreamingWindow,
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        pressure: BlobStreamingPressureAdmission,
        source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
        writer: &mut W,
        counter_strength: CounterEvidenceStrength,
    ) -> Result<Self, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        execute_bounded_ingest(
            request,
            window,
            allocation,
            envelopes,
            pressure,
            source_frames,
            writer,
            counter_strength,
        )
    }
}

pub(crate) fn execute_bounded_ingest<W>(
    request: BlobStreamingIngestRequest,
    window: BlobStreamingWindow,
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    pressure: BlobStreamingPressureAdmission,
    source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
    writer: &mut W,
    counter_strength: CounterEvidenceStrength,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial>
where
    W: BlobStreamingChunkWriter,
{
    counter_strength::require_exact(counter_strength)?;
    let declared_total_bytes = request.declared_total_bytes();
    let (admission, chunking, counters) = admit_stream::admit_stream(request, pressure)?;
    let (admission, chunking, counters) = advance_frontier::advance_frontier(
        source_frames,
        window,
        declared_total_bytes,
        admission,
        chunking,
        counters,
        writer,
    )?;
    let (sequence, counters) =
        finalize_sequence::finalize_sequence(chunking, admission, counters, writer)?;
    emit_ingest_receipt::emit_ingest_receipt(
        sequence,
        allocation,
        envelopes,
        window,
        counter_strength,
        counters,
    )
}
