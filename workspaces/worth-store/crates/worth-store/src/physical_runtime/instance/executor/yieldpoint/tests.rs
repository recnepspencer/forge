use std::sync::{mpsc, Arc};
use std::time::Duration;

use super::{CertificationPhysicalExecutionCheckpoint, PhysicalExecutorYieldpointOwner};

#[test]
fn named_executor_checkpoint_blocks_until_framework_gate_releases() {
    let owner = PhysicalExecutorYieldpointOwner::new();
    let checkpoint = CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch;
    let gate = owner.install(checkpoint);
    let (completed, completion) = mpsc::sync_channel(1);
    let execution = std::thread::spawn(move || {
        owner.pause(checkpoint);
        completed.send(()).unwrap();
    });

    assert!(gate.await_arrival());
    assert!(completion.recv_timeout(Duration::from_millis(20)).is_err());
    gate.release();
    completion.recv_timeout(Duration::from_secs(1)).unwrap();
    execution.join().unwrap();
}

#[test]
fn execution_checkpoints_retain_independent_pause_authority() {
    let owner = PhysicalExecutorYieldpointOwner::new();
    let pre_dispatch =
        owner.install(CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch);
    let post_read =
        owner.install(CertificationPhysicalExecutionCheckpoint::AfterReadBeforeSchedulerSettlement);
    let post_write = owner.install(
        CertificationPhysicalExecutionCheckpoint::AfterExactWriteBeforeSchedulerSettlement,
    );
    let post_residency_write = owner.install(
        CertificationPhysicalExecutionCheckpoint::AfterResidencyWriteBeforeSchedulerSettlement,
    );
    let post_catalog = owner.install(
        CertificationPhysicalExecutionCheckpoint::AfterCatalogReplacementBeforeSchedulerSettlement,
    );

    let execution = std::thread::spawn(move || {
        owner.pause(CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch);
    });
    assert!(pre_dispatch.await_arrival());
    assert_eq!(post_read.arrival_count(), 0);
    assert_eq!(post_write.arrival_count(), 0);
    assert_eq!(post_residency_write.arrival_count(), 0);
    assert_eq!(post_catalog.arrival_count(), 0);
    pre_dispatch.release();
    execution.join().unwrap();
}

#[test]
fn one_arrival_can_run_while_an_independent_arrival_remains_paused() {
    let owner = Arc::new(PhysicalExecutorYieldpointOwner::new());
    let checkpoint = CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch;
    let gate = owner.install(checkpoint);
    let (completed, completion) = mpsc::channel();
    let first = spawn_paused(Arc::clone(&owner), checkpoint, completed.clone(), 0);
    assert!(gate.await_arrivals(1));
    let second = spawn_paused(owner, checkpoint, completed, 1);
    assert!(gate.await_arrivals(2));

    let release = gate
        .release_arrival(1)
        .expect("second arrival has sealed release authority");
    assert_eq!(release.arrival_index(), 1);
    assert!(release.await_resumption());
    if completion.recv_timeout(Duration::from_secs(1)).unwrap() != 1 {
        panic!("MUTANT_PREDICATE:individual-gate-release-broadened");
    }
    if completion.recv_timeout(Duration::from_millis(20)).is_ok() {
        panic!("MUTANT_PREDICATE:individual-gate-release-broadened");
    }

    let release = gate
        .release_arrival(0)
        .expect("first arrival remains independently releasable");
    assert_eq!(release.arrival_index(), 0);
    assert_eq!(completion.recv_timeout(Duration::from_secs(1)).unwrap(), 0);
    first.join().unwrap();
    second.join().unwrap();
}

#[test]
fn released_arrival_acknowledges_resumption_before_a_dependent_dispatch_blocks() {
    let owner = Arc::new(PhysicalExecutorYieldpointOwner::new());
    let checkpoint = CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch;
    let gate = owner.install(checkpoint);
    let (progressed, progress) = mpsc::channel();
    let execution = std::thread::spawn({
        let owner = Arc::clone(&owner);
        move || {
            owner.pause(checkpoint);
            progressed.send("resumed").unwrap();
            owner.pause(checkpoint);
            progressed.send("completed").unwrap();
        }
    });

    assert!(gate.await_arrivals(1));
    let release = gate
        .release_arrival(0)
        .expect("first dispatch has sealed release authority");
    assert!(release.await_resumption());
    assert_eq!(
        progress.recv_timeout(Duration::from_secs(1)).unwrap(),
        "resumed"
    );
    assert!(gate.await_arrivals(2));
    assert!(progress.recv_timeout(Duration::from_millis(20)).is_err());

    gate.release();
    assert_eq!(
        progress.recv_timeout(Duration::from_secs(1)).unwrap(),
        "completed"
    );
    execution.join().unwrap();
}

#[test]
fn selected_arrival_releases_downstream_only_after_resumption() {
    let owner = Arc::new(PhysicalExecutorYieldpointOwner::new());
    let checkpoint = CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch;
    let gate = owner.install(checkpoint);
    let (completed, completion) = mpsc::channel();
    let selected = std::thread::spawn({
        let owner = Arc::clone(&owner);
        let completed = completed.clone();
        move || {
            owner.pause(checkpoint);
            owner.pause(checkpoint);
            completed.send("selected").unwrap();
        }
    });
    assert!(gate.await_arrivals(1));
    let remaining = std::thread::spawn(move || {
        owner.pause(checkpoint);
        completed.send("remaining").unwrap();
    });
    assert!(gate.await_arrivals(2));

    let release = gate
        .select_arrival_then_release_downstream(0)
        .expect("selected arrival must resume before downstream release");
    assert_eq!(release.arrival_index(), 0);
    let first = completion.recv_timeout(Duration::from_secs(1));
    let second = completion.recv_timeout(Duration::from_secs(1));
    if first.is_err() || second.is_err() {
        gate.release();
        let _ = selected.join();
        let _ = remaining.join();
        panic!("MUTANT_PREDICATE:selected-downstream-release-omitted");
    }
    selected.join().unwrap();
    remaining.join().unwrap();
}

#[test]
fn dropping_the_gate_releases_every_blocked_arrival() {
    let owner = PhysicalExecutorYieldpointOwner::new();
    let checkpoint = CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch;
    let gate = owner.install(checkpoint);
    let (completed, completion) = mpsc::channel();
    let execution = std::thread::spawn(move || {
        owner.pause(checkpoint);
        completed.send(()).unwrap();
    });
    assert!(gate.await_arrival());

    drop(gate);
    if completion.recv_timeout(Duration::from_secs(1)).is_err() {
        panic!("MUTANT_PREDICATE:schedule-failure-gate-cleanup-omitted");
    }
    execution.join().unwrap();
}

fn spawn_paused(
    owner: Arc<PhysicalExecutorYieldpointOwner>,
    checkpoint: CertificationPhysicalExecutionCheckpoint,
    completed: mpsc::Sender<usize>,
    identity: usize,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        owner.pause(checkpoint);
        completed.send(identity).unwrap();
    })
}
