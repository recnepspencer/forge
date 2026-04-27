use super::support::*;

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
    assert_eq!(
        receipt.execution_kind(),
        ForgeQueryIntentExecutionKind::Mutating
    );
    assert!(!receipt.is_idempotent_noop());
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
    assert_eq!(receipt.suppressed_effect_count(), 0);
    assert_eq!(receipt.meaningful_effect_suppression_count(), 0);
    assert_eq!(receipt.effect_expression_failure_count(), 0);
    assert!(!receipt.refresh_fallback());
    assert!(!receipt.outcome_digest().is_empty());
    assert!(receipt.produced_mutation_digest().is_some());
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
fn intent_receipt_inspection_explains_strategy_lanes_and_delivery_counters() {
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
        .declare_live_view::<Value>(
            "tasks.intent-inspection",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.intent-inspection", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.intent-inspection",
            ForgeQueryEffectTrigger::computed_view(&computed, ["title.summary"]),
            "ui.intent-inspection",
        ))
        .expect("effect should declare");

    let receipt = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "inspectable-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({
                "entity": "task-1",
                "title": "Intent committed title"
            }),
        ))
        .expect("intent should execute");
    let inspection = runtime
        .inspect_intent_receipt(&receipt)
        .expect("intent receipt should inspect");

    assert_eq!(inspection.intent_name(), "inspectable-intent");
    assert_eq!(
        inspection.execution_kind(),
        ForgeQueryIntentExecutionKind::Mutating
    );
    assert_eq!(inspection.strategy_identity(), "strategy.intent.reconcile");
    assert_eq!(
        inspection.source_lane(),
        ForgeQueryIntentSourceLane::UserAuthored
    );
    assert_eq!(
        inspection.target_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(inspection.outcome_digest(), receipt.outcome_digest());
    assert_eq!(
        inspection.produced_mutation_digest(),
        receipt.produced_mutation_digest()
    );
    assert_eq!(inspection.receipt_digest(), receipt.receipt_digest());
    assert_eq!(inspection.delivery_counters().affected_live_view_count(), 1);
    assert_eq!(
        inspection.delivery_counters().affected_derived_view_count(),
        1
    );
    assert_eq!(
        inspection
            .delivery_counters()
            .considered_computed_view_count(),
        1
    );
    assert_eq!(inspection.delivery_counters().delivered_effect_count(), 1);
    assert!(!inspection.delivery_counters().counter_digest().is_empty());
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn idempotent_intent_noop_emits_receipt_without_mutation_or_signal_routing() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(CountingSignalSink {
            routed: routed.clone(),
        })
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(NoopIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<Value>("tasks.noop-intent", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.noop-intent", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.noop-intent",
            ForgeQueryEffectTrigger::computed_view(&computed, ["title.summary"]),
            "ui.noop-intent",
        ))
        .expect("effect should declare");

    let receipt = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "idempotent-title-reconcile",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({
                "entity": "task-1",
                "title": "already committed"
            }),
        ))
        .expect("idempotent intent should still mint a receipt");

    assert_eq!(
        receipt.execution_kind(),
        ForgeQueryIntentExecutionKind::IdempotentNoop
    );
    assert!(receipt.is_idempotent_noop());
    assert!(receipt.affected_live_view_ids().is_empty());
    assert!(receipt.affected_derived_view_ids().is_empty());
    assert_eq!(receipt.considered_computed_view_count(), 0);
    assert_eq!(receipt.considered_effect_count(), 0);
    assert_eq!(receipt.delivered_effect_count(), 0);
    assert_eq!(receipt.pending_write_intent_count(), 0);
    assert!(!receipt.outcome_digest().is_empty());
    assert!(receipt.produced_mutation_digest().is_none());
    assert_eq!(
        receipt.invariant_evidence(),
        ["test-invariant-authority", "idempotent-noop"]
    );
    assert!(!receipt.commit_identity().is_empty());
    assert!(!receipt.snapshot_token().is_empty());
    assert!(!receipt.receipt_digest().is_empty());
    assert_eq!(
        routed.get(),
        0,
        "no-op intent must not route signal invalidation"
    );
    assert_eq!(runtime.drain_patches(&live).query_delivery_batches.len(), 0);
    assert_eq!(runtime.read_derived(&computed).len(), 0);
    assert_eq!(
        runtime
            .drain_effect_deliveries(&delivery_effect)
            .expect("effect queue should exist")
            .len(),
        0
    );
}

