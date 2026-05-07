use super::super::support::*;

#[test]
fn workspace_batch_aggregates_touched_surfaces_and_remains_inspectable() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.aspect-batch")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-batch-table")
        })
        .expect("live view should declare");
    let computed: ForgeQueryDerivedViewHandle<Value> = workspace
        .computed(
            "tasks.batch-summary",
            |c| {
                c.depends_on_live(&live)
                    .reads(["title.value"])
                    .produces(["ui.batch_summary"])
            },
            TitleListMaintainer,
        )
        .expect("computed view should declare");

    let receipt = workspace
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.aspect("identity.id", "task-1")
                        .aspect("title.value", "Buy milk")
                })
                .insert("Task", |task| {
                    task.aspect("identity.id", "task-2")
                        .aspect("title.value", "Buy bread")
                })
        })
        .expect("aspect-native batch should execute");
    let patches = workspace.observe(&live);
    let materialized = workspace.materialize(&computed);
    let inspection = workspace
        .inspect(&receipt)
        .expect("batch receipt should inspect");
    let state = workspace
        .state(&receipt)
        .expect("batch receipt should expose state posture");

    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        receipt.basis_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(receipt.write_count(), 2);
    assert_eq!(receipt.considered_computed_view_count(), 1);
    assert_eq!(receipt.batch_mutation_evidence().component_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().target_evidence_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().resolved_target_count(), 2);
    assert_eq!(
        receipt.batch_mutation_evidence().target_collection_count(),
        0
    );
    assert_eq!(receipt.batch_mutation_evidence().target_entity_count(), 2);
    assert_eq!(
        receipt.batch_mutation_evidence().causality_bundle_count(),
        2
    );
    assert_eq!(
        receipt.batch_mutation_evidence().provenance_bundle_count(),
        2
    );
    assert_eq!(receipt.batch_mutation_evidence().outcome_class_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().request_digest_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().receipt_digest_count(), 2);
    assert!(!receipt
        .batch_mutation_evidence()
        .aggregate_target_digest()
        .is_empty());
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_causality_digest()
        .is_some());
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_provenance_digest()
        .is_some());
    assert_eq!(
        receipt.touched_aspect_paths(),
        &["identity.id".to_string(), "title.value".to_string()]
    );
    assert_eq!(
        receipt.affected_live_view_ids(),
        &["tasks.batch-table".to_string()]
    );
    assert_eq!(
        receipt.affected_derived_view_ids(),
        &["tasks.batch-summary".to_string()]
    );
    assert_eq!(patches.query_delivery_batches.len(), 2);
    assert_eq!(materialized.len(), 2);

    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(inspection.write_receipt_count(), 2);
            assert_eq!(
                inspection.touched_aspect_paths(),
                &["identity.id".to_string(), "title.value".to_string()]
            );
            assert_eq!(
                inspection.affected_live_view_ids(),
                &["tasks.batch-table".to_string()]
            );
            assert_eq!(
                inspection.affected_derived_view_ids(),
                &["tasks.batch-summary".to_string()]
            );
            assert_eq!(inspection.commit_identities().len(), 2);
            assert_eq!(inspection.component_operations().len(), 2);
            assert_eq!(inspection.component_operations()[0].family(), "insert");
            assert_eq!(
                inspection.component_operations()[0].declared_collection(),
                Some("Task")
            );
            assert_eq!(
                inspection.component_operations()[0].target_collection(),
                Some("Task")
            );
            assert_eq!(
                inspection.component_operations()[0]
                    .target_evidence()
                    .declared()
                    .target_class(),
                ForgeQueryMutationTargetClass::Collection
            );
            assert!(inspection.component_operations()[0]
                .causality_evidence()
                .is_some());
            assert!(inspection.component_operations()[0]
                .provenance_evidence()
                .is_some());
            assert_eq!(
                inspection.component_operations()[0].collections(),
                &["Task".to_string()]
            );
            assert_eq!(
                inspection.component_operations()[0]
                    .declared_aspect_operations()
                    .iter()
                    .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
                    .collect::<Vec<_>>(),
                vec!["set:identity.id".to_string(), "set:title.value".to_string()]
            );
            assert_eq!(
                inspection.basis_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
            assert_eq!(inspection.batch_mutation_evidence().component_count(), 2);
            assert_eq!(
                inspection.batch_mutation_evidence().target_evidence_count(),
                2
            );
            assert_eq!(
                inspection.batch_mutation_evidence().resolved_target_count(),
                2
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .target_collection_count(),
                0
            );
            assert_eq!(
                inspection.batch_mutation_evidence().target_entity_count(),
                2
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .causality_bundle_count(),
                2
            );
            assert_eq!(
                inspection
                    .batch_mutation_evidence()
                    .provenance_bundle_count(),
                2
            );
            assert_eq!(
                inspection.batch_mutation_evidence().outcome_class_count(),
                2
            );
            assert!(inspection
                .batch_mutation_evidence()
                .aggregate_causality_digest()
                .is_some());
            assert!(inspection
                .batch_mutation_evidence()
                .aggregate_provenance_digest()
                .is_some());
            assert!(!inspection.batch_digest().is_empty());
            assert!(!inspection.inspection_digest().is_empty());
        }
        other => panic!("expected batch receipt inspection, got {other:?}"),
    }
    assert_eq!(state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        state.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
}

#[test]
fn preview_batch_stages_multiple_aspect_native_writes_in_preview_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-batch")
        .expect("task runtime should open a named workspace");
    let mut preview = workspace
        .preview_with_options(
            "task-preview-batch",
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");

    let receipt = preview
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.aspect("identity.id", "preview-task-1")
                        .aspect("title.value", "Preview title one")
                })
                .insert("Task", |task| {
                    task.aspect("identity.id", "preview-task-2")
                        .aspect("title.value", "Preview title two")
                })
        })
        .expect("preview batch should stage");
    let outcome = preview.discard();

    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(receipt.basis_lane(), ForgeQueryAuthorityLane::PreviewTruth);
    assert_eq!(receipt.write_count(), 2);
    assert_eq!(receipt.considered_computed_view_count(), 0);
    assert_eq!(
        receipt.touched_aspect_paths(),
        &["identity.id".to_string(), "title.value".to_string()]
    );
    assert_eq!(outcome.authoritative_residue_count(), 0);
}

#[test]
fn preview_batch_uses_batch_target_evidence_without_authority_bundles() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-batch-evidence")
        .expect("task runtime should open a named workspace");
    let mut preview = workspace
        .preview_with_options(
            "task-preview-batch-evidence",
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");

    let receipt = preview
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.aspect("identity.id", "preview-task-1")
                        .aspect("title.value", "Preview title one")
                })
                .insert("Task", |task| {
                    task.aspect("identity.id", "preview-task-2")
                        .aspect("title.value", "Preview title two")
                })
        })
        .expect("preview batch should stage");

    assert_eq!(receipt.batch_mutation_evidence().component_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().target_evidence_count(), 2);
    assert_eq!(receipt.batch_mutation_evidence().resolved_target_count(), 0);
    assert_eq!(
        receipt.batch_mutation_evidence().target_collection_count(),
        2
    );
    assert_eq!(receipt.batch_mutation_evidence().target_entity_count(), 0);
    assert_eq!(
        receipt.batch_mutation_evidence().causality_bundle_count(),
        0
    );
    assert_eq!(
        receipt.batch_mutation_evidence().provenance_bundle_count(),
        0
    );
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_causality_digest()
        .is_none());
    assert!(receipt
        .batch_mutation_evidence()
        .aggregate_provenance_digest()
        .is_none());
}
