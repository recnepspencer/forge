use worth_store_budgets::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationScope, CounterEvidenceStrength,
    FixedMetadataReservation,
};
use worth_store_buffer_pool::{AllocationAdmission, AllocationReceipt, AllocationRequest};
use worth_store_io_scheduler::{
    blob_ingest_background_capacity_for_certification_test,
    blob_ingest_wal_write_background_capacity_for_certification_test,
    foreground_reservation::admitted_point_read_reservation_for_certification_test,
    BackgroundResourceBudget, QueueSlot,
};
use worth_store_physical_backend::{
    BlobBackendChunkWriteObservation, BlobBackendChunkWriteSession,
};
use worth_store_security::StoreTenantScope;

use crate::test_support::blob_scope;
use crate::test_support::physical_payload_for_bytes;
use crate::{
    reject_full_blob_vec_as_streaming_ingest, reject_scalar_backend_api_as_streaming_ingest,
    BlobChunkOrdinal, BlobChunkSize, BlobChunkingRuleAdmission, BlobStreamingChunkWriter,
    BlobStreamingIngest, BlobStreamingIngestDenial, BlobStreamingIngestRequest,
    BlobStreamingPressureAdmission, BlobStreamingSourceFrame, BlobStreamingWindow,
    BlobStreamingWrittenChunk,
};

#[test]
fn bounded_window_size_drives_source_streaming_without_changing_chunk_sequence() {
    let small = run_ingest(source_frames(3, 4), 4).unwrap();
    let large = run_ingest(source_frames(5, 8), 8).unwrap();

    assert_eq!(
        small.sequence().chunk_identity_summary(),
        large.sequence().chunk_identity_summary()
    );
    assert_eq!(
        small.frontier().chunk_tree_root(),
        large.frontier().chunk_tree_root()
    );
    assert_eq!(
        small.frontier().logical_content_digest(),
        large.frontier().logical_content_digest()
    );
    assert_eq!(small.counters().bytes_streamed(), 12);
    assert_eq!(large.counters().bytes_streamed(), 12);
    assert_eq!(small.counters().windows_observed(), 4);
    assert_eq!(large.counters().windows_observed(), 3);
    assert_eq!(small.counters().chunks_read(), 3);
    assert_eq!(large.counters().chunks_read(), 3);
    assert_eq!(small.counters().chunks_written(), 3);
    assert_eq!(large.counters().chunks_written(), 3);
    assert_eq!(small.counters().backend_write_observations(), 3);
    assert_eq!(large.counters().backend_write_observations(), 3);
    assert_eq!(small.residency().peak_resident_bytes(), 4);
    assert_eq!(large.residency().peak_resident_bytes(), 4);
    assert_eq!(small.counters().scheduler_yields(), 1);
    assert_eq!(small.counters().scheduler_waits(), 1);
    assert!(small
        .counter_backed_performance_receipt()
        .counter_rows()
        .iter()
        .any(|row| row.name().as_str().ends_with(".chunks_read") && row.observed_count() == 3));
    assert!(small
        .counter_backed_performance_receipt()
        .counter_rows()
        .iter()
        .any(|row| row.name().as_str().ends_with(".scheduler_waits") && row.observed_count() == 1));
}

#[test]
fn whole_object_scalar_missing_counter_and_envelope_shortcuts_are_denied() {
    assert_eq!(
        reject_full_blob_vec_as_streaming_ingest(b"whole-object".to_vec()),
        BlobStreamingIngestDenial::WholeObjectMaterializationRejected { bytes: 12 }
    );

    let scalar = BlobBackendChunkWriteObservation::reject_scalar_framed_record_api(4);
    assert_eq!(
        reject_scalar_backend_api_as_streaming_ingest(scalar),
        BlobStreamingIngestDenial::ScalarBackendCertificationRejected
    );

    let missing_exact = run_ingest_with_counter_strength(CounterEvidenceStrength::Sampled)
        .expect_err("sampled counters cannot prove streaming residency");
    assert!(matches!(
        missing_exact,
        BlobStreamingIngestDenial::MissingExactCounters {
            actual: CounterEvidenceStrength::Sampled
        }
    ));

    let envelope = run_ingest_with_envelope(2)
        .expect_err("peak resident bytes above the bounded window must deny");
    assert!(matches!(
        envelope,
        BlobStreamingIngestDenial::ResidentEnvelopeExceeded {
            peak_resident_bytes: 4,
            envelope_bytes: 2
        }
    ));

    let scalar_chunk = BlobStreamingWrittenChunk::from_store_chunk_write(
        physical_payload_for_bytes(b"abcd"),
        BlobBackendChunkWriteObservation::reject_scalar_framed_record_api(4),
    );
    assert_eq!(
        scalar_chunk,
        Err(BlobStreamingIngestDenial::ScalarBackendCertificationRejected)
    );
}

#[test]
fn pressure_violation_denies_before_blob_ingest_consumes_source_frames() {
    let pressure = BlobStreamingPressureAdmission::from_s6_background_capacity(
        blob_ingest_background_capacity_for_certification_test(background_budget()),
        1,
        true,
    )
    .expect("S.6 pressure admission should build");
    let denial = BlobStreamingIngest::run_bounded(
        request(),
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation_receipt_and_envelope(4).0,
        allocation_envelope(4),
        pressure,
        source_frames(3, 4),
        &mut TestChunkWriter::new(),
        CounterEvidenceStrength::Exact,
    )
    .expect_err("S.6 pressure violation must deny blob ingest");
    assert_eq!(
        denial,
        BlobStreamingIngestDenial::BackgroundPressureViolation
    );
}