#[test]
fn idempotent_intent_inspection_preserves_outcome_without_mutation_claim() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(NoopIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let receipt = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "inspectable-noop-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1", "title": "already committed" }),
        ))
        .expect("idempotent intent should execute");
    let inspection = runtime
        .inspect_intent_receipt(&receipt)
        .expect("no-op intent receipt should inspect");

    assert_eq!(
        inspection.execution_kind(),
        ForgeQueryIntentExecutionKind::IdempotentNoop
    );
    assert!(!inspection.outcome_digest().is_empty());
    assert_eq!(inspection.produced_mutation_digest(), None);
    assert_eq!(inspection.delivery_counters().affected_live_view_count(), 0);
    assert_eq!(inspection.delivery_counters().delivered_effect_count(), 0);
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn mutating_intent_with_empty_delta_denies_before_signal_routing() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(CountingSignalSink {
            routed: routed.clone(),
        })
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(EmptyMutatingIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "empty-mutating-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1" }),
        ))
        .expect_err("empty mutating execution must use the no-op constructor");

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name,
            stage,
            message,
            evidence,
        } => {
            assert_eq!(intent_name, "empty-mutating-intent");
            assert_eq!(stage, "mutation-receipt-admission");
            assert!(message.contains("idempotent-noop"));
            assert_eq!(evidence.intent_name(), "empty-mutating-intent");
            assert_eq!(evidence.stage(), "mutation-receipt-admission");
            assert_eq!(
                evidence.execution_kind(),
                Some(ForgeQueryIntentExecutionKind::Mutating)
            );
            assert!(evidence.attempt_digest().is_some());
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected mutation receipt admission denial, got {other:?}"),
    }
    assert_eq!(
        routed.get(),
        0,
        "denied empty mutation must not route signal invalidation"
    );
}

