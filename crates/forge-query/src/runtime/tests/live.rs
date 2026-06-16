use super::support::*;

#[test]
fn runtime_declares_live_view_and_routes_minimal_write_patches() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Buy milk")),
            ],
        ))
        .expect("insert should execute through runtime facade");
    let task_id = insert.deltas()[0].entity_identity.clone();
    let insert_patches = runtime.drain_patches(&view);

    assert_eq!(insert.deltas().len(), 1);
    assert_eq!(
        insert.deltas()[0].aspect_paths,
        vec!["identity.id".to_string(), "title.value".to_string()]
    );
    assert_eq!(
        insert.affected_live_view_ids(),
        &["tasks.table".to_string()]
    );
    assert!(insert_patches.live_patches.is_empty());
    assert_eq!(insert_patches.query_delivery_batches.len(), 1);
    assert_eq!(
        insert_patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::CollectionMembershipPatchGroup
    );
    assert_eq!(insert_patches.query_delivery_batches[0].sequence(), 1);

    let update = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: task_id,
            aspect_path: "title.value".to_string(),
            value: Value::String("Buy oat milk".to_string()),
        })
        .expect("update should execute through runtime facade");
    let update_patches = runtime.drain_patches(&view);

    assert_eq!(update.deltas()[0].aspect_paths, vec!["title.value"]);
    assert!(update_patches.live_patches.is_empty());
    assert_eq!(update_patches.query_delivery_batches.len(), 1);
    assert_eq!(
        update_patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );
    assert_eq!(update_patches.query_delivery_batches[0].sequence(), 2);

    let irrelevant = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: update.deltas()[0].entity_identity.clone(),
            aspect_path: "description.value".to_string(),
            value: Value::String("ignored by task table".to_string()),
        })
        .expect("irrelevant update should execute");
    let irrelevant_patches = runtime.drain_patches(&view);
    assert!(irrelevant.affected_live_view_ids().is_empty());
    assert!(irrelevant_patches.query_delivery_batches.is_empty());
}

#[test]
fn runtime_grouped_live_view_uses_backend_baseline_and_delivers_grouped_membership_patch() {
    let mut runtime = stateful_bridge_grouped_task_runtime();
    let table: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.seed-table",
            grouped_task_table_live_request(),
            grouped_task_schema(),
        )
        .expect("table live view should declare before seed write");
    let seed = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Seed task")),
                ("status.value", json!("todo")),
            ],
        ))
        .expect("seed insert should write through table declaration");
    let task_id = seed.deltas()[0].entity_identity.clone();
    let _ = runtime.drain_patches(&table);
    let grouped: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.grouped",
            grouped_task_live_request(),
            grouped_task_schema(),
        )
        .expect("grouped live view should declare with backend-owned baseline");

    let receipt = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: task_id,
            aspect_path: "status.value".to_string(),
            value: Value::String("done".to_string()),
        })
        .expect("grouping aspect update should write");
    let patches = runtime.drain_patches(&grouped);

    assert!(receipt
        .affected_live_view_ids()
        .contains(&"tasks.grouped".to_string()));
    assert_eq!(patches.query_delivery_batches.len(), 1);
    assert_eq!(
        patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::GroupedMembershipPatchGroup
    );
    assert_eq!(
        grouped.subscription_installation().subscription_family(),
        "grouped_collection_membership"
    );
}