#[test]
fn full_object_source_frame_and_unbound_read_reservation_pressure_are_denied() {
    let whole_object = BlobStreamingIngest::run_bounded(
        request_for_total_bytes(4),
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation_receipt_and_envelope(4).0,
        allocation_envelope(4),
        pressure_admission(),
        [source_frame(b"abcd", 4)],
        &mut TestChunkWriter::new(),
        CounterEvidenceStrength::Exact,
    )
    .expect_err("source frame cannot materialize the whole object");
    assert_eq!(
        whole_object,
        BlobStreamingIngestDenial::WholeObjectMaterializationRejected { bytes: 4 }
    );

    let read_lane = BlobStreamingPressureAdmission::reject_unbound_foreground_reservation(
        admitted_point_read_reservation_for_certification_test(),
    );
    assert!(matches!(
        read_lane,
        BlobStreamingIngestDenial::ForegroundReservationLaneMismatch { .. }
    ));
}

#[test]
fn backend_write_observations_must_match_chunk_order_and_bytes() {
    let denial = BlobStreamingIngest::run_bounded(
        request(),
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation_receipt_and_envelope(4).0,
        allocation_envelope(4),
        pressure_admission(),
        source_frames(3, 4),
        &mut TestChunkWriter::with_ordinal_offset(1),
        CounterEvidenceStrength::Exact,
    )
    .expect_err("mismatched backend ordinal must deny");

    assert_eq!(
        denial,
        BlobStreamingIngestDenial::BackendWriteOrdinalMismatch {
            expected: 0,
            actual: 1
        }
    );
}

#[test]
fn backend_writer_payload_must_match_pending_source_bytes() {
    let denial = BlobStreamingIngest::run_bounded(
        request(),
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation_receipt_and_envelope(4).0,
        allocation_envelope(4),
        pressure_admission(),
        source_frames(3, 4),
        &mut SubstitutingChunkWriter,
        CounterEvidenceStrength::Exact,
    )
    .expect_err("writer-chosen payload bytes cannot replace streamed source bytes");

    assert_eq!(
        denial,
        BlobStreamingIngestDenial::BackendWritePayloadMismatch { ordinal: 0 }
    );
}

#[test]
fn blob_ingest_pressure_admits_against_wal_foreground_reservation() {
    let pressure = BlobStreamingPressureAdmission::from_s6_background_capacity(
        blob_ingest_wal_write_background_capacity_for_certification_test(background_budget()),
        1,
        false,
    )
    .expect("S.6 foreground WAL backed blob pressure should admit");
    let ingest = run_ingest_with_pressure(pressure).expect("WAL foreground pressure should yield");
    assert_eq!(ingest.counters().scheduler_yields(), 1);
    assert_eq!(ingest.counters().scheduler_waits(), 1);
}

fn run_ingest(
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
fn run_ingest_with_counter_strength(
    strength: CounterEvidenceStrength,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    run_ingest_with_window_and_envelope(source_frames(3, 4), 4, 4, strength)
}
fn run_ingest_with_envelope(
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
        window,
        allocation,
        envelopes,
        pressure_admission(),
        frames,
        &mut TestChunkWriter::new(),
        strength,
    )
}

fn run_ingest_with_pressure(
    pressure: BlobStreamingPressureAdmission,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    let (allocation, envelopes) = allocation_receipt_and_envelope(4);
    BlobStreamingIngest::run_bounded(
        request(),
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation,
        envelopes,
        pressure,
        source_frames(3, 4),
        &mut TestChunkWriter::new(),
        CounterEvidenceStrength::Exact,
    )
}

fn request() -> BlobStreamingIngestRequest {
    request_for_total_bytes(12)
}

fn request_for_total_bytes(total_bytes: u64) -> BlobStreamingIngestRequest {
    BlobStreamingIngestRequest::new(
        blob_scope("phase9.streaming", StoreTenantScope::TenantPhysicalBoundary),
        rule(),
        total_bytes,
    )
    .unwrap()
}

fn source_frames(frame_bytes: usize, window_bytes: u64) -> Vec<BlobStreamingSourceFrame> {
    b"abcdefghijkl"
        .chunks(frame_bytes)
        .map(|chunk| source_frame(chunk, window_bytes))
        .collect()
}

fn source_frame(bytes: &[u8], window_bytes: u64) -> BlobStreamingSourceFrame {
    BlobStreamingSourceFrame::from_bounded_bytes(
        bytes.to_vec(),
        BlobStreamingWindow::bounded(window_bytes).unwrap(),
    )
    .expect("bounded source frame should admit")
}

fn allocation_receipt_and_envelope(
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

fn pressure_admission() -> BlobStreamingPressureAdmission {
    BlobStreamingPressureAdmission::from_s6_background_capacity(
        blob_ingest_background_capacity_for_certification_test(background_budget()),
        1,
        false,
    )
    .expect("S.6 foreground page-write backed blob pressure should admit")
}

fn background_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap())
}
fn allocation_envelope(streaming_bytes: u64) -> worth_store_budgets::AllocationEnvelopeSet {
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
struct TestChunkWriter {
    ordinal_offset: u64,
}

impl TestChunkWriter {
    const fn new() -> Self {
        Self { ordinal_offset: 0 }
    }
    const fn with_ordinal_offset(ordinal_offset: u64) -> Self {
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
struct SubstitutingChunkWriter;

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
