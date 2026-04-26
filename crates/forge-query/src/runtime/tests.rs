use serde_json::json;

use super::*;
use crate::declarative_live::{DeclarativeLiveViewShape, DeclarativeProjectionField};
use crate::program::{
    ForgeQueryOperation, ForgeQueryPortType, ForgeQueryProgramSource, ForgeQuerySchemaAdapter,
    ForgeQueryTypedPort, ForgeQueryValueExpr, ForgeQueryWriteCommandTemplate,
};
use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RawCommittedPatchEnvelope,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridgeBuilder,
    SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket, SnapshotReadPacketResult,
    SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};

#[test]
fn runtime_builder_rejects_missing_backend_inputs() {
    let error = match ForgeQueryRuntime::builder().build() {
        Ok(_) => panic!("builder should reject missing v1 backend"),
        Err(error) => error,
    };

    assert!(matches!(error, ForgeQueryRuntimeError::MissingBackend));
}

#[test]
fn runtime_builder_rejects_incomplete_backend_parts() {
    let error = ForgeQueryRuntime::builder()
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing bridge should reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingRuntimeBridge
    ));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing schema adapter should reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingSchemaAdapter
    ));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing source adapter should reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingSourceAdapter
    ));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing write authority should reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingWriteAuthority
    ));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing signal sink should reject"),
        Err(error) => error,
    };
    assert!(matches!(error, ForgeQueryRuntimeError::MissingSignalSink));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing subscription activation should reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingSubscriptionActivation
    ));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing preview basis should reject"),
        Err(error) => error,
    };
    assert!(matches!(error, ForgeQueryRuntimeError::MissingPreviewBasis));

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("missing inspector evidence should reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingInspectorEvidence
    ));
}

#[test]
fn runtime_builder_accepts_bridge_backed_backend_parts() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build");
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("external.tasks", task_live_request(), task_schema())
        .expect("external backend should declare live view");
    let receipt = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "external-1" },
                "title": { "value": "External task" },
            }),
        })
        .expect("external write authority should execute");

    assert_eq!(view.name(), "external.tasks");
    assert_eq!(
        view.subscription_installation().subscription_family(),
        "collection_membership"
    );
    assert_eq!(
        view.subscription_installation().authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        view.subscription_installation()
            .counters()
            .activation_input_count(),
        1
    );
    assert!(view
        .subscription_installation()
        .support_evidence()
        .starts_with("test-subscription-activation:external.tasks:"));
    assert!(!view
        .subscription_installation()
        .active_lane_digest()
        .is_empty());
    assert!(!view
        .subscription_installation()
        .consumer_attachment_digest()
        .is_empty());
    assert!(!view
        .subscription_installation()
        .consumer_digest()
        .is_empty());
    assert!(!view
        .subscription_installation()
        .delivery_cursor_digest()
        .is_empty());
    assert_eq!(
        view.subscription_installation()
            .active_lane_counters()
            .active_lane_creation_count(),
        1
    );
    assert_eq!(
        view.subscription_installation()
            .consumer_attachment_counters()
            .consumer_attachment_count(),
        1
    );
    assert_eq!(
        view.subscription_installation()
            .subscription_budget_policy(),
        runtime_subscription_budget_policy()
    );
    assert_eq!(
        view.subscription_installation()
            .active_lifecycle_budget_policy(),
        RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY
    );
    assert_eq!(
        view.subscription_installation()
            .consumer_attachment_budget_policy(),
        RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY
    );
    assert_eq!(
        view.subscription_installation().runtime_budget_digest(),
        runtime_subscription_budget_digest()
    );
    let live_inspection = runtime
        .inspect_live_view(&view)
        .expect("inspector should retain live subscription installation");
    assert_eq!(
        live_inspection.installation_digest(),
        view.subscription_installation().installation_digest()
    );
    assert_eq!(receipt.commit_identity(), "external-commit-1");
    assert_eq!(
        receipt.affected_live_view_ids(),
        &["external.tasks".to_string()]
    );
    {
        let inspector = runtime
            .try_inspect_receipt(&receipt)
            .expect("inspector evidence adapter should inspect receipt");
        assert_eq!(
            inspector.runtime_evidence().artifact_family(),
            "test-write-receipt"
        );
        assert_eq!(
            inspector.runtime_evidence().evidence(),
            &["test-inspector-evidence".to_string()]
        );
    }
    {
        let preview = runtime
            .try_preview("external preview")
            .expect("preview basis adapter should admit preview basis");
        assert_eq!(preview.basis_admission().label(), "external preview");
        assert_eq!(
            preview.basis_admission().evidence(),
            &["test-preview-basis".to_string()]
        );
    }
}

#[test]
fn runtime_live_declaration_denies_backend_admission_before_subscription_install() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(DenyingSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend with denying schema admission should still build");

    let error = runtime
        .declare_live_view::<Value>("external.schema-denied", task_live_request(), task_schema())
        .expect_err("backend admission denial must block subscription installation");

    match error {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name,
            stage,
            message,
        } => {
            assert_eq!(view_name, "external.schema-denied");
            assert_eq!(stage, "backend-live-admission");
            assert!(message.contains("schema admission denied by test adapter"));
        }
        other => panic!("expected backend admission denial, got {other:?}"),
    }
}

#[test]
fn runtime_live_declaration_closes_active_subscription_when_source_declaration_fails() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::fail_declare())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend with failing source declaration should still build");

    let error = runtime
        .declare_live_view::<Value>("external.source-denied", task_live_request(), task_schema())
        .expect_err("source declaration denial must close active subscription");

    match error {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name,
            stage,
            message,
        } => {
            assert_eq!(view_name, "external.source-denied");
            assert_eq!(stage, "source-declaration");
            assert!(message.contains("source declaration denied by test adapter"));
            assert!(message.contains("active subscription closeout:"));
            assert!(message.contains("terminal:true"));
        }
        other => panic!("expected source declaration denial, got {other:?}"),
    }
    assert_eq!(runtime.active_subscriptions.lane_count(), 0);
    assert!(runtime.live_subscriptions.is_empty());
}

#[test]
fn runtime_equivalent_live_declarations_share_active_lane_with_distinct_consumers() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build");

    let first: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("external.tasks.first", task_live_request(), task_schema())
        .expect("first live view should install active lane");
    let second: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("external.tasks.second", task_live_request(), task_schema())
        .expect("equivalent live view should join active lane");

    assert_eq!(
        first.subscription_installation().active_lane_digest(),
        second.subscription_installation().active_lane_digest()
    );
    assert_ne!(
        first
            .subscription_installation()
            .consumer_attachment_digest(),
        second
            .subscription_installation()
            .consumer_attachment_digest()
    );
    assert_eq!(
        second
            .subscription_installation()
            .active_lane_counters()
            .active_lane_join_count(),
        1
    );
    assert_eq!(
        second
            .subscription_installation()
            .active_lane_counters()
            .shared_lane_count(),
        1
    );
    assert_eq!(
        second
            .subscription_installation()
            .consumer_attachment_counters()
            .consumer_attachment_count(),
        1
    );
    assert_eq!(
        second
            .subscription_installation()
            .consumer_attachment_counters()
            .affected_consumer_attachment_width(),
        2
    );
}

