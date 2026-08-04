use std::sync::mpsc::{self, TryRecvError};
use std::time::{Duration, Instant};

use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::CertificationPhysicalRecordSubmission;
use worth_store::physical_runtime::certification::{
    CertificationPhysicalExecutionCheckpoint, CertificationPhysicalExecutionPauseGate,
};
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, CompletedPhysicalCheckpoint, PhysicalCheckpointDeadline,
    PhysicalCheckpointHandle, PhysicalCheckpointIdempotencyKey, PhysicalCheckpointOutcome,
    PhysicalCheckpointProgressPhase, PhysicalCheckpointRequest, PhysicalRecordInitialization,
    PhysicalWalGroupBarrierOutcome, PhysicalWalReclamationObservation, ServingPhysicalRuntime,
};
use worth_store_physical_format::PhysicalCheckpointSource;

use super::super::{configuration, durability_with_wal_policy, media, success};
use super::checkpoint_lifecycle::pause_checkpoint_at_phase;
use super::independent_wal_oracle::inspect_wal_inventory;
use super::wal_rotation::{append_group, prepared, wal_policy};

#[test]
fn published_checkpoint_carries_the_exact_original_multi_rotation_wal_tail() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let (serving, placement) = rotation_world(&store_root);
    let submission = serving.certification_record_submission();
    durable_group(&submission, placement, &[(1, b"checkpoint-base")]);
    let (completed, frozen_source) =
        publish_after_foreground_rotations(&serving, submission, placement);
    assert_independent_tail(&store_root, frozen_source, &completed);
    assert_reclaimed_prefix(&completed, 1);

    serving.close();
}

#[test]
fn foreground_wal_cannot_cross_the_checkpoint_publication_cutover() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let (serving, placement) = rotation_world(&store_root);
    let submission = serving.certification_record_submission();
    durable_group(&submission, placement, &[(31, b"cutover-base")]);
    let (gate, handle, publication_arrival) = checkpoint_at_publication_replacement(&serving);
    let frozen_source = handle.source();
    let (foreground, completion) = start_cutover_foreground(submission, placement);
    assert!(matches!(completion.try_recv(), Err(TryRecvError::Empty)));
    let completed = finish_checkpoint_cutover(&gate, handle, publication_arrival, &completion);
    assert!(matches!(completion.try_recv(), Err(TryRecvError::Empty)));

    let tail = completed.retained_wal_tail();
    let boundary = frozen_source.wal().covered_end_lsn_exclusive();
    assert_eq!(tail.checkpoint_boundary_lsn().get(), boundary);
    assert_eq!(tail.durable_tail_end_lsn_exclusive().get(), boundary);
    assert_eq!(tail.segments().len(), 1);
    assert!(matches!(
        completed.wal_reclamation(),
        PhysicalWalReclamationObservation::NotRequired { .. }
    ));

    gate.release();
    let foreground_end = completion.recv().unwrap();
    foreground.join().unwrap();
    assert!(foreground_end > tail.durable_tail_end_lsn_exclusive().get());
    serving.close();
}

pub(super) fn rotation_world(
    store_root: &std::path::Path,
) -> (ServingPhysicalRuntime, AdmittedRecordPlacementPolicy) {
    let media_owner = media(store_root);
    let durability = durability_with_wal_policy(&media_owner, wal_policy(6));
    let (format, placement, access) = configuration();
    let serving = success(
        media_owner.initialize_record_store(PhysicalRecordInitialization::new(
            format, placement, access, durability,
        )),
    );
    (serving, placement)
}

pub(super) fn durable_group(
    submission: &CertificationPhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
    members: &[(u8, &[u8])],
) -> u64 {
    let group = append_group(
        submission,
        members
            .iter()
            .map(|(identity, payload)| prepared(submission, placement, *identity, payload))
            .collect(),
    );
    let end_lsn = group
        .members()
        .last()
        .unwrap()
        .mutation()
        .reserved()
        .declaration()
        .lsn_range()
        .end_exclusive()
        .get();
    assert!(matches!(
        submission.synchronize_appended_wal_group(group),
        PhysicalWalGroupBarrierOutcome::Durable(_)
    ));
    end_lsn
}

pub(super) fn publish_after_foreground_rotations(
    serving: &ServingPhysicalRuntime,
    submission: CertificationPhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
) -> (CompletedPhysicalCheckpoint, PhysicalCheckpointSource) {
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let handle = start_checkpoint(serving);
    let frozen_source = handle.source();
    assert!(gate.await_arrival());
    let (finished, completion) = mpsc::channel();
    let foreground = std::thread::spawn(move || {
        durable_group(
            &submission,
            placement,
            &[(2, b"second-a"), (3, b"second-b")],
        );
        durable_group(&submission, placement, &[(4, b"third-a"), (5, b"third-b")]);
        finished.send(()).unwrap();
    });
    release_foreground_arrivals(&gate, &completion);
    foreground.join().unwrap();
    let checkpoint_start = gate.release_arrival(0).unwrap();
    assert!(checkpoint_start.await_resumption());
    gate.release();
    (completed_checkpoint(handle), frozen_source)
}

