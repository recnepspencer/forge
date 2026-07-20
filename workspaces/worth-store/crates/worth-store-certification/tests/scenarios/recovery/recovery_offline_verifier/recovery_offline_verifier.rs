#[path = "assertions.rs"]
mod assertions;
use crate::runtime_recovery_fixture;

use worth_store_recovery_physics::{
    FreshRuntimeRecoveryWitness, OfflineRecoveryVerifierConclusion,
    PersistedRecoveryArtifactDenial, RecoveryDeterminismClassification, RecoveryDeterminismReport,
    RecoveryNondeterministicMetadata, RecoveryOfflineVerifier, RecoveryProfileId,
    RecoveryRuntimeClassification, ReopenedRecoveryArtifactAdmissionDenial, ReopenedRecoveryDenial,
    RuntimeRecoveryReport, RuntimeRecoveryReportDenial,
};
use worth_store_test_support::{
    deterministic_recovery_artifacts, duplicate_role_recovery_artifacts,
    incomplete_recovery_artifacts, malformed_recovery_record, reordered_recovery_artifacts,
    runtime_disagreement_recovery_artifacts, runtime_state_mismatch_recovery_artifacts,
    FreshRuntimeRecoveryDriver,
};

use assertions::{
    assert_deterministic_recovery, assert_expected_recovery_counters,
    assert_independent_offline_report, assert_runtime_report_denial,
};
use runtime_recovery_fixture::execute_reopened_recovery_fixture;

#[test]
fn identical_persisted_bytes_recover_to_identical_classification_and_reports() {
    let verifier = verifier();
    let artifacts = deterministic_recovery_artifacts();
    let reordered = reordered_recovery_artifacts();
    let first_offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let second_offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let reordered_offline = verifier.verify_persisted_artifacts(&reordered).unwrap();
    let (first_receipt, first_execution) =
        execute_reopened_recovery_fixture(&first_offline, &artifacts).unwrap();
    let (second_receipt, second_execution) =
        execute_reopened_recovery_fixture(&second_offline, &artifacts).unwrap();

    assert_eq!(
        first_offline.artifact_digest(),
        reordered_offline.artifact_digest(),
        "record iteration order must not enter persisted artifact identity"
    );
    assert_expected_recovery_counters(first_receipt.counters());
    assert_expected_recovery_counters(second_receipt.counters());

    let first_runtime = runtime_report(&first_offline, &artifacts, &first_receipt, first_execution);
    let second_runtime = runtime_report(
        &second_offline,
        &artifacts,
        &second_receipt,
        second_execution,
    );
    let determinism = RecoveryDeterminismReport::compare_repeated_fresh_recovery(
        &first_runtime,
        &second_runtime,
        &first_offline,
        &second_offline,
    );

    assert_eq!(
        first_runtime.classification(),
        RecoveryRuntimeClassification::Recovered
    );
    assert_deterministic_recovery(&determinism);
}

#[test]
fn verifier_runtime_disagreement_is_typed_evidence() {
    let verifier = verifier();
    let artifacts = runtime_disagreement_recovery_artifacts();
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let denial = runtime_report_result(&offline, &artifacts).unwrap_err();

    assert_reopened_admission_denial(
        denial,
        ReopenedRecoveryArtifactAdmissionDenial::VerifierConclusionMismatch,
    );
}

#[test]
fn runtime_state_disagreement_is_typed_admission_evidence() {
    let verifier = verifier();
    let artifacts = runtime_state_mismatch_recovery_artifacts();
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let denial = runtime_report_result(&offline, &artifacts).unwrap_err();

    assert_reopened_runtime_denial(denial, RuntimeRecoveryReportDenial::RecoveredStateMismatch);
}

#[test]
fn offline_verifier_inspects_persisted_records_without_live_runtime_or_cache_reuse() {
    let verifier = verifier();
    let artifacts = deterministic_recovery_artifacts();
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();

    assert_independent_offline_report(&offline);
}