#[test]
fn runtime_live_declaration_denies_before_source_when_subscription_activation_rejects() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(DenyingSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend with denying activation should still build");

    let error = runtime
        .declare_live_view::<Value>("external.denied", task_live_request(), task_schema())
        .expect_err("activation denial must block source declaration");

    match error {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name,
            stage,
            message,
        } => {
            assert_eq!(view_name, "external.denied");
            assert_eq!(stage, "activation-admission");
            assert!(message.contains("activation denied by test adapter"));
        }
        other => panic!("expected live subscription installation denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_profiles_expose_facade_family_posture() {
    let memory_runtime = task_runtime();
    let bridge_runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build");

    for family in [
        ForgeQueryRuntimeFacadeFamily::Read,
        ForgeQueryRuntimeFacadeFamily::Live,
        ForgeQueryRuntimeFacadeFamily::Computed,
        ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryRuntimeFacadeFamily::BranchPreview,
        ForgeQueryRuntimeFacadeFamily::Write,
        ForgeQueryRuntimeFacadeFamily::Inspect,
    ] {
        assert_eq!(
            memory_runtime
                .support_profile()
                .support_for(family)
                .expect("memory support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(
            bridge_runtime
                .support_profile()
                .support_for(family)
                .expect("bridge-backed support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
    }

    assert_eq!(
        bridge_runtime
            .support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::Intent)
            .expect("intent support row should exist")
            .status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert!(bridge_runtime
        .support_profile()
        .support_for(ForgeQueryRuntimeFacadeFamily::Live)
        .expect("live support row should exist")
        .evidence()
        .iter()
        .any(|evidence| evidence == "test-subscription-activation"));
}

#[test]
fn runtime_support_denies_unsupported_write_family_before_execution() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "test backend disabled write authority",
            ),
        ),
    );

    let error = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "external-1" },
                "title": { "value": "Should not write" },
            }),
        })
        .expect_err("unsupported write family should deny before write authority");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Write);
            assert_eq!(denial.reason(), "test backend disabled write authority");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_builder_rejects_support_profiles_that_overclaim_unimplemented_families() {
    let profile = ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
        ForgeQueryRuntimeFamilySupport::supported(
            ForgeQueryRuntimeFacadeFamily::Intent,
            [ForgeQueryAuthorityLane::PendingWriteIntent],
            [ForgeQueryEffectPolicy::AuthoritativeAllowed],
            ["fake-intent-adapter"],
        ),
    );

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(profile)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("support profile must not claim unimplemented facade support"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Intent);
            assert!(denial.reason().contains("intent authority adapter"));
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_computed_family_before_registration() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Computed,
                "test backend disabled computed resources",
            ),
        ),
    );

    let error = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles.unsupported", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect_err("unsupported computed family should deny before registration");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Computed);
            assert_eq!(denial.reason(), "test backend disabled computed resources");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_declares_live_view_and_routes_minimal_write_patches() {
    let mut runtime = task_runtime();
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");

    let insert = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Buy milk" },
            }),
        })
        .expect("insert should execute through runtime facade");
    let task_id = insert.deltas()[0].entity_identity.clone();
    let insert_patches = runtime.drain_patches(&view);

    assert_eq!(insert.deltas().len(), 1);
    assert!(insert.deltas()[0].aspect_paths.is_empty());
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
    let mut runtime = grouped_task_runtime();
    let table: ForgeQueryLiveView<Value> = runtime
        .declare_live_view(
            "tasks.seed-table",
            grouped_task_table_live_request(),
            grouped_task_schema(),
        )
        .expect("table live view should declare before seed write");
    let seed = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Seed task" },
                "status": { "value": "todo" },
            }),
        })
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
fn redeclared_live_view_replaces_runtime_delivery_index_membership() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Task seed" },
            }),
        })
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Issue".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "summary": { "value": "Issue seed" },
            }),
        })
        .expect("issue insert should write");
    let issue_patches = runtime.drain_patches(&issue_view);

    assert_eq!(
        issue_write.affected_live_view_ids(),
        &["shared.surface".to_string()]
    );
    assert_eq!(issue_patches.query_delivery_batches.len(), 1);
}

#[test]
fn compiled_typed_program_installs_runs_and_emits_trace() {
    let mut runtime = task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    let run = runtime
        .run_operation(
            operation,
            vec![ForgeQueryOperationInput::new(
                "title",
                Value::String("Typed task".to_string()),
            )],
        )
        .expect("program should run");
    let trace = runtime.inspect_run(&run).expect("trace should be retained");

    assert_eq!(trace.operation_id(), "create_task");
    assert_eq!(run.outputs()[0].name(), "live:tasks.table");
    assert_eq!(run.outputs()[0].value()[0]["title"]["value"], "Typed task");
    assert!(trace
        .generated_declarations()
        .iter()
        .any(|declaration| declaration == "live:tasks.table"));
    assert_eq!(trace.write_receipts().len(), 1);
    assert_eq!(trace.patch_artifacts().len(), 1);
    assert!(trace
        .patch_artifacts()
        .iter()
        .any(|artifact| artifact.starts_with("query-delivery:tasks.table:")));
}

#[test]
fn compiled_typed_program_rejects_type_mismatch_before_execution() {
    let mut runtime = task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    let error = runtime
        .run_operation(
            operation,
            vec![ForgeQueryOperationInput::new("title", Value::Bool(true))],
        )
        .expect_err("type mismatch should reject before effects execute");

    assert!(matches!(error, ForgeQueryRuntimeError::Program(_)));
}

#[test]
fn runtime_surfaces_authority_lanes_on_public_handles_and_receipts() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.authority", task_live_request(), task_schema())
        .expect("live view should declare");
    let derived = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles.authority", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect("derived view should declare");

    let receipt = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Authority lane task" },
            }),
        })
        .expect("insert should write");
    let patches = runtime.drain_derived_patches(derived.name());
    let inspector = runtime.inspect_receipt(&receipt);

    assert_eq!(
        live.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        derived.authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        inspector.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        patches.derived_patches[0].authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
}

#[test]
fn preview_defaults_to_derive_only_effect_policy_but_keeps_explicit_writes_preview_local() {
    let mut runtime = task_runtime();
    let mut preview = runtime.preview("default policy");

    assert_eq!(preview.effect_policy(), ForgeQueryEffectPolicy::DeriveOnly);
    assert!(preview
        .admit_effect_action(
            ForgeQueryEffectAction::Derive,
            ForgeQueryAuthorityLane::DerivedRuntimeState
        )
        .is_ok());

    let delivery_denial = preview
        .admit_effect_action(
            ForgeQueryEffectAction::Deliver,
            ForgeQueryAuthorityLane::EffectDeliveryState,
        )
        .expect_err("derive-only preview should deny effect delivery");
    assert!(matches!(
        delivery_denial,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));

    let write_denial = preview
        .admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("derive-only preview should deny authoritative write intent");
    assert!(matches!(
        write_denial,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));

    let preview_receipt = preview
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Preview-local task" },
            }),
        })
        .expect("explicit preview write should stage");
    assert_eq!(
        preview_receipt.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );

    let outcome = preview.discard();
    assert_eq!(outcome.effect_policy(), ForgeQueryEffectPolicy::DeriveOnly);
    assert_eq!(outcome.source_lane(), ForgeQueryAuthorityLane::PreviewTruth);
    assert_eq!(outcome.target_lane(), ForgeQueryAuthorityLane::PreviewTruth);
}

#[test]
fn sandboxed_preview_policy_admits_only_sandboxed_write_intents() {
    let mut runtime = task_runtime();
    let preview = runtime.preview_with_options(
        "sandboxed writes",
        ForgeQueryPreviewOptions::derive_only()
            .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
    );

    let admission = preview
        .admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
        )
        .expect("sandboxed write intent should be admitted to preview truth");
    assert_eq!(
        admission.policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );
    assert_eq!(admission.action(), ForgeQueryEffectAction::WriteIntent);
    assert_eq!(
        admission.target_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );

    let denial = preview
        .admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("sandboxed write intent must not target authoritative truth");
    assert!(matches!(
        denial,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));
}