fn release_foreground_arrivals(
    gate: &CertificationPhysicalExecutionPauseGate,
    completion: &mpsc::Receiver<()>,
) {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut next_arrival = 1;
    loop {
        match completion.try_recv() {
            Ok(()) => return,
            Err(TryRecvError::Disconnected) => panic!("retained-tail worker disconnected"),
            Err(TryRecvError::Empty) => {}
        }
        if gate.arrival_count() > next_arrival {
            let release = gate.release_arrival(next_arrival).unwrap();
            assert!(release.await_resumption());
            next_arrival += 1;
        } else {
            assert!(
                Instant::now() < deadline,
                "foreground rotations stalled behind checkpoint capture"
            );
            std::thread::yield_now();
        }
    }
}

fn assert_independent_tail(
    store_root: &std::path::Path,
    frozen_source: PhysicalCheckpointSource,
    completed: &CompletedPhysicalCheckpoint,
) {
    let independent = inspect_wal_inventory(store_root).unwrap();
    assert_eq!(independent.segments(), &[(2, 1), (3, 1)]);
    let boundary = frozen_source.wal().covered_end_lsn_exclusive();
    let expected = independent
        .segment_facts()
        .iter()
        .copied()
        .filter(|segment| segment.lsn_range().1 > boundary)
        .collect::<Vec<_>>();
    let tail = completed.retained_wal_tail();
    assert_eq!(tail.checkpoint_source(), frozen_source);
    assert_eq!(tail.checkpoint_boundary_lsn().get(), boundary);
    assert_eq!(tail.segments().len(), expected.len());
    for (actual, expected) in tail.segments().iter().zip(&expected) {
        assert_eq!(
            (
                actual.artifact().segment().get(),
                actual.artifact().generation().get()
            ),
            expected.identity()
        );
        assert_eq!(
            (
                actual.observed_lsn_range().start().get(),
                actual.observed_lsn_range().end_exclusive().get()
            ),
            expected.lsn_range()
        );
        assert_eq!(actual.physical_bytes(), expected.byte_count());
    }
    assert_eq!(
        tail.retained_physical_bytes(),
        expected
            .iter()
            .map(|segment| segment.byte_count())
            .sum::<u64>()
    );
    assert_eq!(
        tail.durable_tail_end_lsn_exclusive().get(),
        independent.lsn_range().unwrap().1
    );
}

fn assert_reclaimed_prefix(completed: &CompletedPhysicalCheckpoint, expected_segments: u32) {
    match completed.wal_reclamation() {
        PhysicalWalReclamationObservation::Reclaimed(report) => {
            assert_eq!(report.planned_segments(), expected_segments);
            assert_eq!(report.reclaimed_segments(), expected_segments);
            assert!(report.reclaimed_bytes() > 0);
            assert_eq!(report.first_unreclaimed(), None);
        }
        other => panic!("checkpoint did not reclaim its obsolete WAL prefix: {other:?}"),
    }
}

fn checkpoint_at_publication_replacement(
    serving: &ServingPhysicalRuntime,
) -> (
    CertificationPhysicalExecutionPauseGate,
    PhysicalCheckpointHandle,
    usize,
) {
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    let handle = start_checkpoint(serving);
    let publication_arrival = pause_checkpoint_at_phase(
        &gate,
        &handle,
        PhysicalCheckpointProgressPhase::PublicationReplacement,
    );
    (gate, handle, publication_arrival)
}

fn start_cutover_foreground(
    submission: CertificationPhysicalRecordSubmission,
    placement: AdmittedRecordPlacementPolicy,
) -> (std::thread::JoinHandle<()>, mpsc::Receiver<u64>) {
    let (started, start_observation) = mpsc::channel();
    let (finished, completion) = mpsc::channel();
    let foreground = std::thread::spawn(move || {
        started.send(()).unwrap();
        let end_lsn = durable_group(&submission, placement, &[(32, b"must-follow-cutover")]);
        finished.send(end_lsn).unwrap();
    });
    start_observation.recv().unwrap();
    (foreground, completion)
}

fn finish_checkpoint_cutover(
    gate: &CertificationPhysicalExecutionPauseGate,
    handle: PhysicalCheckpointHandle,
    publication_arrival: usize,
    foreground: &mpsc::Receiver<u64>,
) -> CompletedPhysicalCheckpoint {
    let replace = gate.release_arrival(publication_arrival).unwrap();
    assert!(replace.await_resumption());
    assert!(gate.await_arrivals(publication_arrival + 2));
    assert!(matches!(foreground.try_recv(), Err(TryRecvError::Empty)));
    let namespace = gate.release_arrival(publication_arrival + 1).unwrap();
    assert!(namespace.await_resumption());
    completed_checkpoint(handle)
}

fn start_checkpoint(serving: &ServingPhysicalRuntime) -> PhysicalCheckpointHandle {
    match serving.checkpoints().start(checkpoint_request()).into_raw() {
        TransitionOutcome::Success(handle) => handle,
        _ => panic!("retained-tail checkpoint was not admitted"),
    }
}

fn completed_checkpoint(handle: PhysicalCheckpointHandle) -> CompletedPhysicalCheckpoint {
    match handle.wait() {
        PhysicalCheckpointOutcome::Completed(completed) => completed,
        other => panic!("retained-tail checkpoint did not complete: {other:?}"),
    }
}

fn checkpoint_request() -> PhysicalCheckpointRequest {
    PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new([0x73; 32]),
        PhysicalCheckpointDeadline::at(TemporalDuration::temporal_duration(10_000).unwrap()),
    )
}
