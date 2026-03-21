use super::support::{
    batch_create, changed_entities, create_entity, create_entity_outcome,
    capture_inspection_truth_bundle, connectivity_request, current_graph_request,
    create_relation, create_relation_outcome, merge_commit_from_branches,
    persisted_runtime_with_test_schema, read_entity_name, runtime_with_test_schema,
    recent_commit_window, reconstructed_record_inspection, retained_record_inspection,
    snapshot_graph_request, test_schema_registry, version_graph_request,
    EntityMutationIntent, MutationIntent, RecordPayload, RelationalRuntimeApi,
    TransactionOptions, UpdateEntityIntent, VisibilityCachePolicy, WorkerIntentBatch,
};
use crate::facade::history::{BranchId, CommitId};
use crate::facade::identity::{LineageId, StructuralFingerprint};
use crate::facade::inspection::{
    HistoricalInspectionMode, InspectionAccessPath, InspectionAvailability, InspectionOrigin,
    InspectionScope, KindInspectionRequest,
    NeighborInspectionResult, RecentCommitInspectionRequest, StructuralIdentityQueryRequest,
    StructuralIdentityComparisonVerdict,
};
use crate::facade::symbols::Symbol;

#[test]
fn graph_summary_is_scope_explicit_and_canonical() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let _relation = create_relation(&mut runtime, left, right, "rel");

    let summary = runtime
        .inspection_access()
        .graph_summary(&current_graph_request(None, None, true));
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
fn current_graph_surfaces_match_version_and_snapshot_scopes_for_same_truth() {
    let mut runtime = runtime_with_test_schema();
    let left_a = crate::tests::support::create_entity_in_partition(&mut runtime, "left-a", crate::facade::identity::PartitionId(7));
    let left_b = crate::tests::support::create_entity_in_partition(&mut runtime, "left-b", crate::facade::identity::PartitionId(7));
    let right = crate::tests::support::create_entity_in_partition(&mut runtime, "right", crate::facade::identity::PartitionId(11));
    let left_relation = crate::tests::support::create_relation_in_partition(
        &mut runtime,
        left_a,
        left_b,
        "left-rel",
        crate::facade::identity::PartitionId(7),
    );
    let _cross_relation = create_relation(&mut runtime, left_b, right, "cross-rel");
    let snapshot = runtime.visibility_authority().snapshot();
    let version_id = runtime.current_version_id();

    let current_graph = runtime.inspection_access().graph_summary(&current_graph_request(
        Some(vec![crate::facade::identity::PartitionId(7)]),
        Some(vec![crate::facade::identity::KindId(2)]),
        true,
    ));
    let version_graph = runtime.inspection_access().graph_summary(&version_graph_request(
        version_id,
        Some(vec![crate::facade::identity::PartitionId(7)]),
        Some(vec![crate::facade::identity::KindId(2)]),
        true,
    ));
    let snapshot_graph = runtime.inspection_access().graph_summary(&snapshot_graph_request(
        InspectionScope::Snapshot(snapshot.clone()),
        Some(vec![crate::facade::identity::PartitionId(7)]),
        Some(vec![crate::facade::identity::KindId(2)]),
        true,
    ));
    let current_connectivity = runtime
        .inspection_access()
        .connectivity_summary(&connectivity_request(
            InspectionScope::Current,
            Some(vec![crate::facade::identity::PartitionId(7)]),
            Some(vec![crate::facade::identity::KindId(2)]),
            true,
        ));
    let version_connectivity = runtime
        .inspection_access()
        .connectivity_summary(&connectivity_request(
            InspectionScope::Version(version_id),
            Some(vec![crate::facade::identity::PartitionId(7)]),
            Some(vec![crate::facade::identity::KindId(2)]),
            true,
        ));
    let snapshot_connectivity = runtime
        .inspection_access()
        .connectivity_summary(&connectivity_request(
            InspectionScope::Snapshot(snapshot),
            Some(vec![crate::facade::identity::PartitionId(7)]),
            Some(vec![crate::facade::identity::KindId(2)]),
            true,
        ));
    let neighbors_current = runtime
        .inspection_access()
        .neighbors(InspectionScope::Current, left_a);
    let neighbors_version = runtime
        .inspection_access()
        .neighbors(InspectionScope::Version(version_id), left_a);

    assert_eq!(current_graph.entity_count, version_graph.entity_count);
    assert_eq!(current_graph.entity_count, snapshot_graph.entity_count);
    assert_eq!(current_graph.relation_count, version_graph.relation_count);
    assert_eq!(current_graph.relation_count, snapshot_graph.relation_count);
    assert_eq!(current_graph.entity_kinds, version_graph.entity_kinds);
    assert_eq!(current_graph.entity_kinds, snapshot_graph.entity_kinds);
    assert_eq!(current_graph.relation_kinds, version_graph.relation_kinds);
    assert_eq!(current_graph.relation_kinds, snapshot_graph.relation_kinds);
    assert_eq!(current_connectivity.component_count, version_connectivity.component_count);
    assert_eq!(current_connectivity.component_count, snapshot_connectivity.component_count);
    assert_eq!(
        current_connectivity.largest_component_size,
        version_connectivity.largest_component_size
    );
    assert_eq!(
        current_connectivity.largest_component_size,
        snapshot_connectivity.largest_component_size
    );
    assert_eq!(current_connectivity.components, version_connectivity.components);
    assert_eq!(current_connectivity.components, snapshot_connectivity.components);
    assert_eq!(neighbors_current.outgoing_relation_ids, vec![left_relation]);
    assert_eq!(neighbors_current.outgoing_relation_ids, neighbors_version.outgoing_relation_ids);
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
fn structural_identity_evidence_exposes_declared_fingerprint_and_lineage_for_entities_only() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "alpha");
    assert!(runtime.set_entity_structural_identity_for_test(
        entity,
        Some(StructuralFingerprint::new(Symbol(11), 101)),
        Some(LineageId(77)),
    ));

    let entity_evidence = runtime
        .inspection_access()
        .structural_identity(
            InspectionScope::Current,
            crate::facade::transactions::RecordRef::Entity(entity),
        )
        .expect("entity evidence");

    assert_eq!(
        entity_evidence.structural_fingerprint,
        Some(StructuralFingerprint::new(Symbol(11), 101))
    );
    assert_eq!(entity_evidence.lineage_id, Some(LineageId(77)));
    assert!(entity_evidence.degradations.is_empty());

    let relation = create_relation(&mut runtime, entity, entity, "self");
    let relation_evidence = runtime
        .inspection_access()
        .structural_identity(
            InspectionScope::Current,
            crate::facade::transactions::RecordRef::Relation(relation),
        )
        .expect("relation evidence");

    assert!(relation_evidence.structural_fingerprint.is_none());
    assert!(relation_evidence.lineage_id.is_none());
    assert!(relation_evidence
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingStructuralFingerprint));
    assert!(relation_evidence
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingLineageIdentity));
}