#[test]
fn derive_only_preview_denies_operation_write_effects() {
    let mut runtime = task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    let mut preview = runtime.preview("derive-only operation");
    let error = preview
        .run_operation(
            operation,
            vec![ForgeQueryOperationInput::new(
                "title",
                Value::String("Should not stage".to_string()),
            )],
        )
        .expect_err("derive-only preview should deny write-effect operations");

    assert!(matches!(
        error,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));
    assert_eq!(preview.compare_to_authoritative().write_count(), 0);
}

#[test]
fn sandboxed_preview_run_operation_stages_compiled_writes_until_promote() {
    let mut runtime = task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    let preview_run = {
        let mut preview = runtime.preview_with_options(
            "draft create",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        let run = preview
            .run_operation(
                operation.clone(),
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Preview-only task".to_string()),
                )],
            )
            .expect("preview operation should run");

        assert_eq!(run.write_receipts().len(), 1);
        assert!(run.write_receipts()[0]
            .commit_identity()
            .starts_with("preview:draft create"));
        assert_eq!(
            run.write_receipts()[0].authority_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );
        run
    };

    assert_eq!(
        preview_run.outputs()[0].value().as_array().unwrap().len(),
        0
    );

    {
        let mut preview = runtime.preview_with_options(
            "promote create",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Promoted preview task".to_string()),
                )],
            )
            .expect("preview operation should stage");
        let outcome = preview.promote().expect("preview promotion should succeed");
        assert!(outcome.promoted());
        assert_eq!(outcome.write_count(), 1);
        assert_eq!(
            outcome.effect_policy(),
            ForgeQueryEffectPolicy::SandboxedWriteIntent
        );
        assert_eq!(outcome.source_lane(), ForgeQueryAuthorityLane::PreviewTruth);
        assert_eq!(
            outcome.target_lane(),
            ForgeQueryAuthorityLane::AuthoritativeTruth
        );
    }

    let view = runtime
        .declare_live_view::<Value>("tasks.after-preview", task_live_request(), task_schema())
        .expect("live view should declare");
    let rows = runtime.read_live(&view);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].payload["title"]["value"], "Promoted preview task");
}

#[test]
fn preview_run_operation_discard_keeps_authoritative_state_unchanged() {
    let mut runtime = task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    {
        let mut preview = runtime.preview_with_options(
            "discard create",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Discarded preview task".to_string()),
                )],
            )
            .expect("preview operation should stage");
        let outcome = preview.discard();
        assert!(outcome.discarded());
    }

    let view = runtime
        .declare_live_view::<Value>("tasks.after-discard", task_live_request(), task_schema())
        .expect("live view should declare");
    assert!(runtime.read_live(&view).is_empty());
}

#[test]
fn preview_discard_closeout_separates_temporary_writes_from_authoritative_residue() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.preview-closeout", task_live_request(), task_schema())
        .expect("live should declare");

    let outcome = {
        let mut preview = runtime.preview("discard closeout");
        preview.use_view(&live);
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "preview-temp-1" },
                    "title": { "value": "Temporary one" },
                }),
            })
            .expect("first preview write should stage");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "preview-temp-2" },
                    "title": { "value": "Temporary two" },
                }),
            })
            .expect("second preview write should stage");
        preview.discard()
    };
    let closeout = outcome.closeout_evidence();

    assert!(outcome.discarded());
    assert_eq!(closeout.kind(), ForgeQueryPreviewCloseoutKind::Discarded);
    assert_eq!(closeout.preview_binding_count(), 1);
    assert_eq!(closeout.live_binding_count(), 1);
    assert_eq!(closeout.preview_write_staging_count(), 2);
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::PreviewWriteStaging),
        2
    );
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::AuthoritativeResidue),
        0
    );
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert_eq!(closeout.effect_delivery_residue_count(), 0);
    assert_eq!(closeout.pending_write_intent_residue_count(), 0);
    assert!(!closeout.closeout_digest().is_empty());
    assert!(runtime.read_live(&live).is_empty());
}

#[test]
fn preview_promotion_closeout_records_consumed_staging_without_preview_lane_mutation() {
    let mut runtime = task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");
    let outcome = {
        let mut preview = runtime.preview_with_options(
            "promotion closeout",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Promoted closeout task".to_string()),
                )],
            )
            .expect("preview operation should stage");
        preview.promote().expect("preview promotion should succeed")
    };
    let closeout = outcome.closeout_evidence();

    assert!(outcome.promoted());
    assert_eq!(outcome.write_count(), 1);
    assert_eq!(closeout.kind(), ForgeQueryPreviewCloseoutKind::Promoted);
    assert_eq!(closeout.preview_write_staging_count(), 1);
    assert_eq!(closeout.promoted_write_count(), 1);
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::PreviewWriteStaging),
        1
    );
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert_eq!(
        closeout.effect_policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );

    let view = runtime
        .declare_live_view::<Value>(
            "tasks.after-promotion-closeout",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    let rows = runtime.read_live(&view);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].payload["title"]["value"], "Promoted closeout task");
}

#[test]
fn preview_promotion_rejects_stale_basis_before_authority_execution() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(DriftingSnapshotSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("drifting backend should build");

    let error = {
        let mut preview = runtime.preview("stale basis");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "stale-preview" },
                    "title": { "value": "Should not promote" },
                }),
            })
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("drifting authoritative basis should deny promotion")
    };

    match error {
        ForgeQueryRuntimeError::PreviewPromotionStaleBasis(evidence) => {
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::StaleBasis
            );
            assert_eq!(evidence.staged_preview_write_count(), 1);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_ne!(
                evidence.basis_snapshot_token(),
                evidence.promotion_snapshot_token()
            );
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected stale basis promotion denial, got {other:?}"),
    }
}

#[test]
fn preview_promotion_write_failure_is_typed_and_not_silently_dropped() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(DenyingWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("denying write backend should build");

    let error = {
        let mut preview = runtime.preview("write failure");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "denied-preview" },
                    "title": { "value": "Denied preview write" },
                }),
            })
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("write authority denial should fail promotion")
    };

    match error {
        ForgeQueryRuntimeError::PreviewPromotionWriteFailed { evidence } => {
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::WriteFailed
            );
            assert_eq!(evidence.staged_preview_write_count(), 1);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_eq!(evidence.failed_write_sequence(), Some(1));
            assert!(evidence.reason().contains("write authority denied"));
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected write failure promotion denial, got {other:?}"),
    }
}

#[test]
fn preview_promotion_rejects_multi_write_batch_before_partial_authority_execution() {
    let attempted_writes = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(CountingWriteAuthority {
            attempted_writes: attempted_writes.clone(),
        })
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("counting write backend should build");

    let error = {
        let mut preview = runtime.preview("multi write promotion");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "preview-batch-1" },
                    "title": { "value": "First staged write" },
                }),
            })
            .expect("first preview write should stage");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "preview-batch-2" },
                    "title": { "value": "Second staged write" },
                }),
            })
            .expect("second preview write should stage");
        preview
            .promote()
            .expect_err("non-atomic multi-write promotion should deny before authority")
    };

    assert_eq!(attempted_writes.get(), 0);
    match error {
        ForgeQueryRuntimeError::PreviewPromotionAtomicBatchUnsupported(evidence) => {
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported
            );
            assert_eq!(evidence.staged_preview_write_count(), 2);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_eq!(evidence.failed_write_sequence(), None);
            assert!(evidence.reason().contains("atomic promotion support"));
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected atomic batch promotion denial, got {other:?}"),
    }
}

