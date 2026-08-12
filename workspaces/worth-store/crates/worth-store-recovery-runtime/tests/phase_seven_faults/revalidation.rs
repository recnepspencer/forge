use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    PhysicalRecoveryCleanupRemovalDenialKind, RecoveryCleanupArtifactRevalidationDenial,
    RecoveryCleanupRemovalDenialCause,
};
use worth_store_recovery_runtime::{
    PhysicalRecoveryOutcome, RecoveryCleanupDeferralEvidence, RecoveryCleanupPosture,
};

use super::world::{
    cleanup_fault, cleanup_world, empty_fault_schedule, recover_with_schedule, reopen_with_schedule,
};

#[test]
fn cleanup_denial_retains_the_artifact_and_exact_backend_failure() {
    let world = cleanup_world("cleanup-denied");
    let candidate = world.oldest_wal();
    let candidate_bytes = std::fs::metadata(&candidate).unwrap().len();
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
    assert!(matches!(
        physical.cause(),
        RecoveryCleanupRemovalDenialCause::Removal(_)
    ));
    assert_eq!(physical.revalidation().reads_attempted(), 1);
    assert_eq!(physical.revalidation().reads_completed(), 1);
    assert_eq!(physical.revalidation().bytes_read(), candidate_bytes);
    assert_eq!(evidence.counters().artifact_revalidation_reads_attempted, 1);
    assert_eq!(evidence.counters().artifact_revalidation_reads_completed, 1);
    assert_eq!(
        evidence.counters().artifact_revalidation_bytes_read,
        candidate_bytes
    );
}

#[test]
fn cleanup_revalidates_exact_wal_bytes_before_deletion() {
    let world = cleanup_world("cleanup-wal-substitution");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    let mut substituted = std::fs::read(&candidate).unwrap();
    let candidate_bytes = substituted.len() as u64;
    *substituted.last_mut().expect("candidate WAL is nonempty") ^= 0xff;
    std::fs::write(&candidate, substituted).unwrap();

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("stale WAL bytes remain deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("substituted WAL bytes must deny cleanup")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact digest mismatch")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Revalidation(
                RecoveryCleanupArtifactRevalidationDenial::DigestMismatch { .. }
            )
        )
    ));
    let progress = denial.physical().unwrap().revalidation();
    assert_eq!(progress.reads_attempted(), 1);
    assert_eq!(progress.reads_completed(), 1);
    assert_eq!(progress.bytes_read(), candidate_bytes);
    assert_eq!(evidence.counters().artifact_revalidation_mismatches, 1);
    assert_eq!(
        evidence.counters().artifact_revalidation_bytes_read,
        candidate_bytes
    );
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
}

#[test]
fn cleanup_revalidation_read_failure_retains_exact_zero_byte_progress() {
    let world = cleanup_world("cleanup-wal-read-failure");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    std::fs::remove_file(candidate).unwrap();

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("reread failure remains deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("missing cleanup artifact must defer")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact reread failure")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Revalidation(
                RecoveryCleanupArtifactRevalidationDenial::Read(_)
            )
        )
    ));
    let progress = denial.physical().unwrap().revalidation();
    assert_eq!(progress.reads_attempted(), 1);
    assert_eq!(progress.reads_completed(), 0);
    assert_eq!(progress.bytes_read(), 0);
    assert_eq!(evidence.counters().artifact_revalidation_reads_attempted, 1);
    assert_eq!(evidence.counters().artifact_revalidation_reads_completed, 0);
    assert_eq!(evidence.counters().artifact_revalidation_read_failures, 1);
    assert_eq!(evidence.counters().artifact_revalidation_bytes_read, 0);
}

#[test]
fn cleanup_authorization_substitution_denies_before_revalidation_or_effect() {
    let world = cleanup_world("cleanup-authorization-substitution");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    reopened.certification_substitute_cleanup_authorization();

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("authorization mismatch remains deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("authorization mismatch must defer cleanup")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact authorization denial")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Admission
        )
    ));
    let physical = denial.physical().expect("backend denial is retained");
    assert_eq!(
        physical.cause(),
        RecoveryCleanupRemovalDenialCause::Admission
    );
    assert!(physical.queue().is_none());
    assert_eq!(physical.revalidation().reads_attempted(), 0);
    assert_eq!(physical.revalidation().reads_completed(), 0);
    assert_eq!(physical.revalidation().bytes_read(), 0);
    assert_eq!(evidence.counters().artifact_revalidation_reads_attempted, 0);
    assert_eq!(evidence.counters().performed_effects, 0);
    assert!(evidence.performed_removals().is_empty());
    assert!(candidate.exists());
}
