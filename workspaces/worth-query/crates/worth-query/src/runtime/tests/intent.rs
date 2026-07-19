use super::support::*;
use crate::WorthQueryEvidenceScope;

#[test]
fn strategy_intent_commit_routes_query_delivery_and_returns_canonical_receipt() {
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
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("native intent test contracts should admit")
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.intent",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new("computed.intent", test_aspect_touches(["title"]))
                .depends_on_live(&live)
                .produces(test_aspect_touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(WorthQueryEffectDeclaration::deliver(
            "ui.intent",
            WorthQueryEffectTrigger::computed_view(
                &computed,
                test_aspect_touches(["title.summary"]),
            ),
            "ui.intent",
        ))
        .expect("effect should declare");

    let receipt = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "reconcile-task-title",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("title", "Intent committed title")]),
        ))
        .expect("intent should execute");

    assert_eq!(receipt.intent_name(), "reconcile-task-title");
    assert_eq!(receipt.strategy_identity(), "strategy.intent.reconcile");
    assert_eq!(
        receipt.execution_kind(),
        WorthQueryIntentExecutionKind::Mutating
    );
    assert!(!receipt.is_idempotent_noop());
    assert_eq!(receipt.strategy_version(), "1.0");
    assert_eq!(
        receipt.canonical_input_digest(),
        WorthQueryIntentDeclaration::strategy_commit(
            "reconcile-task-title",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("title", "Intent committed title"),]),
        )
        .input_digest()
    );
    assert_eq!(
        receipt.target_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        receipt.terminal_affected_live_view_ids_projection(),
        ["tasks.intent"]
    );
    assert_eq!(
        receipt.terminal_affected_derived_view_ids_projection(),
        ["computed.intent"]
    );
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
    assert_eq!(
        runtime
            .read_derived_result(&computed)
            .expect("computed materialization should execute")
            .row_count(),
        1
    );
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
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.intent-inspection",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new(
                "computed.intent-inspection",
                test_aspect_touches(["title"]),
            )
            .depends_on_live(&live)
            .produces(test_aspect_touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(WorthQueryEffectDeclaration::deliver(
            "ui.intent-inspection",
            WorthQueryEffectTrigger::computed_view(
                &computed,
                test_aspect_touches(["title.summary"]),
            ),
            "ui.intent-inspection",
        ))
        .expect("effect should declare");

    let receipt = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "inspectable-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("title", "Intent committed title")]),
        ))
        .expect("intent should execute");
    let inspection = runtime
        .inspect_intent_receipt(&receipt)
        .expect("intent receipt should inspect");

    assert_eq!(inspection.intent_name(), "inspectable-intent");
    assert_eq!(
        inspection.execution_kind(),
        WorthQueryIntentExecutionKind::Mutating
    );
    assert_eq!(inspection.strategy_identity(), "strategy.intent.reconcile");
    assert_eq!(
        inspection.source_lane(),
        WorthQueryIntentSourceLane::UserAuthored
    );
    assert_eq!(
        inspection.target_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
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
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(TestWriteAuthority)
        .signal_sink(CountingSignalSink {
            routed: routed.clone(),
        })
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(NoopIntentAuthority)
        .support_profile(intent_support_profile())
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("native intent test contracts should admit")
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.noop-intent",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new("computed.noop-intent", test_aspect_touches(["title"]))
                .depends_on_live(&live)
                .produces(test_aspect_touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(WorthQueryEffectDeclaration::deliver(
            "ui.noop-intent",
            WorthQueryEffectTrigger::computed_view(
                &computed,
                test_aspect_touches(["title.summary"]),
            ),
            "ui.noop-intent",
        ))
        .expect("effect should declare");

    let receipt = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "idempotent-title-reconcile",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("title", "already committed")]),
        ))
        .expect("idempotent intent should still mint a receipt");

    assert_eq!(
        receipt.execution_kind(),
        WorthQueryIntentExecutionKind::IdempotentNoop
    );
    assert!(receipt.is_idempotent_noop());
    assert!(receipt
        .terminal_affected_live_view_ids_projection()
        .is_empty());
    assert!(receipt
        .terminal_affected_derived_view_ids_projection()
        .is_empty());
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
    assert!(!receipt.snapshot_evidence_identity().as_str().is_empty());
    assert!(!receipt.receipt_digest().is_empty());
    assert_eq!(
        routed.get(),
        0,
        "no-op intent must not route signal invalidation"
    );
    assert_eq!(runtime.drain_patches(&live).query_delivery_batches.len(), 0);
    assert_eq!(
        runtime
            .read_derived_result(&computed)
            .expect("computed materialization should execute")
            .row_count(),
        0
    );
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
        .intent_authority(NoopIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let receipt = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "inspectable-noop-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("title", "already committed")]),
        ))
        .expect("idempotent intent should execute");
    let inspection = runtime
        .inspect_intent_receipt(&receipt)
        .expect("no-op intent receipt should inspect");

    assert_eq!(
        inspection.execution_kind(),
        WorthQueryIntentExecutionKind::IdempotentNoop
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
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
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
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "empty-mutating-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1")]),
        ))
        .expect_err("empty mutating execution must use the no-op constructor");

    match error {
        WorthQueryRuntimeError::IntentCommitDenied {
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
                Some(WorthQueryIntentExecutionKind::Mutating)
            );
            assert!(evidence.attempt_digest().is_some());
            assert!(!evidence.denial_digest().as_str().is_empty());
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
        .intent_authority(InvariantViolationIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let error = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "inspectable-invariant-denial",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("dependency", "cycle")]),
        ))
        .expect_err("invariant violation must deny");

    let evidence = match error {
        WorthQueryRuntimeError::IntentCommitDenied { evidence, .. } => evidence,
        other => panic!("expected invariant admission denial, got {other:?}"),
    };
    let inspection = runtime
        .inspect_intent_denial(&evidence)
        .expect("denial evidence should inspect");

    assert_eq!(inspection.intent_name(), "inspectable-invariant-denial");
    assert_eq!(inspection.stage(), "invariant-admission");
    assert_eq!(
        inspection.execution_kind(),
        Some(WorthQueryIntentExecutionKind::InvariantViolation)
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
    assert!(inspection.snapshot_identity().is_some());
    assert_eq!(
        inspection.denial_digest(),
        evidence.denial_digest().as_str()
    );
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn invariant_violation_intent_denies_with_evidence_without_partial_publication() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
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
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.invariant-denial",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new("computed.invariant-denial", test_aspect_touches(["title"]))
                .depends_on_live(&live)
                .produces(test_aspect_touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    let delivery_effect = runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(WorthQueryEffectDeclaration::deliver(
            "ui.invariant-denial",
            WorthQueryEffectTrigger::computed_view(
                &computed,
                test_aspect_touches(["title.summary"]),
            ),
            "ui.invariant-denial",
        ))
        .expect("effect should declare");

    let error = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "violates-relational-invariants",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("dependency", "cycle")]),
        ))
        .expect_err("invariant violation must not mint an intent receipt");

    match error {
        WorthQueryRuntimeError::IntentCommitDenied {
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
                Some(WorthQueryIntentExecutionKind::InvariantViolation)
            );
            assert_eq!(
                evidence.invariant_evidence(),
                [
                    "relational-invariant:constraint-a:false",
                    "relational-invariant:constraint-b:false"
                ]
            );
            assert!(evidence.attempt_digest().is_some());
            assert!(evidence.snapshot_identity().is_some());
            assert!(!evidence.denial_digest().as_str().is_empty());
        }
        other => panic!("expected invariant admission denial, got {other:?}"),
    }
    assert_eq!(
        routed.get(),
        0,
        "invariant-denied intent must not route signal invalidation"
    );
    assert_eq!(runtime.drain_patches(&live).query_delivery_batches.len(), 0);
    assert_eq!(
        runtime
            .read_derived_result(&computed)
            .expect("computed materialization should execute")
            .row_count(),
        0
    );
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
    let error = match WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
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
        WorthQueryRuntimeError::UnsupportedFacadeFamily(denial)
            if denial.family() == WorthQueryRuntimeFacadeFamily::Intent
                && denial.reason().contains("intent authority adapter")
    ));
}