#[test]
fn derive_only_preview_binds_handles_and_mutes_effects_without_residue() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.preview-bind", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.preview-bind", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.preview-bind",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "ui.preview",
        ))
        .expect("delivery effect should declare");
    let intent_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "intent.preview-bind",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "preview-intent",
        ))
        .expect("write-intent effect should declare");

    let outcome = {
        let mut preview = runtime.preview("derive-only bindings");
        let live_binding = preview.use_view(&live);
        let computed_binding = preview.use_computed(&computed);
        let delivery_binding = preview
            .use_effect(&delivery_effect)
            .expect("derive-only preview should bind delivery effect muted");
        let intent_binding = preview
            .use_effect(&intent_effect)
            .expect("derive-only preview should bind write-intent effect muted");

        assert_eq!(
            live_binding.family(),
            ForgeQueryPreviewHandleBindingFamily::LiveView
        );
        assert_eq!(
            live_binding.preview_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );
        assert_eq!(
            computed_binding.source_lane(),
            ForgeQueryAuthorityLane::DerivedRuntimeState
        );
        assert_eq!(
            delivery_binding.effect_disposition(),
            Some(ForgeQueryPreviewEffectBindingDisposition::MutedByDeriveOnly)
        );
        assert!(!delivery_binding.effect_delivery_admitted());
        assert_eq!(
            intent_binding.effect_disposition(),
            Some(ForgeQueryPreviewEffectBindingDisposition::MutedByDeriveOnly)
        );
        assert!(!intent_binding.pending_write_intent_admitted());

        preview.discard()
    };

    assert_eq!(outcome.preview_binding_count(), 4);
    assert_eq!(outcome.effect_binding_count(), 2);
    assert_eq!(outcome.effect_delivery_residue_count(), 0);
    assert_eq!(outcome.pending_write_intent_residue_count(), 0);
    assert_eq!(outcome.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_effect_deliveries(&delivery_effect)
        .expect("authoritative delivery queue should still exist")
        .is_empty());
    assert!(runtime
        .drain_effect_deliveries(&intent_effect)
        .expect("authoritative intent queue should still exist")
        .is_empty());
}

#[test]
fn preview_write_routes_bound_live_computed_and_redirected_effect_without_authoritative_residue() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>(
            "tasks.preview-execution",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.preview-execution", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.preview-execution",
            ForgeQueryEffectTrigger::computed_view(&computed, ["title.summary"]),
            "ui.preview",
        ))
        .expect("delivery effect should declare");

    let (execution_evidence, outcome) = {
        let mut preview = runtime.preview_with_options(
            "preview execution",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::Redirected),
        );
        preview.use_view(&live);
        preview.use_computed(&computed);
        preview
            .use_effect(&delivery_effect)
            .expect("redirected preview should admit delivery effect");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "preview-execution-task" },
                    "title": { "value": "Preview execution task" },
                }),
            })
            .expect("preview write should stage and route");
        (
            preview.preview_execution_evidence().to_vec(),
            preview.discard(),
        )
    };

    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::LivePatch
            && evidence.handle_name() == "tasks.preview-execution"
            && evidence.preview_lane() == ForgeQueryAuthorityLane::PreviewTruth
            && !evidence.execution_digest().is_empty()
    }));
    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::ComputedPatch
            && evidence.handle_name() == "computed.preview-execution"
            && evidence.aspect_paths() == ["title.summary"]
    }));
    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::EffectDelivery
            && evidence.handle_name() == "ui.preview-execution"
    }));

    let closeout = outcome.closeout_evidence();
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::SubscriptionState),
        1
    );
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::DerivedRuntimeState),
        1
    );
    assert_eq!(closeout.effect_delivery_residue_count(), 1);
    assert_eq!(closeout.pending_write_intent_residue_count(), 0);
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_patches(&live)
        .query_delivery_batches
        .is_empty());
    assert!(runtime.read_derived(&computed).is_empty());
    assert!(runtime
        .drain_effect_deliveries(&delivery_effect)
        .expect("authoritative effect queue should exist")
        .is_empty());
}

#[test]
fn preview_sandboxed_write_intent_execution_stays_separate_from_delivery_residue() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>(
            "tasks.preview-intent-exec",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let intent_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "intent.preview-execution",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "preview-intent",
        ))
        .expect("write-intent effect should declare");

    let (execution_evidence, outcome) = {
        let mut preview = runtime.preview_with_options(
            "preview intent execution",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        preview.use_view(&live);
        preview
            .use_effect(&intent_effect)
            .expect("sandboxed preview should admit write-intent effect");
        preview
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "preview-intent-task" },
                    "title": { "value": "Preview intent task" },
                }),
            })
            .expect("preview write should route pending intent");
        (
            preview.preview_execution_evidence().to_vec(),
            preview.discard(),
        )
    };

    assert!(execution_evidence.iter().any(|evidence| {
        evidence.kind() == ForgeQueryPreviewExecutionKind::PendingWriteIntent
            && evidence.handle_name() == "intent.preview-execution"
    }));
    let closeout = outcome.closeout_evidence();
    assert_eq!(closeout.effect_delivery_residue_count(), 0);
    assert_eq!(closeout.pending_write_intent_residue_count(), 1);
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_effect_deliveries(&intent_effect)
        .expect("authoritative pending intent queue should exist")
        .is_empty());
}

#[test]
fn strategy_intent_commit_routes_query_delivery_and_returns_canonical_receipt() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<Value>("tasks.intent", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.intent", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.intent",
            ForgeQueryEffectTrigger::computed_view(&computed, ["title.summary"]),
            "ui.intent",
        ))
        .expect("effect should declare");

    let receipt = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "reconcile-task-title",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({
                "entity": "task-1",
                "title": "Intent committed title"
            }),
        ))
        .expect("intent should execute");

    assert_eq!(receipt.intent_name(), "reconcile-task-title");
    assert_eq!(receipt.strategy_identity(), "strategy.intent.reconcile");
    assert_eq!(receipt.strategy_version(), "1.0");
    assert_eq!(
        receipt.canonical_input_digest(),
        ForgeQueryIntentDeclaration::strategy_commit(
            "reconcile-task-title",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({
                "entity": "task-1",
                "title": "Intent committed title"
            }),
        )
        .input_digest()
    );
    assert_eq!(
        receipt.target_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(receipt.affected_live_view_ids(), ["tasks.intent"]);
    assert_eq!(receipt.affected_derived_view_ids(), ["computed.intent"]);
    assert_eq!(receipt.considered_computed_view_count(), 1);
    assert_eq!(receipt.considered_effect_count(), 1);
    assert_eq!(receipt.delivered_effect_count(), 1);
    assert_eq!(receipt.pending_write_intent_count(), 0);
    assert!(!receipt.produced_mutation_digest().is_empty());
    assert!(!receipt.receipt_digest().is_empty());
    assert_eq!(receipt.invariant_evidence(), ["test-invariant-authority"]);
    assert_eq!(runtime.drain_patches(&live).query_delivery_batches.len(), 1);
    assert_eq!(runtime.read_derived(&computed).len(), 1);
    assert_eq!(
        runtime
            .drain_effect_deliveries(&delivery_effect)
            .expect("effect queue should exist")
            .len(),
        1
    );
}

#[test]
fn intent_support_profile_claim_requires_executable_authority_adapter() {
    let error = match ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
    {
        Ok(_) => panic!("intent support claim without adapter should fail"),
        Err(error) => error,
    };

    assert!(matches!(
        error,
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial)
            if denial.family() == ForgeQueryRuntimeFacadeFamily::Intent
                && denial.reason().contains("intent authority adapter")
    ));
}

