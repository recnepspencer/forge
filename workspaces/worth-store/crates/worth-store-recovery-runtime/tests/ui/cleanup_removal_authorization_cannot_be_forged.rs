use worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia;
use worth_store_authority::RecoveryCleanupEffectAuthorization;

fn fake<T>() -> T {
    panic!("compile-only specimen")
}

fn forged_authorization() -> RecoveryCleanupEffectAuthorization {
    RecoveryCleanupEffectAuthorization {
        authority: fake(),
        binding: fake(),
    }
}

fn bypass_backend(media: &AdmittedRecoveryFilesystemMedia) {
    let _ = media.remove_recovery_wal_artifact_scheduled(
        fake(),
        fake(),
        fake(),
        forged_authorization(),
        fake(),
    );
}

fn main() {
    bypass_backend(fake());
}
