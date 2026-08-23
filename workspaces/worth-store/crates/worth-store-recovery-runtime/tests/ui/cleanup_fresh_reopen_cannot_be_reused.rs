use worth_store::physical_runtime::{
    AdmittedRecoveryFilesystemMedia, CompletedPhysicalRecoveryFreshReopen,
    PhysicalRecoveryCoordination,
};
use worth_store_physical_format::VerifiedCheckpointStream;
use worth_store::physical_runtime::recovery_wal::VerifiedWalArtifact;

fn reuse_fresh_reopen(
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    reopened: CompletedPhysicalRecoveryFreshReopen,
    checkpoint: VerifiedCheckpointStream,
    first: VerifiedWalArtifact,
    second: VerifiedWalArtifact,
) {
    let _first = coordination.admit_cleanup_plan(
        media,
        reopened,
        checkpoint.clone(),
        [0x11; 32],
        [first],
    );
    let _second = coordination.admit_cleanup_plan(
        media,
        reopened,
        checkpoint,
        [0x22; 32],
        [second],
    );
}

fn main() {
    let _ = reuse_fresh_reopen;
}
