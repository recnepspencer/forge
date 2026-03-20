use super::support::{
    batch_create, changed_entities, create_entity, create_entity_outcome, create_relation,
    runtime_with_test_schema, test_schema_registry, EntityMutationIntent, MutationIntent,
    RecordPayload, RelationalRuntimeApi, TransactionOptions, UpdateEntityIntent,
    VisibilityCachePolicy, WorkerIntentBatch,
};
use crate::facade::history::{BranchId, CommitId};
use crate::facade::inspection::{
    GraphInspectionRequest, HistoricalInspectionMode, InspectionAccessPath,
    InspectionAvailability, InspectionOrigin, InspectionScope, KindInspectionRequest,
    NeighborInspectionResult, RecentCommitInspectionRequest,
    StructuralIdentityComparisonVerdict,
};

#[test]
fn graph_summary_is_scope_explicit_and_canonical() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let _relation = create_relation(&mut runtime, left, right, "rel");

    let summary = runtime.inspection_access().graph_summary(&GraphInspectionRequest {
        scope: InspectionScope::Current,
        partition_scope: None,
        relation_kind_scope: None,
        summary_only: true,
    });
    let kind_summary = runtime.inspection_access().kind_summary(&KindInspectionRequest {
        scope: InspectionScope::Current,
        partition_scope: None,
        kind_id: crate::facade::identity::KindId(1),
        record_class: crate::facade::inspection::InspectionRecordClass::Entity,
    });

    assert_eq!(summary.entity_count, 2);
    assert_eq!(summary.relation_count, 1);
    assert_eq!(kind_summary.count, 2);
}

#[test]
fn structural_identity_comparison_only_uses_fingerprint_truth() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "alpha");

    let comparison = runtime.inspection_access().compare_structural_identity(
        InspectionScope::Current,
        crate::facade::transactions::RecordRef::Entity(entity),
        crate::facade::transactions::RecordRef::Entity(entity),
    );

    assert_eq!(
        comparison.verdict,
        StructuralIdentityComparisonVerdict::IncomparableMissingFingerprint
    );
}

#[test]
fn historical_record_inspection_and_transaction_staging_are_read_only() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity(&mut runtime, "staged");
    let commit_id = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id)
        .unwrap_or(CommitId(0));

    let historical = runtime.inspection_access().inspect_historical_record(
        &crate::facade::history::BranchId("main".to_string()),
        runtime.current_version_id(),
        crate::facade::transactions::RecordRef::Entity(created),
        HistoricalInspectionMode::RetainedOnly,
    );
    assert_eq!(
        historical.record_observation.availability,
        InspectionAvailability::Direct
    );

    let commit = runtime
        .inspection_access()
        .inspect_commit(commit_id)
        .expect("commit inspection");
    assert!(commit.changed_records.contains(&crate::facade::transactions::RecordRef::Entity(
        created
    )));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("pending"));
    let staging = txn.inspect_staging();
    assert_eq!(staging.batch_count, 1);
    assert!(staging.touched_records.is_empty());

    let neighbors: NeighborInspectionResult = runtime.inspection_access().neighbors(
        InspectionScope::Current,
        created,
    );
    assert!(neighbors.outgoing_relation_ids.is_empty());
    assert!(neighbors.incoming_relation_ids.is_empty());
}

#[test]
fn historical_record_inspection_preserves_requested_branch_context() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity(&mut runtime, "branch-base");
    runtime
        .history_authority()
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .expect("feature branch");

    let inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("feature".to_string()),
        runtime.current_version_id(),
        crate::facade::transactions::RecordRef::Entity(created),
        HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(inspection.branch_id, BranchId("feature".to_string()));
    assert_eq!(
        inspection
            .lineage_resolution_context
            .as_ref()
            .map(|resolution| resolution.branch_id.clone()),
        Some(BranchId("feature".to_string()))
    );
    assert_eq!(
        inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("feature".to_string()))
    );
}

