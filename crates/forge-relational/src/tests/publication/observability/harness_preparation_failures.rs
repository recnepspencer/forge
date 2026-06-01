use super::fixtures::*;

#[test]
fn harness_phase8_certification_matrix_closes_out_preparation_failures() {
    let serial_adapter = InvariantHarnessAdapter::new(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(16),
        )],
        ..InvariantCatalog::default()
    });
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();
    let serial_report = forge_harness::facade::certification_matrix(
        serial_adapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::staged_parallel("staged"))
    .certify()
    .unwrap();
    let serial_summary = certification_case(&serial_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(serial_summary, "SerialPreparationSelected")
            .iter()
            .any(|entry| harness_diagnostic_field_matches(
                entry,
                "reason",
                "insufficient_packet_breadth"
            ))
    );
    assert!(
        harness_diagnostic_entries(serial_summary, "PreparationFailure")
            .iter()
            .any(|entry| harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "serial_strategy_selected"
            ))
    );

    let proof_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PlanningProofInsufficient,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
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
    let proof_summary = certification_case(&proof_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(proof_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "planning_proof_insufficient",
                )
            })
    );

    let isolation_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::PublicationIsolationViolation,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
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
    let isolation_summary = certification_case(&isolation_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(isolation_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "publication_isolation_violation",
                )
            })
    );

    let reducer_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::ReductionIdentityConflict,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
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
    let reducer_summary = certification_case(&reducer_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(reducer_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "reduction_identity_conflict",
                )
            })
    );

    let worker_report = crate::validation::execution::with_test_preparation_fault(
        crate::validation::execution::TestPreparationFault::WorkerEvaluationFailure,
        || {
            let adapter = profitable_commit_boundary_adapter();
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
                adapter,
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
    let worker_summary = certification_case(&worker_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(worker_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "worker_evaluation_failure",
                )
            })
    );

    let consumer_report = crate::publication::logic::with_test_post_commit_fault(
        crate::publication::logic::TestPostCommitFault::ConsumerFailureNonAuthoritative,
        || {
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
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
    let consumer_summary = certification_case(&consumer_report, "post-commit")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(consumer_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "consumer_failure_non_authoritative",
                )
            })
    );

    let fragment_report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::FragmentCanonicalizationFailure,
        || {
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
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
    let fragment_summary = certification_case(&fragment_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(fragment_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(
                    entry,
                    "failure_class",
                    "fragment_canonicalization_failure",
                )
            })
    );

    let overlap_report = crate::authority::commit::with_test_diff_preparation_fault(
        crate::authority::commit::TestDiffPreparationFault::PacketOverlapDetected,
        || {
            let (fixture, batch, request) = harness_phase8_fixture_batch_request();
            forge_harness::facade::certification_matrix(
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
    let overlap_summary = certification_case(&overlap_report, "staged")
        .diagnostics_summary
        .as_ref()
        .unwrap();
    assert!(
        harness_diagnostic_entries(overlap_summary, "PreparationFailure")
            .iter()
            .any(|entry| {
                harness_diagnostic_field_matches(entry, "failure_class", "packet_overlap_detected")
            })
    );
}
