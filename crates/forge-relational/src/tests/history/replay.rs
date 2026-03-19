use crate::facade::diagnostics::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayFailureClass, ReplayMismatchClass,
    ReplayObservableSurface,
};
use crate::tests::support::*;

// CONTRACT: replay
// LANES: success, failure, determinism

#[test]
fn replay_contract_success_reproduces_canonical_surfaces() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        replay.reconstructed_parent_chain,
        vec![outcome.commit.commit_id]
    );
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::Replay)
        .iter()
        .any(|artifact| artifact.kind == DiagnosticsArtifactKind::Comparison));
}

#[test]
fn replay_contract_failure_wrong_branch_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("wrong".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::BranchMismatch));
}

#[test]
fn replay_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "parent");
    let child = create_entity_outcome(&mut runtime, "child");

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(parent.commit.commit_id));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: child.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::MissingParentChain));
}

#[test]
fn replay_contract_success_preserves_merge_parent_order() {
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
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        runtime
            .replay_access()
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .commit
            .parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    assert_eq!(
        runtime
            .replay_access()
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .merge_base_commits,
        vec![main.commit.commit_id]
    );
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::History));
}

#[test]
fn replay_contract_reports_structured_patch_drift_when_canonical_envelope_is_tampered() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    assert!(runtime.history_authority().tamper_commit_patch_for_test(
        outcome.commit.commit_id,
        |patch| {
            patch.records[0].detail =
                PatchDetail::StructuredJson(serde_json::json!({"tampered": true}));
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert_eq!(replay.mismatches.len(), 1);
    assert_eq!(replay.mismatches[0].class, ReplayMismatchClass::PatchDrift);
    assert_eq!(replay.mismatches[0].surface, ReplayObservableSurface::Patch);
    assert!(replay.mismatches[0].expected.is_some());
    assert!(replay.mismatches[0].observed.is_some());
}

#[test]
fn replay_contract_preserves_aspect_bearing_patch_and_history_surfaces() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "after");
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r1");
    let relation = changed_relations(&relation_outcome)[0];
    let expected_entity_history =
        runtime
            .history_access()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let expected_relation_history = runtime.history_access().relation_aspect_history(
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let expected_entity_digest = runtime
        .history_access()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let expected_relation_digest = runtime
        .history_access()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: relation_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Patch));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::Diagnostics));
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::History));
    assert_eq!(expected_entity_history.len(), 2);
    assert_eq!(expected_relation_history.len(), 1);
    assert_eq!(expected_entity_digest.entry_count, 2);
    assert_eq!(expected_relation_digest.entry_count, 1);
    assert_patch_truth_invariants(&updated);
    assert_patch_truth_invariants(&relation_outcome);
}

#[test]
fn replay_and_recovery_preserve_aspect_bearing_truth_across_a_hostile_mixed_workload() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let created = create_entity_outcome(&mut runtime, "anchor");
    let anchor = changed_entities(&created)[0];
    let _updated = update_entity(&mut runtime, anchor, "anchor-updated");
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "net-edge");
    let relation = changed_relations(&relation_outcome)[0];
    let _retained = delete_entity(&mut runtime, source);
    let replace_outcome = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("replace-anchor").push(MutationIntent::Entity(
                EntityMutationIntent::Replace(ReplaceEntityIntent {
                    entity_id: anchor,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("anchor-replaced".to_string()),
                        payload: RecordPayload::StructuredJson(json!({"name":"anchor-replaced"})),
                    },
                }),
            )),
        );
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();

    let start_lineage = runtime.lineage_access().for_record(anchor).unwrap().lineage_id;
    let original_bundle =
        capture_aspect_truth_bundle(&mut runtime, &[anchor], &[relation], &[start_lineage]);

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: replace_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });
    assert!(runtime.replay_access().compare_outcome(&replay));

    let recovery_plan = runtime.durability_access().recovery_plan();
    let mut recovered =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();

    let recovered_bundle =
        capture_aspect_truth_bundle(&mut recovered, &[anchor], &[relation], &[start_lineage]);
    let recovered_replay_check = recovered
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: replace_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(original_bundle.visible_truth, recovered_bundle.visible_truth);
    assert_eq!(original_bundle.diagnostics, recovered_bundle.diagnostics);
    assert_eq!(
        original_bundle.entity_history_digests,
        recovered_bundle.entity_history_digests
    );
    assert_eq!(
        original_bundle.relation_history_digests,
        recovered_bundle.relation_history_digests
    );
    assert_eq!(
        original_bundle.lineage_history_digests,
        recovered_bundle.lineage_history_digests
    );
    assert_eq!(original_bundle.latest_patch, recovered_bundle.latest_patch);
    assert_eq!(original_bundle.latest_replay, recovered_bundle.latest_replay);
    assert!(recovered.replay_access().compare_outcome(&recovered_replay_check));
    assert_eq!(
        recovered_bundle.latest_replay.as_ref().unwrap().commit_id,
        recovered_replay_check.requested.commit_id
    );
    assert_eq!(
        recovered_bundle.latest_replay.as_ref().unwrap().patch,
        recovered_bundle.latest_patch.as_ref().unwrap().clone()
    );
}
