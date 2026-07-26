use std::{
    sync::{mpsc, Arc, Barrier},
    time::Duration,
};

use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, RecordAppendBatch, RecordAppendError, RecordByteLimit,
    RecordCountLimit, RecordReadLimits, RecordScanOutcome, RecordScanRequest,
    RecordStreamFailureKind, RecordWriteSource, RecordWriteSourceError,
};
use worth_store_physical_backend::{MediaOperationRole, MediaPauseGate};

use super::{configuration, serving_from_initialization};
use crate::read_record;

const SEED: &[u8] = b"stable predecessor";
const LEFT: &[u8] = b"left concurrent payload";
const RIGHT: &[u8] = b"right concurrent payload";

struct PausedInlineSource {
    bytes: &'static [u8],
    entered: mpsc::SyncSender<()>,
    release: mpsc::Receiver<()>,
    completed: bool,
}

struct PausedRejectingInlineSource {
    length: u64,
    entered: mpsc::SyncSender<()>,
    release: mpsc::Receiver<()>,
}

impl RecordWriteSource for PausedRejectingInlineSource {
    fn declared_length(&self) -> u64 {
        self.length
    }

    fn read_next(&mut self, _: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        self.entered.send(()).unwrap();
        self.release.recv().unwrap();
        Err(RecordWriteSourceError::ProducerRejected)
    }
}

impl RecordWriteSource for PausedInlineSource {
    fn declared_length(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordWriteSourceError> {
        if self.completed {
            return Ok(0);
        }
        self.entered.send(()).unwrap();
        self.release.recv().unwrap();
        target[..self.bytes.len()].copy_from_slice(self.bytes);
        self.completed = true;
        Ok(self.bytes.len())
    }
}

#[test]
fn disjoint_payload_writes_overlap_while_root_cutover_orders_both_batches() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let initial = serving_from_initialization(&root);
    let seed = initial
        .record_submission()
        .append_batch(RecordAppendBatch::try_from_iter([SEED]).unwrap(), placement)
        .unwrap()
        .record_id(0)
        .unwrap();
    let store = initial.store_identity();
    initial.close();

    let (serving, first_gate, second_gate) =
        super::fault_fixture::serving_from_open_with_two_write_pauses(&root);
    let mut stable_scan = serving
        .records()
        .scan(RecordScanRequest::from_start().with_batch_limit(RecordCountLimit::new(1).unwrap()))
        .unwrap();
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
    let invalidations_before = serving
        .physical_signal_observation()
        .unwrap()
        .aspect_invalidation_count();
    let replacements_before = serving.media_counters().replacements();
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

    let first_reached = reaches_within(&first_gate, Duration::from_secs(3));
    let second_reached = reaches_within(&second_gate, Duration::from_secs(3));
    if !(first_reached && second_reached) {
        first_gate.release();
        second_gate.release();
        let _ = left_thread.join();
        let _ = right_thread.join();
        panic!(
            "both disjoint payload writes must reach the backend before either is released: first={first_reached}, second={second_reached}"
        );
    }

    let mut scratch = [0_u8; 64];
    let stable = stable_scan.read_next_into(&mut scratch).unwrap();
    let RecordScanOutcome::Batch(stable) = stable else {
        panic!("the predecessor scan must remain readable while payload writes are paused")
    };
    assert_eq!(stable.records().len(), 1);
    assert_eq!(stable.records()[0].record_id(), seed);
    let locator = ExternalPhysicalRecordLocator::new(store, seed);
    let seed_session = serving
        .records()
        .open_external(
            locator,
            RecordReadLimits::new(RecordByteLimit::new(SEED.len() as u32).unwrap()),
        )
        .unwrap();
    assert_eq!(read_record(seed_session, SEED.len()).0, SEED);

    first_gate.release();
    second_gate.release();
    let left = left_thread.join().unwrap().unwrap();
    let right = right_thread.join().unwrap().unwrap();
    let mut generations = [left.root_generation(), right.root_generation()];
    generations.sort_unstable();
    assert_eq!(generations, [3, 4]);
    let left_record = left.record_id(0).unwrap();
    let right_record = right.record_id(0).unwrap();
    assert_eq!(
        serving.media_counters().replacements() - replacements_before,
        2
    );

    for (record, expected) in [(left_record, LEFT), (right_record, RIGHT)] {
        let session = serving
            .records()
            .open(
                record,
                RecordReadLimits::new(RecordByteLimit::new(expected.len() as u32).unwrap()),
            )
            .unwrap();
        assert_eq!(read_record(session, expected.len()).0, expected);
    }
    assert_eq!(scan_record_count(&serving), 3);
    assert_eq!(
        serving
            .physical_signal_observation()
            .unwrap()
            .aspect_invalidation_count(),
        invalidations_before,
        "stable-root reads and successful publication must not manufacture dependency changes"
    );
    drop(stable_scan);
    assert!(!serving.close_plan().execute().requires_inspection());

    let reopened = super::super::serving_from_open(&root);
    for (record, expected) in [(seed, SEED), (left_record, LEFT), (right_record, RIGHT)] {
        let session = reopened
            .records()
            .open(
                record,
                RecordReadLimits::new(RecordByteLimit::new(expected.len() as u32).unwrap()),
            )
            .unwrap();
        assert_eq!(read_record(session, expected.len()).0, expected);
    }
    let scanned = scan_records(&reopened);
    assert_eq!(scanned.len(), 3);
    for (record, expected) in [(seed, SEED), (left_record, LEFT), (right_record, RIGHT)] {
        assert!(
            scanned
                .iter()
                .any(|(found, payload)| *found == record && payload == expected),
            "fresh reopen scan omitted record {record:?}"
        );
    }
    assert!(!reopened.close_plan().execute().requires_inspection());
}