#[test]
fn same_process_live_state_reuse_is_denied_as_runtime_evidence() {
    let verifier = verifier();
    let artifacts = deterministic_recovery_artifacts();
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let (_receipt, execution) = execute_reopened_recovery_fixture(&offline, &artifacts).unwrap();
    let denial = FreshRuntimeRecoveryDriver::same_process_live_state_reuse()
        .witness_from_reopened_execution(execution)
        .unwrap_err();

    assert_runtime_report_denial(
        denial,
        RuntimeRecoveryReportDenial::SameProcessLiveStateReuse,
    );
}

#[test]
fn malformed_physical_record_is_admission_denial() {
    let denial = malformed_recovery_record().unwrap_err();

    assert!(matches!(
        denial,
        PersistedRecoveryArtifactDenial::MalformedPhysicalRecord { .. }
    ));
}

#[test]
fn incomplete_physical_record_set_is_typed_offline_evidence() {
    let verifier = verifier();
    let artifacts = incomplete_recovery_artifacts();
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let denial = runtime_report_result(&offline, &artifacts).unwrap_err();

    assert_eq!(
        offline.conclusion(),
        OfflineRecoveryVerifierConclusion::IncompletePhysicalRecordSet
    );
    assert_reopened_admission_denial(
        denial,
        ReopenedRecoveryArtifactAdmissionDenial::VerifierConclusionMismatch,
    );
}

#[test]
fn duplicate_physical_record_roles_are_typed_offline_evidence() {
    let verifier = verifier();
    let artifacts = duplicate_role_recovery_artifacts();
    let offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();

    assert_eq!(
        offline.conclusion(),
        OfflineRecoveryVerifierConclusion::AmbiguousPhysicalRecordSet
    );
}

#[test]
fn allowed_nondeterministic_metadata_is_canonicalized_before_comparison() {
    let verifier = verifier();
    let artifacts = deterministic_recovery_artifacts();
    let first_offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let second_offline = verifier.verify_persisted_artifacts(&artifacts).unwrap();
    let (first_receipt, first_execution) =
        execute_reopened_recovery_fixture(&first_offline, &artifacts).unwrap();
    let (second_receipt, second_execution) =
        execute_reopened_recovery_fixture(&second_offline, &artifacts).unwrap();

    let first_runtime = runtime_report_with_metadata(
        &first_offline,
        &artifacts,
        &first_receipt,
        first_execution,
        vec![
            RecoveryNondeterministicMetadata::ThreadScheduling,
            RecoveryNondeterministicMetadata::WallClockTimestamp,
            RecoveryNondeterministicMetadata::ThreadScheduling,
        ],
    );
    let second_runtime = runtime_report_with_metadata(
        &second_offline,
        &artifacts,
        &second_receipt,
        second_execution,
        vec![
            RecoveryNondeterministicMetadata::WallClockTimestamp,
            RecoveryNondeterministicMetadata::ThreadScheduling,
        ],
    );
    let determinism = RecoveryDeterminismReport::compare_repeated_fresh_recovery(
        &first_runtime,
        &second_runtime,
        &first_offline,
        &second_offline,
    );

    assert_eq!(
        determinism.classification(),
        RecoveryDeterminismClassification::Deterministic
    );
    assert_eq!(
        determinism.allowed_nondeterministic_metadata(),
        &[
            RecoveryNondeterministicMetadata::WallClockTimestamp,
            RecoveryNondeterministicMetadata::ThreadScheduling,
        ]
    );
}

fn verifier() -> RecoveryOfflineVerifier {
    RecoveryOfflineVerifier::for_profile(
        "s4-format-v1",
        "strict-posix-fsync-dir-fsync",
        RecoveryProfileId::strict_offline_recovery_artifacts(),
    )
}

fn runtime_report(
    offline: &worth_store_recovery_physics::OfflineRecoveryVerificationReport,
    artifacts: &worth_store_recovery_physics::PersistedRecoveryArtifacts,
    receipt: &worth_store_recovery_physics::BoundedRecoveryReceipt,
    execution: worth_store_recovery_physics::FreshRuntimeRecoveryExecution,
) -> RuntimeRecoveryReport {
    runtime_report_with_execution(offline, artifacts, receipt, execution).unwrap()
}