#[test]
fn structural_identity_comparison_distinguishes_equal_mismatch_and_family_mismatch() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let other_family = create_entity(&mut runtime, "other-family");

    assert!(runtime.set_entity_structural_identity_for_test(
        left,
        Some(StructuralFingerprint::new(Symbol(21), 500)),
        Some(LineageId(1)),
    ));
    assert!(runtime.set_entity_structural_identity_for_test(
        right,
        Some(StructuralFingerprint::new(Symbol(21), 500)),
        Some(LineageId(2)),
    ));
    assert!(runtime.set_entity_structural_identity_for_test(
        other_family,
        Some(StructuralFingerprint::new(Symbol(22), 500)),
        Some(LineageId(3)),
    ));

    let equal = runtime.inspection_access().compare_structural_identity(
        InspectionScope::Current,
        crate::facade::transactions::RecordRef::Entity(left),
        crate::facade::transactions::RecordRef::Entity(right),
    );
    assert_eq!(
        equal.verdict,
        StructuralIdentityComparisonVerdict::EqualByFingerprint
    );

    assert!(runtime.set_entity_structural_identity_for_test(
        right,
        Some(StructuralFingerprint::new(Symbol(21), 999)),
        Some(LineageId(2)),
    ));
    let mismatch = runtime.inspection_access().compare_structural_identity(
        InspectionScope::Current,
        crate::facade::transactions::RecordRef::Entity(left),
        crate::facade::transactions::RecordRef::Entity(right),
    );
    assert_eq!(
        mismatch.verdict,
        StructuralIdentityComparisonVerdict::NotEqualByFingerprint
    );

    let family_mismatch = runtime.inspection_access().compare_structural_identity(
        InspectionScope::Current,
        crate::facade::transactions::RecordRef::Entity(left),
        crate::facade::transactions::RecordRef::Entity(other_family),
    );
    assert_eq!(
        family_mismatch.verdict,
        StructuralIdentityComparisonVerdict::IncomparableFingerprintFamilyMismatch
    );
}

