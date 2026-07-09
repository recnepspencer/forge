use super::fixtures::*;

#[test]
fn harness_phase8_planning_proof_failures_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PlanningProofInsufficient,
        || {
            worth_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();

    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "planning_proof_insufficient")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_publication_isolation_failures_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PublicationIsolationViolation,
        || {
            worth_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();

    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "publication_isolation_violation")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_reducer_conflicts_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::ReductionIdentityConflict,
        || {
            worth_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();
    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "reduction_identity_conflict")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_worker_evaluation_failures_are_harness_visible() {
    let adapter = profitable_commit_boundary_adapter();
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let bundles = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::WorkerEvaluationFailure,
        || {
            worth_harness::facade::run_matrix(adapter, fixture, request)
                .mutate(batch)
                .profile(ExecutionProfile::staged_parallel("staged"))
                .diagnose()
                .unwrap()
        },
    );
    let summary = bundles[0].diagnostics.as_ref().unwrap().summary.clone();
    let failure_entries = harness_diagnostic_entries(&summary, "PreparationFailure");

    assert!(failure_entries.iter().any(|entry| {
        harness_diagnostic_field_matches(entry, "failure_class", "worker_evaluation_failure")
    }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
    assert!(
        harness_summary_counter(&summary, "preparation_staged_parallel_strategy_count")
            .is_some_and(|count| count >= 1)
    );
}

#[test]
fn harness_phase8_post_commit_consumer_failures_are_harness_visible() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = crate::publication::logic::with_test_post_commit_fault(
        crate::publication::logic::TestPostCommitFault::ConsumerFailureNonAuthoritative,
        || {
            worth_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::full_parallel("post-commit"))
            .certify()
            .unwrap()
        },
    );
    let summary = certification_case(&report, "post-commit")
        .diagnostics_summary
        .as_ref()
        .unwrap();

    assert!(harness_diagnostic_entries(summary, "PreparationFailure")
        .iter()
        .any(|entry| {
            harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "consumer_failure_non_authoritative",
            )
        }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("FullParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("ParallelPostCommitConsumption")
    );
    assert!(
        harness_summary_counter(&summary, "post_commit_serial_strategy_count")
            .is_some_and(|count| count >= 1)
    );
    assert_eq!(
        harness_summary_counter(&summary, "post_commit_parallel_strategy_count"),
        Some(0)
    );
}

#[test]
fn harness_phase8_fragment_canonicalization_failures_are_harness_visible() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::FragmentCanonicalizationFailure,
        || {
            worth_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let summary = certification_case(&report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();

    assert!(harness_diagnostic_entries(summary, "PreparationFailure")
        .iter()
        .any(|entry| {
            harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "fragment_canonicalization_failure",
            )
        }));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
}

#[test]
fn harness_phase8_packet_overlap_failures_are_harness_visible() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::PacketOverlapDetected,
        || {
            worth_harness::facade::certification_matrix(
                RelationalHarnessAdapter,
                fixture,
                request,
                ExecutionProfile::serial("serial"),
            )
            .mutate(batch)
            .candidate(ExecutionProfile::staged_parallel("staged"))
            .certify()
            .unwrap()
        },
    );
    let summary = certification_case(&report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();

    assert!(harness_diagnostic_entries(summary, "PreparationFailure")
        .iter()
        .any(|entry| harness_diagnostic_field_matches(
            entry,
            "failure_class",
            "packet_overlap_detected"
        )));
    assert_eq!(
        harness_summary_field(&summary, "execution_mode"),
        Some("StagedParallel")
    );
    assert_eq!(
        harness_summary_field(&summary, "runtime_execution_model"),
        Some("StagedParallelPreparation")
    );
}
