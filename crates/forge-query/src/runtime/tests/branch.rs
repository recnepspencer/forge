use super::support::*;

fn branch_intent_runtime(attempted: std::rc::Rc<std::cell::Cell<usize>>) -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(CountingIntentAuthority { attempted })
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("intent-capable runtime should build")
}

#[test]
fn branch_local_intent_is_policy_admitted_without_authoritative_execution() {
    let attempted = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = branch_intent_runtime(attempted.clone());

    let mut branch = runtime
        .branch_with_options(
            "branch local intent",
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session should be admitted");
    let receipt = branch
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "branch-reconcile",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({ "entity": "task-1", "title": "branch title" }),
        ))
        .expect("sandboxed branch intent should be admitted");

    assert_eq!(branch.label(), "branch local intent");
    assert_eq!(
        branch.basis_admission().authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(receipt.intent_name(), "branch-reconcile");
    assert_eq!(receipt.strategy_identity(), "strategy.intent.reconcile");
    assert_eq!(receipt.strategy_version(), "1.0");
    assert_eq!(
        receipt.source_lane(),
        ForgeQueryIntentSourceLane::BranchLocal
    );
    assert_eq!(
        receipt.target_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(
        receipt.effect_policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );
    assert!(!receipt.basis_evidence().is_empty());
    assert!(!receipt.basis_snapshot_token().is_empty());
    assert!(!receipt.admission_digest().is_empty());
    assert!(!receipt.receipt_digest().is_empty());
    assert_eq!(branch.branch_intent_receipts(), [receipt.clone()]);
    assert_eq!(
        attempted.get(),
        0,
        "branch-local intent staging must not execute authoritative intent authority"
    );

    let inspection = runtime
        .inspect_branch_intent_receipt(&receipt)
        .expect("branch intent inspection should succeed");
    assert_eq!(inspection.intent_name(), "branch-reconcile");
    assert_eq!(
        inspection.source_lane(),
        ForgeQueryIntentSourceLane::BranchLocal
    );
    assert_eq!(
        inspection.target_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(
        inspection.effect_policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );
    assert!(!inspection.basis_digest().is_empty());
    assert_eq!(
        inspection.basis_snapshot_token(),
        receipt.basis_snapshot_token()
    );
    assert!(!inspection.inspection_digest().is_empty());
}

#[test]
fn derive_only_branch_intent_denies_before_authoritative_execution() {
    let attempted = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = branch_intent_runtime(attempted.clone());

    let error = {
        let mut branch = runtime
            .branch("derive-only branch intent")
            .expect("branch session should be admitted");
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

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name,
            stage,
            message,
            evidence: _,
        } => {
            assert_eq!(intent_name, "branch-denied");
            assert_eq!(stage, "branch-effect-policy-admission");
            assert!(message.contains("sandboxed-write-intent"));
            assert!(message.contains("derive-only"));
        }
        other => panic!("expected branch policy intent denial, got {other:?}"),
    }
    assert_eq!(
        attempted.get(),
        0,
        "branch policy denial must happen before authoritative intent authority"
    );
}

#[test]
fn branch_local_intent_requires_intent_support_for_branch_lane() {
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
        .support_profile(
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
            )),
        )
        .build_backend_from_parts()
        .build()
        .expect("runtime can support authoritative-only intents");

    let error = {
        let mut branch = runtime
            .branch_with_options(
                "branch lane unsupported",
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .expect("branch session should be admitted");
        branch
            .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
                "branch-lane-denied",
                "strategy.intent.reconcile",
                "1.0",
                "intent.reconcile.input.v1",
                json!({ "entity": "task-1" }),
            ))
            .expect_err("branch-local intent requires branch-local support metadata")
    };

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Intent);
            assert!(denial.reason().contains("branch-local-truth"));
        }
        other => panic!("expected branch lane support denial, got {other:?}"),
    }
    assert_eq!(
        attempted.get(),
        0,
        "branch lane support denial must happen before authoritative intent authority"
    );
}
