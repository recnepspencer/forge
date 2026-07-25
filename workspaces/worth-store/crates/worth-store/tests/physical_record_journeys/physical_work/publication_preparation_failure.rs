use std::time::{Duration, Instant};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalReadWorkRequest, PhysicalRecordMutationFailureCause, PhysicalWorkCapacity,
    PhysicalWorkCounterStage, PhysicalWorkReadiness, ReadyPhysicalWork, RecordAppendBatch,
    RecordAppendError, RecordPublicationStage, ServingPhysicalRuntime, UnpublishedRecordBatchCause,
    UnpublishedRecordEffectFate, UnpublishedRecordWorldFate,
};

use super::fixture::{serving_from_initialization_with_work_profile, work_fixture};

#[test]
fn denied_replacement_preparation_invents_no_publication_residue() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (profile, read, _) = work_fixture();
    let capacity = PhysicalWorkCapacity::new(2, 256, 1_024, 1024 * 1024, 4 * 1024 * 1024).unwrap();
    let serving =
        serving_from_initialization_with_work_profile(&root, profile.with_capacity(capacity));
    let first = ready_probe(&serving, read.clone());
    let second = ready_probe(&serving, read);
    let (_, placement, _) = super::configuration();
    let before = serving.media_counters();

    let error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"capacity denied preparation".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let RecordAppendError::Unpublished(failure) = error else {
        panic!("replacement preparation denial must expose reusable unpublished fate: {error:?}")
    };
    assert!(matches!(
        failure.cause(),
        UnpublishedRecordBatchCause::PhysicalWork {
            stage: RecordPublicationStage::CatalogReplacement,
            failure,
        } if failure.cause() == PhysicalRecordMutationFailureCause::SubmissionRejected
    ));
    assert_eq!(
        failure.effect_fate(),
        UnpublishedRecordEffectFate::DeniedBeforeEffect
    );
    assert_eq!(failure.world_fate(), UnpublishedRecordWorldFate::Reusable);
    assert!(failure.physical_work().effects().is_empty());
    assert!(failure.residue().is_empty());
    assert!(serving.publication_residue().is_empty());
    assert_eq!(serving.media_counters(), before);

    drop(first);
    drop(second);
    await_signal_cleanup(&serving);
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"capacity restored"]).unwrap(),
            placement,
        )
        .unwrap();
    assert!(!serving.close_plan().execute().requires_inspection());
}

fn ready_probe(
    serving: &ServingPhysicalRuntime,
    read: PhysicalReadWorkRequest,
) -> ReadyPhysicalWork {
    let receipt = match serving.physical_read_submission().submit(read).into_raw() {
        TransitionOutcome::Success(receipt) => receipt,
        outcome => panic!("the capacity probe must submit: {outcome:?}"),
    };
    let admitted = serving.admit_physical_work(receipt).unwrap();
    match serving.request_physical_work(admitted).unwrap() {
        PhysicalWorkReadiness::Ready(ready) => ready,
        PhysicalWorkReadiness::Blocked(_) => panic!("the capacity probe must become ready"),
    }
}

fn await_signal_cleanup(serving: &ServingPhysicalRuntime) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let observation = serving.physical_signal_observation().unwrap();
        if observation.active_locality_count() == 0
            && observation.active_in_flight_count() == 0
            && serving
                .physical_work_counters()
                .total(PhysicalWorkCounterStage::Terminal)
                == 2
        {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "dropped capacity probe retained Signal work: locality={}, in_flight={}",
            observation.active_locality_count(),
            observation.active_in_flight_count(),
        );
        std::thread::yield_now();
    }
}
