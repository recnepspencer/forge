use super::fixtures::*;

#[test]
fn harness_parity_suite_matches_serial_and_staged_parallel_runs() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::staged_parallel("staged"))
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
    assert!(report.results[0].comparison.mismatches.is_empty());
}

#[test]
fn harness_parity_suite_matches_serial_and_post_commit_parallel_runs() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidate(ExecutionProfile::full_parallel("post-commit"))
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
    assert!(report.results[0].comparison.mismatches.is_empty());
}

#[test]
fn harness_phase8_parity_suite_certifies_all_supported_parallel_lanes() {
    let (fixture, batch, request) = harness_phase8_fixture_batch_request();

    let report = forge_harness::facade::parity_suite(
        RelationalHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::serial("serial"),
    )
    .mutate(batch)
    .candidates([
        ExecutionProfile::staged_parallel("staged"),
        ExecutionProfile::full_parallel("post-commit"),
    ])
    .compare()
    .unwrap();

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
    assert_eq!(report.results[0].baseline_profile, "serial");
    assert_eq!(report.results[0].candidate_profile, "staged");
    assert_eq!(report.results[1].baseline_profile, "serial");
    assert_eq!(report.results[1].candidate_profile, "post-commit");
    assert!(report
        .results
        .iter()
        .all(|result| result.comparison.mismatches.is_empty()));
}

#[test]
fn harness_phase8_fault_injection_soak_does_not_corrupt_following_certification_runs() {
    for _ in 0..3 {
        let (fixture, batch, request) = harness_phase8_fixture_batch_request();
        let fragment = crate::authority::commit::with_test_diff_preparation_fault(
            crate::authority::commit::TestDiffPreparationFault::FragmentCanonicalizationFailure,
            || {
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
        assert!(harness_diagnostic_entries(
            certification_case(&fragment, "staged")
                .diagnostics_summary
                .as_ref()
                .unwrap(),
            "PreparationFailure"
        )
        .iter()
        .any(|entry| {
            harness_diagnostic_field_matches(
                entry,
                "failure_class",
                "fragment_canonicalization_failure",
            )
        }));

        let (fixture, batch, request) = harness_phase8_fixture_batch_request();
        let overlap = crate::authority::commit::with_test_diff_preparation_fault(
            crate::authority::commit::TestDiffPreparationFault::PacketOverlapDetected,
            || {
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
        assert!(harness_diagnostic_entries(
            certification_case(&overlap, "staged")
                .diagnostics_summary
                .as_ref()
                .unwrap(),
            "PreparationFailure"
        )
        .iter()
        .any(|entry| harness_diagnostic_field_matches(
            entry,
            "failure_class",
            "packet_overlap_detected"
        )));

        let (fixture, batch, request) = harness_phase8_fixture_batch_request();
        let clean = forge_harness::facade::parity_suite(
            RelationalHarnessAdapter,
            fixture,
            request,
            ExecutionProfile::serial("serial"),
        )
        .mutate(batch)
        .candidates([
            ExecutionProfile::staged_parallel("staged"),
            ExecutionProfile::full_parallel("post-commit"),
        ])
        .compare()
        .unwrap();

        assert!(clean.matched);
        assert!(clean
            .results
            .iter()
            .all(|result| result.comparison.mismatches.is_empty()));
    }
}