#[test]
fn preview_effect_policy_bindings_distinguish_delivery_and_write_intent() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.preview-policy", task_live_request(), task_schema())
        .expect("live should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.preview-policy",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "ui.preview",
        ))
        .expect("delivery effect should declare");
    let intent_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "intent.preview-policy",
            ForgeQueryEffectTrigger::live_view(&live, ["title"]),
            "preview-intent",
        ))
        .expect("write-intent effect should declare");

    let muted = {
        let mut preview = runtime.preview_with_options(
            "muted effect",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::Muted),
        );
        preview
            .use_effect(&delivery_effect)
            .expect("muted policy should bind but not activate")
    };
    assert_eq!(
        muted.effect_disposition(),
        Some(ForgeQueryPreviewEffectBindingDisposition::Muted)
    );
    assert!(!muted.effect_delivery_admitted());

    let redirected = {
        let mut preview = runtime.preview_with_options(
            "redirected effect",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::Redirected),
        );
        preview
            .use_effect(&delivery_effect)
            .expect("redirected policy should admit preview delivery")
    };
    assert_eq!(
        redirected.effect_disposition(),
        Some(ForgeQueryPreviewEffectBindingDisposition::RedirectedDelivery)
    );
    assert!(redirected.effect_delivery_admitted());

    let sandboxed = {
        let mut preview = runtime.preview_with_options(
            "sandboxed effect",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        preview
            .use_effect(&intent_effect)
            .expect("sandboxed policy should admit preview write intent")
    };
    assert_eq!(
        sandboxed.effect_disposition(),
        Some(ForgeQueryPreviewEffectBindingDisposition::SandboxedWriteIntent)
    );
    assert!(sandboxed.pending_write_intent_admitted());

    let denied = {
        let mut preview = runtime.preview_with_options(
            "sandboxed delivery denial",
            ForgeQueryPreviewOptions::derive_only()
                .with_effect_policy(ForgeQueryEffectPolicy::SandboxedWriteIntent),
        );
        preview
            .use_effect(&delivery_effect)
            .expect_err("sandboxed write intent policy should not admit delivery effects")
    };
    assert!(matches!(
        denied,
        ForgeQueryRuntimeError::EffectPolicyDenied(_)
    ));
}

#[test]
fn derived_view_receives_narrow_or_fallback_patch_notes() {
    let mut runtime = task_runtime();
    let _: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    runtime
        .declare_derived_view(
            ForgeQueryDerivedView::new("task_titles", ["title".to_string()])
                .whole_refresh_fallback(),
        )
        .expect("derived view should declare");
    let insert = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Derived task" },
            }),
        })
        .expect("insert should route to derived view");
    let update = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: insert.deltas()[0].entity_identity.clone(),
            aspect_path: "title.value".to_string(),
            value: Value::String("Derived task renamed".to_string()),
        })
        .expect("title update should route to derived view");

    let patches = runtime.drain_derived_patches("task_titles");

    assert_eq!(
        update.affected_derived_view_ids(),
        &["task_titles".to_string()]
    );
    assert!(update.refresh_fallback());
    assert!(patches
        .derived_patch_notes
        .iter()
        .any(|note| note.starts_with("whole-refresh-fallback")));
}

#[test]
fn maintained_derived_view_materializes_incremental_patches() {
    let mut runtime = task_runtime();
    let _: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect("maintained derived view should declare");

    let insert = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "First title" },
            }),
        })
        .expect("insert should route derived patch");
    let patches = runtime.drain_derived_patches(titles.name());

    assert_eq!(
        insert.affected_derived_view_ids(),
        &["task_titles".to_string()]
    );
    let expected_row = Value::String(insert.deltas()[0].entity_identity.clone());
    assert_eq!(runtime.read_derived(&titles), vec![expected_row.clone()]);
    assert_eq!(patches.derived_patches.len(), 1);
    assert_eq!(patches.derived_patches[0].payload(), &expected_row);

    runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: insert.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("ignored".to_string()),
        })
        .expect("irrelevant update should not route derived patch");
    let irrelevant = runtime.drain_derived_patches(titles.name());

    assert!(irrelevant.derived_patches.is_empty());
}

#[test]
fn nested_computed_views_route_in_deterministic_dependency_order() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.titles", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("source computed view should declare");
    let summary = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.summary", ["title.summary".to_string()])
                .depends_on_derived(&titles)
                .produces(["validation.state".to_string()]),
            SummaryMaintainer,
        )
        .expect("nested computed view should declare");

    let insert = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Nested title" },
            }),
        })
        .expect("insert should update nested computeds");
    let title_patches = runtime.drain_derived_patches(titles.name());
    let summary_patches = runtime.drain_derived_patches(summary.name());

    assert_eq!(
        insert.affected_derived_view_ids(),
        &[
            "computed.summary".to_string(),
            "computed.titles".to_string()
        ]
    );
    assert_eq!(insert.considered_computed_view_count(), 2);
    assert_eq!(title_patches.derived_patches.len(), 1);
    assert_eq!(
        title_patches.derived_patches[0].aspect_paths(),
        &["title.summary".to_string()]
    );
    assert_eq!(summary_patches.derived_patches.len(), 1);
    assert_eq!(
        summary_patches.derived_patches[0].aspect_paths(),
        &["validation.state".to_string()]
    );
    assert_eq!(
        runtime.read_derived(&summary),
        vec![Value::String(format!(
            "summary:{}",
            insert.deltas()[0].entity_identity
        ))]
    );

    runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: insert.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("ignored".to_string()),
        })
        .expect("irrelevant update should still write");
    assert!(runtime
        .drain_derived_patches(titles.name())
        .derived_patches
        .is_empty());
    assert!(runtime
        .drain_derived_patches(summary.name())
        .derived_patches
        .is_empty());
}

#[test]
fn computed_dependency_index_replaces_redeclared_view_membership() {
    let mut runtime = task_issue_memory_runtime();
    let task_live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("task live should declare");
    let issue_live = runtime
        .declare_live_view::<Value>("issues.table", issue_live_request(), issue_schema())
        .expect("issue live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.shared", ["title".to_string()])
                .depends_on_live(&task_live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("task-backed computed should declare");

    runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.shared", ["summary".to_string()])
                .depends_on_live(&issue_live)
                .produces(["issue.summary".to_string()]),
            SummaryMaintainer,
        )
        .expect("redeclared computed should replace old dependency index membership");

    let task_write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Task should not wake redeclared computed" },
            }),
        })
        .expect("task write should execute");
    assert!(task_write.affected_derived_view_ids().is_empty());
    assert_eq!(task_write.considered_computed_view_count(), 0);
    assert!(runtime
        .drain_derived_patches(computed.name())
        .derived_patches
        .is_empty());

    let issue_write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Issue".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "summary": { "value": "Issue wakes computed" },
            }),
        })
        .expect("issue write should execute");
    let issue_patches = runtime.drain_derived_patches(computed.name());

    assert_eq!(
        issue_write.affected_derived_view_ids(),
        &["computed.shared".to_string()]
    );
    assert_eq!(issue_write.considered_computed_view_count(), 1);
    assert_eq!(issue_patches.derived_patches.len(), 1);
    assert_eq!(
        issue_patches.derived_patches[0].aspect_paths(),
        &["issue.summary".to_string()]
    );
}

