use worth_store_budgets::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationScope, CounterEvidenceStrength,
    FixedMetadataReservation,
};
use worth_store_buffer_pool::{AllocationAdmission, AllocationReceipt, AllocationRequest};
use worth_store_io_scheduler::{
    blob_ingest_background_capacity_for_certification_test, BackgroundResourceBudget, QueueSlot,
};
use worth_store_physical_backend::BlobBackendChunkWriteSession;
use worth_store_security::StoreTenantScope;

use crate::test_support::{blob_scope, physical_payload_for_bytes};
use crate::{
    BlobChunkOrdinal, BlobChunkSize, BlobChunkingRuleAdmission, BlobStreamingChunkWriter,
    BlobStreamingIngest, BlobStreamingIngestDenial, BlobStreamingIngestRequest,
    BlobStreamingPressureAdmission, BlobStreamingSourceFrame, BlobStreamingWindow,
    BlobStreamingWrittenChunk,
};

pub(super) fn run_ingest(
    frames: Vec<BlobStreamingSourceFrame>,
    window_bytes: u64,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    run_ingest_with_window_and_envelope(
        frames,
        window_bytes,
        window_bytes,
        CounterEvidenceStrength::Exact,
    )
}

pub(super) fn run_ingest_with_counter_strength(
    strength: CounterEvidenceStrength,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    run_ingest_with_window_and_envelope(source_frames(3, 4), 4, 4, strength)
}

pub(super) fn run_ingest_with_envelope(
    envelope_bytes: u64,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    run_ingest_with_window_and_envelope(
        source_frames(3, 4),
        4,
        envelope_bytes,
        CounterEvidenceStrength::Exact,
    )
}

fn run_ingest_with_window_and_envelope(
    frames: Vec<BlobStreamingSourceFrame>,
    window_bytes: u64,
    envelope_bytes: u64,
    strength: CounterEvidenceStrength,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    let window = BlobStreamingWindow::bounded(window_bytes)?;
    let (allocation, envelopes) = allocation_receipt_and_envelope(envelope_bytes);
    BlobStreamingIngest::run_bounded(
        request(),
        crate::BlobStreamingIngestExecution::new(
            window,
            allocation,
            envelopes,
            pressure_admission(),
            strength,
        ),
        frames,
        &mut TestChunkWriter::new(),
    )
}

pub(super) fn run_ingest_with_pressure(
    pressure: BlobStreamingPressureAdmission,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    let (allocation, envelopes) = allocation_receipt_and_envelope(4);
    BlobStreamingIngest::run_bounded(
        request(),
        crate::BlobStreamingIngestExecution::new(
            BlobStreamingWindow::bounded(4).unwrap(),
            allocation,
            envelopes,
            pressure,
            CounterEvidenceStrength::Exact,
        ),
        source_frames(3, 4),
        &mut TestChunkWriter::new(),
    )
}

pub(super) fn request() -> BlobStreamingIngestRequest {
    request_for_total_bytes(12)
}

pub(super) fn request_for_total_bytes(total_bytes: u64) -> BlobStreamingIngestRequest {
    BlobStreamingIngestRequest::new(
        blob_scope("phase9.streaming", StoreTenantScope::TenantPhysicalBoundary),
        rule(),
        total_bytes,
    )
    .unwrap()
}

pub(super) fn source_frames(
    frame_bytes: usize,
    window_bytes: u64,
) -> Vec<BlobStreamingSourceFrame> {
    b"abcdefghijkl"
        .chunks(frame_bytes)
        .map(|chunk| source_frame(chunk, window_bytes))
        .collect()
}

pub(super) fn source_frame(bytes: &[u8], window_bytes: u64) -> BlobStreamingSourceFrame {
    BlobStreamingSourceFrame::from_bounded_bytes(
        bytes.to_vec(),
        BlobStreamingWindow::bounded(window_bytes).unwrap(),
    )
    .expect("bounded source frame should admit")
}

pub(super) fn allocation_receipt_and_envelope(
    envelope_bytes: u64,
) -> (
    AllocationReceipt,
    worth_store_budgets::AllocationEnvelopeSet,
) {
    let envelopes = allocation_envelope(envelope_bytes);
    let mut admission = AllocationAdmission::from_declaration(envelopes);
    let grant = admission
        .admit(
            AllocationRequest::streaming_window(AllocationScope::Streaming, envelope_bytes)
                .unwrap(),
        )
        .expect("streaming allocation should admit");
    let receipt = admission
        .record_allocation(grant)
        .expect("streaming allocation should record");
    (receipt, envelopes)
}

pub(super) fn pressure_admission() -> BlobStreamingPressureAdmission {
    BlobStreamingPressureAdmission::from_io_qos_background_capacity(
        blob_ingest_background_capacity_for_certification_test(background_budget()),
        1,
        false,
    )
    .expect("S.6 foreground page-write backed blob pressure should admit")
}

pub(super) fn background_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap())
}

pub(super) fn allocation_envelope(
    streaming_bytes: u64,
) -> worth_store_budgets::AllocationEnvelopeSet {
    let budget = AllocationByteBudget::bytes(64).unwrap();
    AllocationEnvelopeDeclaration::declare()
        .foreground(budget)
        .maintenance(budget)
        .recovery(budget)
        .scrub(budget)
        .import_export(budget)
        .streaming(AllocationByteBudget::bytes(streaming_bytes).unwrap())
        .fixed_metadata(FixedMetadataReservation::constant_bytes(16).unwrap())
        .seal()
        .unwrap()
}

fn rule() -> BlobChunkingRuleAdmission {
    BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(4).unwrap()).unwrap()
}

pub(super) struct TestChunkWriter {
    ordinal_offset: u64,
}

impl TestChunkWriter {
    pub(super) const fn new() -> Self {
        Self { ordinal_offset: 0 }
    }

    pub(super) const fn with_ordinal_offset(ordinal_offset: u64) -> Self {
        Self { ordinal_offset }
    }
}

impl BlobStreamingChunkWriter for TestChunkWriter {
    fn write_streaming_chunk(
        &mut self,
        ordinal: BlobChunkOrdinal,
        bytes: &[u8],
    ) -> Result<BlobStreamingWrittenChunk, BlobStreamingIngestDenial> {
        let payload = physical_payload_for_bytes(bytes);
        let backend = BlobBackendChunkWriteSession::for_certification_test_authority()
            .observe_store_chunk_payload(ordinal.get() + self.ordinal_offset, &payload)
            .expect("backend chunk write should observe");
        BlobStreamingWrittenChunk::from_store_chunk_write(payload, backend)
    }
}

pub(super) struct SubstitutingChunkWriter;

impl BlobStreamingChunkWriter for SubstitutingChunkWriter {
    fn write_streaming_chunk(
        &mut self,
        ordinal: BlobChunkOrdinal,
        bytes: &[u8],
    ) -> Result<BlobStreamingWrittenChunk, BlobStreamingIngestDenial> {
        let replacement = vec![b'X'; bytes.len()];
        let payload = physical_payload_for_bytes(&replacement);
        let backend = BlobBackendChunkWriteSession::for_certification_test_authority()
            .observe_store_chunk_payload(ordinal.get(), &payload)
            .expect("backend chunk write should observe");
        BlobStreamingWrittenChunk::from_store_chunk_write(payload, backend)
    }
}