#[test]
fn intent_source_lanes_that_need_policy_deny_before_authority_execution() {
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
        .expect("intent-capable runtime should build");

    for source_lane in [
        WorthQueryIntentSourceLane::EffectTriggered,
        WorthQueryIntentSourceLane::PreviewLocal,
        WorthQueryIntentSourceLane::BranchLocal,
        WorthQueryIntentSourceLane::DerivedRuntime,
    ] {
        let error = runtime
            .execute_intent(
                WorthQueryIntentDeclaration::strategy_commit(
                    "non-user-authored",
                    "strategy.intent.reconcile",
                    "1.0",
                    "intent.reconcile.input.v1",
                    test_intent_input([("entity", "task-1")]),
                )
                .with_source_lane(source_lane),
            )
            .expect_err("non-user-authored lanes require later explicit policy admission");

        match error {
            WorthQueryRuntimeError::IntentCommitDenied {
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
    let mut runtime = WorthQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
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
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "drifting-strategy",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1")]),
        ))
        .expect_err("strategy drift must not mint an intent receipt");

    match error {
        WorthQueryRuntimeError::IntentCommitDenied {
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
                Some(WorthQueryIntentExecutionKind::Mutating)
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
        .intent_authority(DriftingIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");

    let error = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "inspectable-drifting-strategy",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1")]),
        ))
        .expect_err("strategy drift must deny");

    let evidence = match error {
        WorthQueryRuntimeError::IntentCommitDenied { evidence, .. } => evidence,
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
        Some(WorthQueryIntentExecutionKind::Mutating)
    );
    assert_eq!(
        inspection.denial_digest(),
        evidence.denial_digest().as_str()
    );
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn effect_triggered_pending_write_intent_executes_through_intent_authority_once() {
    let mut runtime = WorthQueryRuntime::builder()
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("intent test aspect contracts should install")
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
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
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.effect-intent",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(WorthQueryEffectDeclaration::write_intent(
            "effects.reconcile-title",
            WorthQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");

    let write = runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title from write",
        ))
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
        effect_intent.trigger_commit_evidence_identity().scope(),
        WorthQueryEvidenceScope::EffectTriggerCommitIdentity
    );
    assert_ne!(
        effect_intent.trigger_commit_evidence_identity(),
        write.commit_evidence_identity(),
        "effect trigger identity must wrap, not collapse to, the write receipt commit identity"
    );
    assert_eq!(
        effect_intent.pending_intent_target(),
        "strategy.intent.reconcile"
    );
    assert_eq!(
        effect_intent.source_lane(),
        WorthQueryIntentSourceLane::EffectTriggered
    );
    assert_eq!(
        effect_intent.target_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        effect_intent.phase_evidence().loop_prevention().as_str(),
        "pending-intent-execution-deferred"
    );
    assert_eq!(
        effect_intent.intent_receipt().source_lane(),
        WorthQueryIntentSourceLane::EffectTriggered
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
        effect_intent_inspection
            .trigger_commit_evidence_identity()
            .scope(),
        WorthQueryEvidenceScope::EffectTriggerCommitIdentity
    );
    assert_ne!(
        effect_intent_inspection.trigger_commit_evidence_identity(),
        write.commit_evidence_identity(),
        "effect intent inspection must preserve the typed trigger wrapper identity"
    );
    assert_eq!(
        effect_intent_inspection.effect_policy(),
        WorthQueryEffectPolicy::SandboxedWriteIntent
    );
    assert_eq!(
        effect_intent_inspection.feedback_graph().termination(),
        WorthQueryFeedbackTermination::CommittedResubscribe
    );
    assert!(!effect_intent_inspection.phase_digest().is_empty());
    assert!(!effect_intent_inspection.inspection_digest().is_empty());

    match runtime
        .inspect(&effect_intent)
        .expect("effect-intent receipt target should inspect")
    {
        WorthQueryInspection::EffectIntentReceipt(inspection) => {
            assert_eq!(
                inspection.feedback_graph().phase_nodes(),
                &[
                    WorthQueryFeedbackPhaseNode::TruthRead,
                    WorthQueryFeedbackPhaseNode::Derive,
                    WorthQueryFeedbackPhaseNode::PendingWriteIntent,
                    WorthQueryFeedbackPhaseNode::Commit,
                    WorthQueryFeedbackPhaseNode::BridgeRoute,
                    WorthQueryFeedbackPhaseNode::Resubscribe,
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
            WorthQueryFeedbackPhaseNode::TruthRead,
            WorthQueryFeedbackPhaseNode::Derive,
            WorthQueryFeedbackPhaseNode::PendingWriteIntent,
            WorthQueryFeedbackPhaseNode::Commit,
            WorthQueryFeedbackPhaseNode::BridgeRoute,
            WorthQueryFeedbackPhaseNode::Resubscribe,
        ]
    );
    assert_eq!(
        graph.termination(),
        WorthQueryFeedbackTermination::CommittedResubscribe
    );
    assert_eq!(graph.resubscribed_live_view_count(), 1);
    assert!(!graph.graph_digest().is_empty());
}

#[test]
fn composed_runtime_surface_proves_facade_handles_stay_proof_bearing_across_preview_and_intents() {
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
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build");
    let live = runtime
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.deep-runtime",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should install a subscription");
    let titles = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new("computed.deep.titles", test_aspect_touches(["title"]))
                .depends_on_live(&live)
                .produces(test_aspect_touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("source computed should declare");
    let readiness = runtime
        .declare_maintained_derived_view::<WorthQueryUnrefinedLiveShape>(
            WorthQueryDerivedView::new(
                "computed.deep.readiness",
                test_aspect_touches(["title.summary"]),
            )
            .depends_on_derived(&titles)
            .produces(test_aspect_touches(["validation.state"])),
            SummaryMaintainer,
        )
        .expect("nested computed should declare");
    let effect = runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(
            WorthQueryEffectDeclaration::write_intent(
                "effects.deep.reconcile-readiness",
                WorthQueryEffectTrigger::computed_view(
                    &readiness,
                    test_aspect_touches(["validation.state"]),
                ),
                "strategy.intent.reconcile",
            )
            .with_condition(WorthQueryEffectCondition::expression(
                "expr.validation.ready",
                test_aspect_touches(["validation.state"]),
                test_aspect_touches(["intent.reconcile"]),
            )),
        )
        .expect("conditional write-intent effect should declare");

    let (preview_evidence, preview_outcome) = {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("deep runtime preview"),
                WorthQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should be admitted");
        preview.use_view(&live);
        preview.use_computed(&titles);
        preview.use_computed(&readiness);
        preview
            .use_effect(&effect)
            .expect("sandboxed preview should bind pending-intent effect");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", test_string_aspect_value("preview-deep-task")),
                    (
                        "title.value",
                        test_string_aspect_value("Preview-only deep task"),
                    ),
                ],
            ))
            .expect("preview write should route only preview evidence");
        (
            preview.preview_execution_evidence().to_vec(),
            preview.discard(),
        )
    };

    assert!(preview_evidence.iter().any(|evidence| {
        evidence.kind() == WorthQueryPreviewExecutionKind::LivePatch
            && evidence.handle_name() == "tasks.deep-runtime"
            && evidence.preview_lane() == WorthQueryAuthorityLane::PreviewTruth
    }));
    assert!(preview_evidence.iter().any(|evidence| {
        evidence.kind() == WorthQueryPreviewExecutionKind::ComputedPatch
            && evidence.handle_name() == "computed.deep.titles"
            && evidence.aspect_touches() == test_aspect_touches(["title.summary"]).as_slice()
    }));
    assert!(preview_evidence.iter().any(|evidence| {
        evidence.kind() == WorthQueryPreviewExecutionKind::ComputedPatch
            && evidence.handle_name() == "computed.deep.readiness"
            && evidence.aspect_touches() == test_aspect_touches(["validation.state"]).as_slice()
    }));
    assert!(preview_evidence.iter().any(|evidence| {
        evidence.kind() == WorthQueryPreviewExecutionKind::PendingWriteIntent
            && evidence.handle_name() == "effects.deep.reconcile-readiness"
            && evidence.source_lane() == WorthQueryAuthorityLane::PendingWriteIntent
            && evidence.preview_lane() == WorthQueryAuthorityLane::PreviewTruth
    }));
    assert_eq!(preview_outcome.pending_write_intent_residue_count(), 1);
    assert_eq!(preview_outcome.authoritative_residue_count(), 0);
    assert!(runtime
        .drain_patches(&live)
        .query_delivery_batches
        .is_empty());
    assert_eq!(
        runtime
            .read_derived_result(&readiness)
            .expect("readiness materialization should execute")
            .row_count(),
        0
    );
    assert!(runtime
        .drain_effect_deliveries(&effect)
        .expect("authoritative effect queue should exist")
        .is_empty());

    let receipt = runtime
        .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
            "deep-authoritative-reconcile",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([
                ("entity", "intent-task-1"),
                ("title", "Committed deep title"),
            ]),
        ))
        .expect("authoritative intent should execute");

    assert_eq!(
        receipt.execution_kind(),
        WorthQueryIntentExecutionKind::Mutating
    );
    assert_eq!(
        receipt.terminal_affected_live_view_ids_projection(),
        ["tasks.deep-runtime"]
    );
    assert_eq!(
        receipt.terminal_affected_derived_view_ids_projection(),
        ["computed.deep.readiness", "computed.deep.titles"]
    );
    assert_eq!(receipt.considered_computed_view_count(), 2);
    assert_eq!(receipt.considered_effect_count(), 1);
    assert_eq!(receipt.pending_write_intent_count(), 1);
    assert_eq!(receipt.delivered_effect_count(), 0);
    assert_eq!(
        receipt.source_lane(),
        WorthQueryIntentSourceLane::UserAuthored
    );
    assert_eq!(
        receipt.target_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
    );

    let live_batches = runtime.drain_patches(&live).query_delivery_batches;
    assert_eq!(live_batches.len(), 1);
    assert_eq!(live_batches[0].view_name(), "tasks.deep-runtime");
    assert_eq!(
        live_batches[0].authority_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(
        live_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );
    assert_eq!(live_batches[0].patch_group_width(), 1);

    let readiness_inspection = runtime
        .inspect_derived_view(&readiness)
        .expect("readiness computed should inspect");
    assert_eq!(
        readiness_inspection.upstream_derived_views(),
        &["computed.deep.titles".to_string()]
    );
    assert_eq!(
        readiness_inspection.produced_aspect_touches(),
        test_aspect_touches(["validation.state"]).as_slice()
    );
    assert_eq!(readiness_inspection.materialized_row_count(), 1);
    assert_eq!(readiness_inspection.pending_incremental_patch_count(), 1);

    let effect_inspection = runtime
        .inspect_effect(&effect)
        .expect("pending effect should inspect");
    assert_eq!(
        effect_inspection.condition_descriptor(),
        "expr.validation.ready"
    );
    assert_eq!(
        effect_inspection.pending_write_intent_count(),
        1,
        "authoritative intent should create exactly one pending write intent"
    );
    assert_eq!(
        effect_inspection.latest_delivery_family(),
        Some(&WorthQueryEffectDeliveryFamily::PendingWriteIntent)
    );
    assert_eq!(
        effect_inspection
            .feedback_graph()
            .expect("effect inspection should include feedback graph")
            .phase_nodes(),
        &[
            WorthQueryFeedbackPhaseNode::TruthRead,
            WorthQueryFeedbackPhaseNode::Derive,
            WorthQueryFeedbackPhaseNode::PendingWriteIntent,
        ]
    );

    let effect_intent = runtime
        .execute_next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .expect("effect-triggered intent should use the same intent authority path");
    assert_eq!(
        effect_intent.intent_receipt().source_lane(),
        WorthQueryIntentSourceLane::EffectTriggered
    );
    assert_eq!(
        effect_intent.intent_receipt().target_lane(),
        WorthQueryAuthorityLane::AuthoritativeTruth
    );
    let feedback_graph = runtime
        .inspect_effect_feedback_receipt(&effect_intent)
        .expect("effect-triggered intent should expose feedback graph");
    assert_eq!(
        feedback_graph.phase_nodes(),
        &[
            WorthQueryFeedbackPhaseNode::TruthRead,
            WorthQueryFeedbackPhaseNode::Derive,
            WorthQueryFeedbackPhaseNode::PendingWriteIntent,
            WorthQueryFeedbackPhaseNode::Commit,
            WorthQueryFeedbackPhaseNode::BridgeRoute,
            WorthQueryFeedbackPhaseNode::Resubscribe,
        ]
    );
    assert_eq!(
        feedback_graph.termination(),
        WorthQueryFeedbackTermination::CommittedResubscribe
    );

    let second_live_batches = runtime.drain_patches(&live).query_delivery_batches;
    assert_eq!(
        second_live_batches.len(),
        1,
        "effect-triggered intent should resubscribe through query-shaped delivery"
    );
    assert_eq!(
        second_live_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );

    let branch_receipt = {
        let mut branch = runtime
            .branch_with_options(
                test_session_label("deep runtime branch"),
                WorthQueryBranchOptions::sandboxed_write_intent(),
            )
            .expect("branch session should be admitted");
        branch
            .execute_intent(WorthQueryIntentDeclaration::strategy_commit(
                "deep-branch-reconcile",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                test_intent_input([("entity", "intent-task-1"), ("title", "Branch-only title")]),
            ))
            .expect("branch-local intent should stay branch-local")
    };
    assert_eq!(
        branch_receipt.source_lane(),
        WorthQueryIntentSourceLane::BranchLocal
    );
    assert_eq!(
        branch_receipt.target_lane(),
        WorthQueryAuthorityLane::BranchLocalTruth
    );
    assert!(runtime
        .drain_patches(&live)
        .query_delivery_batches
        .is_empty());
}

