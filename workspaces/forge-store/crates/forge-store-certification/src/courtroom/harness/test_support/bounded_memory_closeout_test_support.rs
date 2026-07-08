pub(crate) use crate::bounded_memory_closeout_pressure_support::{
    harness_evidence, harness_evidence_for_class, harness_evidence_without_acceptance_suite,
    pressure_bundles, synthetic_rejections,
};

use super::record_view_evidence_test_support::{
    admit_payload_frame, allocation_admission, framed_record, resident_frame_table,
};
use crate::{
    BackgroundEnvelopeEvidenceBundle, BoundedMemoryOperationKind, BoundedOperationEnvelopeCounters,
    BoundedOperationEnvelopeReport, CompletedResidencyBoundaryReceipt, FoundationalEvidenceProfile,
    ProtectedIntegrityViewEvidence, RecordViewEvidenceReport, RecordViewEvidenceRow,
};
use forge_store_blob_chunks::LargeRecordStreamingEnvelope;
use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, AllocationRequest, AllocationScope, BackgroundEnvelopeAdmission,
    BackgroundEnvelopeDenialKind, BackgroundEnvelopeRequest, BackgroundMemoryInterferenceReport,
    BackgroundWorkBudgetSnapshot, BackgroundWorkClass, BufferPoolExecutedEvidenceSource,
    FixedMetadataReservation, RecordViewMaterializationProfile,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_maintenance::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};
use forge_store_physical_integrity::ScrubPlanningMemoryEnvelope;
use forge_store_readiness::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
    S2PhysicalSubstrateReadiness,
};
use forge_store_recovery_physics::RecoveryMemoryEnvelope;

pub(crate) fn background_bundle() -> BackgroundEnvelopeEvidenceBundle {
    BackgroundEnvelopeEvidenceBundle::from_envelopes(
        RecoveryMemoryEnvelope::from_admitted(admit(class_request(
            BackgroundWorkClass::RecoveryPlanning,
        )))
        .unwrap(),
        CompactionPlanningMemoryEnvelope::from_admitted(admit(class_request(
            BackgroundWorkClass::CompactionPlanning,
        )))
        .unwrap(),
        ScrubPlanningMemoryEnvelope::from_admitted(admit(class_request(
            BackgroundWorkClass::ScrubPlanning,
        )))
        .unwrap(),
        ImportExportMemoryEnvelope::from_admitted(admit(class_request(
            BackgroundWorkClass::ImportExport,
        )))
        .unwrap(),
        LargeRecordStreamingEnvelope::from_admitted(admit(streaming_request(4096, 256))).unwrap(),
        &complete_interference_reports(),
    )
    .unwrap()
}

pub(crate) fn foundational_receipt() -> CompletedResidencyBoundaryReceipt {
    foundational_receipt_with_protected_view().0
}

pub(crate) fn foundational_receipt_with_protected_view() -> (
    CompletedResidencyBoundaryReceipt,
    ProtectedIntegrityViewEvidence,
) {
    let payload = b"bounded-memory-boundary";
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, 17, 2, payload);
    table
        .resident_frame(admission.resident_frame_token())
        .unwrap();
    let mut allocation = allocation_admission(64);
    let (bounded, zero_copy_report) = {
        let framed = framed_record(17, 2, payload);
        let lease = table.lease_page(admission.resident_frame_token()).unwrap();
        let mut pinned = lease.pin().unwrap();
        let zero_copy = pinned
            .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
            .unwrap();
        let zero_copy_report = RecordViewEvidenceReport::from_zero_copy_view(
            RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes,
            &zero_copy,
        )
        .unwrap();
        let request =
            AllocationRequest::copied_payload(AllocationScope::Foreground, payload.len() as u64)
                .unwrap();
        let grant = allocation.admit(request).unwrap();
        let receipt = allocation.record_allocation(grant).unwrap();
        (zero_copy.bounded_copy(receipt).unwrap(), zero_copy_report)
    };
    let source =
        BufferPoolExecutedEvidenceSource::from_store_execution(&table, &allocation, &bounded)
            .unwrap();
    let receipt = CompletedResidencyBoundaryReceipt::from_executed_store_counters(
        source,
        FoundationalEvidenceProfile::reduced(),
    )
    .unwrap();
    let protected =
        ProtectedIntegrityViewEvidence::from_zero_copy_report(zero_copy_report, &receipt, true)
            .unwrap();
    (receipt, protected)
}

