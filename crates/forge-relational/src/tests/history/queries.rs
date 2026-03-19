use crate::facade::history::{BranchCreateErrorClass, MergeConflictRecord};
use crate::facade::transactions::CommitPhase;
use crate::tests::support::*;

#[test]
fn branch_creation_and_branch_targeted_commits_build_a_version_graph() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let main_second =
        create_entity_outcome_on_branch(&mut runtime, "main-b", BranchId("main".to_string()));
    let graph = runtime.history_access().version_graph();

    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap(),
        &feature_outcome.commit
    );
    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("main".to_string()))
            .unwrap(),
        &main_second.commit
    );
    assert_eq!(
        feature_outcome.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(
        main_second.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(graph.branches.len(), 2);
    assert_eq!(graph.commits.len(), 3);
}

#[test]
fn merge_commit_uses_deterministic_parent_order_and_advances_target_branch() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let merge_outcome = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    assert_eq!(
        merge_outcome.commit.parents,
        vec![
            main_outcome.commit.commit_id,
            feature_outcome.commit.commit_id
        ]
    );
    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&merge_outcome.commit)
    );
    assert_eq!(
        runtime
            .history_access()
            .branch_head(&BranchId("feature".to_string())),
        Some(&feature_outcome.commit)
    );
    let replay = runtime.replay_access();
    let envelope = replay
        .canonical_commit_envelope(merge_outcome.commit.commit_id)
        .unwrap();
    assert_eq!(
        envelope.merge_parent_branches,
        vec![BranchId("feature".to_string())]
    );
    assert_eq!(
        envelope.merge_base_commits,
        vec![main_outcome.commit.commit_id]
    );
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeCommitPublished));
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeBaseResolved));
}

#[test]
fn merge_commit_requires_existing_parent_branch_heads() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let txn = runtime.begin_transaction(
        TransactionOptions::default().merge_from_branches(vec![BranchId("missing".to_string())]),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::InvalidMergeParent
    ));
}

#[test]
fn branch_history_helpers_expose_ancestor_and_merge_base_reasoning() {
    let mut runtime = runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let chain = runtime
        .history_access()
        .ancestor_chain(feature.commit.commit_id);
    let merge_base = runtime
        .history_access()
        .latest_common_ancestor_between_branches(
            &BranchId("main".to_string()),
            &BranchId("feature".to_string()),
        );

    assert_eq!(chain, vec![main.commit.commit_id, feature.commit.commit_id]);
    assert_eq!(merge_base, Some(main.commit.commit_id));
    assert!(runtime.history_access().can_merge_branch_into(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string())
    ));
}

#[test]
fn merge_inspection_reports_overlapping_authority() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "shared");
    let shared = changed_entities(&base)[0];
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, shared, "main-updated");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-updated",
        BranchId("feature".to_string()),
    );
    let inspection = runtime.history_access().inspect_merge(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string()),
    );

    assert_eq!(inspection.merge_base, Some(base.commit.commit_id));
    assert!(!inspection.can_merge);
    assert_eq!(
        inspection.conflicting_records,
        vec![MergeConflictRecord::Entity(shared)]
    );
}

#[test]
fn merge_commit_rejects_overlapping_authority_since_merge_base() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "shared");
    let shared = changed_entities(&base)[0];
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, shared, "main-updated");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-updated",
        BranchId("feature".to_string()),
    );
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("main".to_string())),
        merge_parent_branches: vec![BranchId("feature".to_string())],
        ..TransactionOptions::default()
    });
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::MergeConflictOverlap
    ));
    assert!(error.commit_log().has_rejection(
        CommitPhase::HistoryResolution,
        Some(DiagnosticCode::MergeConflictOverlap),
        None
    ));
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::History)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeConflictOverlap));
}

#[test]
fn duplicate_branch_creation_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let error = runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap_err();

    assert_eq!(error.class, BranchCreateErrorClass::BranchAlreadyExists);
}

#[test]
fn chunked_storage_summary_tracks_visibility_boundaries() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let entity_a = changed_entities(&first)[0];
    let _second = create_entity_outcome(&mut runtime, "e1");
    let snapshot = runtime.visibility_authority().snapshot();
    let _third = create_entity_outcome(&mut runtime, "e2");
    let _update = update_entity(&mut runtime, entity_a, "e0-updated");

    let summary_before_update = runtime
        .storage_access()
        .chunked_storage_summary(snapshot.version_id);
    let summary_current = runtime
        .storage_access()
        .chunked_storage_summary(runtime.history_access().latest_commit().unwrap().version_id);

    assert_eq!(summary_before_update.entity_chunks.len(), 2);
    assert_eq!(summary_before_update.entity_chunks[0].visible_records, 2);
    assert_eq!(summary_before_update.entity_chunks[1].visible_records, 0);
    assert_eq!(summary_current.entity_chunks[1].visible_records, 1);
    assert_eq!(summary_current.entity_chunks[0].slot_len, 2);
}

