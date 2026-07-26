use crate::courtroom::harness::test_support::recovery_memory_allocation_test_support::{
    operation_allocation, recovery_memory_allocation,
};
use crate::{
    BackgroundEnvelopeEvidenceBundle, BackgroundEnvelopeEvidenceDenial, RequiredInterferenceKind,
};
use worth_store_blob_chunks::LargeRecordStreamingEnvelope;
use worth_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationAdmission, AllocationByteBudget,
    AllocationEnvelopeDeclaration, BackgroundEnvelopeAdmission, BackgroundEnvelopeDenialKind,
    BackgroundEnvelopeRequest, BackgroundMemoryInterferenceReport, BackgroundWorkBudgetSnapshot,
    BackgroundWorkClass, FixedMetadataReservation, PhysicalOperationAllocationScope,
};
use worth_store_maintenance::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};
use worth_store_physical_integrity::ScrubPlanningMemoryEnvelope;
use worth_store_recovery_physics::{RecoveryMemoryAllocation, RecoveryMemoryAllocationDenial};

#[test]
fn background_envelope_honesty_suite_certifies_all_classes_and_interference() {
    let recovery = recovery_memory_allocation();
    let compaction =
        CompactionPlanningMemoryEnvelope::from_allocation_grant(maintenance_allocation())
            .expect("maintenance allocation authorizes compaction planning");
    let scrub = ScrubPlanningMemoryEnvelope::from_admitted(admit(class_request(
        BackgroundWorkClass::ScrubPlanning,
    )))
    .expect("scrub wrapper consumes scrub envelope");
    let import_export = ImportExportMemoryEnvelope::from_allocation_grant(maintenance_allocation())
        .expect("maintenance allocation authorizes import-export work");
    let streaming = LargeRecordStreamingEnvelope::from_admitted(admit(streaming_request(128, 512)))
        .expect("streaming wrapper consumes streaming envelope");
    let reports = complete_interference_reports();
    assert_eq!(
        recovery.allocation_scope(),
        PhysicalOperationAllocationScope::Recovery
    );

    assert!(!compaction.proves_compaction_validity());
    let bundle = BackgroundEnvelopeEvidenceBundle::from_envelopes(
        recovery,
        compaction,
        scrub,
        import_export,
        streaming,
        &reports,
    )
    .expect("complete phase 10 evidence certifies");

    assert_eq!(bundle.admitted_classes(), BackgroundWorkClass::ALL);
    assert_eq!(streaming.object_bytes(), 128);
    assert_eq!(streaming.window_bytes(), 512);
    assert!(!scrub.proves_corruption_localization());
    assert!(!streaming.proves_blob_lifecycle_completion());
}

#[test]
fn evidence_rejects_missing_required_interference_reports() {
    let denial = BackgroundEnvelopeEvidenceBundle::from_envelopes(
        recovery_memory_allocation(),
        CompactionPlanningMemoryEnvelope::from_allocation_grant(maintenance_allocation()).unwrap(),
        ScrubPlanningMemoryEnvelope::from_admitted(admit(class_request(
            BackgroundWorkClass::ScrubPlanning,
        )))
        .unwrap(),
        ImportExportMemoryEnvelope::from_allocation_grant(maintenance_allocation()).unwrap(),
        LargeRecordStreamingEnvelope::from_admitted(admit(streaming_request(4096, 512))).unwrap(),
        &[],
    )
    .expect_err("evidence requires typed interference reports");

    assert_eq!(
        denial,
        BackgroundEnvelopeEvidenceDenial::MissingInterferenceReport(
            RequiredInterferenceKind::ForegroundResidency
        )
    );
}

#[test]
fn evidence_rejects_each_missing_interference_report() {
    for required in [
        RequiredInterferenceKind::ForegroundResidency,
        RequiredInterferenceKind::IndefinitePin,
        RequiredInterferenceKind::PinBudgetPressure,
        RequiredInterferenceKind::WholeObject,
        RequiredInterferenceKind::StreamingWindowExceedsEnvelope,
        RequiredInterferenceKind::StreamingEnvelopeExceedsWindow,
    ] {
        let reports = complete_interference_reports();
        let incomplete_reports: Vec<_> = reports
            .into_iter()
            .filter(|report| !report_matches_required_kind(*report, required))
            .collect();

        let denial = BackgroundEnvelopeEvidenceBundle::from_envelopes(
            recovery_memory_allocation(),
            CompactionPlanningMemoryEnvelope::from_allocation_grant(maintenance_allocation())
                .unwrap(),
            ScrubPlanningMemoryEnvelope::from_admitted(admit(class_request(
                BackgroundWorkClass::ScrubPlanning,
            )))
            .unwrap(),
            ImportExportMemoryEnvelope::from_allocation_grant(maintenance_allocation()).unwrap(),
            LargeRecordStreamingEnvelope::from_admitted(admit(streaming_request(4096, 512)))
                .unwrap(),
            &incomplete_reports,
        )
        .expect_err("each phase 10 interference class is required independently");

        assert_eq!(
            denial,
            BackgroundEnvelopeEvidenceDenial::MissingInterferenceReport(required)
        );
    }
}

