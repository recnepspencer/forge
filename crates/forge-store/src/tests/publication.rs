use crate::{
    modes::SimulatedCrashPoint, DurableMutationRequest, ForgeStoreBuilder,
    MaintenanceArtifactFamily, MaintenanceRecoveryDisposition, PublicationClassification,
    PublicationFamily, PublicationState, SnapshotCaptureRequest, SnapshotMaintenanceRecoveryAction,
};

use super::harness::{
    fixtures::runtime::{
        create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
    },
    scenarios::publication::{create_alpha_commit, durable_publication_reports},
};

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

#[test]
fn snapshot_publication_report_classifies_missing_image_and_missing_basis() {
    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store
        .append_canonical_commit(first)
        .expect("first commit should append");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store
        .append_canonical_commit(second)
        .expect("second commit should append");

    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");

    let published = store
        .snapshot_publication_report(snapshot.snapshot_id)
        .expect("published snapshot report should build");
    assert_eq!(
        published.classification(),
        PublicationClassification::RetainTrusted
    );

    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .expect("test should remove snapshot image");
    let missing_image = store
        .snapshot_publication_report(snapshot.snapshot_id)
        .expect("missing-image snapshot report should build");
    assert_eq!(
        missing_image.classification(),
        PublicationClassification::RequireRebuild
    );

    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("fresh store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store
        .append_canonical_commit(first)
        .expect("fresh first commit should append");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store
        .append_canonical_commit(second)
        .expect("fresh second commit should append");
    let second_snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("second snapshot should capture");
    store
        .remove_snapshot_basis_for_test(second_snapshot.snapshot_id)
        .expect("test should remove snapshot basis");
    let missing_basis = store
        .snapshot_publication_report(second_snapshot.snapshot_id)
        .expect("missing-basis snapshot report should build");
    assert_eq!(
        missing_basis.classification(),
        PublicationClassification::RequireQuarantine
    );
}

#[test]
fn snapshot_maintenance_recovery_classifies_invalid_relation_explicitly() {
    let path = super::harness::fixtures::stores::unique_test_sqlite_path(
        "forge-store-snapshot-maintenance-invalid-relation",
    );
    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .expect("store should build");
        let mut runtime = runtime_with_demo_schema();
        let entity_id = create_entity(&mut runtime, "alpha");
        let first = latest_envelope(&runtime);
        store
            .append_canonical_commit(first)
            .expect("first commit should append");
        update_entity_on_branch(&mut runtime, entity_id, "beta", None);
        let second = latest_envelope(&runtime);
        let second_id = second.commit.commit_id;
        store
            .append_canonical_commit(second)
            .expect("second commit should append");
        store
            .capture_snapshot(SnapshotCaptureRequest::new(
                forge_relational::facade::history::BranchId("main".to_string()),
                second_id,
            ))
            .expect("snapshot should capture");
    }

    super::harness::corruption::snapshot::corrupt_first_sqlite_snapshot_image(&path);
    let error = ForgeStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .expect_err("corrupted sqlite snapshot should fail on reopen");
    assert!(matches!(
        error.kind(),
        crate::StoreErrorKind::SnapshotIntegrityFailure
            | crate::StoreErrorKind::SnapshotDigestMismatch
            | crate::StoreErrorKind::BackendIntegrityViolation
    ));

    let mut store = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .expect("in-memory store should build");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store
        .append_canonical_commit(first)
        .expect("first commit should append");
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store
        .append_canonical_commit(second)
        .expect("second commit should append");
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .expect("snapshot should capture");
    store
        .corrupt_snapshot_basis_digest_for_test(snapshot.snapshot_id)
        .expect("test should corrupt snapshot basis digest");

    let report = store
        .snapshot_maintenance_recovery_report(snapshot.snapshot_id)
        .expect("maintenance recovery report should build");
    assert_eq!(
        report.publication_classification(),
        PublicationClassification::RetainTrusted
    );
    assert!(!report.relation_valid());
    assert_eq!(
        report.action(),
        SnapshotMaintenanceRecoveryAction::RequireQuarantine
    );
}

#[test]
fn maintenance_recovery_report_scaffolds_non_snapshot_families_without_faking_presence() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first = latest_envelope(&runtime);
    store.append_canonical_commit(first).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let second = latest_envelope(&runtime);
    let second_id = second.commit.commit_id;
    store.append_canonical_commit(second).unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            forge_relational::facade::history::BranchId("main".to_string()),
            second_id,
        ))
        .unwrap();
    store
        .remove_snapshot_image_for_test(snapshot.snapshot_id)
        .unwrap();

    let report = store.maintenance_recovery_report().unwrap();
    let snapshot_entry = report
        .entries()
        .iter()
        .find(|entry| entry.family() == MaintenanceArtifactFamily::Snapshot)
        .expect("snapshot entry should be present");
    assert_eq!(
        snapshot_entry.disposition(),
        MaintenanceRecoveryDisposition::RequireRebuild
    );

    for family in [
        MaintenanceArtifactFamily::Compaction,
        MaintenanceArtifactFamily::Reclaim,
        MaintenanceArtifactFamily::Capsule,
    ] {
        let entry = report
            .entries()
            .iter()
            .find(|entry| entry.family() == family)
            .expect("scaffolded maintenance family should be present");
        assert_eq!(
            entry.disposition(),
            MaintenanceRecoveryDisposition::NotPresent
        );
    }
    assert_eq!(
        store.counters().interrupted_maintenance_recovery_count,
        0,
        "read-only maintenance reports must not mutate recovery counters"
    );
}
