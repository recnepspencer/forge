use crate::tests::support::*;

// CONTRACT: historical_lineage_resolution
// LANES: success, adversarial, recovery

#[test]
fn historical_lineage_resolution_follows_replace_events() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw("replacement".to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name":"replacement"})),
                },
            }),
        )),
    );
    let outcome = txn.commit().unwrap();
    let resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(&BranchId("main".to_string()), start_lineage);

    assert_eq!(resolution.start, start_lineage);
    assert_eq!(resolution.traversed_event_ids.len(), 1);
    assert_eq!(resolution.resolved.len(), 1);
    assert_ne!(resolution.resolved[0], start_lineage);
    assert_eq!(
        runtime
            .lineage_access()
            .graph(&BranchId("main".to_string()))
            .events
            .iter()
            .filter(|event| event.commit.commit_id == outcome.commit.commit_id)
            .count(),
        2
    );
}

#[test]
fn historical_lineage_resolution_is_branch_local_under_divergent_replacements() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let main_target = create_entity_outcome(&mut runtime, "main-target");
    let feature_target = create_entity_outcome(&mut runtime, "feature-target");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;
    let main_target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&main_target)[0])
        .unwrap()
        .lineage_id;
    let feature_target_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&feature_target)[0])
        .unwrap()
        .lineage_id;
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    let main_candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![start_lineage],
        vec![main_target_lineage],
        "main-branch-resolution",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(main_candidate.candidate_id, main_target.commit.clone())
        .unwrap();
    let feature_candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("feature".to_string()),
        vec![start_lineage],
        vec![feature_target_lineage],
        "feature-branch-resolution",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(
            feature_candidate.candidate_id,
            feature_target.commit.clone(),
        )
        .unwrap();

    let main_resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(&BranchId("main".to_string()), start_lineage);
    let feature_resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(&BranchId("feature".to_string()), start_lineage);

    assert_ne!(main_resolution.resolved, feature_resolution.resolved);
}

#[test]
fn lineage_aspect_history_keeps_origin_events_and_marks_resolution_context() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw("replacement".to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name":"replacement"})),
                },
            }),
        )),
    );
    let replacement = txn.commit().unwrap();
    let history = runtime
        .lineage_access()
        .entity_aspect_history(&BranchId("main".to_string()), start_lineage, None)
        .unwrap();
    let traced = runtime.lineage_access().entity_aspect_history_with_trace(
        &BranchId("main".to_string()),
        start_lineage,
        None,
    );
    let artifact = traced.trace.diagnostic_artifact();
    let digest = traced.lineage_aspect_resolution_digest();

    assert_eq!(history.start_lineage_id, start_lineage);
    assert_eq!(
        traced.trace.requested_target,
        HistoryAspectQueryTarget::Lineage(start_lineage)
    );
    assert_eq!(traced.trace.returned_entries, 3);
    assert_eq!(traced.trace.traversed_lineage_events, 1);
    assert_eq!(
        traced.trace.searched_lineage_event_span,
        Some(crate::facade::history::AspectHistoryLineageEventSpan {
            first_event_id: history.traversed_event_ids[0],
            last_event_id: history.traversed_event_ids[0],
        })
    );
    assert_eq!(artifact.scope, DiagnosticsScope::Lineage);
    assert_eq!(artifact.kind, DiagnosticsArtifactKind::DetailedTrace);
    assert_eq!(
        artifact.entries[0].code,
        DiagnosticCode::LineageAspectHistoryResolved
    );
    assert_eq!(
        digest.requested_target,
        HistoryAspectQueryTarget::Lineage(start_lineage)
    );
    assert_eq!(digest.entry_count, 3);
    assert_eq!(digest.traversed_lineage_events, 1);
    assert_eq!(
        digest.resolved_lineage_chain_len,
        history.resolved_lineage_chain.len()
    );
    assert_eq!(traced.history.as_ref(), Some(&history));
    assert_eq!(history.traversed_event_ids.len(), 1);
    assert_eq!(history.entries.len(), 3);
    assert_lineage_history_origin_invariants(&history.entries, start_lineage);
    assert_eq!(
        history.entries[0].origin.commit_id,
        created.commit.commit_id
    );
    assert_eq!(
        history.entries[1].origin.commit_id,
        replacement.commit.commit_id
    );
    assert_eq!(
        history.entries[2].origin.commit_id,
        replacement.commit.commit_id
    );
}