#[test]
fn paused_inline_source_does_not_own_global_preparation_authority() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let (entered_tx, entered_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let paused = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::builder()
                .push_source(PausedInlineSource {
                    bytes: LEFT,
                    entered: entered_tx,
                    release: release_rx,
                    completed: false,
                })
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    let independent = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::try_from_iter([RIGHT]).unwrap(),
            placement,
        )
        .unwrap();
    let paused_thread = std::thread::spawn(move || paused.publish());
    entered_rx
        .recv_timeout(Duration::from_secs(3))
        .expect("the paused source must enter materialization");
    let (finished_tx, finished_rx) = mpsc::sync_channel(1);
    let independent_thread = std::thread::spawn(move || {
        let result = independent.publish();
        finished_tx.send(()).unwrap();
        result
    });
    if finished_rx.recv_timeout(Duration::from_secs(3)).is_err() {
        release_tx.send(()).unwrap();
        let _ = paused_thread.join();
        let _ = independent_thread.join();
        panic!("an unrelated append must publish while the first source is paused");
    }
    let independent = independent_thread.join().unwrap().unwrap();
    release_tx.send(()).unwrap();
    let paused = paused_thread.join().unwrap().unwrap();
    let mut generations = [independent.root_generation(), paused.root_generation()];
    generations.sort_unstable();
    assert_eq!(generations, [2, 3]);
    for (published, expected) in [(independent, RIGHT), (paused, LEFT)] {
        let session = serving
            .records()
            .open(
                published.record_id(0).unwrap(),
                RecordReadLimits::new(RecordByteLimit::new(expected.len() as u32).unwrap()),
            )
            .unwrap();
        assert_eq!(read_record(session, expected.len()).0, expected);
    }
    assert!(!serving.close_plan().execute().requires_inspection());
}

#[test]
fn failed_prepared_payload_cannot_reuse_identity_or_contaminate_disjoint_success() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (_, placement, _) = configuration();
    let serving = serving_from_initialization(&root);
    let (entered_tx, entered_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let failing = serving
        .record_submission()
        .prepare_append(
            RecordAppendBatch::builder()
                .push_source(PausedRejectingInlineSource {
                    length: LEFT.len() as u64,
                    entered: entered_tx,
                    release: release_rx,
                })
                .build()
                .unwrap(),
            placement,
        )
        .unwrap();
    let failing_thread = std::thread::spawn(move || failing.publish());
    entered_rx
        .recv_timeout(Duration::from_secs(3))
        .expect("the failing producer must hold its private preparation");

    let disjoint = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([RIGHT]).unwrap(),
            placement,
        )
        .unwrap();
    assert_eq!(disjoint.root_generation(), 2);
    let writes_after_disjoint = serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite);
    release_tx.send(()).unwrap();
    let error = failing_thread.join().unwrap().unwrap_err();
    let RecordAppendError::StreamFailed(failure) = error else {
        panic!("a rejected pre-media producer must remain a stream failure: {error:?}")
    };
    assert_eq!(failure.kind(), RecordStreamFailureKind::ProducerRejected);
    assert_eq!(failure.completed_range(), 0..0);
    assert_eq!(
        serving
            .media_counters()
            .attempts_for(MediaOperationRole::PositionedWrite),
        writes_after_disjoint
    );

    let successor = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"healthy successor"]).unwrap(),
            placement,
        )
        .unwrap();
    assert_eq!(successor.root_generation(), 3);
    assert_eq!(scan_record_count(&serving), 2);
    assert!(
        !root
            .join("families/records/segments/segment-0000000000000001-0000000000000001.pages")
            .exists(),
        "the abandoned first segment reservation must remain a hole"
    );
    assert!(root
        .join("families/records/segments/segment-0000000000000002-0000000000000001.pages")
        .exists());
    assert!(root
        .join("families/records/segments/segment-0000000000000002-0000000000000002.pages")
        .exists());
    assert!(!serving.close_plan().execute().requires_inspection());
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

fn scan_record_count(serving: &worth_store::physical_runtime::ServingPhysicalRuntime) -> usize {
    let mut scan = serving
        .records()
        .scan(RecordScanRequest::from_start())
        .unwrap();
    let mut scratch = [0_u8; 256];
    let mut count = 0;
    loop {
        match scan.read_next_into(&mut scratch).unwrap() {
            RecordScanOutcome::Batch(batch) => count += batch.records().len(),
            RecordScanOutcome::Completed(_) => return count,
        }
    }
}

fn scan_records(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
) -> Vec<(worth_store::physical_runtime::PhysicalRecordId, Vec<u8>)> {
    let mut scan = serving
        .records()
        .scan(RecordScanRequest::from_start())
        .unwrap();
    let mut scratch = [0_u8; 256];
    let mut records = Vec::new();
    loop {
        match scan.read_next_into(&mut scratch).unwrap() {
            RecordScanOutcome::Batch(batch) => {
                records.extend(batch.records().iter().enumerate().map(|(index, record)| {
                    (
                        record.record_id(),
                        batch
                            .payload(index)
                            .expect("small concurrency payloads are inline")
                            .to_vec(),
                    )
                }));
            }
            RecordScanOutcome::Completed(_) => return records,
        }
    }
}
