use super::support::*;

#[test]
fn workspace_insert_uses_aspect_native_authoring_and_routes_live_delivery() {
    let mut workspace = task_runtime()
        .workspace("tasks.aspect-insert")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.aspect-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-aspect-table")
        })
        .expect("live view should declare");

    let receipt = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Buy milk")
        })
        .expect("aspect-native insert should execute");
    let patches = workspace.observe(&live);
    let state = workspace
        .state(&receipt)
        .expect("write receipt should expose state posture");

    assert_eq!(receipt.deltas().len(), 1);
    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Insert);
    assert_eq!(
        receipt.basis_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(receipt.declared_collection(), Some("Task"));
    assert_eq!(receipt.declared_entity_identity(), None);
    assert_eq!(
        receipt.deltas()[0].aspect_paths,
        vec!["identity.id".to_string(), "title.value".to_string()]
    );
    assert_eq!(
        receipt
            .declared_aspect_operations()
            .iter()
            .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
            .collect::<Vec<_>>(),
        vec!["set:identity.id".to_string(), "set:title.value".to_string()]
    );
    assert_eq!(
        receipt.affected_live_view_ids(),
        &["tasks.aspect-table".to_string()]
    );
    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(patches.query_delivery_batches.len(), 1);
    assert_eq!(
        patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::CollectionMembershipPatchGroup
    );
    assert_eq!(state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        state.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
}

#[test]
fn workspace_update_supports_multi_aspect_authoring_and_narrows_by_touched_meaning() {
    let mut workspace = task_runtime()
        .workspace("tasks.aspect-update")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.title-only", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-title-only")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Buy milk")
                .aspect("description.value", "whole milk")
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let rename = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.aspect("title.value", "Buy oat milk")
                .aspect("description.value", "oat milk")
        })
        .expect("multi-aspect update should execute");
    let rename_patches = workspace.observe(&live);

    assert_eq!(
        rename.deltas()[0].aspect_paths,
        vec!["title.value".to_string(), "description.value".to_string()]
    );
    assert_eq!(rename.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(
        rename.declared_entity_identity(),
        Some(seed.deltas()[0].entity_identity.as_str())
    );
    assert_eq!(
        rename.affected_live_view_ids(),
        &["tasks.title-only".to_string()]
    );
    assert_eq!(rename_patches.query_delivery_batches.len(), 1);
    assert_eq!(
        rename_patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );

    let irrelevant = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.aspect("description.value", "still hidden")
        })
        .expect("irrelevant aspect update should still execute");
    let irrelevant_patches = workspace.observe(&live);

    assert_eq!(
        irrelevant.deltas()[0].aspect_paths,
        vec!["description.value".to_string()]
    );
    assert!(irrelevant.affected_live_view_ids().is_empty());
    assert!(irrelevant_patches.query_delivery_batches.is_empty());
}

#[test]
fn workspace_update_clear_supports_typed_reset_without_waking_unrelated_surfaces() {
    let mut workspace = task_runtime()
        .workspace("tasks.aspect-clear")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.clear-title-only", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-clear-title-only")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Buy milk")
                .aspect("description.value", "whole milk")
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let hidden_clear = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.clear("description.value")
        })
        .expect("typed clear should execute");
    let hidden_patches = workspace.observe(&live);

    assert_eq!(
        hidden_clear.deltas()[0].aspect_paths,
        vec!["description.value".to_string()]
    );
    assert!(hidden_patches.query_delivery_batches.is_empty());

    let visible_clear = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.clear("title.value")
        })
        .expect("clearing a projected aspect should execute");
    let visible_patches = workspace.observe(&live);
    let rows = workspace.read(&live);
    let inspection = workspace
        .inspect(&visible_clear)
        .expect("clear receipt should inspect");

    assert_eq!(
        visible_clear.deltas()[0].aspect_paths,
        vec!["title.value".to_string()]
    );
    assert_eq!(visible_patches.query_delivery_batches.len(), 1);
    assert_eq!(
        visible_clear
            .declared_aspect_operations()
            .iter()
            .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path()))
            .collect::<Vec<_>>(),
        vec!["clear:title.value".to_string()]
    );
    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
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
    assert_eq!(
        rows[0].payload["title"]["value"],
        Value::Null,
        "typed clear currently lowers to explicit null while the JSON substrate is still underneath"
    );
}

#[test]
fn preview_insert_uses_aspect_native_authoring_and_stays_preview_local() {
    let mut workspace = task_runtime()
        .workspace("tasks.preview-aspect-insert")
        .expect("task runtime should open a named workspace");
    let mut preview = workspace
        .preview_with_options(
            "task-preview",
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");

    let receipt = preview
        .insert("Task", |task| {
            task.aspect("identity.id", "preview-task-1")
                .aspect("title.value", "Preview title")
        })
        .expect("preview insert should stage");
    let outcome = preview.discard();

    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(receipt.basis_lane(), ForgeQueryAuthorityLane::PreviewTruth);
    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Insert);
    assert_eq!(receipt.declared_collection(), Some("Task"));
    assert_eq!(
        receipt.deltas()[0].aspect_paths,
        vec!["identity.id".to_string(), "title.value".to_string()]
    );
    assert_eq!(outcome.authoritative_residue_count(), 0);
}

#[test]
fn workspace_batch_aggregates_touched_surfaces_and_remains_inspectable() {
    let mut workspace = task_runtime()
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
    let mut workspace = task_runtime()
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
            assert_eq!(
                inspection.declared_entity_identity(),
                Some(seed.deltas()[0].entity_identity.as_str())
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

#[test]
fn aspect_native_mutation_builders_reject_empty_or_duplicate_authoring() {
    let mut workspace = task_runtime()
        .workspace("tasks.aspect-errors")
        .expect("task runtime should open a named workspace");

    let empty = workspace
        .insert("Task", |task| task)
        .expect_err("empty aspect mutation should fail closed");
    match empty {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("at least one aspect"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }

    let duplicate = workspace
        .insert("Task", |task| {
            task.aspect("title.value", "Buy milk")
                .aspect("title.value", "Buy oat milk")
        })
        .expect_err("duplicate aspect paths should fail closed");
    match duplicate {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("may only be declared once"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }

    let duplicate_clear = workspace
        .update("entity:1:1:1", |task| {
            task.clear("title.value").aspect("title.value", "Buy milk")
        })
        .expect_err("clear and set of the same aspect should fail closed");
    match duplicate_clear {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("may only be declared once"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }

    let empty_batch = workspace
        .batch(|batch| batch)
        .expect_err("empty mutation batch should fail closed");
    match empty_batch {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("at least one operation"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }
}