#[test]
fn recovery_allocation_rejects_wrong_operation_scope() {
    let maintenance = operation_allocation(PhysicalOperationAllocationScope::Maintenance, 128)
        .expect("bounded maintenance allocation should admit");
    let denial = RecoveryMemoryAllocation::from_allocation_grant(maintenance)
        .expect_err("maintenance allocation cannot authorize recovery");

    assert_eq!(
        denial,
        RecoveryMemoryAllocationDenial::WrongAllocationScope {
            actual: PhysicalOperationAllocationScope::Maintenance,
        }
    );
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

fn maintenance_allocation() -> worth_store_buffer_pool::OperationAllocationGrant {
    operation_allocation(PhysicalOperationAllocationScope::Maintenance, 128)
        .expect("bounded maintenance allocation should admit")
}

fn deny(
    request: worth_store_buffer_pool::BackgroundEnvelopeRequest,
    expected: BackgroundEnvelopeDenialKind,
) -> BackgroundMemoryInterferenceReport {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    let budget = budget_for_expected_interference(expected);
    let report = admission
        .admit(request, budget, &mut allocation)
        .expect_err("request must produce expected interference");
    assert_eq!(report.kind(), expected);
    report
}

fn admit(
    request: worth_store_buffer_pool::BackgroundEnvelopeRequest,
) -> AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    admission
        .admit(request, permissive_budget(), &mut allocation)
        .expect("background envelope admits")
}

fn class_request(
    work_class: BackgroundWorkClass,
) -> worth_store_buffer_pool::BackgroundEnvelopeRequest {
    let builder = match work_class {
        BackgroundWorkClass::RecoveryPlanning => BackgroundEnvelopeRequest::recovery_planning(),
        BackgroundWorkClass::CompactionPlanning => BackgroundEnvelopeRequest::compaction_planning(),
        BackgroundWorkClass::ScrubPlanning => BackgroundEnvelopeRequest::scrub_planning(),
        BackgroundWorkClass::ImportExport => BackgroundEnvelopeRequest::import_export(),
        BackgroundWorkClass::LargeRecordStreaming => {
            return streaming_request(4096, 512);
        }
    };
    builder
        .resident_frames(1)
        .resident_bytes(128)
        .pin_pages_for_bounded_step(1)
        .allocation_bytes(128)
        .finish()
}

fn streaming_request(
    object_bytes: u64,
    window_bytes: u64,
) -> worth_store_buffer_pool::BackgroundEnvelopeRequest {
    BackgroundEnvelopeRequest::large_record_streaming()
        .resident_frames(1)
        .resident_bytes(window_bytes)
        .allocation_bytes(window_bytes)
        .copied_bytes(window_bytes)
        .streaming_window(object_bytes, window_bytes)
        .finish()
}

fn permissive_budget() -> BackgroundWorkBudgetSnapshot {
    BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16)
}

fn budget_for_expected_interference(
    expected: BackgroundEnvelopeDenialKind,
) -> BackgroundWorkBudgetSnapshot {
    match expected {
        BackgroundEnvelopeDenialKind::ForegroundResidencyInterference { .. } => {
            BackgroundWorkBudgetSnapshot::foreground_reserved(4, 2, 0, 8)
        }
        BackgroundEnvelopeDenialKind::PinBudgetWouldBeExceeded { .. } => {
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 1, 2)
        }
        BackgroundEnvelopeDenialKind::IndefinitePinRequested { .. }
        | BackgroundEnvelopeDenialKind::WholeObjectMemoryRequired { .. }
        | BackgroundEnvelopeDenialKind::StreamingWindowExceedsEnvelope { .. }
        | BackgroundEnvelopeDenialKind::StreamingEnvelopeExceedsWindow { .. }
        | BackgroundEnvelopeDenialKind::ForegroundAllocationInterference { .. }
        | BackgroundEnvelopeDenialKind::AllocationDenied(_) => permissive_budget(),
    }
}

fn report_matches_required_kind(
    report: BackgroundMemoryInterferenceReport,
    required: RequiredInterferenceKind,
) -> bool {
    matches!(
        (required, report.kind()),
        (
            RequiredInterferenceKind::ForegroundResidency,
            BackgroundEnvelopeDenialKind::ForegroundResidencyInterference { .. }
        ) | (
            RequiredInterferenceKind::IndefinitePin,
            BackgroundEnvelopeDenialKind::IndefinitePinRequested { .. }
        ) | (
            RequiredInterferenceKind::PinBudgetPressure,
            BackgroundEnvelopeDenialKind::PinBudgetWouldBeExceeded { .. }
        ) | (
            RequiredInterferenceKind::WholeObject,
            BackgroundEnvelopeDenialKind::WholeObjectMemoryRequired { .. }
        ) | (
            RequiredInterferenceKind::StreamingWindowExceedsEnvelope,
            BackgroundEnvelopeDenialKind::StreamingWindowExceedsEnvelope { .. }
        ) | (
            RequiredInterferenceKind::StreamingEnvelopeExceedsWindow,
            BackgroundEnvelopeDenialKind::StreamingEnvelopeExceedsWindow { .. }
        )
    )
}

fn allocation_admission() -> AllocationAdmission {
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

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}
