use std::sync::{Arc, TryLockError};

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchObservation, RuntimeWorldBootstrapOutcome,
};
use crate::lifecycle::{RuntimeWorldCancellationSource, RuntimeWorldCloseDenial};
use crate::publication::{CompositeComponentIntent, NoEffectCause, ProductBranchIntent};

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

fn plan(
    owner: &TestOwner,
    expected: ProductBranchObservation,
    name: &str,
) -> crate::publication::LoweredOwnerComponentPlan {
    crate::lifecycle::RuntimeWorldPreparationService::prepare(
        owner,
        expected,
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named(name).expect("valid operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::signal_only(),
        ),
    )
    .expect("open owner prepares the exact observed head")
}

fn wait_until_bootstrap_is_held(owner: &TestOwner) {
    for _ in 0..100_000 {
        match owner.state.bootstrap.try_lock() {
            Err(TryLockError::WouldBlock) => return,
            Err(TryLockError::Poisoned(error)) => {
                panic!("bootstrap lock poisoned: {error}")
            }
            Ok(guard) => drop(guard),
        }
        std::thread::yield_now();
    }
    panic!("worker never reached the bootstrap admission lock");
}

#[test]
fn publication_attempts_own_independent_phase_state() {
    let (_fixture, owner, expected) = setup();
    let first = plan(&owner, expected.clone(), "first");
    let second = plan(&owner, expected, "second");
    let cancellation = RuntimeWorldCancellationSource::new();

    let first = crate::lifecycle::RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        first,
        &cancellation.token(),
        None,
    )
    .expect("first attempt reserves");
    let second = crate::lifecycle::RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        second,
        &cancellation.token(),
        None,
    )
    .expect("second attempt is not serialized behind the first");

    assert_eq!(owner.state.operation.active(), 2);
    drop(first);
    assert_eq!(owner.state.operation.active(), 1);
    drop(second);
    assert_eq!(owner.state.operation.active(), 0);
}

#[test]
fn reserve_and_close_have_one_atomic_winner_in_both_lock_orders() {
    let (_fixture, owner, expected) = setup();
    let candidate = plan(&owner, expected, "reserve-wins");
    let ledger_gate = owner
        .state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let reserve_owner = Arc::clone(&owner);
    let reserve = std::thread::spawn(move || {
        let cancellation = RuntimeWorldCancellationSource::new();
        crate::lifecycle::RuntimeWorldPreparationService::reserve(
            reserve_owner.as_ref(),
            candidate,
            &cancellation.token(),
            None,
        )
    });
    wait_until_bootstrap_is_held(&owner);
    let close_owner = Arc::clone(&owner);
    let close = std::thread::spawn(move || close_owner.close());
    drop(ledger_gate);

    let attempt = reserve
        .join()
        .expect("reserve worker does not panic")
        .expect("reserve wins after holding bootstrap admission");
    assert_eq!(
        close.join().expect("close worker does not panic"),
        Err(RuntimeWorldCloseDenial::AlreadyClosing)
    );
    drop(attempt);
    owner
        .close()
        .expect("close succeeds after the winner settles");

    let (_fixture, owner, expected) = setup();
    let candidate = plan(&owner, expected, "close-wins");
    let ledger_gate = owner
        .state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let close_owner = Arc::clone(&owner);
    let close = std::thread::spawn(move || close_owner.close());
    wait_until_bootstrap_is_held(&owner);
    let reserve_owner = Arc::clone(&owner);
    let reserve = std::thread::spawn(move || {
        let cancellation = RuntimeWorldCancellationSource::new();
        crate::lifecycle::RuntimeWorldPreparationService::reserve(
            reserve_owner.as_ref(),
            candidate,
            &cancellation.token(),
            None,
        )
    });
    drop(ledger_gate);

    close
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
