use super::super::support::*;

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
