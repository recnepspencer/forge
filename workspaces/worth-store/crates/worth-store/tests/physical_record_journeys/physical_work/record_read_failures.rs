use std::time::{Duration, Instant};

use worth_store::physical_runtime::{
    PhysicalWorkCounterStage, RecordAppendBatch, RecordByteLimit, RecordReadDenial,
    RecordReadLimits,
};
use worth_store_physical_backend::{MediaFaultDirective, MediaOperationRole};

use super::super::serving_from_open;
use super::{configuration, serving_from_initialization};

const PAYLOAD: &[u8] = b"canonical read failure cleanup";

#[test]
fn denied_before_effect_read_releases_retry_pending_work_and_signal_locality() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let initial = serving_from_initialization(&root);
    let record = initial
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([PAYLOAD]).unwrap(),
            placement,
        )
        .unwrap()
        .record_id(0)
        .unwrap();
    initial.close();

    let calibration = serving_from_open(&root);
    let bootstrap_reads = calibration
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedRead);
    calibration.close();

    let serving = super::fault_fixture::serving_from_open_with_positioned_read_fault(
        &root,
        bootstrap_reads + 1,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    );
    let limits = RecordReadLimits::new(RecordByteLimit::new(PAYLOAD.len() as u32).unwrap());
    let before_media = serving.media_counters();
    let before_terminal = serving
        .physical_work_counters()
        .total(PhysicalWorkCounterStage::Terminal);
    let invalidations_before = serving
        .physical_signal_observation()
        .unwrap()
        .aspect_invalidation_count();
    let failure = match serving.records().open(record, limits) {
        Ok(_) => panic!("a denied backend read must not construct a record session"),
        Err(failure) => failure,
    };

    assert_eq!(failure.denial(), RecordReadDenial::ArtifactDamaged);
    assert!(failure.observation().physical_work_count() > 0);
    await_read_cleanup(&serving);
    let after_media = serving.media_counters();
    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .aspect_invalidation_count(),
        invalidations_before + 1,
        "a dispatched projection failure must invalidate its exact dependency once"
    );
    assert_eq!(
        after_media.attempts_for(MediaOperationRole::PositionedRead)
            - before_media.attempts_for(MediaOperationRole::PositionedRead),
        1
    );
    assert_eq!(
        after_media.completed_bytes_for(MediaOperationRole::PositionedRead)
            - before_media.completed_bytes_for(MediaOperationRole::PositionedRead),
        0
    );
    assert!(
        serving
            .physical_work_counters()
            .total(PhysicalWorkCounterStage::Terminal)
            > before_terminal,
        "dropping the failed public read must terminally release retry-pending work"
    );
    assert!(serving.close_plan().execute().requires_inspection());
}

fn await_read_cleanup(serving: &worth_store::physical_runtime::ServingPhysicalRuntime) {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let observation = serving.physical_signal_observation().unwrap();
        if observation.active_locality_count() == 0 && observation.active_in_flight_count() == 0 {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "failed read retained Signal state: locality={}, in_flight={}",
            observation.active_locality_count(),
            observation.active_in_flight_count(),
        );
        std::thread::yield_now();
    }
}