#[test]
fn unified_inspect_routes_live_effect_and_write_receipt_targets() {
    let mut runtime = stateful_bridge_task_runtime();
    let live: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.inspect-target", task_live_request(), task_schema())
        .expect("live view should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.inspect-target",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "ui.inspect",
        ))
        .expect("effect should declare");
    let receipt = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Inspect target")),
            ],
        ))
        .expect("write should execute");

    let live_inspection = runtime.inspect(&live).expect("live target should inspect");
    let effect_inspection = runtime
        .inspect(&effect)
        .expect("effect target should inspect");
    let receipt_inspection = runtime
        .inspect(&receipt)
        .expect("receipt target should inspect");

    match live_inspection {
        ForgeQueryInspection::LiveView(inspection) => {
            assert_eq!(inspection.view_name(), "tasks.inspect-target");
        }
        other => panic!("expected live inspection, got {other:?}"),
    }

    match effect_inspection {
        ForgeQueryInspection::Effect(inspection) => {
            assert_eq!(inspection.name(), "ui.inspect-target");
            assert!(inspection.feedback_graph().is_some());
        }
        other => panic!("expected effect inspection, got {other:?}"),
    }

    match receipt_inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(inspection.mutation_family(), "insert");
            assert_eq!(
                inspection.authority_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
            assert_eq!(
                inspection.basis_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
            assert_eq!(inspection.declared_collection(), Some("Task"));
            assert_eq!(inspection.commit_identity(), receipt.commit_identity());
            assert!(!inspection.inspection_digest().is_empty());
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }
}

#[test]
fn live_view_inspection_reconstructs_subscription_proof_chain() {
    let mut runtime = stateful_bridge_task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");

    let installation = view.subscription_installation();
    let inspection = runtime
        .inspect_live_view_explanation(&view)
        .expect("live view explanation should inspect retained installation");

    assert_eq!(inspection.view_name(), "tasks.table");
    assert_eq!(
        inspection.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(inspection.query_projection().label().as_str(), installation.query_projection().label().as_str());
    assert_eq!(
        inspection.view_shape_projection().label().as_str(),
        installation.view_shape_projection().label().as_str()
    );
    assert_eq!(
        inspection.subscription_family(),
        installation.subscription_family()
    );
    assert_eq!(
        inspection.subscription_family_projection().label().as_str(),
        installation.subscription_family_projection().label().as_str()
    );
    assert_eq!(
        inspection.subscription_declaration_projection().label().as_str(),
        installation.subscription_declaration_projection().label().as_str()
    );
    assert_eq!(
        inspection.bridge_declaration_projection().label().as_str(),
        installation.bridge_declaration_projection().label().as_str()
    );
    assert_eq!(
        inspection.admission_projection().label().as_str(),
        installation.admission_projection().label().as_str()
    );
    assert_eq!(
        inspection.activation_projection().label().as_str(),
        installation.activation_projection().label().as_str()
    );
    assert_eq!(
        inspection.basis_binding_projection().label().as_str(),
        installation.basis_binding_projection().label().as_str()
    );
    assert_eq!(
        inspection.signal_strategy_projection().label().as_str(),
        installation.signal_strategy_projection().label().as_str()
    );
    assert_eq!(
        inspection.active_lane_projection().label().as_str(),
        installation.active_lane_projection().label().as_str()
    );
    assert_eq!(
        inspection.consumer_attachment_projection().label().as_str(),
        installation.consumer_attachment_projection().label().as_str()
    );
    assert_eq!(inspection.consumer_projection().label().as_str(), installation.consumer_projection().label().as_str());
    assert_eq!(
        inspection.delivery_cursor_projection().label().as_str(),
        installation.delivery_cursor_projection().label().as_str()
    );
    assert_eq!(
        inspection.subscription_budget_policy(),
        installation.subscription_budget_policy()
    );
    assert_eq!(
        inspection.active_lifecycle_budget_policy(),
        installation.active_lifecycle_budget_policy()
    );
    assert_eq!(
        inspection.consumer_attachment_budget_policy(),
        installation.consumer_attachment_budget_policy()
    );
    assert_eq!(
        inspection.runtime_budget_projection().label().as_str(),
        installation.runtime_budget_projection().label().as_str()
    );
    assert_eq!(
        inspection.support_projection().label().as_str(),
        installation.support_projection().label().as_str()
    );
    assert_eq!(
        inspection.installation_projection().label().as_str(),
        installation.installation_projection().label().as_str()
    );
    assert!(!inspection.inspection_projection().label().as_str().is_empty());

    let counters = inspection.counters();
    assert_eq!(
        counters.declaration_counter_for_reporting(),
        installation.counters().counter_projection().label()
    );
    assert_eq!(
        counters.active_lane_counter_for_reporting(),
        installation.active_lane_counters().counter_projection().label()
    );
    assert_eq!(
        counters.consumer_attachment_counter_for_reporting(),
        installation.consumer_attachment_counters().counter_projection().label()
    );
    assert_eq!(counters.family_selection_count(), 1);
    assert_eq!(counters.declaration_count(), 1);
    assert_eq!(counters.bridge_lowering_count(), 1);
    assert_eq!(counters.admission_count(), 1);
    assert_eq!(counters.activation_input_count(), 1);
    assert_eq!(counters.active_lane_admission_count(), 1);
    assert_eq!(counters.active_lane_creation_count(), 1);
    assert_eq!(counters.active_lane_handle_issue_count(), 1);
    assert_eq!(counters.consumer_attachment_count(), 1);
    assert_eq!(counters.consumer_attachment_denial_count(), 0);
}

#[test]
fn grouped_live_view_inspection_preserves_grouped_family_and_baseline_support() {
    let mut runtime = stateful_bridge_grouped_task_runtime();
    let table: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.seed-table",
            grouped_task_table_live_request(),
            grouped_task_schema(),
        )
        .expect("table live view should declare before grouped view");
    let _ = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Seed task")),
                ("status.value", json!("todo")),
            ],
        ))
        .expect("seed insert should write through table declaration");
    let _ = runtime.drain_patches(&table);
    let grouped: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.grouped",
            grouped_task_live_request(),
            grouped_task_schema(),
        )
        .expect("grouped live view should declare with backend-owned baseline");

    let inspection = runtime
        .inspect_live_view_explanation(&grouped)
        .expect("grouped live view explanation should inspect retained installation");

    assert_eq!(
        inspection.subscription_family(),
        "grouped_collection_membership"
    );
    assert_eq!(
        inspection.subscription_family_projection().label().as_str(),
        grouped
            .subscription_installation()
            .subscription_family_projection()
            .label()
            .as_str()
    );
    assert_eq!(
        inspection.support_projection().label().as_str(),
        grouped
            .subscription_installation()
            .support_projection()
            .label()
            .as_str()
    );
    assert!(!inspection.support_projection().label().as_str().is_empty());
    assert_eq!(inspection.counters().family_selection_count(), 1);
    assert_eq!(inspection.counters().declaration_count(), 1);
    assert_eq!(inspection.counters().bridge_lowering_count(), 1);
    assert_eq!(inspection.counters().admission_count(), 1);
    assert_eq!(inspection.counters().active_lane_creation_count(), 1);
    assert_eq!(inspection.counters().consumer_attachment_count(), 1);
    assert!(!inspection.inspection_projection().label().as_str().is_empty());
}

