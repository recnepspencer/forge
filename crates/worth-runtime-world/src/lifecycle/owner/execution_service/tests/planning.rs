use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::*;

#[test]
fn relational_only_executes_the_canonical_relational_owner_path() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "relational-only",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        None,
    );
    let settlement = settled(execute_without_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().relational_posture(),
        RelationalAttemptProgressPosture::Settled
    );
    assert_eq!(
        settlement.progress().signal_posture(),
        SignalAttemptProgressPosture::Untouched
    );
    assert_ne!(
        settlement.successor_basis().unwrap().relational_basis(),
        expected.basis().relational_basis()
    );
    let successor = settlement.successor_basis().cloned().unwrap();
    settlement
        .ready(successor)
        .expect("settled Relational evidence forms a ready product publication");
}

#[test]
fn signal_only_uses_the_real_mutation_owner_without_relational_contact() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "signal-only",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::signal_only(),
        None,
    );
    let settlement = settled(execute_with_empty_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().relational_posture(),
        RelationalAttemptProgressPosture::Untouched
    );
    assert_eq!(
        settlement.progress().signal_posture(),
        SignalAttemptProgressPosture::Performed
    );
    assert_ne!(
        settlement
            .successor_basis()
            .unwrap()
            .signal_basis()
            .admission_identity(),
        expected.basis().signal_basis().admission_identity()
    );
    let successor = settlement.successor_basis().cloned().unwrap();
    settlement
        .ready(successor)
        .expect("settled Signal evidence forms a ready product publication");
}

#[test]
fn both_changed_settles_relational_before_signal_and_preserves_both_bases() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "both-changed",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_and_signal(RelationalTransactionIntent::ordinary()),
        None,
    );
    let relational_branch = expected.basis().relational_basis().identity().clone();
    let expected_relational = expected.basis().relational_basis().clone();
    let relational_was_observed = Arc::new(AtomicBool::new(false));
    let observed_for_callback = Arc::clone(&relational_was_observed);
    let owner_for_callback = Arc::clone(&owner);
    let runtime_cancellation = RuntimeWorldCancellationSource::new();
    let runtime_token = runtime_cancellation.token();
    let signal_cancellation = SignalOwnerCancellationSource::new();
    let signal_token = signal_cancellation.token();
    let mut context = ();
    let settlement = settled(RuntimeWorldOwnerExecutionService::execute(
        owner.as_ref(),
        reserve(&owner, plan),
        CompositeExecutionBorrow::signal(&mut context, &signal_token, move |_| {
            let (_, observed) = owner_for_callback
                .state
                .relational
                .basis_port()
                .observe_branch(&relational_branch)
                .map_err(|_| {
                    worth_signal::facade::SignalError::invalid_input(
                        "Relational successor observation was unavailable",
                    )
                })?;
            if observed == expected_relational {
                return Err(worth_signal::facade::SignalError::invalid_input(
                    "Signal owner ran before the Relational successor",
                ));
            }
            observed_for_callback.store(true, Ordering::Release);
            Ok(())
        }),
        &runtime_token,
    ));
    assert!(
        relational_was_observed.load(Ordering::Acquire),
        "the real Signal callback must observe the Relational successor"
    );
    assert_eq!(
        settlement.progress().relational_posture(),
        RelationalAttemptProgressPosture::Settled
    );
    assert_eq!(
        settlement.progress().signal_posture(),
        SignalAttemptProgressPosture::Performed
    );
    let successor = settlement.successor_basis().cloned().unwrap();
    assert_ne!(
        successor.relational_basis(),
        expected.basis().relational_basis()
    );
    assert_ne!(
        successor.signal_basis().admission_identity(),
        expected.basis().signal_basis().admission_identity()
    );
    settlement
        .ready(successor.clone())
        .expect("the ordered owner results form one exact successor");
}