#[test]
fn computed_handle_inspection_reports_dependencies_aspects_and_materialization() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.inspectable", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Inspectable task" },
            }),
        })
        .expect("write should materialize computed output");

    let evidence = runtime
        .inspect_derived_view(&computed)
        .expect("computed handle should inspect");

    assert_eq!(evidence.name(), "computed.inspectable");
    assert_eq!(
        evidence.authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
    assert_eq!(evidence.upstream_live_views(), &["tasks.table".to_string()]);
    assert!(evidence.upstream_derived_views().is_empty());
    assert_eq!(evidence.dependency_aspects(), &["title".to_string()]);
    assert_eq!(evidence.produced_aspects(), &["title.summary".to_string()]);
    assert_eq!(evidence.materialized_row_count(), 1);
    assert_eq!(evidence.pending_patch_count(), 1);

    let foreign_runtime = task_runtime();
    let error = foreign_runtime
        .inspect_derived_view(&computed)
        .expect_err("foreign computed handle should not inspect in another runtime");
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingDerivedView(_)
    ));
}

#[test]
fn effect_delivery_routes_from_live_trigger_with_expression_metadata() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.title-badges",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.badges",
            )
            .with_condition(ForgeQueryEffectCondition::expression(
                "expr.title.badge",
                ["title"],
                ["ui.badge"],
            )),
        )
        .expect("effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Effect task" },
            }),
        })
        .expect("write should route effect");
    let evidence = runtime
        .inspect_effect(&effect)
        .expect("effect should inspect before drain");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("effect deliveries should drain");

    assert_eq!(write.considered_effect_count(), 1);
    assert_eq!(write.delivered_effect_count(), 1);
    assert_eq!(write.suppressed_effect_count(), 0);
    assert_eq!(write.effect_expression_failure_count(), 0);
    assert_eq!(evidence.name(), "ui.title-badges");
    assert_eq!(evidence.trigger_source(), "tasks.table");
    assert_eq!(
        evidence.trigger_source_kind(),
        ForgeQueryEffectTriggerSourceKind::LiveView
    );
    assert_eq!(evidence.condition_descriptor(), "expr.title.badge");
    assert_eq!(evidence.condition_inputs(), &["title".to_string()]);
    assert_eq!(evidence.condition_outputs(), &["ui.badge".to_string()]);
    assert_eq!(evidence.pending_delivery_count(), 1);
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].family(),
        &ForgeQueryEffectDeliveryFamily::Delivered
    );
    assert_eq!(deliveries[0].target(), "ui.badges");
    assert_eq!(
        deliveries[0].authority_lane(),
        ForgeQueryAuthorityLane::EffectDeliveryState
    );
    assert_eq!(deliveries[0].aspect_paths(), &["title".to_string()]);
    assert_eq!(deliveries[0].payload()["condition"], "expr.title.badge");
}

#[test]
fn effect_delivery_routes_from_computed_trigger_after_computed_patch() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.titles.effect", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.summary-badges",
            ForgeQueryEffectTrigger::computed_view(&titles, ["title.summary"]),
            "ui.summary",
        ))
        .expect("computed-triggered effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Computed effect task" },
            }),
        })
        .expect("write should route computed effect");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("effect deliveries should drain");

    assert_eq!(write.considered_computed_view_count(), 1);
    assert_eq!(write.considered_effect_count(), 1);
    assert_eq!(write.delivered_effect_count(), 1);
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].trigger_source_kind(),
        ForgeQueryEffectTriggerSourceKind::ComputedView
    );
    assert_eq!(deliveries[0].trigger_source(), "computed.titles.effect");
    assert_eq!(deliveries[0].aspect_paths(), &["title.summary".to_string()]);
    assert_eq!(
        runtime.read_derived(&titles),
        vec![Value::String(write.deltas()[0].entity_identity.clone())]
    );
}

#[test]
fn computed_effect_does_not_replay_stale_undrained_computed_patch() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.titles.stale-effect", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.stale-summary-badges",
            ForgeQueryEffectTrigger::computed_view(&titles, ["title.summary"]),
            "ui.summary",
        ))
        .expect("computed-triggered effect should declare");

    runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "First effect task" },
            }),
        })
        .expect("first write should route computed effect");
    let first_deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("first effect deliveries should drain");
    assert_eq!(first_deliveries.len(), 1);

    let unrelated = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: runtime.read_derived(&titles)[0]
                .as_str()
                .expect("computed row should be an entity id")
                .to_string(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("irrelevant".to_string()),
        })
        .expect("irrelevant write should not replay stale computed patch");
    let stale_deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("stale effect deliveries should drain");

    assert_eq!(unrelated.considered_computed_view_count(), 1);
    assert!(unrelated.affected_derived_view_ids().is_empty());
    assert_eq!(unrelated.considered_effect_count(), 0);
    assert!(stale_deliveries.is_empty());
}

#[test]
fn effect_expression_suppression_and_failure_are_typed_and_counted() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let suppressed_effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.suppressed",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.suppressed",
            )
            .with_condition(ForgeQueryEffectCondition::expression(
                "expr.needs-validation",
                ["validation.state"],
                ["ui.badge"],
            )),
        )
        .expect("suppressed effect should declare");
    let failing_effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.failing",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.failing",
            )
            .with_condition(ForgeQueryEffectCondition::failing_expression(
                "expr.fail.validation",
                ["title"],
                ["ui.badge"],
            )),
        )
        .expect("failing effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Conditional task" },
            }),
        })
        .expect("write should route effects");
    let suppressed_evidence = runtime
        .inspect_effect(&suppressed_effect)
        .expect("suppressed terminal artifact should inspect before drain");
    let suppressed = runtime
        .drain_effect_deliveries(&suppressed_effect)
        .expect("suppressed effect should drain");
    let failed = runtime
        .drain_effect_deliveries(&failing_effect)
        .expect("failing effect should drain");

    assert_eq!(write.considered_effect_count(), 2);
    assert_eq!(write.delivered_effect_count(), 0);
    assert_eq!(write.suppressed_effect_count(), 1);
    assert_eq!(write.effect_expression_failure_count(), 1);
    assert_eq!(
        suppressed[0].family(),
        &ForgeQueryEffectDeliveryFamily::Suppressed
    );
    assert_eq!(suppressed_evidence.pending_delivery_count(), 1);
    assert!(suppressed[0]
        .reason()
        .expect("suppression reason should exist")
        .contains("inputs were not changed"));
    assert_eq!(
        failed[0].family(),
        &ForgeQueryEffectDeliveryFamily::ExpressionFailed
    );
    assert!(failed[0]
        .reason()
        .expect("failure reason should exist")
        .contains("deterministic failure"));
}

#[test]
fn meaningful_change_suppression_counts_semantic_delta_suppression() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::deliver(
                "ui.meaningful-title",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "ui.badges",
            )
            .with_meaningful_change_suppression(),
        )
        .expect("meaningful effect should declare");

    let inserted = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Meaningful task" },
            }),
        })
        .expect("insert should deliver because whole-row delta is meaningful");
    assert_eq!(inserted.delivered_effect_count(), 1);
    assert_eq!(inserted.meaningful_effect_suppression_count(), 0);
    assert_eq!(
        runtime
            .drain_effect_deliveries(&effect)
            .expect("insert delivery should drain")
            .len(),
        1
    );

    let churn = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: inserted.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("semantic-churn".to_string()),
        })
        .expect("irrelevant aspect update should be suppressed as churn");
    let evidence = runtime
        .inspect_effect(&effect)
        .expect("meaningful effect should inspect");
    let suppressed = runtime
        .drain_effect_deliveries(&effect)
        .expect("suppressed effect should drain");

    assert_eq!(churn.considered_effect_count(), 1);
    assert_eq!(churn.delivered_effect_count(), 0);
    assert_eq!(churn.suppressed_effect_count(), 1);
    assert_eq!(churn.meaningful_effect_suppression_count(), 1);
    assert_eq!(
        evidence.suppression_policy(),
        ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
    );
    assert_eq!(evidence.counters().meaningful_suppressions(), 1);
    assert_eq!(suppressed.len(), 1);
    assert_eq!(
        suppressed[0].family(),
        &ForgeQueryEffectDeliveryFamily::Suppressed
    );
    assert_eq!(
        suppressed[0].suppression_policy(),
        ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
    );
    assert!(suppressed[0]
        .reason()
        .expect("meaningful suppression should explain itself")
        .contains("meaningful semantic delta suppression"));
}

