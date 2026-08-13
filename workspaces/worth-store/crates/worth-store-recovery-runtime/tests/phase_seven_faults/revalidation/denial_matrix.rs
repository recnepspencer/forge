use std::io::Write;

use worth_store::physical_runtime::{
    PhysicalRecoveryCleanupRemovalDenialKind, RecoveryCleanupArtifactRevalidationDenial,
    RecoveryCleanupRemovalDenialCause,
};
use worth_store_recovery_runtime::{
    PhysicalRecoveryOutcome, RecoveryCleanupDeferralEvidence, RecoveryCleanupPosture,
};

use super::{cleanup_world, current_checkpoint_bytes, empty_fault_schedule, reopen_with_schedule};

#[test]
fn missing_checkpoint_retains_the_exact_failed_first_read() {
    let world = cleanup_world("cleanup-checkpoint-read-failure");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    std::fs::remove_file(checkpoint_path(&world.root)).unwrap();

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("missing checkpoint remains deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("missing checkpoint must defer cleanup")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact checkpoint read failure")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Revalidation(
                RecoveryCleanupArtifactRevalidationDenial::CheckpointRead(_)
            )
        )
    ));
    let progress = denial.physical().unwrap().revalidation();
    assert_eq!(progress.reads_attempted(), 1);
    assert_eq!(progress.reads_completed(), 0);
    assert_eq!(progress.bytes_read(), 0);
    assert_eq!(evidence.counters().artifact_revalidation_read_failures, 1);
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
}

#[test]
fn oversized_checkpoint_retains_the_exact_length_mismatch() {
    let world = cleanup_world("cleanup-checkpoint-length-mismatch");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    let expected = current_checkpoint_bytes(&world.root);
    append_byte(&checkpoint_path(&world.root));

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("oversized checkpoint remains deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("oversized checkpoint must defer cleanup")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact checkpoint length mismatch")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Revalidation(
                RecoveryCleanupArtifactRevalidationDenial::CheckpointLengthMismatch {
                    expected_bytes,
                    observed_bytes,
                }
            )
        ) if *expected_bytes == expected && *observed_bytes == expected + 1
    ));
    let progress = denial.physical().unwrap().revalidation();
    assert_eq!(progress.reads_attempted(), 1);
    assert_eq!(progress.reads_completed(), 1);
    assert_eq!(progress.bytes_read(), expected + 1);
    assert_eq!(evidence.counters().artifact_revalidation_mismatches, 1);
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
}

#[test]
fn malformed_checkpoint_retains_the_exact_digest_denial() {
    let world = cleanup_world("cleanup-checkpoint-decode-denial");
    let candidate = world.oldest_wal();
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    let path = checkpoint_path(&world.root);
    let mut bytes = std::fs::read(&path).unwrap();
    let expected = bytes.len() as u64;
    bytes[0] ^= 0xff;
    std::fs::write(path, bytes).unwrap();

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("malformed checkpoint remains deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("malformed checkpoint must defer cleanup")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact checkpoint digest denial")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Revalidation(
                RecoveryCleanupArtifactRevalidationDenial::CheckpointDigestMismatch { .. }
            )
        )
    ));
    let progress = denial.physical().unwrap().revalidation();
    assert_eq!(progress.reads_attempted(), 1);
    assert_eq!(progress.reads_completed(), 1);
    assert_eq!(progress.bytes_read(), expected);
    assert_eq!(evidence.counters().artifact_revalidation_mismatches, 1);
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
}

#[test]
fn oversized_wal_retains_the_checkpoint_prefix_and_wal_length_mismatch() {
    let world = cleanup_world("cleanup-wal-length-mismatch");
    let candidate = world.oldest_wal();
    let expected_wal = std::fs::metadata(&candidate).unwrap().len();
    let expected_checkpoint = current_checkpoint_bytes(&world.root);
    let reopened = reopen_with_schedule(&world.root, empty_fault_schedule());
    append_byte(&candidate);

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("oversized WAL remains deferred cleanup debt")
    };
    let RecoveryCleanupPosture::Deferred(evidence) = handoff.cleanup_posture() else {
        panic!("oversized WAL must defer cleanup")
    };
    let [RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. }] = evidence.deferrals()
    else {
        panic!("one exact WAL length mismatch")
    };
    assert!(matches!(
        denial.kind(),
        PhysicalRecoveryCleanupRemovalDenialKind::Media(
            RecoveryCleanupRemovalDenialCause::Revalidation(
                RecoveryCleanupArtifactRevalidationDenial::LengthMismatch {
                    expected_bytes,
                    observed_bytes,
                }
            )
        ) if *expected_bytes == expected_wal && *observed_bytes == expected_wal + 1
    ));
    let progress = denial.physical().unwrap().revalidation();
    assert_eq!(progress.reads_attempted(), 2);
    assert_eq!(progress.reads_completed(), 2);
    assert_eq!(
        progress.bytes_read(),
        expected_checkpoint + expected_wal + 1
    );
    assert_eq!(evidence.counters().artifact_revalidation_mismatches, 1);
    assert!(candidate.exists());
    assert!(evidence.performed_removals().is_empty());
}

fn checkpoint_path(root: &std::path::Path) -> std::path::PathBuf {
    root.join("families").join("checkpoint.current")
}

fn append_byte(path: &std::path::Path) {
    std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap()
        .write_all(&[0xff])
        .unwrap();
}
