//! The focused court proving a paused settlement holds no runtime-wide
//! authority.
//!
//! Every wait here is bounded. A branch parked inside its settlement executor
//! holds an admitted runtime operation, and the owner close waits on exactly
//! that, so an unbounded wait would replace this court's named diagnostic with
//! a hung lib suite.

use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

use super::owner_service_phase3::{fork_from_main, perform_write_with_control};
use crate::tests::support::*;

/// Bound on the paused-settlement court's waits. A paused branch holds an
/// admitted runtime operation and the owner close waits on exactly that, so an
/// unbounded wait here would hang the lib suite instead of naming the defect.
const PAUSED_SETTLEMENT_COURT_TIMEOUT: Duration = Duration::from_secs(5);

/// Release the settlement pause without ever blocking the court. The barrier
/// half runs on a proxy, so a branch that never parked cannot strand the
/// release, and a branch that parks late is still freed.
fn open_settlement_pause(release: &Arc<Barrier>) {
    let release = Arc::clone(release);
    std::thread::spawn(move || {
        release.wait();
    });
}

/// Settlement holds no runtime-wide authority. A branch paused inside its own
/// settlement executor never contacts, and never blocks, an unrelated branch
/// running the full convenience commit path.
#[test]
fn phase3_paused_settlement_does_not_block_an_unrelated_branch_commit() {
    let runtime = runtime_with_test_schema();
    create_entity(&runtime, "phase3-settlement-independence-anchor");
    fork_from_main(&runtime, "paused-settlement");
    fork_from_main(&runtime, "progressing-settlement");

    let reached = Arc::new(Barrier::new(2));
    let release = Arc::new(Barrier::new(2));
    let control = crate::mvcc::RelationalOperationControl::uninterrupted().with_boundary_pause(
        crate::mvcc::RelationalInterruptionBoundary::Settlement,
        Arc::clone(&reached),
        Arc::clone(&release),
    );
    let performed =
        perform_write_with_control(&runtime, "paused-settlement", "paused-write", control);
    let paused_commit_id = performed.canonical_commit().commit.commit_id;
    let paused_cell = runtime
        .history
        .branch_cell(&BranchId("paused-settlement".to_owned()))
        .expect("paused branch cell");
    let contacts_before = paused_cell.coordination().contact_count();
    let waits_before = paused_cell.coordination().wait_count();

    let settlement = runtime.settlement_port();
    let paused_port = settlement.clone();
    let paused_thread =
        std::thread::spawn(move || paused_port.settle_performed_publication(performed));

    let (arrived, arrival) = sync_channel(1);
    let arrival_proxy = Arc::clone(&reached);
    std::thread::spawn(move || {
        arrival_proxy.wait();
        let _ = arrived.send(());
    });
    if arrival
        .recv_timeout(PAUSED_SETTLEMENT_COURT_TIMEOUT)
        .is_err()
    {
        open_settlement_pause(&release);
        panic!("branch A never reached its settlement executor pause");
    }
    assert!(
        !paused_thread.is_finished(),
        "branch A remains paused inside its settlement executor"
    );
    let mut progressing = test_owner_begin_transaction_for_branch(
        &runtime,
        BranchId("progressing-settlement".to_owned()),
    );
    progressing
        .push_batch(batch_create("progressing-write"))
        .expect("test staging stays within configured resource budgets");
    // Branch B commits on a worker so this court can bound it. Only a thread
    // outside the commit can hold the completion receiver and convict
    // serialization by name instead of blocking here forever.
    let (finished, completion) = sync_channel(1);
    let progressing_result = std::thread::scope(|scope| {
        let committer = scope.spawn(|| {
            let committed = runtime
                .commit_branch_transaction(progressing)
                .expect("branch B commits end to end while branch A settlement is paused");
            let _ = finished.send(());
            committed
        });
        if completion
            .recv_timeout(PAUSED_SETTLEMENT_COURT_TIMEOUT)
            .is_err()
        {
            open_settlement_pause(&release);
            panic!("branch B serialized behind branch A's paused settlement executor");
        }
        match committer.join() {
            Ok(committed) => committed,
            Err(worker_panic) => std::panic::resume_unwind(worker_panic),
        }
    });
    assert!(
        !paused_thread.is_finished(),
        "branch B did not release branch A"
    );
    assert_eq!(
        paused_cell.coordination().contact_count(),
        contacts_before,
        "branch B never contacts branch A coordination",
    );
    assert_eq!(
        paused_cell.coordination().wait_count(),
        waits_before,
        "branch B never waits on branch A coordination",
    );
    assert!(settlement.retains_pending_settlement(paused_commit_id));
    assert!(!settlement.retains_pending_settlement(progressing_result.commit.commit_id));

    open_settlement_pause(&release);
    let deadline = Instant::now() + PAUSED_SETTLEMENT_COURT_TIMEOUT;
    while !paused_thread.is_finished() {
        assert!(
            Instant::now() < deadline,
            "branch A never finished settling after its pause opened"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    let paused_result = paused_thread
        .join()
        .expect("paused settlement worker joins")
        .expect("branch A settles after release");
    assert_eq!(paused_result.commit.commit_id, paused_commit_id);
    assert!(!settlement.retains_pending_settlement(paused_commit_id));
    release_test_commit_snapshot(&runtime, &paused_result);
    release_test_commit_snapshot(&runtime, &progressing_result);
}
