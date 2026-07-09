use super::super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::facade::TraversalSelector;
use crate::runtime::async_result_state::runtime_async_checkpoint_label_identity;
use crate::schema_view::{QuerySchemaView, SchemaRelationView};

pub(in super::super) fn read_domain_invariant_denied_error() -> WorthQueryRuntimeError {
    let mut workspace =
        bridge_runtime_with_support(WorthQueryRuntimeSupportProfile::bridge_backed(
            "test-subscription-activation",
            "test-preview-basis",
            "test-inspector-evidence",
        ))
        .workspace("runtime.read-composition.stop-class-domain-invariant")
        .expect("read-backed runtime should open a workspace");

    workspace
        .define_read_family_with_invariant_pack(
            "neighbors-stop-class",
            "no_traversal_reads",
            |read| {
                read.anchored_detail(
                    "user",
                    expanded_manager_schema(),
                    |query| {
                        query
                            .project(
                                AspectFieldSelector::new("identity", "id")
                                    .expect("identity projection should build"),
                            )
                            .traverse(
                                TraversalSelector::bounded("manager", 2)
                                    .expect("bounded traversal should build"),
                            )
                    },
                    |shape| {
                        shape.field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                    },
                )
            },
            |context| {
                let summary = context.read_domain_invariant_summary();
                if summary.declared_traversal_clause_count() > 0 {
                    Err(WorthQueryReadInvariantPackViolation::new(
                        "no_traversal_reads",
                        "this domain hook denies traversal-bearing reads",
                    ))
                } else {
                    Ok(())
                }
            },
        )
        .expect_err("denied invariant packs should reject before execution")
}

pub(in super::super) fn intent_commit_denied_error() -> WorthQueryRuntimeError {
    let attempted = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(CountingIntentAuthority {
            attempted: attempted.clone(),
        })
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent runtime should build");

    let mut branch = runtime
        .branch(test_session_label("derive-only branch intent"))
        .expect("branch should admit");
    branch
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "branch-denied",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1")]),
        ))
        .expect_err("derive-only branch must deny write intents")
}

pub(in super::super) fn intent_execution_routing_failed_error() -> WorthQueryRuntimeError {
    let mut runtime = bridge_runtime_with_support_and_intent_authority(
        intent_support_profile(),
        TestIntentAuthority,
    );
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "phase-three-routing-stop-class",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        test_intent_input([("entity", "task-1")]),
    );
    let handoff = runtime
        .admit_authoritative_intent_for_execution(declaration.clone())
        .expect("authoritative handoff should admit");
    let binding = runtime.prepare_authoritative_intent_execution_binding(handoff.clone());
    let execution = runtime
        .backend
        .execute_intent(binding.declaration())
        .expect("backend execution should succeed");
    let admitted_handoff = WorthQueryAdmittedIntentExecutionHandoff::from(handoff);
    let snapshot_evidence_identity = execution
        .mutation_receipt()
        .snapshot_identity
        .evidence_identity();
    let execution_provenance =
        WorthQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution.outcome_digest(),
            &snapshot_evidence_identity,
        );
    let decision_trace_envelope = WorthQueryIntentDecisionTraceEnvelope::for_admitted_execution(
        &admitted_handoff,
        &execution,
    );

    runtime.intent_execution_routing_error(
        &declaration,
        &execution,
        execution_provenance,
        decision_trace_envelope,
        WorthQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: "tasks.phase-three-routing-stop-class".to_string(),
            stage: "delivery-window",
            message: "simulated route failure".to_string(),
        },
    )
}

pub(in super::super) fn preview_promotion_write_failed_error() -> WorthQueryRuntimeError {
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(DenyingWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("denying write backend should build");

    let mut preview = runtime
        .preview(test_session_label("stop-class-write-failure"))
        .expect("preview should admit");
    preview
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("denied-preview")),
                (
                    "title.value",
                    test_string_aspect_value("Denied preview write"),
                ),
            ],
        ))
        .expect("preview write should stage");
    preview.promote().expect_err("promotion should fail")
}

pub(in super::super) fn preview_promotion_stale_basis_error() -> WorthQueryRuntimeError {
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(DriftingSnapshotIdentityAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("drifting backend should build");
    let mut preview = runtime
        .preview(test_session_label("stop-class stale basis"))
        .expect("preview session should admit");
    preview
        .write(insert_command(
            "Task",
            [
                (
                    "identity.id",
                    test_string_aspect_value("stale-preview-stop-class"),
                ),
                (
                    "title.value",
                    test_string_aspect_value("Should not promote"),
                ),
            ],
        ))
        .expect("preview write should stage");
    preview
        .promote()
        .expect_err("drifting basis should deny promotion")
}

pub(in super::super) fn preview_promotion_atomic_batch_unsupported_error() -> WorthQueryRuntimeError
{
    let attempted_writes = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(CountingWriteAuthority {
            attempted_writes: attempted_writes.clone(),
        })
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("counting backend should build");
    let mut preview = runtime
        .preview(test_session_label("stop-class atomic batch"))
        .expect("preview session should admit");
    preview
        .write(insert_command(
            "Task",
            [
                (
                    "identity.id",
                    test_string_aspect_value("preview-batch-stop-class-1"),
                ),
                (
                    "title.value",
                    test_string_aspect_value("First staged write"),
                ),
            ],
        ))
        .expect("first preview write should stage");
    preview
        .write(insert_command(
            "Task",
            [
                (
                    "identity.id",
                    test_string_aspect_value("preview-batch-stop-class-2"),
                ),
                (
                    "title.value",
                    test_string_aspect_value("Second staged write"),
                ),
            ],
        ))
        .expect("second preview write should stage");
    let error = preview
        .promote()
        .expect_err("multi-write promotion should deny before authority");
    assert_eq!(attempted_writes.get(), 0);
    error
}

pub(in super::super) fn preview_promotion_rebinding_required_error() -> WorthQueryRuntimeError {
    let mut runtime = stateful_bridge_task_runtime();
    let view: WorthQueryLiveView<WorthQueryNativeRow> = runtime
        .declare_live_view(
            "tasks.preview-promotion-stop-class-mismatch",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    let (_, generation_digest) = live_subscription_async_identity(&runtime, view.name());
    runtime
        .project_async_result_state(
            view.name(),
            &WorthQueryRuntimeAsyncResultProjection::completion_state(
                BridgeAsyncCompletionState::Denied(
                    BridgeAsyncCompletionDenialClass::SignalLifecycleDenied,
                ),
                "async:preview-stop-class-mismatch",
            ),
            &runtime_async_checkpoint_label_identity("basis:drifted"),
            &generation_digest,
        )
        .expect("preview mismatch should remain typed");
    let mut preview = runtime
        .preview_with_options(
            test_session_label("preview promotion stop class mismatch"),
            WorthQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should admit");
    preview.use_view(&view);
    preview
        .write(insert_command(
            "Task",
            [
                (
                    "identity.id",
                    test_string_aspect_value("preview-promotion-stop-class-mismatch"),
                ),
                (
                    "title.value",
                    test_string_aspect_value("Should require rebinding"),
                ),
            ],
        ))
        .expect("preview write should stage");
    preview
        .promote()
        .expect_err("crossed residue should require rebinding")
}

fn expanded_manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition-expanded",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [SchemaRelationView::new(
            crate::authoring::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            2,
        )],
    )
}
