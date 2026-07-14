use crate::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration,
    AllocationRequestKind, BackgroundEnvelopeAdmission, BackgroundEnvelopeDenialKind,
    BackgroundEnvelopeRequest, BackgroundWorkBudgetSnapshot, BackgroundWorkClass,
    FixedMetadataReservation,
};

#[test]
fn background_work_classes_consume_explicit_non_foreground_envelopes() {
    for work_class in BackgroundWorkClass::ALL {
        let mut admission = BackgroundEnvelopeAdmission::new();
        let mut allocation = allocation_admission();
        let request = request_for_class(work_class);

        let envelope = admission
            .admit(request, permissive_budget(), &mut allocation)
            .expect("background work class has an admitted envelope");

        assert_eq!(envelope.work_class(), work_class);
        assert_eq!(envelope.allocation_scope(), work_class.allocation_scope());
        assert_eq!(
            envelope.allocation_receipt().kind(),
            expected_allocation_kind(work_class)
        );
        assert_ne!(
            envelope.allocation_scope(),
            worth_store_budgets::AllocationScope::Foreground
        );
        assert_eq!(envelope.allocation_bytes(), 128);
        assert_eq!(envelope.counters().admitted(), 1);
        assert!(!envelope.proves_wal_recovery());
        assert!(!envelope.proves_scrub_correctness());
        assert!(!envelope.proves_compaction_validity());
        assert!(!envelope.proves_blob_lifecycle_completion());
        assert!(!envelope.proves_repair_behavior());
    }
}

#[test]
fn foreground_resident_reservation_interference_denies_before_admission() {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    let request = BackgroundEnvelopeRequest::recovery_planning()
        .resident_frames(3)
        .resident_bytes(384)
        .allocation_bytes(128)
        .finish();
    let budget = BackgroundWorkBudgetSnapshot::foreground_reserved(4, 2, 0, 8);

    let report = admission
        .admit(request, budget, &mut allocation)
        .expect_err("background work cannot steal foreground resident frames");

    assert_eq!(
        report.kind(),
        BackgroundEnvelopeDenialKind::ForegroundResidencyInterference {
            requested_frames: 3,
            background_available_frames: 2,
            foreground_reserved_frames: 2,
        }
    );
    assert_eq!(report.counters().foreground_interference_denials(), 1);
    assert_eq!(report.counters().denied(), 1);
}

#[test]
fn indefinite_pin_and_whole_object_background_memory_are_denied() {
    let mut allocation = allocation_admission();
    let mut admission = BackgroundEnvelopeAdmission::new();
    let indefinite_pin = BackgroundEnvelopeRequest::scrub_planning()
        .pin_indefinitely(1)
        .allocation_bytes(128)
        .finish();

    let pin_report = admission
        .admit(indefinite_pin, permissive_budget(), &mut allocation)
        .expect_err("indefinite background pins are not admitted");
    assert_eq!(
        pin_report.kind(),
        BackgroundEnvelopeDenialKind::IndefinitePinRequested { requested_pages: 1 }
    );
    assert_eq!(pin_report.counters().indefinite_pin_denials(), 1);

    let whole_object = BackgroundEnvelopeRequest::large_record_streaming()
        .allocation_bytes(128)
        .streaming_window(16 * 1024, 128)
        .whole_object_memory_bytes(16 * 1024)
        .finish();
    let whole_object_report = admission
        .admit(whole_object, permissive_budget(), &mut allocation)
        .expect_err("whole-object memory is not an S.2 streaming envelope");
    assert_eq!(
        whole_object_report.kind(),
        BackgroundEnvelopeDenialKind::WholeObjectMemoryRequired {
            object_bytes: 16 * 1024,
            envelope_bytes: 128,
        }
    );
    assert_eq!(
        whole_object_report
            .counters()
            .whole_object_materialization_attempts(),
        1
    );
}

#[test]
fn streaming_memory_is_bounded_by_window_not_object_size() {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    let request = BackgroundEnvelopeRequest::large_record_streaming()
        .resident_frames(1)
        .resident_bytes(512)
        .allocation_bytes(512)
        .copied_bytes(512)
        .streaming_window(64 * 1024, 512)
        .finish();

    let envelope = admission
        .admit(request, permissive_budget(), &mut allocation)
        .expect("streaming admits when window fits envelope");

    assert_eq!(envelope.streaming_object_bytes(), 64 * 1024);
    assert_eq!(envelope.streaming_window_bytes(), 512);
    assert_eq!(envelope.allocation_bytes(), 512);
    assert_eq!(envelope.allocation_receipt().bytes(), 512);
    assert_eq!(envelope.counters().allocation_bytes_allocated(), 512);
    assert_eq!(envelope.counters().streaming_window_bytes(), 512);
    assert_eq!(
        envelope.allocation_receipt().kind(),
        AllocationRequestKind::StreamingWindow
    );
    assert_eq!(envelope.counters().copied_bytes(), 512);

    let too_wide = BackgroundEnvelopeRequest::large_record_streaming()
        .allocation_bytes(512)
        .streaming_window(64 * 1024, 1024)
        .finish();
    let report = admission
        .admit(too_wide, permissive_budget(), &mut allocation)
        .expect_err("streaming window cannot exceed admitted envelope");
    assert_eq!(
        report.kind(),
        BackgroundEnvelopeDenialKind::StreamingWindowExceedsEnvelope {
            window_bytes: 1024,
            envelope_bytes: 512,
        }
    );
    assert_eq!(report.counters().deferred(), 1);

    let oversized_envelope = BackgroundEnvelopeRequest::large_record_streaming()
        .allocation_bytes(512)
        .streaming_window(64 * 1024, 128)
        .finish();
    let oversized_report = admission
        .admit(oversized_envelope, permissive_budget(), &mut allocation)
        .expect_err("streaming envelope cannot exceed window counter");
    assert_eq!(
        oversized_report.kind(),
        BackgroundEnvelopeDenialKind::StreamingEnvelopeExceedsWindow {
            envelope_bytes: 512,
            window_bytes: 128,
        }
    );
}

fn request_for_class(work_class: BackgroundWorkClass) -> crate::BackgroundEnvelopeRequest {
    let builder = match work_class {
        BackgroundWorkClass::RecoveryPlanning => BackgroundEnvelopeRequest::recovery_planning(),
        BackgroundWorkClass::CompactionPlanning => BackgroundEnvelopeRequest::compaction_planning(),
        BackgroundWorkClass::ScrubPlanning => BackgroundEnvelopeRequest::scrub_planning(),
        BackgroundWorkClass::ImportExport => BackgroundEnvelopeRequest::import_export(),
        BackgroundWorkClass::LargeRecordStreaming => {
            BackgroundEnvelopeRequest::large_record_streaming().streaming_window(1024, 128)
        }
    };
    builder
        .resident_frames(1)
        .resident_bytes(128)
        .pin_pages_for_bounded_step(1)
        .allocation_bytes(128)
        .finish()
}

fn expected_allocation_kind(work_class: BackgroundWorkClass) -> AllocationRequestKind {
    match work_class {
        BackgroundWorkClass::LargeRecordStreaming => AllocationRequestKind::StreamingWindow,
        BackgroundWorkClass::RecoveryPlanning
        | BackgroundWorkClass::CompactionPlanning
        | BackgroundWorkClass::ScrubPlanning
        | BackgroundWorkClass::ImportExport => AllocationRequestKind::BackgroundWorkMemory,
    }
}

fn permissive_budget() -> BackgroundWorkBudgetSnapshot {
    BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16)
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