#[test]
fn invariant_denial_inspection_explains_failed_invariants_without_commit_identity() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(InvariantViolationIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "inspectable-invariant-denial",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1", "dependency": "cycle" }),
        ))
        .expect_err("invariant violation must deny");

    let evidence = match error {
        ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => evidence,
        other => panic!("expected invariant admission denial, got {other:?}"),
    };
    let inspection = runtime
        .inspect_intent_denial(&evidence)
        .expect("denial evidence should inspect");

    assert_eq!(inspection.intent_name(), "inspectable-invariant-denial");
    assert_eq!(inspection.stage(), "invariant-admission");
    assert_eq!(
        inspection.execution_kind(),
        Some(ForgeQueryIntentExecutionKind::InvariantViolation)
    );
    assert_eq!(
        inspection.returned_strategy_identity(),
        Some("strategy.intent.reconcile")
    );
    assert_eq!(
        inspection.invariant_evidence(),
        [
            "relational-invariant:constraint-a:false",
            "relational-invariant:constraint-b:false"
        ]
    );
    assert!(inspection.attempt_digest().is_some());
    assert!(inspection.snapshot_token().is_some());
    assert_eq!(inspection.denial_digest(), evidence.denial_digest());
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn invariant_violation_intent_denies_with_evidence_without_partial_publication() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(CountingSignalSink {
            routed: routed.clone(),
        })
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(InvariantViolationIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<Value>("tasks.invariant-denial", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.invariant-denial", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::deliver(
            "ui.invariant-denial",
            ForgeQueryEffectTrigger::computed_view(&computed, ["title.summary"]),
            "ui.invariant-denial",
        ))
        .expect("effect should declare");

    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "violates-relational-invariants",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1", "dependency": "cycle" }),
        ))
        .expect_err("invariant violation must not mint an intent receipt");

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name,
            stage,
            message,
            evidence,
        } => {
            assert_eq!(intent_name, "violates-relational-invariants");
            assert_eq!(stage, "invariant-admission");
            assert!(message.contains("relational invariants failed"));
            assert_eq!(evidence.intent_name(), "violates-relational-invariants");
            assert_eq!(evidence.stage(), "invariant-admission");
            assert_eq!(
                evidence.execution_kind(),
                Some(ForgeQueryIntentExecutionKind::InvariantViolation)
            );
            assert_eq!(
                evidence.invariant_evidence(),
                [
                    "relational-invariant:constraint-a:false",
                    "relational-invariant:constraint-b:false"
                ]
            );
            assert!(evidence.attempt_digest().is_some());
            assert!(evidence.snapshot_token().is_some());
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected invariant admission denial, got {other:?}"),
    }
    assert_eq!(
        routed.get(),
        0,
        "invariant-denied intent must not route signal invalidation"
    );
    assert_eq!(runtime.drain_patches(&live).query_delivery_batches.len(), 0);
    assert_eq!(runtime.read_derived(&computed).len(), 0);
    assert_eq!(
        runtime
            .drain_effect_deliveries(&delivery_effect)
            .expect("effect queue should exist")
            .len(),
        0
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
fn intent_source_lanes_that_need_policy_deny_before_authority_execution() {
    let attempted = std::rc::Rc::new(std::cell::Cell::new(0));
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
            attempted: attempted.clone(),
        })
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    for source_lane in [
        ForgeQueryIntentSourceLane::EffectTriggered,
        ForgeQueryIntentSourceLane::PreviewLocal,
        ForgeQueryIntentSourceLane::BranchLocal,
        ForgeQueryIntentSourceLane::DerivedRuntime,
    ] {
        let error = runtime
            .execute_intent(
                ForgeQueryIntentDeclaration::strategy_commit(
                    "non-user-authored",
                    "strategy.intent.reconcile",
                    "1.0",
                    "intent.reconcile.input.v1",
                    json!({ "entity": "task-1" }),
                )
                .with_source_lane(source_lane),
            )
            .expect_err("non-user-authored lanes require later explicit policy admission");

        match error {
            ForgeQueryRuntimeError::IntentCommitDenied {
                intent_name,
                stage,
                message,
                evidence,
            } => {
                assert_eq!(intent_name, "non-user-authored");
                assert_eq!(stage, "source-lane-admission");
                assert!(message.contains(source_lane.as_str()));
                assert_eq!(evidence.execution_kind(), None);
                assert_eq!(evidence.source_lane(), source_lane);
            }
            other => panic!("expected source lane intent denial, got {other:?}"),
        }
    }

    assert_eq!(
        attempted.get(),
        0,
        "source lane denial must happen before the intent authority can publish"
    );
}

#[test]
fn intent_execution_strategy_drift_denies_before_signal_routing() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(CountingSignalSink {
            routed: routed.clone(),
        })
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(DriftingIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "drifting-strategy",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1" }),
        ))
        .expect_err("strategy drift must not mint an intent receipt");

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name,
            stage,
            message,
            evidence,
        } => {
            assert_eq!(intent_name, "drifting-strategy");
            assert_eq!(stage, "strategy-admission");
            assert!(message.contains("returned strategy"));
            assert_eq!(
                evidence.execution_kind(),
                Some(ForgeQueryIntentExecutionKind::Mutating)
            );
            assert_eq!(evidence.strategy_identity(), "strategy.intent.reconcile");
            assert_eq!(
                evidence.returned_strategy_identity(),
                Some("strategy.intent.other")
            );
            assert_eq!(
                evidence.returned_strategy_descriptor_digest(),
                Some("test-strategy-descriptor-digest")
            );
        }
        other => panic!("expected backend execution denial before routing, got {other:?}"),
    }
    assert_eq!(
        routed.get(),
        0,
        "drifted intent execution must not route signal invalidations"
    );
}