#[test]
fn chunk_diagnostics_and_packet_plans_are_public_and_stable() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let second = create_entity_outcome(&mut runtime, "e1");
    let entity_a = changed_entities(&first)[0];
    let entity_b = changed_entities(&second)[0];
    let snapshot = runtime.visibility_authority().snapshot();
    let packet = QueryWorkPacket::bulk(
        "pair",
        vec![RecordRef::Entity(entity_a), RecordRef::Entity(entity_b)],
    );

    let plan = runtime
        .storage_access()
        .plan_read_packet(&snapshot, &packet)
        .unwrap();
    let diagnostics = runtime
        .storage_access()
        .chunk_diagnostics(snapshot.version_id);

    assert_eq!(plan.target_count, 2);
    assert_eq!(plan.entity_chunk_indexes, vec![0]);
    assert_eq!(diagnostics.entity_chunks_total, 1);
    assert_eq!(diagnostics.entity_chunks_with_visible_records, 1);
}

#[test]
fn record_local_aspect_history_reads_committed_patch_truth() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "after");
    let history =
        runtime
            .history_access()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let traced = runtime.history_access().entity_aspect_history_with_trace(
        &BranchId("main".to_string()),
        entity,
        None,
    );
    let artifact = traced.trace.diagnostic_artifact();
    let digest = traced.aspect_history_digest();
    let resolved_aspects = CanonicalAspectSet::new([
        AspectKey(InternedString::Raw("lifecycle".to_string())),
        AspectKey(InternedString::Raw("name".to_string())),
    ]);

    assert_eq!(history.len(), 2);
    assert_eq!(traced.entries, history);
    assert_eq!(
        traced.trace.requested_target,
        HistoryAspectQueryTarget::Entity(entity)
    );
    assert_eq!(traced.trace.returned_entries, 2);
    assert_eq!(traced.trace.resolved_aspects, resolved_aspects);
    assert_eq!(
        traced.trace.searched_commit_span,
        Some(AspectHistoryCommitSpan {
            first_commit_id: created.commit.commit_id,
            last_commit_id: updated.commit.commit_id,
        })
    );
    assert_eq!(traced.trace.searched_lineage_event_span, None);
    assert_eq!(traced.trace.traversed_lineage_events, 0);
    assert_eq!(artifact.scope, DiagnosticsScope::History);
    assert_eq!(artifact.kind, DiagnosticsArtifactKind::DetailedTrace);
    assert_eq!(artifact.entries.len(), 1);
    assert_eq!(
        artifact.entries[0].code,
        DiagnosticCode::AspectHistoryResolved
    );
    assert_eq!(
        digest.requested_target,
        HistoryAspectQueryTarget::Entity(entity)
    );
    assert_eq!(digest.entry_count, 2);
    assert_eq!(digest.resolved_aspects, resolved_aspects);
    assert_direct_history_origin_invariants(&history, RecordRef::Entity(entity));
    assert_eq!(history[0].origin.commit_id, created.commit.commit_id);
    assert_eq!(history[0].origin.target, RecordRef::Entity(entity));
    assert_eq!(history[0].origin.changed_aspects, resolved_aspects);
    assert_eq!(history[1].origin.commit_id, updated.commit.commit_id);

    let filtered = runtime.history_access().entity_aspect_history(
        &BranchId("main".to_string()),
        entity,
        Some(&AspectFilter {
            mode: AspectFilterMode::All,
            aspects: RequestedAspectSet::new([AspectKey(InternedString::Raw("name".to_string()))]),
        }),
    );
    let any_filtered = runtime.history_access().entity_aspect_history(
        &BranchId("main".to_string()),
        entity,
        Some(&AspectFilter {
            mode: AspectFilterMode::Any,
            aspects: RequestedAspectSet::new([
                AspectKey(InternedString::Raw("missing".to_string())),
                AspectKey(InternedString::Raw("name".to_string())),
            ]),
        }),
    );
    assert_eq!(filtered.len(), 2);
    assert_eq!(any_filtered.len(), 2);
}