#[test]
fn structural_identity_query_is_family_scoped_and_entity_only() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let ignored = create_entity(&mut runtime, "ignored");
    let _relation = create_relation(&mut runtime, left, right, "rel");

    assert!(runtime.set_entity_structural_identity_for_test(
        left,
        Some(StructuralFingerprint::new(Symbol(31), 1)),
        Some(LineageId(10)),
    ));
    assert!(runtime.set_entity_structural_identity_for_test(
        right,
        Some(StructuralFingerprint::new(Symbol(31), 2)),
        Some(LineageId(11)),
    ));
    assert!(runtime.set_entity_structural_identity_for_test(
        ignored,
        Some(StructuralFingerprint::new(Symbol(32), 3)),
        Some(LineageId(12)),
    ));

    let queried = runtime
        .inspection_access()
        .query_structural_identity(&StructuralIdentityQueryRequest {
            scope: InspectionScope::Current,
            partition_scope: None,
            fingerprint_family: Symbol(31),
        });

    assert_eq!(queried.len(), 2);
    assert!(queried
        .iter()
        .all(|evidence| evidence.record_class == crate::facade::inspection::InspectionRecordClass::Entity));
    assert!(queried.iter().all(|evidence| {
        evidence
            .structural_fingerprint
            .is_some_and(|fingerprint| fingerprint.family == Symbol(31))
    }));
}

#[test]
fn structural_identity_historical_scope_does_not_leak_reused_slot_sidecars() {
    let mut runtime = runtime_with_test_schema();
    let original = create_entity_outcome(&mut runtime, "original");
    let original_entity = changed_entities(&original)[0];
    assert!(runtime.set_entity_structural_identity_for_test(
        original_entity,
        Some(StructuralFingerprint::new(Symbol(41), 111)),
        Some(LineageId(41)),
    ));
    let replacement_entity = runtime
        .simulate_entity_slot_reuse_for_test(
            original_entity,
            RecordPayload::StructuredJson(serde_json::json!({"name":"replacement"})),
            Some(StructuralFingerprint::new(Symbol(42), 222)),
            Some(LineageId(42)),
        )
        .expect("replacement entity");
    assert_eq!(original_entity.local_slot, replacement_entity.local_slot);

    let historical = runtime
        .inspection_access()
        .structural_identity(
            InspectionScope::Version(original.version_id),
            crate::facade::transactions::RecordRef::Entity(original_entity),
        )
        .expect("historical structural identity");
    let current = runtime
        .inspection_access()
        .structural_identity(
            InspectionScope::Current,
            crate::facade::transactions::RecordRef::Entity(replacement_entity),
        )
        .expect("current replacement structural identity");

    assert!(historical.structural_fingerprint.is_none());
    assert!(historical.lineage_id.is_none());
    assert!(historical
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingStructuralFingerprint));
    assert!(historical
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingLineageIdentity));
    assert_eq!(
        current.structural_fingerprint,
        Some(StructuralFingerprint::new(Symbol(42), 222))
    );
    assert_eq!(current.lineage_id, Some(LineageId(42)));
}

