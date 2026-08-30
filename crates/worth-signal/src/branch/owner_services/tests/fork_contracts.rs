use std::sync::mpsc;
use std::thread;

use crate::branch::{validate_signal_branch_name, SignalBranchForkOperationDenial};
use crate::data::graph::SignalGraph;

use super::super::SignalOwnerCancellationSource;
use super::progress_bound::{wait_until_progress, worker_park, PROGRESS_BOUND};
use super::runtime_root::{runtime_with_two_branches, runtime_with_two_branches_from_graph};

#[test]
fn exact_fork_contract_reports_current_clone_cost_and_orders_installation() {
    let mut graph = SignalGraph::new();
    for _ in 0..64 {
        graph.create_node();
    }
    let (mut runtime, _, source_branch, source_basis) = runtime_with_two_branches_from_graph(graph);
    let (_, mutation, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = mutation.upgrade_owner().expect("owner remains live");
    let admission = owner.admit().expect("fork operation admits");
    let source_cell = owner
        .lookup_cell(&admission, source_branch.id)
        .expect("source cell is live");
    source_cell
        .with_state(&admission, |state, _| {
            state
                .state_mut()
                .mutation_ledger_mut()
                .clear_all(Some(crate::state::SignalSnapshotId(701)));
        })
        .expect("source receives a mutation-sensitive pre-fork journal");
    let source_ledger = source_cell
        .with_state(&admission, |state, _| {
            state.state().mutation_ledger().clone()
        })
        .expect("source journal is inspectable before fork");
    let original_children = owner
        .metadata
        .branch_children(&admission, source_branch.id)
        .expect("source lineage is inspectable before fork");
    let reservation = owner
        .reserve_fork_destination(
            &admission,
            &source_basis,
            validate_signal_branch_name("phase-3-exact-fork").expect("identity validates"),
        )
        .expect("destination reserves before source capture");
    let destination = reservation.branch().clone();
    let cancellation = SignalOwnerCancellationSource::new();
    let before = owner.cost_snapshot();
    let installed = reservation
        .install(
            &source_cell,
            &admission,
            &source_basis,
            &cancellation.token(),
        )
        .expect("destination installs after source capture releases its cell");

    installed
        .with_state(&admission, |state, _| {
            assert_eq!(state.branch_id(), destination.id);
            assert_eq!(state.state().branch_id(), destination.id);
            assert_eq!(state.state().graph().current_branch().id, destination.id);
            assert_eq!(state.state().graph().active_node_count(), 64);
        })
        .expect("installed destination is immediately complete");
    let after = owner.cost_snapshot();
    assert_eq!(
        after.fork_source_captures(),
        before.fork_source_captures() + 1
    );
    assert_eq!(
        after.fork_destination_preparations(),
        before.fork_destination_preparations() + 1
    );
    assert_eq!(
        after.fork_destination_installations(),
        before.fork_destination_installations() + 1
    );
    assert_eq!(
        after.forked_mutable_graph_nodes_copied(),
        before.forked_mutable_graph_nodes_copied() + 64,
        "Phase 3 reports the legacy clone honestly; Phase 4 owns structural sharing"
    );
    assert_eq!(owner.live_count(), 3);
    assert_eq!(owner.reservation_count(), 0);
    let observed_source_ledger = source_cell
        .with_state(&admission, |state, _| {
            state.state().mutation_ledger().clone()
        })
        .expect("successful fork source remains inspectable");
    let mut expected_source_ledger = source_ledger;
    expected_source_ledger.clear_all(source_branch.head_snapshot_id);
    assert_eq!(observed_source_ledger, expected_source_ledger);
    let children = owner
        .metadata
        .branch_children(&admission, source_branch.id)
        .expect("successful fork lineage is committed");
    let mut expected_children = original_children;
    expected_children.push(destination.id);
    expected_children.sort_unstable();
    assert_eq!(children, expected_children);
}

#[test]
fn late_fork_cancellation_drops_preconstructed_destination_without_source_movement() {
    let (mut runtime, _, source_branch, source_basis) = runtime_with_two_branches();
    let (_, mutation, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = mutation.upgrade_owner().expect("owner remains live");
    let setup = owner.admit().expect("setup admits");
    let source_cell = owner
        .lookup_cell(&setup, source_branch.id)
        .expect("source cell is live");
    source_cell
        .with_state(&setup, |state, _| {
            state
                .state_mut()
                .mutation_ledger_mut()
                .clear_all(Some(crate::state::SignalSnapshotId(701)));
        })
        .expect("source receives a mutation-sensitive baseline");
    let source_ledger = source_cell
        .with_state(&setup, |state, _| state.state().mutation_ledger().clone())
        .expect("source ledger is inspectable");
    let original_children = owner
        .metadata
        .branch_children(&setup, source_branch.id)
        .expect("lineage inspection is owner-admitted");
    drop(setup);

    let holder_admission = owner.admit().expect("holder admits");
    let fork_admission = owner.admit().expect("fork admits independently");
    let reservation = owner
        .reserve_fork_destination(
            &fork_admission,
            &source_basis,
            validate_signal_branch_name("phase-3-late-cancelled-fork").expect("identity validates"),
        )
        .expect("destination reserves");
    let cancellation = SignalOwnerCancellationSource::new();
    let before = owner.cost_snapshot();
    let waits_before = source_cell.cost_snapshot().waits();
    let (holder_tx, holder_rx) = mpsc::sync_channel(1);
    let (fork_tx, fork_rx) = mpsc::sync_channel(1);

    thread::scope(|scope| {
        let (park, mut control) = worker_park();
        scope.spawn(|| {
            let result = source_cell.with_state(&holder_admission, |_, _| {
                park.park("late-cancellation source-cell holder");
            });
            let _ = holder_tx.send(result);
        });
        control.wait_until_parked("late-cancellation source-cell holder");
        scope.spawn(|| {
            let result = reservation.install(
                &source_cell,
                &fork_admission,
                &source_basis,
                &cancellation.token(),
            );
            let _ = fork_tx.send(result);
        });
        let reached_wait = wait_until_progress("fork source-cell contention", || {
            source_cell.cost_snapshot().waits() == waits_before + 1
        });
        cancellation.cancel();
        control.release();
        let fork_result = fork_rx.recv_timeout(PROGRESS_BOUND);
        let holder_result = holder_rx.recv_timeout(PROGRESS_BOUND);
        assert!(
            reached_wait,
            "fork did not reach the bounded source-cell wait"
        );
        assert!(
            matches!(
                fork_result,
                Ok(Err(SignalBranchForkOperationDenial::CancelledNoMovement))
            ),
            "cancelled fork did not return the exact no-movement denial"
        );
        assert_eq!(holder_result, Ok(Ok(())));
    });

    let after = owner.cost_snapshot();
    assert_eq!(
        after.fork_destination_preparations(),
        before.fork_destination_preparations() + 1,
        "late cancellation must cross destination preconstruction"
    );
    assert_eq!(after.fork_source_captures(), before.fork_source_captures());
    assert_eq!(
        after.fork_destination_installations(),
        before.fork_destination_installations()
    );
    assert_fork_source_and_capacity_unchanged(
        &owner,
        &source_cell,
        source_branch.id,
        &source_ledger,
        &original_children,
    );

    let healthy_admission = owner.admit().expect("healthy twin admits");
    let healthy = owner
        .reserve_fork_destination(
            &healthy_admission,
            &source_basis,
            validate_signal_branch_name("phase-3-healthy-fork").expect("identity validates"),
        )
        .expect("healthy destination reserves");
    healthy
        .install(
            &source_cell,
            &healthy_admission,
            &source_basis,
            &SignalOwnerCancellationSource::new().token(),
        )
        .expect("healthy twin installs immediately after cancellation cleanup");
    assert_eq!(owner.live_count(), 3);
}

fn assert_fork_source_and_capacity_unchanged(
    owner: &std::sync::Arc<super::super::SignalOwner<(), (), ()>>,
    source_cell: &std::sync::Arc<
        super::super::SignalBranchExecutionCell<super::super::SignalBranchCellState<(), (), ()>>,
    >,
    source_branch_id: crate::state::SignalBranchId,
    source_ledger: &crate::logic::transaction::BranchMutationLedger,
    original_children: &[crate::state::SignalBranchId],
) {
    let admission = owner.admit().expect("cleanup inspection admits");
    let observed_ledger = source_cell
        .with_state(&admission, |state, _| {
            state.state().mutation_ledger().clone()
        })
        .expect("source remains inspectable");
    assert_eq!(&observed_ledger, source_ledger);
    assert_eq!(owner.live_count(), 2);
    assert_eq!(owner.reservation_count(), 0);
    assert_eq!(
        owner
            .metadata
            .branch_children(&admission, source_branch_id)
            .expect("lineage cleanup is owner-admitted"),
        original_children
    );
}