#[test]
fn bulk_like_aspect_history_filters_and_query_packets_stay_stable_after_recovery() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let hub = create_entity(&mut runtime, "hub");
    let leaves = (0..8)
        .map(|index| {
            create_entity_in_partition(
                &mut runtime,
                &format!("leaf-{index}"),
                PartitionId((index % 3 + 7) as u32),
            )
        })
        .collect::<Vec<_>>();
    let relations = leaves
        .iter()
        .enumerate()
        .map(|(index, leaf)| {
            create_relation_in_partition(
                &mut runtime,
                hub,
                *leaf,
                &format!("edge-{index}"),
                PartitionId((index % 4 + 20) as u32),
            )
        })
        .collect::<Vec<_>>();
    for (index, leaf) in leaves.iter().enumerate() {
        let _ = update_entity(&mut runtime, *leaf, &format!("leaf-{index}-updated"));
    }
    let packet = QueryWorkPacket::bulk(
        "fanout-entities",
        leaves
            .iter()
            .copied()
            .map(RecordRef::Entity)
            .collect::<Vec<_>>(),
    );
    let snapshot = runtime.visibility_authority().snapshot();
    let planned_packet = runtime
        .storage_access()
        .plan_read_packet(&snapshot, &packet)
        .unwrap();
    let before_recovery_reads = runtime
        .visibility_reads()
        .execute_read_packet(&snapshot, &packet)
        .unwrap();
    let delete_outcome = delete_entity(&mut runtime, hub);
    let expected_entity_digests = leaves
        .iter()
        .map(|entity| {
            runtime
                .history_access()
                .entity_aspect_history_with_trace(
                    &BranchId("main".to_string()),
                    *entity,
                    Some(&AspectFilter {
                        mode: AspectFilterMode::All,
                        aspects: RequestedAspectSet::new([aspect_key("name")]),
                    }),
                )
                .aspect_history_digest()
        })
        .collect::<Vec<_>>();
    let expected_relation_digests = relations
        .iter()
        .map(|relation| {
            runtime
                .history_access()
                .relation_aspect_history_with_trace(
                    &BranchId("main".to_string()),
                    *relation,
                    Some(&AspectFilter {
                        mode: AspectFilterMode::All,
                        aspects: RequestedAspectSet::new([
                            aspect_key("source"),
                            aspect_key("target"),
                        ]),
                    }),
                )
                .aspect_history_digest()
        })
        .collect::<Vec<_>>();
    let expected_lifecycle_relation_digests = relations
        .iter()
        .map(|relation| {
            runtime
                .history_access()
                .relation_aspect_history_with_trace(
                    &BranchId("main".to_string()),
                    *relation,
                    Some(&AspectFilter {
                        mode: AspectFilterMode::Any,
                        aspects: RequestedAspectSet::new([aspect_key("lifecycle")]),
                    }),
                )
                .aspect_history_digest()
        })
        .collect::<Vec<_>>();
    runtime.durability_authority().checkpoint().unwrap();
    let recovery_plan = runtime.durability_access().recovery_plan();

    let mut recovered =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let recovery = recovered.durability_authority().recover(recovery_plan).unwrap();
    let recovered_snapshot = recovered.visibility_authority().snapshot();
    let recovered_reads = recovered
        .visibility_reads()
        .execute_read_packet(&recovered_snapshot, &packet)
        .unwrap();
    let recovered_entity_digests = leaves
        .iter()
        .map(|entity| {
            recovered
                .history_access()
                .entity_aspect_history_with_trace(
                    &BranchId("main".to_string()),
                    *entity,
                    Some(&AspectFilter {
                        mode: AspectFilterMode::All,
                        aspects: RequestedAspectSet::new([aspect_key("name")]),
                    }),
                )
                .aspect_history_digest()
        })
        .collect::<Vec<_>>();
    let recovered_relation_digests = relations
        .iter()
        .map(|relation| {
            recovered
                .history_access()
                .relation_aspect_history_with_trace(
                    &BranchId("main".to_string()),
                    *relation,
                    Some(&AspectFilter {
                        mode: AspectFilterMode::All,
                        aspects: RequestedAspectSet::new([
                            aspect_key("source"),
                            aspect_key("target"),
                        ]),
                    }),
                )
                .aspect_history_digest()
        })
        .collect::<Vec<_>>();
    let recovered_lifecycle_relation_digests = relations
        .iter()
        .map(|relation| {
            recovered
                .history_access()
                .relation_aspect_history_with_trace(
                    &BranchId("main".to_string()),
                    *relation,
                    Some(&AspectFilter {
                        mode: AspectFilterMode::Any,
                        aspects: RequestedAspectSet::new([aspect_key("lifecycle")]),
                    }),
                )
                .aspect_history_digest()
        })
        .collect::<Vec<_>>();

    assert_eq!(recovery.latest_commit, Some(delete_outcome.commit.clone()));
    assert_eq!(planned_packet.target_count, leaves.len());
    assert_eq!(before_recovery_reads.entities.len(), leaves.len());
    assert_eq!(before_recovery_reads.entities, recovered_reads.entities);
    assert_eq!(expected_entity_digests, recovered_entity_digests);
    assert_eq!(expected_relation_digests, recovered_relation_digests);
    assert_eq!(
        expected_lifecycle_relation_digests,
        recovered_lifecycle_relation_digests
    );
    assert!(expected_entity_digests.iter().all(|digest| digest.entry_count == 2));
    assert!(expected_relation_digests.iter().all(|digest| digest.entry_count == 1));
    assert!(expected_lifecycle_relation_digests
        .iter()
        .all(|digest| digest.entry_count == 2));
}
