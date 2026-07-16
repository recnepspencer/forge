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

pub struct BlobStreamingIngestExecution {
    window: BlobStreamingWindow,
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
    pressure: BlobStreamingPressureAdmission,
    counter_strength: CounterEvidenceStrength,
}

impl BlobStreamingIngestExecution {
    pub fn new(
        window: BlobStreamingWindow,
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        pressure: BlobStreamingPressureAdmission,
        counter_strength: CounterEvidenceStrength,
    ) -> Self {
        Self {
            window,
            allocation,
            envelopes,
            pressure,
            counter_strength,
        }
    }
}

impl BlobStreamingIngest {
    pub(crate) fn run_bounded<W>(
        request: BlobStreamingIngestRequest,
        execution: BlobStreamingIngestExecution,
        source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
        writer: &mut W,
    ) -> Result<Self, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        execute_bounded_ingest(request, execution, source_frames, writer)
    }
}

pub(crate) fn execute_bounded_ingest<W>(
    request: BlobStreamingIngestRequest,
    execution: BlobStreamingIngestExecution,
    source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
    writer: &mut W,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial>
where
    W: BlobStreamingChunkWriter,
{
    counter_strength::require_exact(execution.counter_strength)?;
    let declared_total_bytes = request.declared_total_bytes();
    let (admission, chunking, counters) = admit_stream::admit_stream(request, execution.pressure)?;
    let (admission, chunking, counters) = advance_frontier::advance_frontier(
        source_frames,
        execution.window,
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
        execution.allocation,
        execution.envelopes,
        execution.window,
        execution.counter_strength,
        counters,
    )
}