#[test]
fn relational_fork_exact_consumes_the_owner_reservation_and_returns_target_basis() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let input = fixture.relational_fork_input("relational-fork-exact", None);
    let plan = plan_with_relational_fork(
        &owner,
        &expected,
        "relational-fork-exact",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ForkExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        input,
        None,
    );
    let settlement = settled(execute_without_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().relational_posture(),
        RelationalAttemptProgressPosture::Performed
    );
    assert_eq!(settlement.progress().owner_effect_count(), 1);
    let successor = settlement.successor_basis().cloned().unwrap();
    assert_ne!(
        successor.relational_basis().identity(),
        expected.basis().relational_basis().identity()
    );
    settlement
        .ready(successor)
        .expect("the exact fork evidence forms a ready publication");
}

#[test]
fn relational_fork_and_advance_uses_the_real_admission_and_publication_path() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let input = fixture.relational_fork_input(
        "relational-fork-and-advance",
        Some(WorkerIntentBatch::new("relational-fork-and-advance")),
    );
    let plan = plan_with_relational_fork(
        &owner,
        &expected,
        "relational-fork-and-advance",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ForkAndAdvance,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        input,
        None,
    );
    let settlement = settled(execute_without_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().relational_posture(),
        RelationalAttemptProgressPosture::Settled
    );
    assert_eq!(settlement.progress().owner_effect_count(), 1);
    let successor = settlement.successor_basis().cloned().unwrap();
    assert_ne!(
        successor.relational_basis().identity(),
        expected.basis().relational_basis().identity()
    );
    settlement
        .ready(successor)
        .expect("the fork-and-advance evidence forms a ready publication");
}

#[test]
fn reuse_exact_signal_does_not_contact_signal_when_relational_changes() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "reuse-signal",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        None,
    );
    let settlement = settled(execute_without_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().signal_posture(),
        SignalAttemptProgressPosture::Untouched
    );
    assert_eq!(
        settlement.progress().owner_effect_count(),
        1,
        "only the requested Relational owner moved"
    );
}

#[test]
fn signal_fork_exact_reserves_then_consumes_without_an_advance() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "signal-fork-exact",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkExact,
        ),
        CompositeComponentIntent::signal_only(),
        Some("signal-fork-exact"),
    );
    let settlement = settled(execute_without_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().signal_posture(),
        SignalAttemptProgressPosture::Performed
    );
    let successor = settlement.successor_basis().cloned().unwrap();
    assert_ne!(
        successor.signal_basis().admission_identity(),
        expected.basis().signal_basis().admission_identity()
    );
    settlement
        .ready(successor)
        .expect("ForkExact owner evidence is publishable");
}

#[test]
fn signal_fork_and_advance_consumes_the_same_reservation_before_advance() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "signal-fork-and-advance",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
        ),
        CompositeComponentIntent::signal_only(),
        Some("signal-fork-and-advance"),
    );
    let settlement = settled(execute_with_empty_signal(&owner, reserve(&owner, plan)));
    assert_eq!(
        settlement.progress().signal_posture(),
        SignalAttemptProgressPosture::Performed
    );
    let successor = settlement.successor_basis().cloned().unwrap();
    assert_ne!(
        successor.signal_basis().admission_identity(),
        expected.basis().signal_basis().admission_identity()
    );
    settlement
        .ready(successor)
        .expect("ForkAndAdvance owner evidence is publishable");
}

#[test]
fn signal_fork_and_advance_requires_the_signal_owner_borrow_and_releases_its_reservation() {
    let (fixture, owner, expected) = setup();
    let denied_plan = plan(
        &fixture,
        &owner,
        &expected,
        "signal-fork-and-advance-without-signal",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
        ),
        CompositeComponentIntent::signal_only(),
        Some("signal-fork-and-advance-without-signal"),
    );
    let denied = execute_without_signal(&owner, reserve(&owner, denied_plan));
    assert!(matches!(
        denied,
        OwnerExecutionOutcome::NoEffect(no_effect)
            if no_effect.cause() == crate::publication::NoEffectCause::OwnerUnavailable
    ));

    let retry_plan = plan(
        &fixture,
        &owner,
        &expected,
        "signal-fork-and-advance-without-signal-retry",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
        ),
        CompositeComponentIntent::signal_only(),
        Some("signal-fork-and-advance-without-signal"),
    );
    assert!(matches!(
        execute_with_empty_signal(&owner, reserve(&owner, retry_plan)),
        OwnerExecutionOutcome::Settled(_)
    ));
}
