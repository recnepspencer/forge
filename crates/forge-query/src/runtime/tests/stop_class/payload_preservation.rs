use super::super::support::*;

#[test]
fn unsupported_facade_family_stop_class_preserves_denied_family_and_reason() {
    let denial = ForgeQueryRuntimeSupportDenial::new(
        ForgeQueryRuntimeFacadeFamily::Temporal,
        ForgeQueryRuntimeFamilySupportStatus::Unsupported,
        Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        "temporal lane is support-gated",
    );
    let error = ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial);

    match error.stop_class() {
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
            reason,
        } => {
            assert_eq!(family, ForgeQueryRuntimeFacadeFamily::Temporal);
            assert_eq!(status, ForgeQueryRuntimeFamilySupportStatus::Unsupported);
            assert_eq!(
                teaching_posture,
                Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly)
            );
            assert_eq!(reason, "temporal lane is support-gated");
        }
        other => panic!("expected family admission denial stop class, got {other:?}"),
    }
}

#[test]
fn graph_domain_invariant_stop_class_preserves_hook_and_invariant_families() {
    let graph_domain_denial = ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
        "graph.family",
        "first graph domain invariant wording",
        ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
            vec!["Task".to_string()],
            vec!["task_symbol".to_string()],
            vec!["same_batch_entity_relation_identity_edges".to_string()],
            vec!["mixed_existing_target_followup_mutation".to_string()],
            graph_domain_fixture_digest("program"),
            graph_domain_fixture_digest("breadth"),
            "components=1".to_string(),
        ),
    );
    let reworded = ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
        "graph.family",
        "second graph domain invariant wording",
        graph_domain_denial.domain_invariant_summary().clone(),
    );
    let first_digest = graph_domain_denial.denial_digest().to_string();
    let second_digest = reworded.denial_digest().to_string();
    let first_error =
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(graph_domain_denial);
    let second_error = ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(reworded);

    for error in [&first_error, &second_error] {
        match error.stop_class() {
            ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { denial } => {
                assert_eq!(denial.hook_family(), "domain_invariant_pack_hook");
                assert_eq!(denial.invariant_family(), "graph.family");
            }
            other => panic!("expected graph domain invariant stop class, got {other:?}"),
        }
    }
    assert_ne!(first_error.to_string(), second_error.to_string());
    assert_eq!(
        first_digest, second_digest,
        "graph domain invariant denial digest must not change when only message text changes"
    );
}

fn graph_domain_fixture_digest(
    role: &'static str,
) -> crate::evidence_identity::ForgeQueryEvidenceIdentity {
    crate::evidence_identity::forge_query_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("role"),
        "payload-graph-domain-fixture",
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture"),
        role,
    )
    .seal()
}

#[test]
fn preview_promotion_stop_class_preserves_kind_and_evidence() {
    let mut runtime = ForgeQueryRuntime::builder()
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

    let error = {
        let mut preview = runtime
            .preview(test_session_label("stop-class-write-failure"))
            .expect("preview should admit");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("denied-preview")),
                    ("title.value", json!("Denied preview write")),
                ],
            ))
            .expect("preview write should stage");
        preview.promote().expect_err("promotion should fail")
    };

    match error.stop_class() {
        ForgeQueryStopClass::PreviewPromotionDenied { kind, evidence } => {
            assert_eq!(kind, ForgeQueryPreviewPromotionDenialKind::WriteFailed);
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::WriteFailed
            );
            assert_eq!(evidence.failed_write_sequence(), Some(1));
        }
        other => panic!("expected preview promotion stop class, got {other:?}"),
    }
}

