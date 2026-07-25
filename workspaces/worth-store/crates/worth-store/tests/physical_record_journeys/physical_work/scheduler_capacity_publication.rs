use std::{
    sync::{mpsc, Arc, Barrier},
    time::Duration,
};

use worth_store::physical_runtime::{
    PhysicalRecordMutationFailureCause, PhysicalWorkCapacity, RecordAppendBatch,
    RecordAppendDenial, RecordAppendError,
};
use worth_store_io_scheduler::foreground_reservation::{
    ForegroundReservationAdmissionDenial, ForegroundReservationResourceShortfall,
    PhysicalInstanceForegroundAdmissionDenial,
};
use worth_store_physical_backend::MediaPauseGate;

const LEFT: &[u8] = b"scheduler capacity left";
const RIGHT: &[u8] = b"scheduler capacity right";
const DENIED: &[u8] = b"scheduler capacity denied";
const PAGE_GRANT_BYTES: u64 = 16_384;

#[test]
fn ordinary_append_reports_exact_live_scheduler_exhaustion_and_recovers_after_settlement() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = super::configuration();
    super::serving_from_initialization(&root).close();
    let (profile, _, _) = super::work_fixture();
    let capacity = PhysicalWorkCapacity::new(
        8,
        256,
        1_024,
        PAGE_GRANT_BYTES as usize,
        (PAGE_GRANT_BYTES * 2) as usize,
    )
    .unwrap();
    let (serving, first_gate, second_gate) =
        super::fault_fixture::serving_from_open_with_two_write_pauses_and_profile(
            &root,
            profile.with_capacity(capacity),
        );
    let left = serving
        .record_submission()
        .prepare_append(RecordAppendBatch::try_from_iter([LEFT]).unwrap(), placement)
        .unwrap();
    let right = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::try_from_iter([RIGHT]).unwrap(),
            placement,
        )
        .unwrap();
    let denied = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::try_from_iter([DENIED]).unwrap(),
            placement,
        )
        .unwrap();
    let start = Arc::new(Barrier::new(3));
    let left_start = Arc::clone(&start);
    let left_thread = std::thread::spawn(move || {
        left_start.wait();
        left.publish()
    });
    let right_start = Arc::clone(&start);
    let right_thread = std::thread::spawn(move || {
        right_start.wait();
        right.publish()
    });
    start.wait();
    if !both_reached(&first_gate, &second_gate) {
        first_gate.release();
        second_gate.release();
        let left = left_thread.join().unwrap();
        let right = right_thread.join().unwrap();
        panic!(
            "two ordinary facade writes did not concurrently hold scheduler reservations: left={left:?}, right={right:?}"
        );
    }

    let saturated = serving.physical_scheduler_capacity();
    assert_eq!(saturated.active_reservations(), 2);
    assert_eq!(saturated.available().bandwidth_tokens(), 0);
    let media_before_denial = serving.media_counters();
    let (denied_tx, denied_rx) = mpsc::sync_channel(1);
    let denied_thread = std::thread::spawn(move || {
        denied_tx.send(denied.publish()).unwrap();
    });
    let denied = match denied_rx.recv_timeout(Duration::from_secs(3)) {
        Ok(result) => result.unwrap_err(),
        Err(_) => {
            first_gate.release();
            second_gate.release();
            let _ = left_thread.join();
            let _ = right_thread.join();
            let _ = denied_thread.join();
            panic!("the third append did not reach scheduler denial while two effects were paused");
        }
    };
    denied_thread.join().unwrap();
    assert_eq!(serving.media_counters(), media_before_denial);
    assert_exact_bandwidth_denial(&denied);
    let denied_snapshot = serving.physical_scheduler_capacity();
    assert_eq!(denied_snapshot.active_reservations(), 2);
    assert_eq!(denied_snapshot.denied_reservations(), 1);

    first_gate.release();
    second_gate.release();
    left_thread.join().unwrap().unwrap();
    right_thread.join().unwrap().unwrap();
    let released = serving.physical_scheduler_capacity();
    assert_eq!(released.active_reservations(), 0);
    assert_eq!(released.available(), released.configured());
    assert_eq!(
        serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"scheduler capacity restored"]).unwrap(),
                placement,
            )
            .unwrap()
            .observation()
            .records(),
        1
    );
    let final_snapshot = serving.physical_scheduler_capacity();
    assert_eq!(final_snapshot.active_reservations(), 0);
    assert_eq!(final_snapshot.available(), final_snapshot.configured());
    let closed = serving.close_plan().execute();
    assert!(
        !closed.requires_inspection(),
        "pre-effect scheduler denial left terminal inspection posture: records={:?}, residency={}, drain={:?}, signal={:?}, cancellations={}, summary={:?}, media={:?}",
        closed.shutdown().records().posture(),
        closed.shutdown().residency().requires_inspection(),
        closed.shutdown().work().drain(),
        closed.shutdown().signal(),
        closed.shutdown().signal_cancellation_failures(),
        closed.shutdown().signal_summary(),
        closed.shutdown().media().release(),
    );
}

fn assert_exact_bandwidth_denial(error: &RecordAppendError) {
    let RecordAppendError::Denied(RecordAppendDenial::PhysicalWorkUnavailable(failure)) = error
    else {
        panic!("scheduler pressure must remain an explicit pre-effect denial: {error:?}");
    };
    assert!(matches!(
        failure.cause(),
        PhysicalRecordMutationFailureCause::SchedulerReservationDenied(
            PhysicalInstanceForegroundAdmissionDenial::Foreground(
                ForegroundReservationAdmissionDenial::InsufficientCapacity(
                    ForegroundReservationResourceShortfall::BandwidthToken {
                        requested: PAGE_GRANT_BYTES,
                        available: 0,
                    }
                )
            )
        )
    ));
}

fn both_reached(first: &MediaPauseGate, second: &MediaPauseGate) -> bool {
    let first_reached = reaches_within(first, Duration::from_secs(3));
    let second_reached = reaches_within(second, Duration::from_secs(3));
    first_reached && second_reached
}

fn reaches_within(gate: &MediaPauseGate, timeout: Duration) -> bool {
    let (reached, waiting) = mpsc::channel();
    let gate = gate.clone();
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let _ = reached.send(());
    });
    waiting.recv_timeout(timeout).is_ok()
}
