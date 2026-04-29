use super::super::support::*;

#[test]
fn workspace_delete_receipt_preserves_family_target_and_inspection_posture() {
    let mut workspace = task_runtime()
        .workspace("tasks.aspect-delete")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.delete-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-delete-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Buy milk")
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let receipt = workspace
        .delete(seed.deltas()[0].entity_identity.clone())
        .expect("delete should execute");
    let state = workspace
        .state(&receipt)
        .expect("delete receipt should expose state posture");
    let inspection = workspace
        .inspect(&receipt)
        .expect("delete receipt should inspect");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Delete);
    assert_eq!(receipt.declared_collection(), None);
    assert_eq!(
        receipt.declared_entity_identity(),
        Some(seed.deltas()[0].entity_identity.as_str())
    );
    assert_eq!(receipt.target_collection(), Some("Task"));
    assert_eq!(
        receipt.target_entity_identity(),
        Some(seed.deltas()[0].entity_identity.as_str())
    );
    assert_eq!(
        receipt.target_evidence().declared().target_class(),
        ForgeQueryMutationTargetClass::Entity
    );
    assert_eq!(
        receipt.target_evidence().resolved().target_class(),
        ForgeQueryMutationTargetClass::Entity
    );
    assert!(receipt.causality_evidence().is_some());
    assert!(receipt.provenance_evidence().is_some());
    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        receipt.basis_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(state.kind(), ForgeQueryRuntimeStateKind::Ready);

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(inspection.mutation_family(), "delete");
            assert_eq!(
                inspection.basis_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
            assert_eq!(inspection.target_collection(), Some("Task"));
            assert_eq!(
                inspection.target_entity_identity(),
                Some(seed.deltas()[0].entity_identity.as_str())
            );
            assert_eq!(
                inspection.declared_entity_identity(),
                Some(seed.deltas()[0].entity_identity.as_str())
            );
            assert!(inspection.causality_evidence().is_some());
            assert!(inspection.provenance_evidence().is_some());
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn workspace_delete_with_preserves_touched_aspects_and_metadata_for_routing() {
    let mut workspace = task_runtime()
        .workspace("tasks.aspect-delete-with")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.delete-with-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-delete-with-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Buy milk")
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let receipt = workspace
        .delete_with(seed.deltas()[0].entity_identity.clone(), |delete| {
            delete.touch("title.value").metadata("author", "worth-topo")
        })
        .expect("delete with declared meaning should execute");
    let patches = workspace.observe(&live);
    let inspection = workspace
        .inspect(&receipt)
        .expect("delete-with receipt should inspect");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Delete);
    assert_eq!(receipt.declared_collection(), None);
    assert_eq!(
        receipt.deltas()[0].aspect_paths,
        vec!["title.value".to_string()]
    );
    assert_eq!(
        receipt
            .declared_aspect_operations()
            .iter()
            .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
            .collect::<Vec<_>>(),
        vec!["clear:title.value".to_string()]
    );
    assert_eq!(
        receipt.affected_live_view_ids(),
        &["tasks.delete-with-table".to_string()]
    );
    assert_eq!(patches.query_delivery_batches.len(), 1);

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(inspection.declared_collection(), None);
            assert_eq!(
                inspection
                    .mutation_metadata()
                    .get("author")
                    .and_then(Value::as_str),
                Some("worth-topo")
            );
            assert_eq!(
                inspection
                    .declared_aspect_operations()
                    .iter()
                    .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
                    .collect::<Vec<_>>(),
                vec!["clear:title.value".to_string()]
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn preview_delete_with_preserves_declared_target_collection_and_delete_meaning() {
    let mut workspace = task_runtime()
        .workspace("tasks.preview-delete-with")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.preview-delete-with-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-preview-delete-with-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Preview delete target")
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview_with_options(
            "task-preview-delete-with",
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");

    let receipt = preview
        .delete_with(seed.deltas()[0].entity_identity.clone(), |delete| {
            delete
                .target_collection("Task")
                .touch("title.value")
                .metadata("author", "worth-topo")
        })
        .expect("preview delete with declared target should stage");
    let inspection = workspace
        .inspect(&receipt)
        .expect("preview delete-with receipt should inspect");

    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(receipt.declared_collection(), Some("Task"));
    assert_eq!(receipt.target_collection(), Some("Task"));
    assert_eq!(
        receipt
            .declared_aspect_operations()
            .iter()
            .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
            .collect::<Vec<_>>(),
        vec!["clear:title.value".to_string()]
    );

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(inspection.declared_collection(), Some("Task"));
            assert_eq!(inspection.target_collection(), Some("Task"));
            assert_eq!(
                inspection
                    .mutation_metadata()
                    .get("author")
                    .and_then(Value::as_str),
                Some("worth-topo")
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn workspace_batch_routes_shared_computeds_once_per_batch_boundary() {
    let mut workspace = task_runtime()
        .workspace("tasks.batch-computed-routing")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.batch-routing-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-batch-routing-table")
        })
        .expect("live view should declare");
    let _computed: ForgeQueryDerivedViewHandle<Value> = workspace
        .computed(
            "tasks.batch-routing-summary",
            |c| {
                c.depends_on_live(&live)
                    .reads(["title.value"])
                    .produces(["ui.batch_routing_summary"])
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

    assert_eq!(
        receipt.considered_computed_view_count(),
        1,
        "shared computed surfaces should route once per batch boundary, not once per sub-write"
    );
}
