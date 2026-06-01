use super::*;

#[test]
fn historical_record_inspection_and_transaction_staging_are_read_only() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity(&mut runtime, "staged");
    let commit_id = runtime
        .history()
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
        .inspect_what_happened()
        .inspect_commit(commit_id)
        .expect("commit inspection");
    assert!(commit
        .changed_records
        .contains(&crate::facade::transactions::RecordRef::Entity(created)));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("pending"));
    let staging = txn.inspect_staging();
    assert_eq!(staging.batch_count, 1);
    assert!(staging.touched_records.is_empty());

    let neighbors: NeighborInspectionResult = runtime
        .inspect_what_happened()
        .neighbors(InspectionScope::Current, created);
    assert!(neighbors.outgoing_relation_ids.is_empty());
    assert!(neighbors.incoming_relation_ids.is_empty());
}

#[test]
fn historical_record_inspection_preserves_requested_branch_context() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity(&mut runtime, "branch-base");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("feature branch");

    let inspection = runtime.inspect_what_happened().inspect_historical_record(
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
        .inspect_what_happened()
        .open_historical_view(created.version_id, HistoricalInspectionMode::RetainedOnly);
    assert!(retained_only.view.is_none());
    assert_eq!(
        retained_only.availability,
        InspectionAvailability::UnavailableByRetention
    );

    let reconstructed = runtime.inspect_what_happened().inspect_historical_record(
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

    let inspection = runtime.inspect_what_happened().inspect_historical_record(
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

    let entity_retained = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("main".to_string()),
        entity_version,
        crate::facade::transactions::RecordRef::Entity(source),
        HistoricalInspectionMode::RetainedOnly,
    );
    let relation_retained = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("main".to_string()),
        relation_outcome.version_id,
        crate::facade::transactions::RecordRef::Relation(relation),
        HistoricalInspectionMode::RetainedOnly,
    );
    let entity_reconstructed = runtime.inspect_what_happened().inspect_historical_record(
        &BranchId("main".to_string()),
        entity_version,
        crate::facade::transactions::RecordRef::Entity(source),
        HistoricalInspectionMode::AllowCanonicalReconstruction,
    );
    let relation_reconstructed = runtime.inspect_what_happened().inspect_historical_record(
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
    assert!(relation_reconstructed
        .structural_identity_evidence
        .is_some());
}

#[test]
fn historical_relation_inspection_keeps_direct_commit_history_when_retained_only_blocks_record_truth(
) {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "historical-rel");
    let relation = crate::tests::support::changed_relations(&relation_outcome)[0];
    let _later = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&relation_outcome.snapshot));

    let inspection = runtime.inspect_what_happened().inspect_historical_record(
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
    let relation_outcome =
        create_relation_outcome(&mut runtime, source, target, "reconstructed-rel");
    let relation = crate::tests::support::changed_relations(&relation_outcome)[0];
    let _later = create_entity_outcome(&mut runtime, "later");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&relation_outcome.snapshot));

    let inspection = runtime.inspect_what_happened().inspect_historical_record(
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
    assert_eq!(
        structural.availability,
        InspectionAvailability::Reconstructed
    );
    assert!(structural.structural_fingerprint.is_none());
    assert!(structural.lineage_id.is_none());
    assert!(structural
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingStructuralFingerprint));
    assert!(structural
        .degradations
        .contains(&crate::facade::inspection::InspectionDegradation::MissingLineageIdentity));
}
