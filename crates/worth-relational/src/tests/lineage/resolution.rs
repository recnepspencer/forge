use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope};
use crate::facade::history::{AspectHistoryLineageEventSpan, BranchId, HistoryAspectQueryTarget};
use crate::facade::identity::{KindId, PartitionId};
use crate::facade::lineage::{
    HistoricalResolutionBoundednessBasis, HistoricalResolutionDigestMode,
    HistoricalResolutionRequest, LineageGraphRequest, LineageGraphTraversalBasis,
};
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, WorkerIntentBatch,
};
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

    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("replacement"),
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "replacement",
                    ),
                },
            }),
        )),
    );
    let outcome = txn.commit(&mut runtime).unwrap();
    let resolution =
        runtime
            .lineage_access()
            .resolve_historical_lineage(HistoricalResolutionRequest {
                branch_id: BranchId("main".to_string()),
                lineage_id: start_lineage,
                boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
            });

    assert_eq!(resolution.start, start_lineage);
    assert_eq!(
        resolution.boundedness_basis,
        HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed
    );
    assert_eq!(resolution.traversed_event_ids.len(), 1);
    assert_eq!(resolution.resolved.len(), 1);
    assert_ne!(resolution.resolved[0], start_lineage);
    assert_eq!(resolution.metrics.traversed_event_count, 1);
    assert!(resolution.metrics.event_visit_count >= 1);
    assert_eq!(resolution.metrics.resolved_lineage_count, 1);
    assert_eq!(
        resolution.digest_basis().digest_mode(),
        HistoricalResolutionDigestMode::ExactDigestCanonicalOrder
    );
    assert_eq!(resolution.digest_basis().requested_start(), start_lineage);
    assert_eq!(
        resolution.digest_basis().canonical_traversed_event_ids(),
        resolution.traversed_event_ids
    );
    assert_eq!(resolution.trace.digest_basis(), resolution.digest_basis());
    assert_eq!(
        runtime
            .lineage_access()
            .graph(LineageGraphRequest {
                branch_id: BranchId("main".to_string()),
                traversal_basis: LineageGraphTraversalBasis::FullBranchGraphMaterialization,
            })
            .events
            .iter()
            .filter(|event| event.commit.commit_id == outcome.commit.commit_id)
            .count(),
        2
    );
}

#[test]
fn failed_durable_append_installs_no_lineage_under_reused_commit_id() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "failed-lineage-source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .expect("source lineage")
        .lineage_id();
    let failed_commit_id = runtime.history.preview_next_commit_id();
    let before = runtime.lineage_access().graph(LineageGraphRequest {
        branch_id: BranchId("main".to_owned()),
        traversal_basis: LineageGraphTraversalBasis::FullBranchGraphMaterialization,
    });

    runtime.durability.fail_next_append = true;
    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction.push_batch(WorkerIntentBatch::new("failed-replacement").push(
        MutationIntent::Entity(EntityMutationIntent::Replace(ReplaceEntityIntent {
            entity_id: entity,
            replacement: crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("failed-successor"),
                fields: single_string_aspect_field_patch(
                    aspect_key("name"),
                    field_key("name"),
                    "failed-successor",
                ),
            },
        })),
    ));
    let error = transaction
        .commit(&mut runtime)
        .expect_err("injected durable failure must abort publication");
    assert!(format!("{error:?}").contains("test-injected durable append failure"));

    let after_failure = runtime.lineage_access().graph(LineageGraphRequest {
        branch_id: BranchId("main".to_owned()),
        traversal_basis: LineageGraphTraversalBasis::FullBranchGraphMaterialization,
    });
    assert_eq!(after_failure.nodes, before.nodes);
    assert_eq!(after_failure.events, before.events);
    let successor = create_entity_outcome(&mut runtime, "reused-commit-successor");
    assert_eq!(successor.commit.commit_id, failed_commit_id);
    let resolution =
        runtime
            .lineage_access()
            .resolve_historical_lineage(HistoricalResolutionRequest {
                branch_id: BranchId("main".to_owned()),
                lineage_id: start_lineage,
                boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
            });
    assert_eq!(resolution.resolved, vec![start_lineage]);
    assert!(resolution.traversed_event_ids.is_empty());
}

#[test]
fn historical_lineage_resolution_does_not_scan_unrelated_branch_events() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;

    for index in 0..6 {
        let label = format!("unrelated-{index}");
        let _ = create_entity_outcome(&mut runtime, &label);
    }

    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("replacement"),
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "replacement",
                    ),
                },
            }),
        )),
    );
    let _ = txn.commit(&mut runtime).unwrap();

    let total_branch_events = runtime
        .lineage_access()
        .graph(LineageGraphRequest {
            branch_id: BranchId("main".to_string()),
            traversal_basis: LineageGraphTraversalBasis::FullBranchGraphMaterialization,
        })
        .metrics
        .event_count;

    let resolution =
        runtime
            .lineage_access()
            .resolve_historical_lineage(HistoricalResolutionRequest {
                branch_id: BranchId("main".to_string()),
                lineage_id: start_lineage,
                boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
            });

    assert_eq!(resolution.metrics.traversed_event_count, 1);
    assert_eq!(resolution.metrics.event_visit_count, 1);
    assert!(total_branch_events > resolution.metrics.event_visit_count);
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

    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: entity,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("replacement"),
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "replacement",
                    ),
                },
            }),
        )),
    );
    let replacement = txn.commit(&mut runtime).unwrap();
    let history = runtime
        .lineage_access()
        .entity_aspect_history(
            HistoricalResolutionRequest {
                branch_id: BranchId("main".to_string()),
                lineage_id: start_lineage,
                boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
            },
            None,
        )
        .unwrap();
    let traced = runtime.lineage_access().entity_aspect_history_with_trace(
        HistoricalResolutionRequest {
            branch_id: BranchId("main".to_string()),
            lineage_id: start_lineage,
            boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
        },
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
        Some(AspectHistoryLineageEventSpan {
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
        history.resolved_lineage_chain.len() as u64
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
