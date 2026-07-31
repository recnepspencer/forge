use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRecordInitialization, PhysicalRecordWritebackFailureCause,
    PhysicalRuntimeAdmission, PhysicalSignalSettlementOutcome, PhysicalStore,
    PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition, PhysicalWritebackCounterSnapshot,
    RecordAppendBatch, RecordAppendError, RecordPublicationStage, RecordServingTerminalPosture,
    ServingPhysicalRuntime, UnpublishedRecordBatchCause,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::{scenario_configuration::dense_configuration, success};

#[test]
fn ordinary_candidate_tail_no_effect_is_typed_and_discards_dirty_residency() {
    let target = second_candidate_write_ordinal();
    let page_bytes = dense_page_bytes();
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let serving = serving_with_fault(
        &root,
        target,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    );
    let before = serving.residency_observation().writebacks();

    let unpublished = append_two_page_segment(&serving)
        .expect_err("the ordinary candidate-tail writeback must receive the fault");
    let RecordAppendError::Unpublished(unpublished) = unpublished else {
        panic!("a proven no-effect candidate-tail writeback is typed unpublished")
    };
    let UnpublishedRecordBatchCause::FrameWriteback { stage, failure, .. } = unpublished.cause()
    else {
        panic!("the failure must retain its ordinary frame-writeback cause")
    };

    assert_eq!(*stage, RecordPublicationStage::CandidateDataWrite);
    assert_eq!(
        failure.cause(),
        PhysicalRecordWritebackFailureCause::RetryableNoEffect
    );
    assert_eq!(
        failure.effect_fate(),
        PhysicalWorkEffectFate::ProvenNoEffect
    );
    assert_eq!(
        failure.recovery(),
        Some(PhysicalWorkRecoveryDisposition::RetryExact)
    );
    assert!(failure.effect().is_none());
    assert_eq!(
        failure.signal(),
        Some(PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth)
    );
    assert!(unpublished.requires_inspection());
    assert_no_effect_residency_and_artifact(serving, before, &root, page_bytes);
}

fn assert_no_effect_residency_and_artifact(
    serving: ServingPhysicalRuntime,
    before: PhysicalWritebackCounterSnapshot,
    root: &Path,
    page_bytes: u64,
) {
    let after = serving.residency_observation();
    assert_eq!(after.writebacks().attempts(), before.attempts() + 1);
    assert_eq!(after.writebacks().retryable(), before.retryable() + 1);
    assert_eq!(after.writebacks().exact_receipts(), before.exact_receipts());
    assert_eq!(after.counters().dirty_frames(), 0);
    assert_eq!(after.counters().candidate_frames(), 0);
    assert_eq!(after.counters().active_writeback_claims(), 0);
    assert_eq!(segment_length(root), page_bytes);
    let closed = serving.close();
    assert_eq!(
        closed.records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
    assert!(!closed.residency().requires_inspection());
}

#[test]
fn ordinary_candidate_tail_partial_effect_retains_dirty_inspection_truth() {
    let target = second_candidate_write_ordinal();
    let page_bytes = dense_page_bytes();
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let serving = serving_with_fault(&root, target, MediaFaultDirective::AllowPrefix { bytes: 3 });
    let before = serving.residency_observation().writebacks();

    let unpublished = append_two_page_segment(&serving)
        .expect_err("the ordinary candidate-tail writeback must receive the partial effect");
    let RecordAppendError::Unpublished(unpublished) = unpublished else {
        panic!("a partial candidate-tail writeback is typed unpublished")
    };
    let UnpublishedRecordBatchCause::FrameWriteback { stage, failure, .. } = unpublished.cause()
    else {
        panic!("the partial effect must retain its frame-writeback cause")
    };

    assert_eq!(*stage, RecordPublicationStage::CandidateDataWrite);
    assert_eq!(
        failure.cause(),
        PhysicalRecordWritebackFailureCause::InspectionRequired
    );
    assert_eq!(failure.effect_fate(), PhysicalWorkEffectFate::Indeterminate);
    assert_eq!(
        failure.recovery(),
        Some(PhysicalWorkRecoveryDisposition::InspectionRequired)
    );
    assert!(failure.effect().is_some());
    assert_eq!(
        failure.signal(),
        Some(PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth)
    );
    assert!(unpublished.requires_inspection());
    assert_partial_effect_residency_and_artifact(serving, before, &root, page_bytes);
}

fn assert_partial_effect_residency_and_artifact(
    serving: ServingPhysicalRuntime,
    before: PhysicalWritebackCounterSnapshot,
    root: &Path,
    page_bytes: u64,
) {
    let after = serving.residency_observation();
    assert_eq!(after.writebacks().attempts(), before.attempts() + 1);
    assert_eq!(
        after.writebacks().indeterminate(),
        before.indeterminate() + 1
    );
    assert_eq!(
        after.writebacks().inspection_required(),
        before.inspection_required() + 1
    );
    assert_eq!(after.writebacks().exact_receipts(), before.exact_receipts());
    assert_eq!(after.counters().dirty_frames(), 1);
    assert_eq!(after.counters().candidate_frames(), 1);
    assert_eq!(after.counters().active_writeback_claims(), 0);
    assert_eq!(segment_length(root), page_bytes + 3);
    let closed = serving.close();
    assert_eq!(
        closed.records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
    assert!(closed.residency().requires_inspection());
}

fn second_candidate_write_ordinal() -> u64 {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("control");
    let (format, placement, access) = dense_configuration(2);
    let serving = success(initialize_record_store!(
        super::media(&root),
        |durability| PhysicalRecordInitialization::new(format, placement, access, durability)
    ));
    let before = serving
        .media_counters()
        .identified_operation_attempts_for(MediaOperationRole::PositionedWrite);
    let writebacks_before = serving.residency_observation().writebacks();
    append_two_page_segment(&serving).unwrap();
    let writebacks_after = serving.residency_observation().writebacks();
    assert_eq!(
        writebacks_after.attempts(),
        writebacks_before.attempts() + 1
    );
    serving.close();
    before + 2
}

fn serving_with_fault(
    root: &Path,
    target: u64,
    directive: MediaFaultDirective,
) -> worth_store::physical_runtime::ServingPhysicalRuntime {
    let (format, placement, access) = dense_configuration(2);
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority
            .rule(MediaOperationRole::PositionedWrite, target, directive)
            .for_identified_operation_ordinal()])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("the ordinary writeback fault schedule must be admitted"),
    };
    let serving = success(initialize_record_store!(media, |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    assert_eq!(
        serving
            .media_counters()
            .identified_operation_attempts_for(MediaOperationRole::PositionedWrite),
        target - 2
    );
    serving
}

fn append_two_page_segment(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
) -> worth_store::physical_runtime::RecordPublicationOutcome {
    let (_, placement, _) = dense_configuration(2);
    let records = (0_u8..4)
        .map(|value| vec![value; 3_000])
        .collect::<Vec<_>>();
    serving.record_submission().append_batch(
        RecordAppendBatch::try_from_iter(records.iter()).unwrap(),
        placement,
    )
}

fn segment_length(root: &Path) -> u64 {
    std::fs::metadata(
        root.join("families/records/segments/segment-0000000000000001-0000000000000001.pages"),
    )
    .unwrap()
    .len()
}

fn dense_page_bytes() -> u64 {
    let (format, _, _) = dense_configuration(2);
    u64::from(format.declaration().page_size().bytes())
}
