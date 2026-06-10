use super::super::support::*;
use super::consumer_support::{
    expanded_manager_schema, route_consumer_stop_class, ConsumerStopRoute,
};
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::facade::TraversalSelector;

#[test]
fn public_api_family_admission_denial_surfaces_typed_family_status_posture_and_reason() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("stop-class-consumer-public-admission")
        .expect("workspace should open");

    let error = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect_err("support-gated temporal family must fail closed for public admission");

    match error.stop_class() {
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
            reason,
        } => {
            assert_eq!(family, ForgeQueryRuntimeFacadeFamily::Temporal);
            assert_eq!(status, ForgeQueryRuntimeFamilySupportStatus::Supported);
            assert_eq!(
                teaching_posture,
                Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly)
            );
            assert!(!reason.is_empty());
        }
        other => panic!("expected family-admission stop class, got {other:?}"),
    }
}

#[test]
fn consumer_router_handles_public_runtime_stops_without_string_matching() {
    let family_error = stateful_bridge_task_runtime()
        .workspace("stop-class-consumer-family-route")
        .expect("workspace should open")
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect_err("support-gated temporal family must fail closed");

    assert_eq!(
        route_consumer_stop_class(&family_error),
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

    let read_error = {
        let mut workspace =
            bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
                "test-subscription-activation",
                "test-preview-basis",
                "test-inspector-evidence",
            ))
            .workspace("stop-class-consumer-read-invariant")
            .expect("workspace should open");
        workspace
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
            .expect_err("denied invariant packs should reject before execution")
    };

    assert_eq!(
        route_consumer_stop_class(&read_error),
        ConsumerStopRoute::ReadCompositionDomainInvariantDenied("domain_invariant_pack_hook")
    );
}

#[test]
fn typed_family_admission_matching_survives_message_rewording_while_string_probe_drifts() {
    let first_error = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                "first temporal wording",
            ),
        ),
    )
    .workspace("stop-class-consumer-reword-first")
    .expect("workspace should open")
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
    .expect_err("unsupported temporal family should fail closed");

    let second_error = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                "second temporal wording",
            ),
        ),
    )
    .workspace("stop-class-consumer-reword-second")
    .expect("workspace should open")
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
    .expect_err("unsupported temporal family should fail closed");

    assert_eq!(
        route_consumer_stop_class(&first_error),
        ConsumerStopRoute::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        }
    );
    assert_eq!(
        route_consumer_stop_class(&second_error),
        ConsumerStopRoute::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        }
    );

    assert_ne!(
        first_error.to_string(),
        second_error.to_string(),
        "presentation wording should be allowed to drift while stop-class routing stays stable"
    );
}