#[test]
fn commit_inspection_is_canonical_and_not_story_shaped() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "commit-inspection");
    let commit_id = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id)
        .expect("latest commit");

    let inspection = runtime
        .inspection_access()
        .inspect_commit(commit_id)
        .expect("commit inspection");
    let history = runtime.history_access();
    let envelope = history
        .commit_envelope(commit_id)
        .expect("commit envelope");

    assert_eq!(inspection.origin, InspectionOrigin::CanonicalCommitStorage);
    assert_eq!(inspection.access_path, InspectionAccessPath::CommitIndexRead);
    assert_eq!(inspection.commit.commit_id, commit_id);
    assert_eq!(
        inspection.changed_records,
        vec![crate::facade::transactions::RecordRef::Entity(entity)]
    );
    assert_eq!(inspection.lineage_event_ids, envelope.lineage_event_ids);
    assert_eq!(
        inspection.changed_aspects,
        crate::publication::patch::data::CanonicalAspectSet::new(
            envelope
                .patch
                .records
                .iter()
                .flat_map(|record| record.aspects.iter().cloned())
        )
    );
}

#[test]
fn historical_open_fails_closed_without_retained_state_and_reconstructs_canonically_when_allowed() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "historical");
    let later = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&later.snapshot));

    let retained_only = runtime
        .inspection_access()
        .open_historical_view(created.version_id, HistoricalInspectionMode::RetainedOnly);
    assert!(retained_only.view.is_none());
    assert_eq!(
        retained_only.availability,
        InspectionAvailability::UnavailableByRetention
    );

    let reconstructed = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        created.version_id,
        crate::facade::transactions::RecordRef::Entity(changed_entities(&created)[0]),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );
    assert_eq!(
        reconstructed.record_observation.availability,
        InspectionAvailability::Reconstructed
    );
    assert!(reconstructed.record_observation.value.is_some());
    assert_eq!(
        reconstructed
            .retention_availability_observation
            .as_ref()
            .map(|observation| observation.retained_directly),
        Some(false)
    );
}

#[test]
fn historical_record_inspection_keeps_subresults_separate_when_retained_only_blocks_record_truth() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "historical");
    let entity = changed_entities(&created)[0];
    let _updated = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));

    let inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        created.version_id,
        crate::facade::transactions::RecordRef::Entity(entity),
        HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(
        inspection.record_observation.availability,
        InspectionAvailability::UnavailableByRetention
    );
    assert!(inspection.record_observation.value.is_none());
    assert!(inspection.structural_identity_evidence.is_none());
    assert_eq!(
        inspection
            .retention_availability_observation
            .as_ref()
            .map(|observation| observation.availability),
        Some(InspectionAvailability::UnavailableByRetention)
    );
    assert_eq!(
        inspection
            .lineage_resolution_context
            .as_ref()
            .map(|resolution| resolution.branch_id.clone()),
        Some(BranchId("main".to_string()))
    );
    assert_eq!(
        inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.availability),
        Some(InspectionAvailability::Direct)
    );
    assert_eq!(
        inspection
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.access_path),
        Some(InspectionAccessPath::CommitIndexRead)
    );
}

#[test]
fn transaction_inspection_never_projects_hypothetical_committed_truth() {
    let mut runtime = runtime_with_test_schema();
    let baseline = runtime.inspection_access().graph_summary(&GraphInspectionRequest {
        scope: InspectionScope::Current,
        partition_scope: None,
        relation_kind_scope: None,
        summary_only: true,
    });

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("pending"));
    let staging = txn.inspect_staging();
    let during_staging = runtime.inspection_access().graph_summary(&GraphInspectionRequest {
        scope: InspectionScope::Current,
        partition_scope: None,
        relation_kind_scope: None,
        summary_only: true,
    });

    assert_eq!(staging.batch_count, 1);
    assert_eq!(during_staging.entity_count, baseline.entity_count);
    assert_eq!(during_staging.relation_count, baseline.relation_count);
}

