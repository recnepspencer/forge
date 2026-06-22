use super::super::support::*;

#[test]
fn workspace_insert_uses_aspect_native_authoring_and_routes_live_delivery() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.aspect-insert")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.aspect-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-aspect-table")
        })
        .expect("live view should declare");

    let receipt = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Buy milk"),
            )
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
    assert_eq!(
        receipt.terminal_declared_collection_projection(),
        Some("Task")
    );
    assert_eq!(receipt.declared_entity_identity(), None);
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("Task")
    );
    assert_eq!(
        receipt.target_evidence().declared().target_class(),
        ForgeQueryMutationTargetClass::Collection
    );
    assert_eq!(
        receipt
            .target_evidence()
            .declared()
            .collection()
            .map(|collection| collection.as_str()),
        Some("Task")
    );
    assert_eq!(
        receipt.target_evidence().resolved().target_class(),
        ForgeQueryMutationTargetClass::Entity
    );
    assert_eq!(
        receipt
            .target_evidence()
            .resolved()
            .collection()
            .map(|collection| collection.as_str()),
        Some("Task")
    );
    assert!(receipt
        .target_evidence()
        .resolved()
        .entity_identity()
        .is_some());
    let causality = receipt
        .causality_evidence()
        .expect("authoritative insert should retain bridge causality");
    let provenance = receipt
        .provenance_evidence()
        .expect("authoritative insert should retain bridge provenance");
    assert!(!causality.causality_digest().is_empty());
    assert!(!causality.truth_trigger_digest().is_empty());
    assert!(!provenance.execution_record_digest().is_empty());
    assert!(!provenance.feedback_provenance_digest().is_empty());
    assert!(provenance.authoritative_artifact_digest().is_some());
    assert_eq!(
        receipt.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["identity.id", "title.value"]).as_slice()
    );
    assert_eq!(
        receipt
            .declared_aspect_operations()
            .iter()
            .map(|operation| {
                format!(
                    "{}:{}",
                    operation.kind(),
                    operation.aspect_touch().admitted_touch_digest_part()
                )
            })
            .collect::<Vec<_>>(),
        vec!["set:identity:id".to_string(), "set:title:value".to_string()]
    );
    assert_eq!(
        receipt.terminal_affected_live_view_ids_projection(),
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
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.aspect-update")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.title-only", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-title-only")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Buy milk"),
            )
            .aspect(
                test_aspect_touch("description.value"),
                test_string_aspect_value("whole milk"),
            )
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let rename = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Buy oat milk"),
            )
            .aspect(
                test_aspect_touch("description.value"),
                test_string_aspect_value("oat milk"),
            )
        })
        .expect("multi-aspect update should execute");
    let rename_patches = workspace.observe(&live);

    assert_eq!(
        rename.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["title.value", "description.value"]).as_slice()
    );
    assert_eq!(rename.mutation_family(), ForgeQueryMutationFamily::Update);
    assert_eq!(
        rename.declared_entity_identity(),
        Some(&seed.deltas()[0].entity_identity)
    );
    assert_eq!(
        rename.terminal_affected_live_view_ids_projection(),
        &["tasks.title-only".to_string()]
    );
    assert_eq!(rename_patches.query_delivery_batches.len(), 1);
    assert_eq!(
        rename_patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );

    let irrelevant = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.aspect(
                test_aspect_touch("description.value"),
                test_string_aspect_value("still hidden"),
            )
        })
        .expect("irrelevant aspect update should still execute");
    let irrelevant_patches = workspace.observe(&live);

    assert_eq!(
        irrelevant.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["description.value"]).as_slice()
    );
    assert!(irrelevant
        .terminal_affected_live_view_ids_projection()
        .is_empty());
    assert!(irrelevant_patches.query_delivery_batches.is_empty());
}

#[test]
fn write_receipt_inspection_retains_authored_mutation_metadata() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.aspect-metadata")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.metadata-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-metadata-table")
        })
        .expect("live view should declare");

    let receipt = workspace
        .insert("Task", |task| {
            task.metadata("author", "worth-topo")
                .metadata("intent", "topology-refresh")
                .aspect(
                    test_aspect_touch("identity.id"),
                    test_string_aspect_value("task-1"),
                )
                .aspect(
                    test_aspect_touch("title.value"),
                    test_string_aspect_value("Metadata receipt"),
                )
        })
        .expect("aspect-native insert should execute");
    let inspection = workspace
        .inspect(&receipt)
        .expect("write receipt should inspect");

    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .mutation_metadata()
                    .get(&test_mutation_metadata_key("author"))
                    .map(|value| value.native_digest_text()),
                Some("worth-topo")
            );
            assert_eq!(
                inspection
                    .mutation_metadata()
                    .get(&test_mutation_metadata_key("intent"))
                    .map(|value| value.native_digest_text()),
                Some("topology-refresh")
            );
            assert_eq!(
                inspection.target_evidence().declared().target_class(),
                ForgeQueryMutationTargetClass::Collection
            );
            assert!(inspection.causality_evidence().is_some());
            assert!(inspection.provenance_evidence().is_some());
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn workspace_update_clear_supports_typed_reset_without_waking_unrelated_surfaces() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.aspect-clear")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.clear-title-only", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-clear-title-only")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Buy milk"),
            )
            .aspect(
                test_aspect_touch("description.value"),
                test_string_aspect_value("whole milk"),
            )
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let hidden_clear = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.clear(test_aspect_touch("description.value"))
        })
        .expect("typed clear should execute");
    let hidden_patches = workspace.observe(&live);

    assert_eq!(
        hidden_clear.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["description.value"]).as_slice()
    );
    assert!(hidden_patches.query_delivery_batches.is_empty());

    let visible_clear = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.clear(test_aspect_touch("title.value"))
        })
        .expect("clearing a projected aspect should execute");
    let visible_patches = workspace.observe(&live);
    let rows = workspace.read(&live);
    let inspection = workspace
        .inspect(&visible_clear)
        .expect("clear receipt should inspect");

    assert_eq!(
        visible_clear.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["title.value"]).as_slice()
    );
    assert_eq!(visible_patches.query_delivery_batches.len(), 1);
    assert_eq!(
        visible_clear
            .declared_aspect_operations()
            .iter()
            .map(|operation| {
                format!(
                    "{}:{}",
                    operation.kind(),
                    operation.aspect_touch().admitted_touch_digest_part()
                )
            })
            .collect::<Vec<_>>(),
        vec!["clear:title:value".to_string()]
    );
    match inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(
                inspection
                    .declared_aspect_operations()
                    .iter()
                    .map(|operation| {
                        format!(
                            "{}:{}",
                            operation.kind(),
                            operation.aspect_touch().admitted_touch_digest_part()
                        )
                    })
                    .collect::<Vec<_>>(),
                vec!["clear:title:value".to_string()]
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
    assert_eq!(
        test_native_scalar_value(&rows[0], "title.value"),
        Some(&AspectValue::Null),
        "typed clear lowers to explicit native null"
    );
}