#[test]
fn strategy_drift_denial_inspection_keeps_declared_and_returned_strategy_separate() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(DriftingIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "inspectable-drifting-strategy",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1" }),
        ))
        .expect_err("strategy drift must deny");

    let evidence = match error {
        ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => evidence,
        other => panic!("expected strategy drift denial, got {other:?}"),
    };
    let inspection = runtime
        .inspect_intent_denial(&evidence)
        .expect("strategy drift denial should inspect");

    assert_eq!(inspection.strategy_identity(), "strategy.intent.reconcile");
    assert_eq!(
        inspection.returned_strategy_identity(),
        Some("strategy.intent.other")
    );
    assert_eq!(
        inspection.returned_strategy_descriptor_digest(),
        Some("test-strategy-descriptor-digest")
    );
    assert_eq!(
        inspection.execution_kind(),
        Some(ForgeQueryIntentExecutionKind::Mutating)
    );
    assert_eq!(inspection.denial_digest(), evidence.denial_digest());
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn effect_triggered_pending_write_intent_executes_through_intent_authority_once() {
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
        .declare_live_view::<Value>("tasks.effect-intent", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.reconcile-title",
            ForgeQueryEffectTrigger::live_view(&live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: "task-1".to_string(),
            aspect_path: "title.value".to_string(),
            value: json!("title from write"),
        })
        .expect("write should route pending effect intent");
    assert_eq!(write.pending_write_intent_count(), 1);
    assert_eq!(
        runtime
            .inspect_effect(&effect)
            .expect("effect should inspect")
            .pending_write_intent_count(),
        1
    );

    let effect_intent = runtime
        .execute_next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .expect("pending effect intent should execute through intent authority");

    assert_eq!(effect_intent.effect_name(), "effects.reconcile-title");
    assert_eq!(
        effect_intent.trigger_commit_identity(),
        write.commit_identity()
    );
    assert_eq!(
        effect_intent.pending_intent_target(),
        "strategy.intent.reconcile"
    );
    assert_eq!(
        effect_intent.source_lane(),
        ForgeQueryIntentSourceLane::EffectTriggered
    );
    assert_eq!(
        effect_intent.target_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        effect_intent.phase_evidence().loop_prevention().as_str(),
        "pending-intent-execution-deferred"
    );
    assert_eq!(
        effect_intent.intent_receipt().source_lane(),
        ForgeQueryIntentSourceLane::EffectTriggered
    );
    assert_eq!(
        effect_intent.intent_receipt().strategy_identity(),
        "strategy.intent.reconcile"
    );
    assert!(!effect_intent.receipt_digest().is_empty());

    let inspected = runtime
        .inspect_effect(&effect)
        .expect("effect should retain only follow-up pending intent");
    assert_eq!(
        inspected.pending_write_intent_count(),
        1,
        "executing one pending intent must not recursively auto-execute feedback"
    );

    let effect_intent_inspection = runtime
        .inspect_effect_intent_receipt(&effect_intent)
        .expect("effect intent receipt should inspect");
    assert_eq!(
        effect_intent_inspection.effect_name(),
        "effects.reconcile-title"
    );
    assert_eq!(
        effect_intent_inspection.trigger_commit_identity(),
        write.commit_identity()
    );
    assert_eq!(
        effect_intent_inspection.effect_policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );
    assert_eq!(
        effect_intent_inspection.feedback_graph().termination(),
        ForgeQueryFeedbackTermination::CommittedResubscribe
    );
    assert!(!effect_intent_inspection.phase_digest().is_empty());
    assert!(!effect_intent_inspection.inspection_digest().is_empty());

    match runtime
        .inspect(&effect_intent)
        .expect("effect-intent receipt target should inspect")
    {
        ForgeQueryInspection::EffectIntentReceipt(inspection) => {
            assert_eq!(
                inspection.feedback_graph().phase_nodes(),
                &[
                    ForgeQueryFeedbackPhaseNode::TruthRead,
                    ForgeQueryFeedbackPhaseNode::Derive,
                    ForgeQueryFeedbackPhaseNode::PendingWriteIntent,
                    ForgeQueryFeedbackPhaseNode::Commit,
                    ForgeQueryFeedbackPhaseNode::BridgeRoute,
                    ForgeQueryFeedbackPhaseNode::Resubscribe,
                ]
            );
        }
        other => panic!("expected effect-intent receipt inspection, got {other:?}"),
    }

    let graph = runtime
        .inspect_effect_feedback_receipt(&effect_intent)
        .expect("effect feedback receipt should expose phase graph");
    assert_eq!(
        graph.phase_nodes(),
        &[
            ForgeQueryFeedbackPhaseNode::TruthRead,
            ForgeQueryFeedbackPhaseNode::Derive,
            ForgeQueryFeedbackPhaseNode::PendingWriteIntent,
            ForgeQueryFeedbackPhaseNode::Commit,
            ForgeQueryFeedbackPhaseNode::BridgeRoute,
            ForgeQueryFeedbackPhaseNode::Resubscribe,
        ]
    );
    assert_eq!(
        graph.termination(),
        ForgeQueryFeedbackTermination::CommittedResubscribe
    );
    assert_eq!(graph.resubscribed_live_view_count(), 1);
    assert!(!graph.graph_digest().is_empty());
}