#[test]
fn redeclared_live_view_replaces_runtime_delivery_index_membership() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("bridge-backed runtime should build");
    let task_view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("shared.surface", task_live_request(), task_schema())
        .expect("task live view should declare");
    let task_seed = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Task seed")),
            ],
        ))
        .expect("task seed should write");
    let _ = runtime.drain_patches(&task_view);

    let issue_view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("shared.surface", issue_live_request(), issue_schema())
        .expect("same live view name should redeclare against issue collection");
    let stale_task_update = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: task_seed.deltas()[0].entity_identity.clone(),
            aspect_path: "title.value".to_string(),
            value: Value::String("Task update after redeclare".to_string()),
        })
        .expect("task update should still write");
    let stale_task_patches = runtime.drain_patches(&issue_view);

    assert!(stale_task_update.affected_live_view_ids().is_empty());
    assert!(stale_task_patches.query_delivery_batches.is_empty());

    let issue_write = runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", json!("")),
                ("summary.value", json!("Issue seed")),
            ],
        ))
        .expect("issue insert should write");
    let issue_patches = runtime.drain_patches(&issue_view);

    assert_eq!(
        issue_write.affected_live_view_ids(),
        &["shared.surface".to_string()]
    );
    assert_eq!(issue_patches.query_delivery_batches.len(), 1);
}