#[test]
fn structural_identity_recovery_preserves_current_evidence_and_queries() {
    let mut runtime = persisted_runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    assert!(runtime.set_entity_structural_identity_for_test(
        left,
        Some(StructuralFingerprint::new(Symbol(51), 1001)),
        Some(LineageId(501)),
    ));
    assert!(runtime.set_entity_structural_identity_for_test(
        right,
        Some(StructuralFingerprint::new(Symbol(51), 1002)),
        Some(LineageId(502)),
    ));
    runtime.durability_authority().checkpoint().unwrap();
    let expected_left = runtime
        .inspection_access()
        .structural_identity(
            InspectionScope::Current,
            crate::facade::transactions::RecordRef::Entity(left),
        )
        .expect("expected left evidence");
    let expected_query = runtime
        .inspection_access()
        .query_structural_identity(&StructuralIdentityQueryRequest {
            scope: InspectionScope::Current,
            partition_scope: None,
            fingerprint_family: Symbol(51),
        });

    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let actual_left = recovered
        .inspection_access()
        .structural_identity(
            InspectionScope::Current,
            crate::facade::transactions::RecordRef::Entity(left),
        )
        .expect("actual left evidence");
    let actual_query = recovered
        .inspection_access()
        .query_structural_identity(&StructuralIdentityQueryRequest {
            scope: InspectionScope::Current,
            partition_scope: None,
            fingerprint_family: Symbol(51),
        });

    assert_eq!(expected_left, actual_left);
    assert_eq!(expected_query, actual_query);
}

#[test]
fn inspection_truth_bundle_recovery_parity_holds_for_current_and_historical_surfaces() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "bundle");
    let entity = changed_entities(&created)[0];
    let _relation = create_relation(&mut runtime, entity, entity, "self");
    runtime.durability_authority().checkpoint().unwrap();

    let expected = capture_inspection_truth_bundle(
        &runtime,
        &BranchId("main".to_string()),
        entity,
        created.version_id,
    );
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let actual = capture_inspection_truth_bundle(
        &recovered,
        &BranchId("main".to_string()),
        entity,
        created.version_id,
    );

    assert_eq!(expected, actual);
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

    let historical = retained_record_inspection(
        &runtime,
        &crate::facade::history::BranchId("main".to_string()),
        runtime.current_version_id(),
        crate::facade::transactions::RecordRef::Entity(created),
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
fn merge_commit_inspection_stays_envelope_projected() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "merge-base");
    let entity = changed_entities(&created)[0];
    runtime
        .history_authority()
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .expect("feature branch");

    let mut feature_txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("feature".to_string())),
        ..TransactionOptions::default()
    });
    feature_txn.push_batch(
        WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(serde_json::json!({"name":"feature"})),
            }),
        )),
    );
    feature_txn.commit().expect("feature update");

    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let merge_commit_id = merge.commit.commit_id;
    let history = runtime.history_access();
    let envelope = history
        .commit_envelope(merge_commit_id)
        .expect("merge commit envelope");
    assert_eq!(envelope.merge_parent_branches, vec![BranchId("feature".to_string())]);

    let inspection = runtime
        .inspection_access()
        .inspect_commit(merge_commit_id)
        .expect("merge commit inspection");

    assert_eq!(inspection.commit, envelope.commit);
    assert_eq!(
        inspection.changed_records,
        envelope
            .patch
            .records
            .iter()
            .map(|record| record.target.clone())
            .collect::<Vec<_>>()
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
fn historical_inspection_matrix_keeps_entity_and_relation_subresults_honest_across_modes() {
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
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "matrix-rel");
    let relation = crate::tests::support::changed_relations(&relation_outcome)[0];
    let entity_version = runtime.current_version_id();
    let _later = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&relation_outcome.snapshot));

    let entity_retained = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        entity_version,
        crate::facade::transactions::RecordRef::Entity(source),
        HistoricalInspectionMode::RetainedOnly,
    );
    let relation_retained = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        relation_outcome.version_id,
        crate::facade::transactions::RecordRef::Relation(relation),
        HistoricalInspectionMode::RetainedOnly,
    );
    let entity_reconstructed = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        entity_version,
        crate::facade::transactions::RecordRef::Entity(source),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );
    let relation_reconstructed = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        relation_outcome.version_id,
        crate::facade::transactions::RecordRef::Relation(relation),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );

    assert_eq!(
        entity_retained.record_observation.availability,
        InspectionAvailability::UnavailableByRetention
    );
    assert!(entity_retained.lineage_resolution_context.is_some());
    assert_eq!(
        entity_retained
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.availability),
        Some(InspectionAvailability::Direct)
    );
    assert!(entity_retained.structural_identity_evidence.is_none());

    assert_eq!(
        relation_retained.record_observation.availability,
        InspectionAvailability::UnavailableByRetention
    );
    assert!(relation_retained.lineage_resolution_context.is_none());
    assert_eq!(
        relation_retained
            .aspect_history_observation
            .as_ref()
            .map(|observation| observation.availability),
        Some(InspectionAvailability::Direct)
    );
    assert!(relation_retained.structural_identity_evidence.is_none());

    assert_eq!(
        entity_reconstructed.record_observation.availability,
        InspectionAvailability::Reconstructed
    );
    assert!(entity_reconstructed.lineage_resolution_context.is_some());
    assert!(entity_reconstructed.structural_identity_evidence.is_some());

    assert_eq!(
        relation_reconstructed.record_observation.availability,
        InspectionAvailability::Reconstructed
    );
    assert!(relation_reconstructed.lineage_resolution_context.is_none());
    assert!(relation_reconstructed.structural_identity_evidence.is_some());
}

