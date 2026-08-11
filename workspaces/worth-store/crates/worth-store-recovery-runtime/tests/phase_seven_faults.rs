#![cfg(feature = "certification-test-authority")]

#[allow(dead_code)]
mod phase_three_support;
#[path = "phase_seven_faults/world.rs"]
mod world;

use world::{
    assert_child_succeeded, cleanup_fault, cleanup_world, empty_fault_schedule,
    recover_with_schedule, reopen_with_schedule, required_child_root, run_child,
};

#[test]
fn ambiguous_cleanup_survives_process_death_and_a_second_fresh_recovery() {
    let world = cleanup_world("cleanup-crash-restart");
    let candidate = world.oldest_wal();
    let first = run_child("phase_seven_indeterminate_cleanup_process", &world.root);
    assert_child_succeeded("indeterminate cleanup", &first);
    assert!(
        !candidate.exists(),
        "the ambiguous removal escaped before death"
    );
    let second = run_child("phase_seven_reopen_after_cleanup_process", &world.root);
    assert_child_succeeded("post-cleanup recovery", &second);
}

#[test]
#[ignore = "launched by the Phase 7 cleanup crash-matrix parent"]
fn phase_seven_indeterminate_cleanup_process() {
    let root = required_child_root();
    let handoff = recover_with_schedule(
        &root,
        cleanup_fault(MediaFaultDirective::IndeterminateAfterEffect),
    );
    assert!(matches!(
        handoff.cleanup_posture(),
        RecoveryCleanupPosture::Deferred(_)
    ));
}

#[test]
#[ignore = "launched by the Phase 7 cleanup crash-matrix parent"]
fn phase_seven_reopen_after_cleanup_process() {
    let root = required_child_root();
    let PhysicalRecoveryOutcome::Recovered(handoff) =
        reopen_with_schedule(&root, empty_fault_schedule()).finish()
    else {
        panic!("ambiguous cleanup debt cannot invalidate a fresh recovery")
    };
    assert!(!handoff
        .cleanup_posture()
        .evidence()
        .dispositions()
        .is_empty());
}
use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    PhysicalRecoveryCleanupAdmissionDenialKind, PhysicalRecoveryCleanupCommandStage,
    PhysicalRecoveryCleanupFreshnessReadDenialKind, PhysicalRecoveryCleanupRemovalDenialKind,
    PhysicalRecoveryCleanupRemovalIndeterminate, PhysicalSignalSettlementOutcome,
    PhysicalWorkSchedulerPosture,
};
use worth_store_recovery_runtime::{
    PhysicalRecoveryOutcome, RecoveryCleanupDeferralEvidence, RecoveryCleanupDispositionKind,
    RecoveryCleanupPosture,
};

#[test]
fn cleanup_denial_retains_the_artifact_and_exact_backend_failure() {
    let world = cleanup_world("cleanup-denied");
    let candidate = world.oldest_wal();
    let schedule = cleanup_fault(MediaFaultDirective::FailBefore {
        kind: std::io::ErrorKind::PermissionDenied,
        raw_os_error: None,
    });
    let handoff = recover_with_schedule(&world.root, schedule);
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("cleanup denial produces deferred recovered handoff")
    };
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
    assert_eq!(evidence.counters().denied_before_effect, 1);
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact denied cleanup settlement")
    };
    let physical = denial.physical().expect("backend denial is retained");
    assert_eq!(
        physical.failure().kind(),
        worth_store::physical_runtime::ArtifactTreeFailureKind::DeniedBeforeEffect
    );
    assert_eq!(
        physical.failure().io_kind(),
        Some(std::io::ErrorKind::PermissionDenied)
    );
}

#[test]
fn ambiguous_cleanup_effect_is_deferred_without_performed_authority() {
    let world = cleanup_world("cleanup-indeterminate");
    let candidate = world.oldest_wal();
    let handoff = recover_with_schedule(
        &world.root,
        cleanup_fault(MediaFaultDirective::IndeterminateAfterEffect),
    );
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("ambiguous cleanup still returns recovered handoff with deferred debt")
    };
    assert!(
        !candidate.exists(),
        "the physical deletion may have escaped"
    );
    assert!(evidence.performed_removals().is_empty());
    assert_eq!(evidence.counters().indeterminate_effects, 1);
    assert_eq!(evidence.counters().performed_effects, 0);
    assert!(matches!(
        evidence.deferrals(),
        [RecoveryCleanupDeferralEvidence::IndeterminateEffect { .. }]
    ));
    assert!(evidence.dispositions().iter().any(|disposition| {
        disposition.kind()
            == RecoveryCleanupDispositionKind::Deferred(
                worth_store_recovery_runtime::RecoveryCleanupDeferralReason::IndeterminateEffect,
            )
    }));
}