fn runtime_report_with_metadata(
    offline: &worth_store_recovery_physics::OfflineRecoveryVerificationReport,
    artifacts: &worth_store_recovery_physics::PersistedRecoveryArtifacts,
    receipt: &worth_store_recovery_physics::BoundedRecoveryReceipt,
    execution: worth_store_recovery_physics::FreshRuntimeRecoveryExecution,
    metadata: Vec<RecoveryNondeterministicMetadata>,
) -> RuntimeRecoveryReport {
    let witness = fresh_runtime_witness(artifacts, execution);
    RuntimeRecoveryReport::from_verified_bounded_recovery(
        witness,
        offline,
        RecoveryRuntimeClassification::Recovered,
        receipt,
        metadata,
    )
    .unwrap()
}

fn runtime_report_result(
    offline: &worth_store_recovery_physics::OfflineRecoveryVerificationReport,
    artifacts: &worth_store_recovery_physics::PersistedRecoveryArtifacts,
) -> Result<RuntimeRecoveryReport, ReopenedRecoveryDenial> {
    let (receipt, execution) = execute_reopened_recovery_fixture(offline, artifacts)?;
    runtime_report_with_execution(offline, artifacts, &receipt, execution)
        .map_err(ReopenedRecoveryDenial::Runtime)
}

fn runtime_report_with_execution(
    offline: &worth_store_recovery_physics::OfflineRecoveryVerificationReport,
    artifacts: &worth_store_recovery_physics::PersistedRecoveryArtifacts,
    receipt: &worth_store_recovery_physics::BoundedRecoveryReceipt,
    execution: worth_store_recovery_physics::FreshRuntimeRecoveryExecution,
) -> Result<RuntimeRecoveryReport, RuntimeRecoveryReportDenial> {
    let witness = fresh_runtime_witness(artifacts, execution);
    runtime_report_with_witness(witness, offline, receipt)
}

fn runtime_report_with_witness(
    witness: FreshRuntimeRecoveryWitness,
    offline: &worth_store_recovery_physics::OfflineRecoveryVerificationReport,
    receipt: &worth_store_recovery_physics::BoundedRecoveryReceipt,
) -> Result<RuntimeRecoveryReport, RuntimeRecoveryReportDenial> {
    RuntimeRecoveryReport::from_verified_bounded_recovery(
        witness,
        offline,
        RecoveryRuntimeClassification::Recovered,
        receipt,
        Vec::new(),
    )
}

fn fresh_runtime_witness(
    artifacts: &worth_store_recovery_physics::PersistedRecoveryArtifacts,
    execution: worth_store_recovery_physics::FreshRuntimeRecoveryExecution,
) -> FreshRuntimeRecoveryWitness {
    let evidence = verifier().verify_fresh_runtime_reopen(artifacts).unwrap();
    FreshRuntimeRecoveryDriver::from_reopen_harness_evidence(evidence)
        .witness_from_reopened_execution(execution)
        .unwrap()
}

fn assert_reopened_runtime_denial(
    denial: ReopenedRecoveryDenial,
    expected: RuntimeRecoveryReportDenial,
) {
    match denial {
        ReopenedRecoveryDenial::Runtime(denial) => assert_runtime_report_denial(denial, expected),
        ReopenedRecoveryDenial::Admission(denial) => {
            panic!("reopened artifact admission failed before runtime report: {denial:?}")
        }
        ReopenedRecoveryDenial::Redo(denial) => {
            panic!("bounded recovery fixture failed before runtime admission: {denial:?}")
        }
    }
}

fn assert_reopened_admission_denial(
    denial: ReopenedRecoveryDenial,
    expected: ReopenedRecoveryArtifactAdmissionDenial,
) {
    match denial {
        ReopenedRecoveryDenial::Admission(denial) => assert_eq!(denial, expected),
        ReopenedRecoveryDenial::Runtime(denial) => {
            panic!("runtime report failed before artifact admission denial: {denial:?}")
        }
        ReopenedRecoveryDenial::Redo(denial) => {
            panic!("bounded recovery fixture failed before artifact admission: {denial:?}")
        }
    }
}
