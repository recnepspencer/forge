use std::{sync::mpsc, time::Duration};

use worth_store::physical_runtime::{
    PhysicalRecordMutationFailureCause, PhysicalWorkIdentity, PreparedRecordAppend,
    PublishedRecordBatch, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordWriteSource, RecordWriteSourceError, ServingPhysicalRuntime,
};
use worth_store_io_scheduler::foreground_reservation::{
    ForegroundReservationAdmissionDenial, ForegroundReservationResourceShortfall,
    PhysicalInstanceForegroundAdmissionDenial,
};
use worth_store_physical_backend::MediaPauseGate;

const LEFT: &[u8] = b"phase-16-left-append";
const RIGHT: &[u8] = b"phase-16-right-append";
const DENIED: &[u8] = b"phase-16-capacity-denied";
const PAGE_GRANT_BYTES: u64 = 16_384;

pub(super) struct AppendEvidence {
    pub generations: [u64; 2],
    pub work: Vec<PhysicalWorkIdentity>,
}

struct PausedAppend {
    prepared: PreparedRecordAppend,
    entered: mpsc::Receiver<()>,
    release: mpsc::SyncSender<()>,
}

pub(super) fn prepare_and_publish_independently(
    serving: &ServingPhysicalRuntime,
    gates: &super::fixture::MaelstromPauseGates,
) -> AppendEvidence {
    let left = prepare_paused(serving, LEFT);
    let right = prepare_paused(serving, RIGHT);
    let (_, placement, _) = super::super::configuration();
    let denied = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::try_from_iter([DENIED]).unwrap(),
            placement,
        )
        .unwrap();
    let (left, right) = publish_under_capacity_siege(serving, gates, left, right, denied);
    let mut generations = [left.root_generation(), right.root_generation()];
    generations.sort_unstable();
    let mut work = left
        .physical_work()
        .effects()
        .iter()
        .chain(right.physical_work().effects())
        .map(|effect| effect.identity())
        .collect::<Vec<_>>();
    work.sort_by_key(|identity| identity.operation().get());
    work.dedup();
    AppendEvidence { generations, work }
}

fn prepare_paused(serving: &ServingPhysicalRuntime, bytes: &'static [u8]) -> PausedAppend {
    let (_, placement, _) = super::super::configuration();
    let (entered_tx, entered) = mpsc::sync_channel(1);
    let (release, release_rx) = mpsc::sync_channel(1);
    let prepared = serving
        .record_submission()
        .prepare_append(
            batch(PausedPreparationSource::new(bytes, entered_tx, release_rx)),
            placement,
        )
        .unwrap();
    PausedAppend {
        prepared,
        entered,
        release,
    }
}

fn publish_under_capacity_siege(
    serving: &ServingPhysicalRuntime,
    gates: &super::fixture::MaelstromPauseGates,
    left: PausedAppend,
    right: PausedAppend,
    denied: PreparedRecordAppend,
) -> (PublishedRecordBatch, PublishedRecordBatch) {
    std::thread::scope(|scope| {
        let left_thread = scope.spawn(move || left.prepared.publish());
        let right_thread = scope.spawn(move || right.prepared.publish());
        left.entered
            .recv_timeout(Duration::from_secs(3))
            .expect("left append publication did not enter source materialization");
        right
            .entered
            .recv_timeout(Duration::from_secs(3))
            .expect("right append publication was globally serialized");
        left.release.send(()).unwrap();
        right.release.send(()).unwrap();
        require_concurrent_reservations(gates, &left_thread, &right_thread);
        assert_live_capacity_denial(serving, denied);
        gates.first_append.release();
        gates.second_append.release();
        let published = (
            left_thread.join().unwrap().unwrap(),
            right_thread.join().unwrap().unwrap(),
        );
        let released = serving.physical_scheduler_capacity();
        assert_eq!(released.active_reservations(), 0);
        assert_eq!(released.available(), released.configured());
        published
    })
}

fn require_concurrent_reservations(
    gates: &super::fixture::MaelstromPauseGates,
    left: &std::thread::ScopedJoinHandle<'_, Result<PublishedRecordBatch, RecordAppendError>>,
    right: &std::thread::ScopedJoinHandle<'_, Result<PublishedRecordBatch, RecordAppendError>>,
) {
    if !both_reached(&gates.first_append, &gates.second_append) {
        gates.first_append.release();
        gates.second_append.release();
        panic!(
            "two append effects did not concurrently hold scheduler reservations: left_finished={}, right_finished={}",
            left.is_finished(),
            right.is_finished()
        );
    }
}

fn assert_live_capacity_denial(serving: &ServingPhysicalRuntime, denied: PreparedRecordAppend) {
    let saturated = serving.physical_scheduler_capacity();
    assert_eq!(saturated.active_reservations(), 2);
    assert_eq!(saturated.available().bandwidth_tokens(), 0);
    let media_before_denial = serving.media_counters();
    let denied = denied.publish().unwrap_err();
    assert_eq!(serving.media_counters(), media_before_denial);
    assert_exact_bandwidth_denial(&denied);
    let denied_snapshot = serving.physical_scheduler_capacity();
    assert_eq!(denied_snapshot.active_reservations(), 2);
    assert_eq!(denied_snapshot.denied_reservations(), 1);
}

fn assert_exact_bandwidth_denial(error: &RecordAppendError) {
    let RecordAppendError::Denied(RecordAppendDenial::PhysicalWorkUnavailable(failure)) = error
    else {
        panic!("live scheduler pressure must remain a pre-effect denial: {error:?}");
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
    reaches_within(first) && reaches_within(second)
}

fn reaches_within(gate: &MediaPauseGate) -> bool {
    let (reached, waiting) = mpsc::channel();
    let gate = gate.clone();
    std::thread::spawn(move || {
        gate.wait_until_reached();
        let _ = reached.send(());
    });
    waiting.recv_timeout(Duration::from_secs(3)).is_ok()
}

fn batch(source: PausedPreparationSource) -> RecordAppendBatch {
    RecordAppendBatch::builder()
        .push_source(source)
        .build()
        .unwrap()
}

struct PausedPreparationSource {
    bytes: &'static [u8],
    entered: mpsc::SyncSender<()>,
    release: mpsc::Receiver<()>,
    emitted: bool,
}

impl PausedPreparationSource {
    fn new(
        bytes: &'static [u8],
        entered: mpsc::SyncSender<()>,
        release: mpsc::Receiver<()>,
    ) -> Self {
        Self {
            bytes,
            entered,
            release,
            emitted: false,
        }
    }
}

impl RecordWriteSource for PausedPreparationSource {
    fn declared_length(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        if self.emitted {
            return Ok(0);
        }
        self.entered.send(()).unwrap();
        self.release.recv().unwrap();
        target[..self.bytes.len()].copy_from_slice(self.bytes);
        self.emitted = true;
        Ok(self.bytes.len())
    }
}