#[test]
fn effect_declaration_rejects_missing_triggers_before_registration() {
    let mut runtime = task_runtime();
    let missing = ForgeQueryEffectDeclaration::deliver(
        "ui.missing",
        ForgeQueryEffectTrigger::live_view_name("tasks.missing", ["title"]),
        "ui.badges",
    );
    let error = runtime
        .declare_effect::<Value>(missing)
        .expect_err("missing live trigger should reject");

    match error {
        ForgeQueryRuntimeError::EffectDeclaration {
            effect_name,
            stage,
            message,
        } => {
            assert_eq!(effect_name, "ui.missing");
            assert_eq!(stage, "trigger-admission");
            assert!(message.contains("tasks.missing"));
        }
        other => panic!("expected effect declaration denial, got {other:?}"),
    }
}

#[test]
fn effect_declaration_rejects_truth_delivery_without_intent_boundary() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let declaration = ForgeQueryEffectDeclaration::deliver(
        "ui.truth-smuggle",
        ForgeQueryEffectTrigger::live_view(&live, ["title"]),
        "Task",
    )
    .with_target_lane(ForgeQueryAuthorityLane::AuthoritativeTruth);

    let error = runtime
        .declare_effect::<Value>(declaration)
        .expect_err("effect delivery must not target truth");

    match error {
        ForgeQueryRuntimeError::EffectDeclaration { stage, message, .. } => {
            assert_eq!(stage, "authority-admission");
            assert!(message.contains("intent authority"));
        }
        other => panic!("expected authority admission denial, got {other:?}"),
    }
}

#[test]
fn write_intent_effect_lowers_to_pending_intent_with_phase_evidence() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::write_intent(
                "intent.reconcile-title",
                ForgeQueryEffectTrigger::live_view(&live, ["title"]),
                "reconcile-title-slug",
            )
            .with_condition(ForgeQueryEffectCondition::expression(
                "expr.title.needs-slug",
                ["title"],
                ["intent.slug"],
            )),
        )
        .expect("pending write-intent effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Intent task" },
            }),
        })
        .expect("write should route pending intent effect");
    let evidence = runtime
        .inspect_effect(&effect)
        .expect("pending intent effect should inspect");
    let deliveries = runtime
        .drain_effect_deliveries(&effect)
        .expect("pending intent work should drain through effect queue");

    assert_eq!(
        effect.authority_lane(),
        ForgeQueryAuthorityLane::PendingWriteIntent
    );
    assert_eq!(write.considered_effect_count(), 1);
    assert_eq!(write.delivered_effect_count(), 0);
    assert_eq!(write.pending_write_intent_count(), 1);
    assert_eq!(evidence.pending_delivery_count(), 0);
    assert_eq!(evidence.pending_write_intent_count(), 1);
    assert_eq!(
        evidence
            .latest_phase_evidence()
            .expect("phase evidence should exist")
            .phases(),
        &[
            ForgeQueryEffectPhase::TruthRead,
            ForgeQueryEffectPhase::Derive,
            ForgeQueryEffectPhase::PendingWriteIntent,
        ]
    );
    assert_eq!(deliveries.len(), 1);
    assert_eq!(
        deliveries[0].family(),
        &ForgeQueryEffectDeliveryFamily::PendingWriteIntent
    );
    assert_eq!(
        deliveries[0].authority_lane(),
        ForgeQueryAuthorityLane::PendingWriteIntent
    );
    assert_eq!(deliveries[0].target(), "reconcile-title-slug");
    assert_eq!(
        deliveries[0].phase_evidence().loop_prevention(),
        ForgeQueryEffectLoopPrevention::PendingIntentExecutionDeferred
    );
    assert_eq!(
        deliveries[0].phase_evidence().loop_prevention().as_str(),
        "pending-intent-execution-deferred"
    );
    assert_eq!(
        deliveries[0].phase_evidence().idempotence(),
        ForgeQueryEffectIdempotence::PendingIntentReceiptIdentity
    );
    assert!(deliveries[0]
        .reason()
        .expect("pending intent explanation should exist")
        .contains("pending write intent"));
}

#[test]
fn write_intent_effect_rejects_authoritative_truth_target() {
    let mut runtime = task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let declaration = ForgeQueryEffectDeclaration::write_intent(
        "intent.truth-smuggle",
        ForgeQueryEffectTrigger::live_view(&live, ["title"]),
        "direct-truth-write",
    )
    .with_effect_policy(ForgeQueryEffectPolicy::AuthoritativeAllowed)
    .with_target_lane(ForgeQueryAuthorityLane::AuthoritativeTruth);

    let error = runtime
        .declare_effect::<Value>(declaration)
        .expect_err("write intent cannot target truth directly");

    match error {
        ForgeQueryRuntimeError::EffectDeclaration { stage, message, .. } => {
            assert_eq!(stage, "write-intent-admission");
            assert!(message.contains("pending write intent authority"));
        }
        other => panic!("expected write intent admission denial, got {other:?}"),
    }
}

#[test]
fn computed_dependency_admission_rejects_missing_or_cyclic_upstream_views() {
    let mut runtime = task_runtime();
    let missing_live = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.missing-live", ["title".to_string()])
                .depends_on_live_name("tasks.not-declared"),
            TitleListMaintainer,
        )
        .expect_err("missing live dependency should reject before registration");
    match missing_live {
        ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
            assert!(message.contains("tasks.not-declared"));
        }
        other => panic!("expected computed declaration error, got {other:?}"),
    }

    let missing = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.missing", ["title.summary".to_string()])
                .depends_on_derived_name("computed.unknown"),
            SummaryMaintainer,
        )
        .expect_err("missing computed dependency should reject before registration");
    match missing {
        ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
            assert!(message.contains("computed.unknown"));
        }
        other => panic!("expected computed declaration error, got {other:?}"),
    }

    let first = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.first", ["title".to_string()])
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("first computed should declare");
    let second = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.second", ["title.summary".to_string()])
                .depends_on_derived(&first)
                .produces(["validation.state".to_string()]),
            SummaryMaintainer,
        )
        .expect("second computed should declare");

    let cycle = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.first", ["validation.state".to_string()])
                .depends_on_derived(&second),
            SummaryMaintainer,
        )
        .expect_err("redeclared computed dependency should not create a cycle");
    match cycle {
        ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
            assert!(message.contains("cycle"));
        }
        other => panic!("expected computed cycle declaration error, got {other:?}"),
    }
}

struct FakeDsl;

struct FakeSchemaAdapter;

struct TitleListMaintainer;
struct SummaryMaintainer;

impl ForgeQueryDerivedViewMaintainer for TitleListMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let row = Value::String(delta.entity_identity.clone());
        materialization.push_row(row.clone());
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "derived-test-commit",
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            row,
        )
    }
}

impl ForgeQueryDerivedViewMaintainer for SummaryMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let row = Value::String(format!("summary:{}", delta.entity_identity));
        materialization.replace_rows([row.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "derived-summary-commit",
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            row,
        )
    }
}

