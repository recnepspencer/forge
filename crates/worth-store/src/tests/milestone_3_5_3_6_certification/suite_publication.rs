use super::*;

fn milestone_3_5_suite() -> CertificationSuite<String, String> {
    let reports = durable_publication_reports();
    let failures = milestone_3_5_failures();
    let normalize = |report: &crate::PublicationWriteOutcome| {
        serde_json::to_string(
            &report
                .family_states()
                .iter()
                .map(|state| (state.family(), state.state(), state.source_admitted()))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };

    let path = unique_test_store_path("worth-store-m35-gap-certification");
    let mut durable = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .durable_mode(runtime_with_demo_schema())
        .build()
        .unwrap();
    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .unwrap();
    drop(durable);
    force_branch_head_gap(&path);
    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let gap_report = reopened
        .durable_publication_report(
            acknowledged.durable_mutation_id(),
            Some(acknowledged.persisted().envelope().commit.commit_id),
        )
        .unwrap();

    CertificationSuite::new(DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "publication_family_equivalence",
            vec![
                LaneResult::new("local_file", normalize(&reports.local_report)),
                LaneResult::new("sqlite", normalize(&reports.sqlite_report)),
            ],
            &[AssertionClass::Equality],
        ))
        .with_rejection_row(RejectionRow::new(
            "publication_gap_classification",
            vec![LaneResult::new(
                "branch_head_gap",
                format!("{:?}", gap_report.classification()),
            )],
            &[AssertionClass::TypedFailure],
        ))
        .with_rejection_row(RejectionRow::new(
            "typed_media_failures",
            vec![LaneResult::new(
                "failure_kinds",
                serde_json::to_string(
                    &failures
                        .iter()
                        .map(|failure| failure.kind.clone())
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            )],
            &[AssertionClass::TypedFailure],
        ))
}

#[test]
fn milestone_3_5_certification_harness_scaffolds_publication_suite() {
    let suite = milestone_3_5_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
    assert_rejection_payloads_present(&suite.rejection_rows()[0]);
    let completeness = evaluate_completeness(&suite, &DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}
