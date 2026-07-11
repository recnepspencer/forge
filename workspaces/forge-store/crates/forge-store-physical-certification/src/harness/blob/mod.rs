mod fixture;
mod foundational_profile;
mod heavy_qualification;
#[cfg(test)]
mod heavy_qualification_tests;
mod lowering;
mod oracle_observation;
mod profile;
#[cfg(any(test, feature = "certification-test-support"))]
mod replay;
mod resume_recovery;
mod scenario_seed;
mod shortcut_denial;
#[cfg(test)]
mod tests;

pub use foundational_profile::BlobHarnessMaterializedProfile;
pub use lowering::{
    lower_blob_simulation_seed_plan, BlobHarnessLoweredSeedPlan, BlobHarnessLoweringDenial,
};
pub use oracle_observation::S7BlobHarnessOracleObservation;
pub use profile::{BlobHarnessProfile, BlobHarnessProfileSet};
#[cfg(any(test, feature = "certification-test-support"))]
pub use replay::blob_harness_replay_artifacts_for_certification;
#[cfg(any(test, feature = "certification-test-support"))]
pub use replay::{
    coverage_matrix_for_seed as synthetic_blob_harness_coverage_matrix_for_test_support,
    replay_bundle_for_seed as synthetic_blob_harness_replay_bundle_for_test_support,
};
pub use resume_recovery::{
    S7BlobResumeCrashPoint, S7BlobResumeExpectedOutcome, S7BlobResumeRecoveryScenario,
};
pub use scenario_seed::{BlobHarnessScenarioSeed, BlobHarnessScenarioSeedBuilder};
pub use shortcut_denial::{BlobHarnessShortcutAttempt, BlobHarnessShortcutDenial};
