use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, TryLockError};

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{ProductBranchObservation, RuntimeWorldBootstrapOutcome};
use crate::lifecycle::RuntimeWorldCloseDenial;
use crate::publication::RuntimeWorldCancellationSource;
use crate::publication::{CompositePublicationIntent, NoEffectCause};

type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

fn setup() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let mut fixture = reference_test_fixture::real_fixture(8, 8);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(
            super::bootstrap_budgets(),
            crate::lifecycle::RuntimeWorldClock::from_source(super::FixedClock),
        ))
        .expect("managed owner construction"),
    );
    let performed = match owner.bootstrap_root(fixture.bootstrap_intent()) {
        RuntimeWorldBootstrapOutcome::Performed(performed) => performed,
        RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!("bootstrap unexpectedly denied: {:?}", no_effect.cause())
        }
    };
    (fixture, owner, performed.product_branch().clone())
}

/// One Signal-only publication reservation off the given head. Preparation
/// and reservation are a single owner step, so a racing caller cannot hold a
/// lowered plan without the capacity that authorizes it.
fn prepare(
    owner: &TestOwner,
    expected: ProductBranchObservation,
) -> Result<
    crate::publication::PreparedCompositePublicationWithSignal,
    crate::publication::NoEffectCompositePublication,
> {
    let cancellation = RuntimeWorldCancellationSource::new();
    crate::lifecycle::RuntimeWorldPreparationService::prepare_publication(
        owner,
        expected,
        CompositePublicationIntent::with_signal(None),
        &cancellation.token(),
        None,
    )
}

/// Wait for a reservation worker to reach the bootstrap admission lock.
/// Reservation admission holds that lock across the operation ledger it then
/// blocks on, so the hold is a real observable rather than a sampled instant.
/// The budget is wall-clock, not a yield count: several Runtime World
/// worktrees build and test on one machine, so a scheduler starved of cores
/// must not be reported as a lost race. The wait fails by name, never hangs.
#[track_caller]
fn wait_until_bootstrap_is_held(owner: &TestOwner) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        match owner.state.bootstrap.try_lock() {
            Err(TryLockError::WouldBlock) => return,
            Err(TryLockError::Poisoned(error)) => {
                panic!("bootstrap lock poisoned: {error}")
            }
            Ok(guard) => drop(guard),
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    panic!("reservation worker never reached the bootstrap admission lock");
}

/// Wait for a close worker to enter close admission. Close reads the bootstrap
/// state and releases that lock again before it takes the operation ledger, so
/// unlike a reservation it is never parked on a lock a test can observe. What
/// can be observed is that the worker has entered `close()`; the settle window
/// after that is bounded wall-clock, and the race it sets up still fails by
/// name in the assertions that follow rather than hanging here.
#[track_caller]
fn wait_until_close_is_admitting(entered: &AtomicBool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !entered.load(Ordering::SeqCst) {
        if std::time::Instant::now() >= deadline {
            panic!("close worker never entered owner close admission");
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    std::thread::sleep(std::time::Duration::from_millis(250));
}

#[test]
fn publication_attempts_own_independent_phase_state() {
    let (_fixture, owner, expected) = setup();
    let first = prepare(&owner, expected.clone()).expect("first attempt reserves");
    let second =
        prepare(&owner, expected).expect("second attempt is not serialized behind the first");

    assert_eq!(owner.state.operation.active(), 2);
    drop(first);
    assert_eq!(owner.state.operation.active(), 1);
    drop(second);
    assert_eq!(owner.state.operation.active(), 0);
}

#[test]
fn reserve_and_close_have_one_atomic_winner_in_both_lock_orders() {
    let (_fixture, owner, expected) = setup();
    let ledger_gate = owner
        .state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let reserve_owner = Arc::clone(&owner);
    let reserve_head = expected.clone();
    let reserve = std::thread::spawn(move || prepare(reserve_owner.as_ref(), reserve_head));
    wait_until_bootstrap_is_held(&owner);
    let close_owner = Arc::clone(&owner);
    let close = std::thread::spawn(move || close_owner.close());
    drop(ledger_gate);

    let attempt = reserve
        .join()
        .expect("reserve worker does not panic")
        .expect("reserve wins after holding bootstrap admission");
    assert_eq!(
        close
            .join()
            .expect("close worker does not panic")
            .expect_err("the reserving winner blocks the close drain"),
        RuntimeWorldCloseDenial::AlreadyClosing
    );
    drop(attempt);
    let _report = owner
        .close()
        .expect("close succeeds after the winner settles");

    let (_fixture, owner, expected) = setup();
    let ledger_gate = owner
        .state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let close_owner = Arc::clone(&owner);
    let entered = Arc::new(AtomicBool::new(false));
    let close_entered = Arc::clone(&entered);
    let close = std::thread::spawn(move || {
        close_entered.store(true, Ordering::SeqCst);
        close_owner.close()
    });
    wait_until_close_is_admitting(&entered);
    let reserve_owner = Arc::clone(&owner);
    let reserve = std::thread::spawn(move || prepare(reserve_owner.as_ref(), expected));
    drop(ledger_gate);

    let _report = close
        .join()
        .expect("close worker does not panic")
        .expect("close wins after holding bootstrap admission");
    let denial = reserve
        .join()
        .expect("reserve worker does not panic")
        .expect_err("closed owner denies the trailing reservation");
    assert_eq!(denial.cause(), NoEffectCause::OwnerUnavailable);
    assert_eq!(owner.state.operation.active(), 0);
}
