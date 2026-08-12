use worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia;
use worth_store_authority::{
    RecoveryCleanupEffectBinding, RecoveryCleanupEffectIssuer,
};

fn supplied<T>() -> T {
    panic!("compile-only supplied production value")
}

fn bypass(media: &AdmittedRecoveryFilesystemMedia) {
    let issuer = RecoveryCleanupEffectIssuer::admit(supplied()).unwrap();
    let authorization = issuer.authorize(supplied::<RecoveryCleanupEffectBinding>());
    let _ = media.remove_recovery_wal_artifact_scheduled(
        supplied(),
        supplied(),
        supplied(),
        authorization,
        supplied(),
        supplied(),
    );
}

fn main() {
    bypass(supplied());
}
