use forge_store_budgets::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationScope, CounterEvidenceStrength,
    FixedMetadataReservation,
};
use forge_store_buffer_pool::{AllocationAdmission, AllocationRequest};
use forge_store_io_scheduler::{
    blob_ingest_deferred_background_capacity_for_certification_test,
    blob_ingest_denied_background_capacity_for_certification_test,
    blob_ingest_rebind_background_capacity_for_certification_test,
    blob_ingest_stale_background_capacity_for_certification_test,
    blob_ingest_throttled_background_capacity_for_certification_test,
    checkpoint_flush_wal_background_capacity_for_certification_test, BackgroundIoPressureClass,
    BackgroundPacingStaleRebindKind, BackgroundResourceBudget, QueueSlot,
};
use forge_store_physical_backend::BlobBackendChunkWriteSession;
use forge_store_security::StoreTenantScope;

use crate::test_support::physical_payload_for_bytes;
use crate::test_support::blob_scope;
use crate::{
    BlobChunkOrdinal, BlobChunkSize, BlobChunkingRuleAdmission, BlobStreamingChunkWriter,
    BlobStreamingIngest, BlobStreamingIngestDenial, BlobStreamingIngestRequest,
    BlobStreamingPressureAdmission, BlobStreamingSourceFrame, BlobStreamingWindow,
    BlobStreamingWrittenChunk,
};

#[test]
fn non_blob_s6_background_capacity_cannot_enter_blob_ingest_pressure() {
    let denial = BlobStreamingPressureAdmission::from_s6_background_capacity(
        checkpoint_flush_wal_background_capacity_for_certification_test(background_budget()),
        1,
        false,
    )
    .expect_err("non-blob S.6 background capacity must not satisfy blob ingest pressure");

    assert_eq!(
        denial,
        BlobStreamingIngestDenial::BackgroundPressureClassMismatch {
            actual: BackgroundIoPressureClass::CheckpointFlush
        }
    );
}

#[test]
fn throttled_blob_pressure_paces_ingest_with_exact_scheduler_counters() {
    let requested = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(2).unwrap());
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    let pressure = BlobStreamingPressureAdmission::from_s6_background_capacity(
        blob_ingest_throttled_background_capacity_for_certification_test(requested, admitted),
        0,
        false,
    )
    .expect("throttled S.6 blob pressure should admit into blob pressure");
    let ingest = run_ingest(pressure).expect("throttled blob pressure should pace ingest");

    assert_eq!(ingest.counters().scheduler_throttles(), 1);
    assert_eq!(ingest.counters().scheduler_admissions(), 1);
    assert_eq!(ingest.counters().scheduler_yields(), 0);
    assert_eq!(ingest.counters().chunks_read(), 3);
}

#[test]
fn deferred_denied_and_stale_blob_pressure_deny_before_source_consumption() {
    let requested = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(3).unwrap());
    let admitted = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
    let debt_limit = BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());

    assert_pressure_denial(
        blob_ingest_deferred_background_capacity_for_certification_test(requested),
        BlobStreamingIngestDenial::BackgroundPressureDeferred,
    );
    assert_pressure_denial(
        blob_ingest_denied_background_capacity_for_certification_test(
            requested, admitted, debt_limit,
        ),
        BlobStreamingIngestDenial::BackgroundPressureDenied,
    );
    assert_pressure_denial(
        blob_ingest_stale_background_capacity_for_certification_test(background_budget()),
        BlobStreamingIngestDenial::BackgroundPressureStale {
            kind: BackgroundPacingStaleRebindKind::Stale,
        },
    );
    assert_pressure_denial(
        blob_ingest_rebind_background_capacity_for_certification_test(background_budget()),
        BlobStreamingIngestDenial::BackgroundPressureStale {
            kind: BackgroundPacingStaleRebindKind::RebindRequired,
        },
    );
}

fn background_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap())
}

fn assert_pressure_denial(
    capacity: forge_store_io_scheduler::BackgroundCapacityAdmission,
    expected: BlobStreamingIngestDenial,
) {
    let pressure = BlobStreamingPressureAdmission::from_s6_background_capacity(capacity, 0, false)
        .expect("S.6 blob pressure admission should build");
    let denial = run_ingest(pressure).expect_err("non-current blob pressure must deny ingest");
    assert_eq!(denial, expected);
}

fn run_ingest(
    pressure: BlobStreamingPressureAdmission,
) -> Result<BlobStreamingIngest, BlobStreamingIngestDenial> {
    let envelopes = allocation_envelope();
    let mut admission = AllocationAdmission::from_declaration(envelopes);
    let grant = admission
        .admit(AllocationRequest::streaming_window(AllocationScope::Streaming, 4).unwrap())
        .expect("streaming allocation should admit");
    let allocation = admission
        .record_allocation(grant)
        .expect("streaming allocation should record");
    BlobStreamingIngest::run_bounded(
        request(),
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation,
        envelopes,
        pressure,
        source_frames(),
        &mut TestChunkWriter,
        CounterEvidenceStrength::Exact,
    )
}

fn request() -> BlobStreamingIngestRequest {
    BlobStreamingIngestRequest::new(
        blob_scope(
            "phase9.streaming.pressure",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(4).unwrap()).unwrap(),
        12,
    )
    .unwrap()
}

fn source_frames() -> Vec<BlobStreamingSourceFrame> {
    b"abcdefghijkl"
        .chunks(3)
        .map(|chunk| {
            BlobStreamingSourceFrame::from_bounded_bytes(
                chunk.to_vec(),
                BlobStreamingWindow::bounded(4).unwrap(),
            )
            .expect("bounded source frame should admit")
        })
        .collect()
}

fn allocation_envelope() -> forge_store_budgets::AllocationEnvelopeSet {
    let budget = AllocationByteBudget::bytes(64).unwrap();
    AllocationEnvelopeDeclaration::declare()
        .foreground(budget)
        .maintenance(budget)
        .recovery(budget)
        .scrub(budget)
        .import_export(budget)
        .streaming(AllocationByteBudget::bytes(4).unwrap())
        .fixed_metadata(FixedMetadataReservation::constant_bytes(16).unwrap())
        .seal()
        .unwrap()
}

struct TestChunkWriter;

impl BlobStreamingChunkWriter for TestChunkWriter {
    fn write_streaming_chunk(
        &mut self,
        ordinal: BlobChunkOrdinal,
        bytes: &[u8],
    ) -> Result<BlobStreamingWrittenChunk, BlobStreamingIngestDenial> {
        let payload = physical_payload_for_bytes(bytes);
        let backend = BlobBackendChunkWriteSession::for_certification_test_authority()
            .observe_store_chunk_payload(ordinal.get(), &payload)
            .expect("backend chunk write should observe");
        BlobStreamingWrittenChunk::from_store_chunk_write(payload, backend)
    }
}
