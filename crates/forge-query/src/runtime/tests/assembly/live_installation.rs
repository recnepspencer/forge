use super::super::support::*;

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