#[test]
fn historical_relation_inspection_keeps_direct_commit_history_when_retained_only_blocks_record_truth() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "historical-rel");
    let relation = crate::tests::support::changed_relations(&relation_outcome)[0];
    let _later = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&relation_outcome.snapshot));

    let inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        relation_outcome.version_id,
        crate::facade::transactions::RecordRef::Relation(relation),
        HistoricalInspectionMode::RetainedOnly,
    );

    assert_eq!(
        inspection.record_observation.availability,
        InspectionAvailability::UnavailableByRetention
    );
    assert!(inspection.record_observation.value.is_none());
    assert!(inspection.lineage_resolution_context.is_none());
    assert!(inspection.structural_identity_evidence.is_none());
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
            .map(|observation| observation.query_result.trace.branch_id.clone()),
        Some(BranchId("main".to_string()))
    );
}

#[test]
fn historical_relation_inspection_reconstructs_record_truth_without_inventing_lineage() {
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
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "reconstructed-rel");
    let relation = crate::tests::support::changed_relations(&relation_outcome)[0];
    let _later = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&relation_outcome.snapshot));

    let inspection = runtime.inspection_access().inspect_historical_record(
        &BranchId("main".to_string()),
        relation_outcome.version_id,
        crate::facade::transactions::RecordRef::Relation(relation),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );

    assert_eq!(
        inspection.record_observation.availability,
        InspectionAvailability::Reconstructed
    );
    match inspection.record_observation.value {
        Some(crate::facade::inspection::HistoricalRecordValue::Relation(ref record)) => {
            assert_eq!(record.relation_id, relation);
            assert_eq!(record.source, source);
            assert_eq!(record.target, target);
        }
        _ => panic!("expected reconstructed relation record"),
    }
    assert!(inspection.lineage_resolution_context.is_none());
    let structural = inspection
        .structural_identity_evidence
        .as_ref()
        .expect("relation structural evidence");
    assert_eq!(structural.availability, InspectionAvailability::Reconstructed);
    assert!(structural.structural_fingerprint.is_none());
    assert!(structural.lineage_id.is_none());
    assert!(structural
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingStructuralFingerprint));
    assert!(structural
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingLineageIdentity));
}

#[test]
fn transaction_inspection_never_projects_hypothetical_committed_truth() {
    let mut runtime = runtime_with_test_schema();
    let baseline = runtime
        .inspection_access()
        .graph_summary(&current_graph_request(None, None, true));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("pending"));
    let staging = txn.inspect_staging();
    let during_staging = runtime
        .inspection_access()
        .graph_summary(&current_graph_request(None, None, true));

    assert_eq!(staging.batch_count, 1);
    assert_eq!(during_staging.entity_count, baseline.entity_count);
    assert_eq!(during_staging.relation_count, baseline.relation_count);
}

