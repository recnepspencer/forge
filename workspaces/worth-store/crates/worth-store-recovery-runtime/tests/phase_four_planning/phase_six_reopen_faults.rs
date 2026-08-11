use worth_store::physical_runtime::{
    PhysicalRecoveryFreshReopenDenialKind, PhysicalRecoveryFreshReopenStage,
    PhysicalSignalSettlementOutcome, PhysicalWorkSchedulerPosture,
};
use worth_store_recovery_runtime::PhysicalRecoveryOutcome;

#[test]
fn selector_scheduler_failure_retains_the_completed_read() {
    assert_reopen_scheduler_failure(PhysicalRecoveryFreshReopenStage::CurrentSelector, 1, 0);
}

#[test]
fn root_scheduler_failure_retains_both_completed_reads() {
    assert_reopen_scheduler_failure(PhysicalRecoveryFreshReopenStage::RootManifest, 1, 1);
}

#[test]
fn selector_signal_failure_retains_the_completed_read() {
    assert_reopen_signal_failure(PhysicalRecoveryFreshReopenStage::CurrentSelector, 1, 0);
}

#[test]
fn root_signal_failure_retains_both_completed_reads() {
    assert_reopen_signal_failure(PhysicalRecoveryFreshReopenStage::RootManifest, 1, 1);
}

fn assert_reopen_scheduler_failure(
    stage: PhysicalRecoveryFreshReopenStage,
    selector_reads: u64,
    root_reads: u64,
) {
    let retained_root =
        super::prepare_ordinary_recovery_root(&format!("c8-phase6-reopen-scheduler-{stage:?}"));
    let published = super::selected_ordinary_recovery(retained_root.path())
        .plan()
        .unwrap()
        .stage()
        .unwrap()
        .publish()
        .unwrap();
    published.certification_fail_reopen_scheduler_settlement_at(stage);

    let Err(PhysicalRecoveryOutcome::PublicationIndeterminate(outcome)) = published.reopen() else {
        panic!("a nonterminal reopen Signal settlement must preserve indeterminate evidence")
    };
    let failure = outcome
        .reopen_failure()
        .expect("fresh-reopen failure evidence");
    assert_eq!(failure.counters().selector_reads_completed, selector_reads);
    assert_eq!(failure.counters().root_reads_completed, root_reads);
    assert!(failure.counters().bytes_read > 0);
    assert_eq!(
        failure.denial().kind(),
        PhysicalRecoveryFreshReopenDenialKind::SchedulerSettlement(
            PhysicalWorkSchedulerPosture::RejectedAfterEffect,
        )
    );
    assert!(outcome.recovery_effects() > 0);
}

fn assert_reopen_signal_failure(
    stage: PhysicalRecoveryFreshReopenStage,
    selector_reads: u64,
    root_reads: u64,
) {
    let retained_root =
        super::prepare_ordinary_recovery_root(&format!("c8-phase6-reopen-signal-{stage:?}"));
    let published = super::selected_ordinary_recovery(retained_root.path())
        .plan()
        .unwrap()
        .stage()
        .unwrap()
        .publish()
        .unwrap();
    published.certification_fail_reopen_signal_settlement_at(stage);

    let Err(PhysicalRecoveryOutcome::PublicationIndeterminate(outcome)) = published.reopen() else {
        panic!("a nonterminal reopen Signal settlement must preserve indeterminate evidence")
    };
    let failure = outcome
        .reopen_failure()
        .expect("fresh-reopen failure evidence");
    assert_eq!(failure.counters().selector_reads_completed, selector_reads);
    assert_eq!(failure.counters().root_reads_completed, root_reads);
    assert!(failure.counters().bytes_read > 0);
    assert_eq!(
        failure.denial().kind(),
        PhysicalRecoveryFreshReopenDenialKind::SignalSettlement(
            PhysicalSignalSettlementOutcome::DerivedStateUnavailable,
        )
    );
    assert!(outcome.recovery_effects() > 0);
}
