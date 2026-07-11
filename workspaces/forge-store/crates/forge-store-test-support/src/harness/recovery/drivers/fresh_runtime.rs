use forge_store_recovery_physics::{RecoveryOfflineVerifier, RecoveryProfileId};

use crate::deterministic_recovery_artifacts;

pub use forge_store_recovery_physics::{FreshRuntimeRecoveryDriver, RecoveryRuntimePosture};

pub fn deterministic_recovery_fresh_runtime_driver() -> FreshRuntimeRecoveryDriver {
    let artifacts = deterministic_recovery_artifacts();
    let evidence = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_offline_recovery_artifacts(),
    )
    .verify_fresh_runtime_reopen(&artifacts)
    .expect("deterministic S.4 artifacts must reopen");
    FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
}
