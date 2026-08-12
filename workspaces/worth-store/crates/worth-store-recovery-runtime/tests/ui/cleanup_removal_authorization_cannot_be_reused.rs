use worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia;
use worth_store_authority::RecoveryCleanupEffectAuthorization;

fn fake<T>() -> T {
    panic!("compile-only specimen")
}

fn reuse(
    media: &AdmittedRecoveryFilesystemMedia,
    authorization: RecoveryCleanupEffectAuthorization,
) {
    let _ = media.remove_recovery_wal_artifact_scheduled(
        fake(), fake(), fake(), authorization, fake(),
    );
    let _ = media.remove_recovery_wal_artifact_scheduled(
        fake(), fake(), fake(), authorization, fake(),
    );
}

fn main() {
    reuse(fake(), fake());
}
