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
    let settlement = settled(execute_with_empty_signal(&owner, reserve(&owner, plan)));
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