#[test]
fn transaction_inspection_savepoint_rollback_scrubs_abandoned_work_and_commit_truth() {
    let mut runtime = runtime_with_test_schema();
    let existing = create_entity(&mut runtime, "existing");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("kept"));
    let savepoint = txn.create_savepoint();
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: existing,
                payload: RecordPayload::StructuredJson(serde_json::json!({"name":"abandoned"})),
            }),
        )),
    );
    txn.push_batch(batch_create("abandoned"));

    let before_rollback = txn.inspect_staging();
    assert_eq!(before_rollback.batch_count, 3);
    assert_eq!(before_rollback.savepoints.len(), 1);
    assert!(before_rollback
        .touched_records
        .contains(&crate::facade::transactions::RecordRef::Entity(existing)));
    assert_eq!(before_rollback.intent_counts.create_count, 2);
    assert_eq!(before_rollback.intent_counts.entity_mutation_count, 1);

    txn.rollback_to_savepoint(savepoint)
        .expect("rollback to savepoint");

    let after_rollback = txn.inspect_staging();
    assert_eq!(after_rollback.batch_count, 1);
    assert!(after_rollback.savepoints.is_empty());
    assert!(after_rollback.touched_records.is_empty());
    assert_eq!(after_rollback.intent_counts.create_count, 1);
    assert_eq!(after_rollback.intent_counts.entity_mutation_count, 0);

    let committed = txn.commit().expect("commit surviving staged work");
    let committed_entity = changed_entities(&committed)[0];
    let commit_inspection = runtime
        .inspection_access()
        .inspect_commit(committed.commit.commit_id)
        .expect("commit inspection");

    assert_eq!(
        commit_inspection.changed_records,
        vec![crate::facade::transactions::RecordRef::Entity(committed_entity)]
    );
    assert!(!commit_inspection
        .changed_records
        .contains(&crate::facade::transactions::RecordRef::Entity(existing)));
}

#[test]
fn transaction_inspection_marks_lineage_affecting_intents_without_previewing_commit_or_history() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "replace-target");
    let baseline_latest_commit = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id);
    let baseline_window = runtime
        .inspection_access()
        .inspect_recent_commits(&RecentCommitInspectionRequest {
            branch_id: Some(BranchId("main".to_string())),
            limit: 8,
        });

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace")
            .push(MutationIntent::Entity(EntityMutationIntent::Replace(
                crate::transactions::data::ReplaceEntityIntent {
                    entity_id: entity,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: crate::facade::identity::PartitionId::main(),
                        kind_id: crate::facade::identity::KindId(1),
                        client_key: crate::symbols::data::InternedString::Raw(
                            "replacement".to_string(),
                        ),
                        payload: RecordPayload::StructuredJson(
                            serde_json::json!({"name":"replacement"}),
                        ),
                    },
                },
            ))),
    );

    let staging = txn.inspect_staging();
    assert!(staging.contains_lineage_affecting_intents);
    assert_eq!(staging.intent_counts.entity_mutation_count, 1);
    assert_eq!(
        staging.touched_records,
        vec![crate::facade::transactions::RecordRef::Entity(entity)]
    );

    let latest_commit_during_staging = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id);
    let window_during_staging = runtime
        .inspection_access()
        .inspect_recent_commits(&RecentCommitInspectionRequest {
            branch_id: Some(BranchId("main".to_string())),
            limit: 8,
        });
    let current = retained_record_inspection(
        &runtime,
        &BranchId("main".to_string()),
        runtime.current_version_id(),
        crate::facade::transactions::RecordRef::Entity(entity),
    );

    assert_eq!(latest_commit_during_staging, baseline_latest_commit);
    assert_eq!(window_during_staging, baseline_window);
    let current_name = match current.record_observation.value {
        Some(crate::facade::inspection::HistoricalRecordValue::Entity(ref record)) => {
            read_entity_name(record)
        }
        _ => None,
    };
    assert_eq!(current_name, Some("replace-target"));
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

    let main = reconstructed_record_inspection(
        &runtime,
        &BranchId("main".to_string()),
        created.version_id,
        crate::facade::transactions::RecordRef::Entity(entity),
    );
    let feature = reconstructed_record_inspection(
        &runtime,
        &BranchId("feature".to_string()),
        created.version_id,
        crate::facade::transactions::RecordRef::Entity(entity),
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
    let feature_window = recent_commit_window(&runtime, &BranchId("feature".to_string()), 8);
    let main_window = recent_commit_window(&runtime, &BranchId("main".to_string()), 8);

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
