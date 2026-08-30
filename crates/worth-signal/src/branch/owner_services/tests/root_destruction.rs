use std::sync::mpsc;
use std::thread;

use crate::branch::owner_services::{
    SignalOwnerAdmissionDenial, SignalOwnerCancellationSource, SignalOwnerLifecycleObservation,
};

use super::progress_bound::PROGRESS_BOUND;
use super::runtime_root::runtime_with_two_branches;

#[test]
fn root_drop_inside_admitted_callback_requests_close_without_self_deadlock() {
    let (mut runtime, _, branch, basis) = runtime_with_two_branches();
    let (port, _, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = port.upgrade_owner().expect("sealed owner remains live");
    let admission = owner.admit().expect("canonical callback admits");
    let cell = owner
        .lookup_cell(&admission, branch.id)
        .expect("the canonical target cell is installed");
    let (done_tx, done_rx) = mpsc::sync_channel(1);

    thread::spawn(move || {
        let cancellation = SignalOwnerCancellationSource::new();
        let outcome = cell.advance_exact::<(), (), _>(
            &admission,
            &basis,
            &mut (),
            &cancellation.token(),
            move |_| {
                drop(runtime);
                Ok(())
            },
        );
        let (observation, transaction) = outcome
            .expect("root destruction does not erase performed callback work")
            .into_parts();
        let canonical = cell
            .observe_exact(&admission)
            .expect("already-admitted work can observe the performed canonical state");
        let closing = owner.lifecycle_observation();
        let late_denial = owner
            .admit()
            .expect_err("a close request rejects every later admission");
        let movements = cell.cost_snapshot().movements();
        drop(admission);
        let closed = owner.lifecycle_observation();
        let _ = done_tx.send((
            observation,
            canonical,
            transaction.touched_nodes,
            movements,
            closing,
            late_denial,
            closed,
        ));
    });

    let (observation, canonical, touched_nodes, movements, closing, late_denial, closed) = done_rx
        .recv_timeout(PROGRESS_BOUND)
        .expect("root destruction must not wait on its own callback admission");
    assert_eq!(canonical, observation);
    assert_eq!(canonical.generation().get(), 1);
    assert_eq!(touched_nodes, 0);
    assert_eq!(movements, 1);
    assert_eq!(closing, SignalOwnerLifecycleObservation::Closing);
    assert_eq!(late_denial, SignalOwnerAdmissionDenial::OwnerUnavailable);
    assert_eq!(closed, SignalOwnerLifecycleObservation::Closed);
}