#[test]
fn historical_inspection_stays_branch_local_under_divergence_and_reclaim_pressure() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .visibility_cache_policy(VisibilityCachePolicy {
            enabled: true,
            protect_branch_heads: false,
            protect_replay_retained: false,
            protect_active_snapshots: false,
            recent_version_window: 0,
        })
        .build();
    let created = create_entity_outcome(&mut runtime, "base");
    let entity = changed_entities(&created)[0];
    runtime
        .history_authority()
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .expect("feature branch");

    let main_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(WorkerIntentBatch::new("main-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(serde_json::json!({"name":"main"})),
            }),
        )));
        txn.commit().expect("main update")
    };
    let feature_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        txn.push_batch(WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(serde_json::json!({"name":"feature"})),
            }),
        )));
        txn.commit().expect("feature update")
    };

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&main_update.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&feature_update.snapshot));

    let retained_only = runtime
        .inspection_access()
        .open_historical_view(created.version_id, HistoricalInspectionMode::RetainedOnly);
    assert!(retained_only.view.is_none());
    assert_eq!(
        retained_only.availability,
        InspectionAvailability::UnavailableByRetention
    );

    let main = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        created.version_id,
        crate::facade::transactions::RecordRef::Entity(entity),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );
    let feature = runtime.inspection_access().inspect_historical_record(
        &BranchId("feature".to_string()),
        created.version_id,
        crate::facade::transactions::RecordRef::Entity(entity),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );

    assert_eq!(
        main.record_observation.availability,
        InspectionAvailability::Reconstructed
    );
    assert_eq!(
        feature.record_observation.availability,
        InspectionAvailability::Reconstructed
    );
    assert_eq!(
        main.aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("main".to_string()))
    );
    assert_eq!(
        feature
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("feature".to_string()))
    );
    assert_ne!(
        main.aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.entries.len()),
        feature
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.query_result.entries.len())
    );
}

#[test]
fn recent_commit_inspection_and_branch_head_reads_stay_branch_local() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "base");
    let entity = changed_entities(&base)[0];
    runtime
        .history_authority()
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .expect("feature branch");

    let main_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(WorkerIntentBatch::new("main-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(serde_json::json!({"name":"main"})),
            }),
        )));
        txn.commit().expect("main update")
    };
    let feature_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        txn.push_batch(WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(serde_json::json!({"name":"feature"})),
            }),
        )));
        txn.commit().expect("feature update")
    };

    let feature_head = runtime
        .inspection_access()
        .inspect_branch_head(&BranchId("feature".to_string()))
        .expect("feature branch head");
    let feature_window = runtime
        .inspection_access()
        .inspect_recent_commits(&RecentCommitInspectionRequest {
            branch_id: Some(BranchId("feature".to_string())),
            limit: 8,
        });
    let main_window = runtime
        .inspection_access()
        .inspect_recent_commits(&RecentCommitInspectionRequest {
            branch_id: Some(BranchId("main".to_string())),
            limit: 8,
        });

    assert_eq!(feature_head.commit.branch_id, BranchId("feature".to_string()));
    assert_eq!(feature_head.commit.commit_id, feature_update.commit.commit_id);
    assert_eq!(
        feature_window.branch_head.as_ref().map(|head| head.commit_id),
        Some(feature_update.commit.commit_id)
    );
    assert!(feature_window
        .commits
        .iter()
        .all(|inspection| inspection.commit.branch_id == BranchId("feature".to_string())));
    assert!(main_window
        .commits
        .iter()
        .all(|inspection| inspection.commit.branch_id == BranchId("main".to_string())));
    assert!(main_window
        .commits
        .iter()
        .any(|inspection| inspection.commit.commit_id == main_update.commit.commit_id));
    assert!(!feature_window
        .commits
        .iter()
        .any(|inspection| inspection.commit.commit_id == main_update.commit.commit_id));
}
