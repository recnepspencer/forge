use crate::runtime_recovery_fixture;

use worth_store_physical_certification::IndependentVerifierObservation;
use worth_store_recovery_physics::{
    FreshRuntimeRecoveryDriver, RecoveryNondeterministicMetadata, RecoveryOfflineVerifier,
    RecoveryProfileId, RecoveryRuntimeClassification, RuntimeRecoveryComparisonReport,
    RuntimeRecoveryReport,
};
use worth_store_test_support::harness::recovery::{
    deterministic_recovery_artifacts, runtime_disagreement_recovery_artifacts,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // This shared fixture module is compiled independently by each integration lane.
pub enum RuntimeComparisonFixture {
    Equivalent,
    ArtifactDigestMismatch,
}

pub fn observed_runtime_comparison(
    fixture: RuntimeComparisonFixture,
) -> IndependentVerifierObservation {
    let (runtime, equivalent_offline) = runtime_and_offline_reports();
    let offline = match fixture {
        RuntimeComparisonFixture::Equivalent => equivalent_offline,
        RuntimeComparisonFixture::ArtifactDigestMismatch => verifier()
            .verify_persisted_artifacts(&runtime_disagreement_recovery_artifacts())
            .unwrap(),
    };
    IndependentVerifierObservation::from_runtime_recovery_comparison(
        &RuntimeRecoveryComparisonReport::compare(&runtime, &offline),
    )
}

fn runtime_and_offline_reports() -> (
    RuntimeRecoveryReport,
    worth_store_recovery_physics::OfflineRecoveryVerificationReport,
) {
    let artifacts = deterministic_recovery_artifacts();
    let offline = verifier().verify_persisted_artifacts(&artifacts).unwrap();
    let (receipt, execution) =
        runtime_recovery_fixture::execute_reopened_recovery_fixture(&offline, &artifacts).unwrap();
    let evidence = verifier().verify_fresh_runtime_reopen(&artifacts).unwrap();
    let witness = FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
        .witness_from_reopened_execution(execution)
        .unwrap();
    let runtime = RuntimeRecoveryReport::from_verified_bounded_recovery(
        witness,
        &offline,
        RecoveryRuntimeClassification::Recovered,
        &receipt,
        Vec::<RecoveryNondeterministicMetadata>::new(),
    )
    .unwrap();
    (runtime, offline)
}

fn verifier() -> RecoveryOfflineVerifier {
    RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_offline_recovery_artifacts(),
    )
}
