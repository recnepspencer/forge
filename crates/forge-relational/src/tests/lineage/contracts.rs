use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::identity::{KindId, LineageId, PartitionId};
use crate::facade::lineage::{LineageEventKind, LineageResolutionStatus};
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, TransactionOptions,
    WorkerIntentBatch,
};
use crate::tests::support::*;

// CONTRACT: lineage
// LANES: success, failure_boundary, determinism

#[test]
fn lineage_contract_correspondence_stays_advisory_until_promoted() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "left");
    let second = create_entity_outcome(&mut runtime, "right");
    let left_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let right_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![left_lineage],
        vec![right_lineage],
        "candidate",
    );
    let graph_before = runtime
        .lineage_access()
        .graph(&BranchId("main".to_string()));
    let resolution = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let graph_after = runtime
        .lineage_access()
        .graph(&BranchId("main".to_string()));

    assert_eq!(graph_before.events.len(), 2);
    assert_eq!(graph_before.correspondence_candidates.len(), 1);
    assert_eq!(resolution.status, LineageResolutionStatus::Promoted);
    assert_eq!(graph_after.events.len(), 3);
}

#[test]
fn lineage_contract_failure_invalid_references_do_not_promote() {
    let mut runtime = runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "anchor");
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![LineageId(999)],
        vec![LineageId(1000)],
        "invalid",
    );

    let resolution = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, commit.commit.clone());

    assert!(resolution.is_none());
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::Lineage)
        .iter()
        .any(|artifact| artifact
            .entries
            .iter()
            .any(|entry| entry.code == DiagnosticCode::InvariantViolation)));
}

#[test]
fn lineage_contract_branch_divergence_is_queryable() {
    let mut runtime = runtime_with_test_schema();
    let _main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let divergence = runtime.lineage_access().divergence_between_branches(
        &BranchId("main".to_string()),
        &BranchId("feature".to_string()),
    );

    assert!(!divergence.right_only_event_ids.is_empty());
    assert!(!divergence.shared_lineage_ids.is_empty());
}

#[test]
fn lineage_contract_delete_emits_retire_event() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "retired");
    let entity = changed_entities(&created)[0];
    let deleted = delete_entity(&mut runtime, entity);
    let graph = runtime
        .lineage_access()
        .graph(&BranchId("main".to_string()));

    assert!(graph
        .events
        .iter()
        .any(|event| event.commit.commit_id == deleted.commit.commit_id
            && event.kind == LineageEventKind::Retire));
}

#[test]
fn lineage_contract_replace_emits_replace_edge() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::InternedString::Raw(
                        "replacement".to_string(),
                    ),
                    payload: crate::payloads::data::RecordPayload::StructuredJson(
                        serde_json::json!({"name":"replacement"}),
                    ),
                },
            }),
        )),
    );
    let outcome = txn.commit().unwrap();
    let graph = runtime
        .lineage_access()
        .graph(&BranchId("main".to_string()));

    assert!(graph.events.iter().any(|event| {
        event.commit.commit_id == outcome.commit.commit_id
            && event.kind == LineageEventKind::Replace
            && event.sources.len() == 1
            && event.targets.len() == 1
    }));
}

#[test]
fn lineage_contract_multiple_same_shape_replacements_do_not_cross_wire_targets() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_outcome(&mut runtime, "left");
    let right = create_entity_outcome(&mut runtime, "right");
    let left_entity = changed_entities(&left)[0];
    let right_entity = changed_entities(&right)[0];

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace-many")
            .push(MutationIntent::Entity(EntityMutationIntent::Replace(
                ReplaceEntityIntent {
                    entity_id: left_entity,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: crate::symbols::data::InternedString::Raw(
                            "replacement-left".to_string(),
                        ),
                        payload: crate::payloads::data::RecordPayload::StructuredJson(
                            serde_json::json!({"name":"replacement"}),
                        ),
                    },
                },
            )))
            .push(MutationIntent::Entity(EntityMutationIntent::Replace(
                ReplaceEntityIntent {
                    entity_id: right_entity,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: crate::symbols::data::InternedString::Raw(
                            "replacement-right".to_string(),
                        ),
                        payload: crate::payloads::data::RecordPayload::StructuredJson(
                            serde_json::json!({"name":"replacement"}),
                        ),
                    },
                },
            ))),
    );
    let outcome = txn.commit().unwrap();
    let graph = runtime
        .lineage_access()
        .graph(&BranchId("main".to_string()));
    let replace_events = graph
        .events
        .iter()
        .filter(|event| {
            event.commit.commit_id == outcome.commit.commit_id
                && event.kind == LineageEventKind::Replace
        })
        .collect::<Vec<_>>();

    assert_eq!(replace_events.len(), 2);
    assert_ne!(replace_events[0].targets, replace_events[1].targets);
}