pub(crate) fn operation_reports(
    receipt: &CompletedResidencyBoundaryReceipt,
    background: &BackgroundEnvelopeEvidenceBundle,
) -> Vec<BoundedOperationEnvelopeReport> {
    let foreground = foreground_counters(receipt);
    vec![
        BoundedOperationEnvelopeReport::from_counters(
            BoundedMemoryOperationKind::AdmittedRead,
            foreground,
        )
        .unwrap(),
        BoundedOperationEnvelopeReport::from_counters(
            BoundedMemoryOperationKind::AdmittedWrite,
            foreground,
        )
        .unwrap(),
        background_report(background, BackgroundWorkClass::RecoveryPlanning),
        background_report(background, BackgroundWorkClass::CompactionPlanning),
        background_report(background, BackgroundWorkClass::LargeRecordStreaming),
    ]
}

pub(crate) fn s2_readiness() -> S2PhysicalSubstrateReadiness {
    prove_s2_physical_substrate_readiness(
        close_s1_physical_substrate_readiness(
            AcceptedHandoffReadiness::from_s0_artifacts(
                ROADMAP_2_S1_SCOPE,
                HandoffEvidenceDigestSet::new(
                    StableDigest::new("sha256:bounded-memory-backend").unwrap(),
                    StableDigest::new("sha256:bounded-memory-deferred").unwrap(),
                    StableDigest::new("sha256:bounded-memory-harness").unwrap(),
                    StableDigest::new("sha256:bounded-memory-terms").unwrap(),
                    StableDigest::new("sha256:bounded-memory-audit").unwrap(),
                    StableDigest::new("sha256:bounded-memory-complexity").unwrap(),
                    StableDigest::new("sha256:bounded-memory-provenance").unwrap(),
                ),
            )
            .unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn complete_interference_reports() -> [BackgroundMemoryInterferenceReport; 6] {
    [
        deny(
            BackgroundEnvelopeRequest::recovery_planning()
                .resident_frames(3)
                .resident_bytes(384)
                .allocation_bytes(128)
                .finish(),
            BackgroundEnvelopeDenialKind::ForegroundResidencyInterference {
                requested_frames: 3,
                background_available_frames: 2,
                foreground_reserved_frames: 2,
            },
        ),
        deny(
            BackgroundEnvelopeRequest::scrub_planning()
                .pin_indefinitely(1)
                .allocation_bytes(128)
                .finish(),
            BackgroundEnvelopeDenialKind::IndefinitePinRequested { requested_pages: 1 },
        ),
        deny(
            BackgroundEnvelopeRequest::compaction_planning()
                .pin_pages_for_bounded_step(2)
                .allocation_bytes(128)
                .finish(),
            BackgroundEnvelopeDenialKind::PinBudgetWouldBeExceeded {
                requested_pages: 2,
                pinned_pages_used: 1,
                pinned_page_budget: 2,
            },
        ),
        deny(
            BackgroundEnvelopeRequest::large_record_streaming()
                .allocation_bytes(128)
                .streaming_window(4096, 128)
                .whole_object_memory_bytes(4096)
                .finish(),
            BackgroundEnvelopeDenialKind::WholeObjectMemoryRequired {
                object_bytes: 4096,
                envelope_bytes: 128,
            },
        ),
        deny(
            BackgroundEnvelopeRequest::large_record_streaming()
                .allocation_bytes(128)
                .streaming_window(4096, 256)
                .finish(),
            BackgroundEnvelopeDenialKind::StreamingWindowExceedsEnvelope {
                window_bytes: 256,
                envelope_bytes: 128,
            },
        ),
        deny(
            BackgroundEnvelopeRequest::large_record_streaming()
                .allocation_bytes(256)
                .streaming_window(4096, 128)
                .finish(),
            BackgroundEnvelopeDenialKind::StreamingEnvelopeExceedsWindow {
                envelope_bytes: 256,
                window_bytes: 128,
            },
        ),
    ]
}

fn foreground_counters(
    receipt: &CompletedResidencyBoundaryReceipt,
) -> BoundedOperationEnvelopeCounters {
    let resident = receipt.resident_memory().counters();
    let copy = receipt.copy_materialization().counters();
    BoundedOperationEnvelopeCounters::exact(
        resident.resident_bytes().as_bytes(),
        resident
            .pin_lifecycle()
            .successful_pin_count()
            .max(resident.pin_lifecycle().active_pinned_pages()),
        resident.dirty_state().dirty_pages().as_pages(),
        receipt
            .allocation()
            .counters()
            .scope(AllocationScope::Foreground)
            .allocated_bytes(),
        copy.copied_bytes(),
        copy.materialized_bytes(),
    )
}

fn background_report(
    background: &BackgroundEnvelopeEvidenceBundle,
    work_class: BackgroundWorkClass,
) -> BoundedOperationEnvelopeReport {
    let counters = background.envelope_for(work_class).unwrap().counters();
    let operation = match work_class {
        BackgroundWorkClass::RecoveryPlanning => BoundedMemoryOperationKind::RecoveryPlanning,
        BackgroundWorkClass::CompactionPlanning => BoundedMemoryOperationKind::CompactionPlanning,
        BackgroundWorkClass::LargeRecordStreaming => {
            BoundedMemoryOperationKind::LargeRecordStreaming
        }
        BackgroundWorkClass::ScrubPlanning | BackgroundWorkClass::ImportExport => {
            unreachable!("phase 12 operation report does not close this class")
        }
    };
    BoundedOperationEnvelopeReport::from_counters(
        operation,
        BoundedOperationEnvelopeCounters::exact(
            counters.resident_bytes_admitted(),
            counters.pinned_pages_admitted() as u64,
            0,
            counters.allocation_bytes_allocated(),
            counters.copied_bytes(),
            0,
        ),
    )
    .unwrap()
}

fn deny(
    request: BackgroundEnvelopeRequest,
    expected: BackgroundEnvelopeDenialKind,
) -> BackgroundMemoryInterferenceReport {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_envelopes();
    let report = admission
        .admit(request, budget_for(expected), &mut allocation)
        .unwrap_err();
    assert_eq!(report.kind(), expected);
    report
}

fn admit(request: BackgroundEnvelopeRequest) -> AdmittedBackgroundEnvelope {
    BackgroundEnvelopeAdmission::new()
        .admit(request, permissive_budget(), &mut allocation_envelopes())
        .unwrap()
}

fn class_request(class: BackgroundWorkClass) -> BackgroundEnvelopeRequest {
    match class {
        BackgroundWorkClass::RecoveryPlanning => BackgroundEnvelopeRequest::recovery_planning(),
        BackgroundWorkClass::CompactionPlanning => BackgroundEnvelopeRequest::compaction_planning(),
        BackgroundWorkClass::ScrubPlanning => BackgroundEnvelopeRequest::scrub_planning(),
        BackgroundWorkClass::ImportExport => BackgroundEnvelopeRequest::import_export(),
        BackgroundWorkClass::LargeRecordStreaming => return streaming_request(4096, 256),
    }
    .resident_frames(1)
    .resident_bytes(128)
    .pin_pages_for_bounded_step(1)
    .allocation_bytes(128)
    .finish()
}

fn streaming_request(object_bytes: u64, window_bytes: u64) -> BackgroundEnvelopeRequest {
    BackgroundEnvelopeRequest::large_record_streaming()
        .resident_frames(1)
        .resident_bytes(window_bytes)
        .allocation_bytes(window_bytes)
        .copied_bytes(window_bytes)
        .streaming_window(object_bytes, window_bytes)
        .finish()
}

fn allocation_envelopes() -> AllocationAdmission {
    AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(512))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    )
}

fn budget_for(expected: BackgroundEnvelopeDenialKind) -> BackgroundWorkBudgetSnapshot {
    match expected {
        BackgroundEnvelopeDenialKind::ForegroundResidencyInterference { .. } => {
            BackgroundWorkBudgetSnapshot::foreground_reserved(4, 2, 0, 8)
        }
        BackgroundEnvelopeDenialKind::PinBudgetWouldBeExceeded { .. } => {
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 1, 2)
        }
        _ => permissive_budget(),
    }
}

fn permissive_budget() -> BackgroundWorkBudgetSnapshot {
    BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16)
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}