#[test]
fn owner_sampled_generation_change_rejects_the_stale_cleanup_plan() {
    let world = cleanup_world("cleanup-stale-plan");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    reopened.certification_shift_cleanup_generation();
    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("stale cleanup plan remains maintenance debt, not recovery failure")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("generation shift defers cleanup")
    };
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
    assert_eq!(evidence.counters().freshness_evaluations, 1);
    assert_eq!(evidence.counters().actions_attempted, 0);
    assert!(matches!(
        evidence.deferrals(),
        [RecoveryCleanupDeferralEvidence::PublishedGenerationChanged {
            expected,
            observed,
            ..
        }] if *observed == expected + 1
    ));
}

#[test]
fn cleanup_scheduler_rejection_after_removal_retains_escaped_effect_without_performed_authority() {
    let world = cleanup_world("cleanup-scheduler-settlement");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    reopened.certification_fail_cleanup_scheduler_settlement_at(
        PhysicalRecoveryCleanupCommandStage::Removal,
    );
    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("cleanup scheduler debt cannot invalidate recovered publication")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("escaped removal with rejected scheduler settlement is deferred")
    };
    assert!(!candidate.exists());
    assert!(evidence.performed_removals().is_empty());
    assert_eq!(evidence.counters().removal_scheduler_submitted, 1);
    assert_eq!(evidence.counters().removal_scheduler_settled, 1);
    assert_eq!(evidence.counters().performed_effects, 0);
    let [RecoveryCleanupDeferralEvidence::IndeterminateEffect {
        evidence:
            PhysicalRecoveryCleanupRemovalIndeterminate::Scheduler {
                posture, signal, ..
            },
        ..
    }] = evidence.deferrals()
    else {
        panic!("one exact scheduler-indeterminate cleanup settlement")
    };
    assert_eq!(*posture, PhysicalWorkSchedulerPosture::RejectedAfterEffect);
    assert_eq!(
        *signal,
        PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth
    );
}

#[test]
fn cleanup_background_deferral_cancels_before_the_physical_effect() {
    let world = cleanup_world("cleanup-background-deferral");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    reopened.certification_defer_cleanup_background();
    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("scheduler deferral remains cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("background pacing deferral retains the candidate")
    };
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
    assert_eq!(evidence.counters().removal_scheduler_submitted, 1);
    assert_eq!(evidence.counters().removal_scheduler_deferred, 1);
    assert_eq!(evidence.counters().removal_scheduler_cancelled, 1);
    assert_eq!(evidence.counters().removal_scheduler_settled, 0);
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact pre-effect cleanup deferral")
    };
    let PhysicalRecoveryCleanupRemovalDenialKind::Admission(admission) = denial.kind() else {
        panic!("cleanup deferral retains Store admission evidence")
    };
    assert!(matches!(
        admission.kind(),
        PhysicalRecoveryCleanupAdmissionDenialKind::BackgroundPacing(_)
    ));
}

#[test]
fn freshness_scheduler_and_signal_failures_stop_before_cleanup_effects() {
    for (label, signal) in [
        ("cleanup-freshness-scheduler", false),
        ("cleanup-freshness-signal", true),
    ] {
        let world = cleanup_world(label);
        let candidate = world.oldest_wal();
        let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
        if signal {
            reopened.certification_fail_cleanup_freshness_signal_settlement();
        } else {
            reopened.certification_fail_cleanup_scheduler_settlement_at(
                PhysicalRecoveryCleanupCommandStage::FreshnessRead,
            );
        }
        let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
            panic!("freshness failure remains cleanup debt")
        };
        let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
            panic!("freshness settlement failure defers cleanup")
        };
        assert!(candidate.exists());
        assert!(evidence.performed_removals().is_empty());
        assert_eq!(evidence.counters().actions_attempted, 0);
        assert_eq!(evidence.counters().freshness_reads_completed, 1);
        assert_eq!(evidence.counters().freshness_scheduler_submitted, 1);
        assert_eq!(evidence.counters().freshness_scheduler_settled, 1);
        let [RecoveryCleanupDeferralEvidence::Freshness { failure, .. }] = evidence.deferrals()
        else {
            panic!("one exact freshness failure")
        };
        let kind = failure.read().expect("completed freshness read").kind();
        assert!(if signal {
            matches!(
                kind,
                PhysicalRecoveryCleanupFreshnessReadDenialKind::SignalSettlement(
                    PhysicalSignalSettlementOutcome::DerivedStateUnavailable
                )
            )
        } else {
            matches!(
                kind,
                PhysicalRecoveryCleanupFreshnessReadDenialKind::SchedulerSettlement(
                    PhysicalWorkSchedulerPosture::RejectedAfterEffect
                )
            )
        });
    }
}
