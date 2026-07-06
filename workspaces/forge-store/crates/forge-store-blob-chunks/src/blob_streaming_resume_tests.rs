use forge_store_budgets::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationScope, CounterEvidenceStrength,
    FixedMetadataReservation,
};
use forge_store_buffer_pool::{AllocationAdmission, AllocationRequest};
use forge_store_io_scheduler::{
    blob_ingest_background_capacity_for_certification_test, BackgroundResourceBudget, QueueSlot,
};
use forge_store_physical_backend::BlobBackendChunkWriteSession;
use forge_store_security::StoreTenantScope;

use crate::blob_chunk_physical_test_support::physical_payload_for_bytes;
use crate::blob_chunk_test_support::{admitted_multichunk_sequence_for_scope, blob_scope};
use crate::blob_generation_registry_test_support::current_authority;
use crate::{
    run_resumable_streaming_ingest, BlobChunkOrdinal, BlobChunkSize, BlobChunkingRuleAdmission,
    BlobResumeSessionAdmitted, BlobResumeSessionDeclaration, BlobResumeStoreAuthority,
    BlobStreamingChunkWriter, BlobStreamingIngestDenial, BlobStreamingIngestRequest,
    BlobStreamingPressureAdmission, BlobStreamingResumeAdmission, BlobStreamingSourceFrame,
    BlobStreamingWindow, BlobStreamingWrittenChunk,
};

#[test]
fn public_streaming_ingest_requires_and_records_resume_session_admission() {
    let session = admitted_resume_session();
    let resume_admission = BlobStreamingResumeAdmission::from_admitted_resume_session(&session);
    let envelopes = allocation_envelope();
    let mut allocation = AllocationAdmission::from_declaration(envelopes);
    let grant = allocation
        .admit(AllocationRequest::streaming_window(AllocationScope::Streaming, 4).unwrap())
        .unwrap();
    let allocation = allocation.record_allocation(grant).unwrap();
    let pressure = BlobStreamingPressureAdmission::from_s6_background_capacity(
        blob_ingest_background_capacity_for_certification_test(background_budget()),
        1,
        false,
    )
    .unwrap();

    let ingest = run_resumable_streaming_ingest(
        request(),
        resume_admission,
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation,
        envelopes,
        pressure,
        source_frames(),
        &mut TestChunkWriter,
        CounterEvidenceStrength::Exact,
    )
    .unwrap();

    assert_eq!(
        ingest.resumability().resume_session_digest(),
        Some(session.export_session_id().as_str())
    );
}

#[test]
fn public_streaming_ingest_denies_request_not_bound_to_resume_session() {
    let session = admitted_resume_session();
    let resume_admission = BlobStreamingResumeAdmission::from_admitted_resume_session(&session);
    let envelopes = allocation_envelope();
    let mut allocation = AllocationAdmission::from_declaration(envelopes);
    let grant = allocation
        .admit(AllocationRequest::streaming_window(AllocationScope::Streaming, 4).unwrap())
        .unwrap();
    let allocation = allocation.record_allocation(grant).unwrap();
    let pressure = BlobStreamingPressureAdmission::from_s6_background_capacity(
        blob_ingest_background_capacity_for_certification_test(background_budget()),
        1,
        false,
    )
    .unwrap();

    let denial = run_resumable_streaming_ingest(
        request_for_total_bytes(8),
        resume_admission,
        BlobStreamingWindow::bounded(4).unwrap(),
        allocation,
        envelopes,
        pressure,
        source_frames(),
        &mut TestChunkWriter,
        CounterEvidenceStrength::Exact,
    )
    .expect_err("mismatched request must not enter resume-bound ingest");

    assert_eq!(
        denial,
        BlobStreamingIngestDenial::ResumeSessionRequestMismatch
    );
}

fn admitted_resume_session() -> BlobResumeSessionAdmitted {
    let sequence = admitted_multichunk_sequence_for_scope(
        blob_scope(
            "phase12.resume.streaming",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        b"abcdefghijkl",
        4,
    );
    let leaf = sequence.proof_frontier().ordered_leaves()[0].clone();
    let rule =
        BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(4).unwrap()).unwrap();
    let declaration =
        BlobResumeSessionDeclaration::new(leaf.security_metadata(), rule, 12).unwrap();
    let authority = current_authority("phase12.resume.streaming.authority", "resume");
    BlobResumeSessionAdmitted::admit(
        declaration,
        BlobResumeStoreAuthority::from_current_store_authority(authority),
    )
}

fn request() -> BlobStreamingIngestRequest {
    request_for_total_bytes(12)
}

fn request_for_total_bytes(total_bytes: u64) -> BlobStreamingIngestRequest {
    BlobStreamingIngestRequest::new(
        blob_scope(
            "phase12.resume.streaming",
            StoreTenantScope::TenantPhysicalBoundary,
        ),
        BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(4).unwrap()).unwrap(),
        total_bytes,
    )
    .unwrap()
}

fn source_frames() -> Vec<BlobStreamingSourceFrame> {
    b"abcdefghijkl"
        .chunks(4)
        .map(|chunk| {
            BlobStreamingSourceFrame::from_bounded_bytes(
                chunk.to_vec(),
                BlobStreamingWindow::bounded(4).unwrap(),
            )
            .unwrap()
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

fn background_budget() -> BackgroundResourceBudget {
    BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap())
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
            .unwrap();
        BlobStreamingWrittenChunk::from_store_chunk_write(payload, backend)
    }
}
