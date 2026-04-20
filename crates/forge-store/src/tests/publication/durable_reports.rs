use super::*;

#[test]
fn durable_publication_reports_match_across_backend_families() {
    let result = durable_publication_reports();
    let local_report = result.local_report;
    let sqlite_report = result.sqlite_report;

    assert_eq!(
        local_report.classification(),
        PublicationClassification::RetainTrusted
    );
    assert_eq!(
        sqlite_report.classification(),
        PublicationClassification::RetainTrusted
    );
    assert!(local_report.sufficient_for_published_truth());
    assert!(sqlite_report.sufficient_for_published_truth());
    assert_eq!(
        local_report
            .family_states()
            .iter()
            .map(|state| (state.family(), state.state()))
            .collect::<Vec<_>>(),
        sqlite_report
            .family_states()
            .iter()
            .map(|state| (state.family(), state.state()))
            .collect::<Vec<_>>()
    );
    assert!(!serde_json::to_string(&local_report)
        .expect("local publication report should serialize")
        .is_empty());
}

#[test]
fn durable_publication_report_classifies_branch_head_gap_explicitly() {
    let mut durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");
    let acknowledged = durable
        .execute_mutation(DurableMutationRequest::new(
            "create-alpha",
            create_alpha_commit,
        ))
        .expect("durable mutation should acknowledge");
    let (mut store, _runtime) = durable.shutdown();
    store
        .clear_branch_heads_for_test()
        .expect("test should be able to clear branch heads");

    let report = store
        .durable_publication_report(
            acknowledged.durable_mutation_id(),
            Some(acknowledged.persisted().envelope().commit.commit_id),
        )
        .expect("publication report should build");

    assert_eq!(
        report.classification(),
        PublicationClassification::FinishPublication
    );
    let branch_head = report
        .family_states()
        .iter()
        .find(|state| state.family() == PublicationFamily::BranchHeadPublication)
        .expect("branch head family should be present");
    assert_eq!(branch_head.state(), PublicationState::Unpublished);
}

#[test]
fn durable_publication_report_classifies_missing_authoritative_append_explicitly() {
    let mut durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");

    let durable_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            SimulatedCrashPoint::AfterCanonicalResultRecorded,
        )
        .expect("crash simulation should record canonical result");

    let report = durable
        .store()
        .durable_publication_report(durable_mutation_id, None)
        .expect("publication report should build");

    assert_eq!(
        report.classification(),
        PublicationClassification::FinishPublication
    );
    let authoritative_append = report
        .family_states()
        .iter()
        .find(|state| state.family() == PublicationFamily::AuthoritativeCommitAppendUnit)
        .expect("authoritative append family should be present");
    assert_eq!(authoritative_append.state(), PublicationState::Unpublished);
}

#[test]
fn durable_publication_report_classifies_missing_acknowledgment_marker_explicitly() {
    let mut durable = ForgeStoreBuilder::new()
        .in_memory()
        .durable_mode(runtime_with_demo_schema())
        .build()
        .expect("durable store should build");

    let durable_mutation_id = durable
        .execute_mutation_until_crash(
            DurableMutationRequest::new("create-alpha", create_alpha_commit),
            SimulatedCrashPoint::AfterAuthoritativeAppendPublished,
        )
        .expect("crash simulation should publish authoritative truth before acknowledgment");

    let report = durable
        .store()
        .durable_publication_report(durable_mutation_id, None)
        .expect("publication report should build");

    assert_eq!(
        report.classification(),
        PublicationClassification::FinishPublication
    );
    let acknowledgment = report
        .family_states()
        .iter()
        .find(|state| state.family() == PublicationFamily::AcknowledgmentEligibility)
        .expect("acknowledgment family should be present");
    assert_eq!(
        acknowledgment.state(),
        PublicationState::BarrierCompleteButNotPublished
    );
}
