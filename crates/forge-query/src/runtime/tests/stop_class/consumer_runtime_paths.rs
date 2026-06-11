use super::super::support::*;
use super::consumer_support::{
    expanded_manager_schema, route_consumer_stop_class, ConsumerStopRoute,
};
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::facade::TraversalSelector;

#[test]
fn consumer_router_handles_runtime_generated_stop_classes_without_string_matching() {
    let public_family_error = stateful_bridge_task_runtime()
        .workspace("consumer-stop-class-public-family")
        .expect("workspace should open")
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect_err("support-gated temporal family must fail closed");
    assert_eq!(
        route_consumer_stop_class(&public_family_error),
        ConsumerStopRoute::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        }
    );

    let intent_error = {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence)
            .intent_authority(CountingIntentAuthority {
                attempted: std::rc::Rc::new(std::cell::Cell::new(0)),
            })
            .support_profile(intent_support_profile())
            .build_backend_from_parts()
            .build()
            .expect("intent runtime should build");
        let mut branch = runtime
            .branch(test_session_label("consumer-router-intent"))
            .expect("branch");
        branch
            .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
                "branch-denied",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                json!({ "entity": "task-1" }),
            ))
            .expect_err("derive-only branch must deny write intents")
    };
    assert_eq!(
        route_consumer_stop_class(&intent_error),
        ConsumerStopRoute::IntentCommitDenied
    );

    let preview_error = {
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
            .expect("preview runtime should build");
        let mut preview = runtime
            .preview(test_session_label("consumer-router-preview"))
            .expect("preview");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("consumer-preview")),
                    ("title.value", json!("Denied preview write")),
                ],
            ))
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("promotion should fail with a typed denial")
    };
    assert_eq!(
        route_consumer_stop_class(&preview_error),
        ConsumerStopRoute::PreviewPromotionDenied(
            ForgeQueryPreviewPromotionDenialKind::WriteFailed,
        )
    );

    let read_error = bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ))
    .workspace("consumer-stop-class-read-domain-invariant")
    .expect("workspace should open")
    .define_read_family_with_invariant_pack(
        "neighbors-consumer",
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
                Err(ForgeQueryReadInvariantPackViolation::new(
                    "no_traversal_reads",
                    "this domain hook denies traversal-bearing reads",
                ))
            } else {
                Ok(())
            }
        },
    )
    .expect_err("denied invariant packs should reject before execution");
    assert_eq!(
        route_consumer_stop_class(&read_error),
        ConsumerStopRoute::ReadCompositionDomainInvariantDenied("domain_invariant_pack_hook")
    );

    let routing_error = {
        let mut runtime = bridge_runtime_with_support_and_intent_authority(
            intent_support_profile(),
            TestIntentAuthority,
        );
        let declaration = ForgeQueryIntentDeclaration::strategy_commit(
            "phase-three-routing-stop-class",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1"}),
        );
        let handoff = runtime
            .admit_authoritative_intent_for_execution(declaration.clone())
            .expect("authoritative handoff should admit");
        let binding = runtime.prepare_authoritative_intent_execution_binding(handoff.clone());
        let execution = runtime
            .backend
            .execute_intent(binding.declaration())
            .expect("backend execution should succeed");
        let admitted_handoff = ForgeQueryAdmittedIntentExecutionHandoff::from(handoff);
        let execution_provenance = ForgeQueryIntentExecutionProvenance::for_authoritative_binding(
            &binding,
            execution.outcome_digest(),
            execution.mutation_receipt().snapshot_token.as_str(),
        );
        let decision_trace_envelope = ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution(
            &admitted_handoff,
            &execution,
        );
        runtime.intent_execution_routing_error(
            &declaration,
            &execution,
            execution_provenance,
            decision_trace_envelope,
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: "tasks.phase-three-routing-stop-class".to_string(),
                stage: "delivery-window",
                message: "simulated route failure".to_string(),
            },
        )
    };
    assert_eq!(
        route_consumer_stop_class(&routing_error),
        ConsumerStopRoute::IntentExecutionRoutingFailed(
            ForgeQueryRuntimeDeclarationFailureKind::LiveSubscriptionInstallation,
        )
    );
}
