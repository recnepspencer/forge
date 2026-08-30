use std::sync::mpsc::{self, TryRecvError};
use std::sync::Arc;
use std::thread;

use super::super::{
    SignalOwnerAdmissionDenial, SignalOwnerLifecycleObservation, SignalOwnerLifecycleState,
    SignalOwnerServiceCounters,
};

#[test]
fn close_drains_admitted_work_and_monotonically_denies_late_admission() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(41, Arc::clone(&counters));
    let admission = lifecycle.admit(41).expect("open owner admits work");
    let (closed_sender, closed_receiver) = mpsc::channel();
    let closing_lifecycle = Arc::clone(&lifecycle);

    let closer = thread::spawn(move || {
        closing_lifecycle
            .close(41)
            .expect("the owner closes its lifecycle");
        closed_sender.send(()).expect("close result is observed");
    });

    while lifecycle.observation() == SignalOwnerLifecycleObservation::Open {
        thread::yield_now();
    }
    assert_eq!(
        lifecycle.observation(),
        SignalOwnerLifecycleObservation::Closing
    );
    assert_eq!(closed_receiver.try_recv(), Err(TryRecvError::Empty));
    assert!(matches!(
        lifecycle.admit(41),
        Err(SignalOwnerAdmissionDenial::OwnerUnavailable)
    ));

    drop(admission);
    closed_receiver
        .recv()
        .expect("drain completes after release");
    closer.join().expect("close thread remains healthy");
    assert_eq!(
        lifecycle.observation(),
        SignalOwnerLifecycleObservation::Closed
    );
    assert!(matches!(
        lifecycle.admit(41),
        Err(SignalOwnerAdmissionDenial::OwnerUnavailable)
    ));
    lifecycle
        .close(41)
        .expect("closed is terminal and idempotent");
    assert_eq!(counters.snapshot().close_batches(), 1);
}

#[test]
fn foreign_owner_cannot_admit_or_close_the_lifecycle() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(7, counters);

    assert!(matches!(
        lifecycle.admit(8),
        Err(SignalOwnerAdmissionDenial::ForeignOwner)
    ));
    assert!(lifecycle.close(8).is_err());
    assert_eq!(
        lifecycle.observation(),
        SignalOwnerLifecycleObservation::Open
    );
}
