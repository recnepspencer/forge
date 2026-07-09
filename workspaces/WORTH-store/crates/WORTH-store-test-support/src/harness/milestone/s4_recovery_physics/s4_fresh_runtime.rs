use worth_store_recovery_physics::{RecoveryOfflineVerifier, RecoveryProfileId};

use crate::deterministic_s4_recovery_artifacts;

pub use worth_store_recovery_physics::{FreshRuntimeRecoveryDriver, RecoveryRuntimePosture};

pub fn deterministic_s4_fresh_runtime_driver() -> FreshRuntimeRecoveryDriver {
    let artifacts = deterministic_s4_recovery_artifacts();
    let evidence = RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_s4(),
    )
    .verify_fresh_runtime_reopen(&artifacts)
    .expect("deterministic S.4 artifacts must reopen");
    FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
}