#[test]
fn effect_triggered_idempotent_intent_noop_consumes_pending_work_without_feedback() {
    let routed = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = WorthQueryRuntime::builder()
        .aspect_contracts(stateful_bridge_aspect_contracts())
        .expect("intent test aspect contracts should install")
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
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
        .declare_live_view::<WorthQueryUnrefinedLiveShape>(
            "tasks.effect-noop-intent",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<WorthQueryUnrefinedLiveShape>(WorthQueryEffectDeclaration::write_intent(
            "effects.noop-reconcile-title",
            WorthQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");

    let write = runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "already reconciled title",
        ))
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
        WorthQueryIntentExecutionKind::IdempotentNoop
    );
    assert!(effect_intent.intent_receipt().is_idempotent_noop());
    assert_eq!(
        effect_intent.intent_receipt().source_lane(),
        WorthQueryIntentSourceLane::EffectTriggered
    );
    assert_eq!(
        effect_intent.phase_evidence().idempotence(),
        WorthQueryEffectIdempotence::PendingIntentReceiptIdentity
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
            WorthQueryFeedbackPhaseNode::TruthRead,
            WorthQueryFeedbackPhaseNode::Derive,
            WorthQueryFeedbackPhaseNode::PendingWriteIntent,
            WorthQueryFeedbackPhaseNode::Commit,
        ]
    );
    assert_eq!(
        graph.termination(),
        WorthQueryFeedbackTermination::CoalescedNoMutation
    );
    assert_eq!(graph.resubscribed_live_view_count(), 0);
}
