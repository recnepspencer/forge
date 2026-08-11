use worth_store::physical_runtime::{
    AdmittedRecoveryFilesystemMedia, CompletedPhysicalRecoveryFreshReopen,
    PhysicalRecoveryCoordination, PhysicalRecoveryFreshnessPort,
};
use worth_store_physical_format::VerifiedCheckpointStream;
use worth_store_recovery_physics::WalSegmentInspection;

fn substitute_raw_wal_inspection(
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    reopened: &CompletedPhysicalRecoveryFreshReopen,
    checkpoint: &VerifiedCheckpointStream,
    raw: WalSegmentInspection,
) {
    let _ = PhysicalRecoveryFreshnessPort::sample_cleanup(
        coordination,
        media,
        [1_u8; 32],
        reopened,
        checkpoint,
        raw,
    );
}

fn main() {}
