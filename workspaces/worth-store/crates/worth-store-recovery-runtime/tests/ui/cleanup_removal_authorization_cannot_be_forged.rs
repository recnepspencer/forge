use worth_store::physical_runtime::{
    AdmittedRecoveryFilesystemMedia, PhysicalRecoveryCoordination,
    PhysicalRecoveryFreshnessPort, StoreRecoveryCleanupPlan,
};
use worth_store_recovery_physics::WalSegmentArtifactIdentity;

fn fake<T>() -> T {
    panic!("compile-only specimen")
}

fn bypass_store_owner(
    coordination: &PhysicalRecoveryCoordination,
    media: &AdmittedRecoveryFilesystemMedia,
    plan: &mut StoreRecoveryCleanupPlan<'_>,
    artifact: WalSegmentArtifactIdentity,
) {
    let _ = PhysicalRecoveryFreshnessPort::sample_cleanup(
        coordination,
        media,
        plan,
        artifact,
    );
    let _ = coordination.execute_cleanup_removal(media, fake::<()>());
}

fn main() {
    bypass_store_owner(fake(), fake(), fake(), fake());
}