impl ForgeQuerySchemaAdapter for FakeSchemaAdapter {
    fn schema_view(&self, operation_id: &str) -> Option<QuerySchemaView> {
        (operation_id == "create_task").then(task_schema)
    }
}

impl ForgeQueryProgramSource for FakeDsl {
    fn compile_program<A>(
        self,
        schema_adapter: &A,
    ) -> Result<ForgeQueryProgram, ForgeQueryProgramError>
    where
        A: ForgeQuerySchemaAdapter + ?Sized,
    {
        let schema_view = schema_adapter
            .schema_view("create_task")
            .ok_or_else(|| ForgeQueryProgramError::new("missing schema for create_task"))?;
        ForgeQueryProgram::new(
            "fake.strict.dsl",
            [ForgeQueryOperation::new("create_task")
                .with_input(ForgeQueryTypedPort::new(
                    "title",
                    ForgeQueryPortType::String,
                ))
                .requires(ForgeQueryAuthorityRequirement::Live)
                .requires(ForgeQueryAuthorityRequirement::Writeback)
                .with_effect(ForgeQueryProgramEffect::DeclareLiveView {
                    name: "tasks.table".to_string(),
                    request: task_live_request(),
                    schema_view,
                })
                .with_effect(ForgeQueryProgramEffect::WriteTemplate(
                    ForgeQueryWriteCommandTemplate::Insert {
                        collection: "Task".to_string(),
                        payload: ForgeQueryValueExpr::object([
                            (
                                "identity".to_string(),
                                ForgeQueryValueExpr::object([(
                                    "id".to_string(),
                                    ForgeQueryValueExpr::literal(Value::String(String::new())),
                                )]),
                            ),
                            (
                                "title".to_string(),
                                ForgeQueryValueExpr::object([(
                                    "value".to_string(),
                                    ForgeQueryValueExpr::input("title"),
                                )]),
                            ),
                        ]),
                    },
                ))
                .with_effect(ForgeQueryProgramEffect::ReadLive {
                    view_name: "tasks.table".to_string(),
                })
                .with_effect(ForgeQueryProgramEffect::DrainPatches {
                    view_name: "tasks.table".to_string(),
                })],
        )
    }
}

impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

struct TestSchemaAdapter;

struct DenyingSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for DenyingSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "schema admission denied by test adapter",
        ))
    }
}

#[derive(Default)]
struct TestSourceAdapter {
    live_views: BTreeMap<String, String>,
    fail_declare: bool,
}

impl TestSourceAdapter {
    fn fail_declare() -> Self {
        Self {
            live_views: BTreeMap::new(),
            fail_declare: true,
        }
    }
}

impl ForgeQueryRuntimeSourceAdapter for TestSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        if self.fail_declare {
            return Err(ForgeQueryWorkspaceError::new(
                "source declaration denied by test adapter",
            ));
        }
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn snapshot_token(&self) -> String {
        "external-snapshot".to_string()
    }
}

#[derive(Default)]
struct DriftingSnapshotSourceAdapter {
    snapshot_sequence: std::cell::Cell<u64>,
}

impl ForgeQueryRuntimeSourceAdapter for DriftingSnapshotSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        let next = self.snapshot_sequence.get() + 1;
        self.snapshot_sequence.set(next);
        format!("drifting-snapshot-{next}")
    }
}

struct TestWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TestWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let collection = match command {
            ForgeQueryWriteCommand::Insert { collection, .. } => collection,
            ForgeQueryWriteCommand::UpdateAspect { .. } => "Task".to_string(),
            ForgeQueryWriteCommand::Delete { .. } => "Task".to_string(),
        };
        Ok(ForgeQueryMutationReceipt {
            commit_identity: "external-commit-1".to_string(),
            snapshot_token: "external-snapshot-1".to_string(),
            deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                collection,
                entity_identity: "external-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: Vec::new(),
            }],
        })
    }
}

struct DenyingWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for DenyingWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        _command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "write authority denied by test",
        ))
    }
}

struct CountingWriteAuthority {
    attempted_writes: std::rc::Rc<std::cell::Cell<usize>>,
}

impl ForgeQueryRuntimeWriteAuthorityAdapter for CountingWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.attempted_writes
            .set(self.attempted_writes.get().saturating_add(1));
        let mut authority = TestWriteAuthority;
        authority.write(_bridge, _relational_runtime, command)
    }
}

struct TestIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for TestIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: "external-intent-commit-1".to_string(),
            snapshot_token: "external-intent-snapshot-1".to_string(),
            deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity: "intent-task-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["title.value".to_string()],
            }],
        };
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "test-strategy-descriptor-digest",
            declaration.input_digest(),
            crate::identity::hash_parts(&[
                "test-intent-produced-mutation".to_string(),
                mutation_receipt.commit_identity.clone(),
                mutation_receipt.snapshot_token.clone(),
            ]),
            ["test-invariant-authority"],
            mutation_receipt,
        ))
    }
}

struct TestSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TestSignalSink {
    fn route_write_receipt(
        &mut self,
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

struct TestSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "test-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "test-subscription-activation:{view_name}:{}",
            activation.activation_digest()
        ))
    }
}

struct DenyingSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DenyingSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "denying-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        _view_name: &str,
        _activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "activation denied by test adapter",
        ))
    }
}

struct TestPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for TestPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["test-preview-basis"],
        ))
    }
}

struct TestInspectorEvidence;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for TestInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "test-write-receipt",
            receipt.authority_lane(),
            ["test-inspector-evidence"],
        ))
    }
}

#[derive(Clone, Debug)]
struct TestBridgeSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for TestBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("external-snapshot"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new("entity", "aspect", "field")],
        ))
    }
}

impl SnapshotReadSource for TestBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(TestSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

struct TestSnapshotReader {
    identity: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for TestSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| SnapshotReadRecord::new(read.request_key(), Vec::new()))
                .collect(),
        ))
    }
}

struct TestBridgeSink;

impl InvalidationSink for TestBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn test_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(TestBridgeSource)
        .with_signal_sink(TestBridgeSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("external-test"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("external-test"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("test bridge should build")
}

fn bridge_runtime_with_support(profile: ForgeQueryRuntimeSupportProfile) -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(profile)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build")
}

fn task_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .in_memory_collections([ForgeQueryCollection::new(
            "Task",
            [
                crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
            ],
        )])
        .build()
        .expect("runtime should build")
}

fn task_issue_memory_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .in_memory_collections([
            ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "Issue",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new(
                        "summary.value",
                        "summary.value",
                    ),
                ],
            ),
        ])
        .build()
        .expect("runtime should build")
}

fn grouped_task_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .in_memory_collections([ForgeQueryCollection::new(
            "Task",
            [
                crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                crate::memory_workspace::ForgeQueryAspect::new("status.value", "status.value"),
            ],
        )])
        .build()
        .expect("runtime should build")
}

fn intent_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Intent,
        [ForgeQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["test-intent-authority"],
    ))
}

fn task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

fn task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
        ],
        [],
    )
}

fn issue_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Issue", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("summary", "value").delivered_as("summary"))
        .order_by(DeclarativeProjectionField::new("summary", "value"))
}

fn issue_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-issue",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("summary", "value", SchemaFieldKind::String),
        ],
        [],
    )
}

fn grouped_task_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::kanban_grouped("status"))
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .project(DeclarativeProjectionField::new("status", "value").delivered_as("status"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

fn grouped_task_table_live_request() -> DeclarativeLiveQueryRequest {
    DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
        .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
        .project(DeclarativeProjectionField::new("status", "value").delivered_as("status"))
        .order_by(DeclarativeProjectionField::new("title", "value"))
}

fn grouped_task_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-grouped-task",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [],
    )
}