#[test]
fn effect_triggered_idempotent_intent_noop_consumes_pending_work_without_feedback() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(CountingSignalSink {
            routed: routed.clone(),
        })
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(NoopIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<Value>(
            "tasks.effect-noop-intent",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.noop-reconcile-title",
            ForgeQueryEffectTrigger::live_view(&live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");

    let write = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: "task-1".to_string(),
            aspect_path: "title.value".to_string(),
            value: json!("already reconciled title"),
        })
        .expect("write should route pending effect intent");
    assert_eq!(write.pending_write_intent_count(), 1);
    assert_eq!(
        routed.get(),
        1,
        "the original authoritative write routes once"
    );

    let effect_intent = runtime
        .execute_next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .expect("pending effect intent should execute as an idempotent no-op");

    assert_eq!(
        effect_intent.intent_receipt().execution_kind(),
        ForgeQueryIntentExecutionKind::IdempotentNoop
    );
    assert!(effect_intent.intent_receipt().is_idempotent_noop());
    assert_eq!(
        effect_intent.intent_receipt().source_lane(),
        ForgeQueryIntentSourceLane::EffectTriggered
    );
    assert_eq!(
        effect_intent.phase_evidence().idempotence(),
        ForgeQueryEffectIdempotence::PendingIntentReceiptIdentity
    );
    assert_eq!(
        runtime
            .inspect_effect(&effect)
            .expect("pending no-op should be consumed")
            .pending_write_intent_count(),
        0
    );
    assert_eq!(
        routed.get(),
        1,
        "idempotent effect intent must not emit a second signal invalidation"
    );

    let graph = runtime
        .inspect_effect_feedback_receipt(&effect_intent)
        .expect("idempotent effect intent should expose coalesced graph");
    assert_eq!(
        graph.phase_nodes(),
        &[
            ForgeQueryFeedbackPhaseNode::TruthRead,
            ForgeQueryFeedbackPhaseNode::Derive,
            ForgeQueryFeedbackPhaseNode::PendingWriteIntent,
            ForgeQueryFeedbackPhaseNode::Commit,
        ]
    );
    assert_eq!(
        graph.termination(),
        ForgeQueryFeedbackTermination::CoalescedNoMutation
    );
    assert_eq!(graph.resubscribed_live_view_count(), 0);
}