#[test]
fn preview_promotion_stop_class_preserves_all_denial_kinds() {
    let stale_basis_error = {
        let mut runtime = ForgeQueryRuntime::builder()
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
                    ("identity.id", json!("stale-preview-stop-class")),
                    ("title.value", json!("Should not promote")),
                ],
            ))
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("drifting basis should deny promotion")
    };

    match stale_basis_error.stop_class() {
        ForgeQueryStopClass::PreviewPromotionDenied { kind, evidence } => {
            assert_eq!(kind, ForgeQueryPreviewPromotionDenialKind::StaleBasis);
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::StaleBasis
            );
            assert_eq!(evidence.staged_preview_write_count(), 1);
            assert_eq!(evidence.promoted_write_count(), 0);
        }
        other => panic!("expected stale-basis stop class, got {other:?}"),
    }

    let atomic_batch_error = {
        let attempted_writes = std::rc::Rc::new(std::cell::Cell::new(0));
        let mut runtime = ForgeQueryRuntime::builder()
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
                    ("identity.id", json!("preview-batch-stop-class-1")),
                    ("title.value", json!("First staged write")),
                ],
            ))
            .expect("first preview write should stage");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-batch-stop-class-2")),
                    ("title.value", json!("Second staged write")),
                ],
            ))
            .expect("second preview write should stage");
        let error = preview
            .promote()
            .expect_err("multi-write promotion should deny before authority");
        assert_eq!(attempted_writes.get(), 0);
        error
    };

    match atomic_batch_error.stop_class() {
        ForgeQueryStopClass::PreviewPromotionDenied { kind, evidence } => {
            assert_eq!(
                kind,
                ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported
            );
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported
            );
            assert_eq!(evidence.staged_preview_write_count(), 2);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_eq!(evidence.failed_write_sequence(), None);
        }
        other => panic!("expected atomic-batch stop class, got {other:?}"),
    }

    let rebinding_required_error = {
        let mut runtime = stateful_bridge_task_runtime();
        let view: ForgeQueryLiveView<Value> = runtime
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
                &ForgeQueryRuntimeAsyncResultProjection::completion_state(
                    BridgeAsyncCompletionState::Denied(
                        BridgeAsyncCompletionDenialClass::SignalLifecycleDenied,
                    ),
                    "async:preview-stop-class-mismatch",
                ),
                "basis:drifted",
                &generation_digest,
            )
            .expect("preview mismatch should remain typed");
        let mut preview = runtime
            .preview_with_options(
                test_session_label("preview promotion stop class mismatch"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should admit");
        preview.use_view(&view);
        preview
            .write(insert_command(
                "Task",
                [
                    (
                        "identity.id",
                        json!("preview-promotion-stop-class-mismatch"),
                    ),
                    ("title.value", json!("Should require rebinding")),
                ],
            ))
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("crossed residue should require rebinding")
    };

    match rebinding_required_error.stop_class() {
        ForgeQueryStopClass::PreviewPromotionDenied { kind, evidence } => {
            assert_eq!(
                kind,
                ForgeQueryPreviewPromotionDenialKind::RebindingRequired
            );
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::RebindingRequired
            );
            assert_eq!(evidence.crossed_authoritative_residue_count(), 1);
            assert_eq!(
                evidence.recovery_posture(),
                "discard_preview_and_readmit_authoritative"
            );
        }
        other => panic!("expected rebinding-required stop class, got {other:?}"),
    }
}

#[test]
fn preview_operation_effect_denial_stop_class_preserves_typed_label_identity() {
    let label = test_session_label("preview.operation.effect.denied");
    let error = ForgeQueryRuntimeError::PreviewOperationEffectDenied {
        label: label.clone(),
        stage: "effect-admission",
        message: "preview effect denied".to_string(),
    };

    match error.stop_class() {
        ForgeQueryStopClass::PreviewOperationEffectDenied {
            label: classified_label,
            stage,
            message,
        } => {
            assert_eq!(classified_label.identity_digest(), label.identity_digest());
            assert_eq!(stage, "effect-admission");
            assert_eq!(message, "preview effect denied");
        }
        other => panic!("expected preview operation effect stop class, got {other:?}"),
    }
}

#[test]
fn intent_commit_stop_class_preserves_stage_and_evidence() {
    let attempted = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
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

    let error = {
        let mut branch = runtime
            .branch(test_session_label("derive-only branch intent"))
            .expect("branch should admit");
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

    match error.stop_class() {
        ForgeQueryStopClass::IntentCommitDenied {
            intent_name,
            stage,
            evidence,
            ..
        } => {
            assert_eq!(intent_name, "branch-denied");
            assert_eq!(stage, "branch-effect-policy-admission");
            assert_eq!(evidence.intent_name(), "branch-denied");
            assert_eq!(evidence.stage(), "branch-effect-policy-admission");
        }
        other => panic!("expected intent commit stop class, got {other:?}"),
    }
}
