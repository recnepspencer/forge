use super::support::*;

#[test]
fn runtime_write_denies_when_signal_routing_receipt_drifts_from_write_receipt() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(DriftingSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend with drifting signal receipt should still build");
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("external.tasks", task_live_request(), task_schema())
        .expect("live view should declare before hostile write");

    let error = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Signal receipt drift")),
            ],
        ))
        .expect_err("signal routing receipt drift must deny the write");

    assert_workspace_write_error(
        error,
        "signal invalidation routing receipt drifted from write receipt",
    );
    assert!(runtime
        .drain_patches(&view)
        .query_delivery_batches
        .is_empty());
}

#[test]
fn runtime_batch_write_denies_when_signal_routing_batch_width_drifts_from_receipts() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TruncatingBatchSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend with truncating signal batch sink should still build");
    let view: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("external.tasks.batch", task_live_request(), task_schema())
        .expect("live view should declare before hostile batch write");

    let error = runtime
        .write_batch(vec![
            insert_command(
                "Task",
                [
                    ("identity.id", json!("")),
                    ("title.value", json!("first hostile batch write")),
                ],
            ),
            insert_command(
                "Task",
                [
                    ("identity.id", json!("")),
                    ("title.value", json!("second hostile batch write")),
                ],
            ),
        ])
        .expect_err("signal routing batch width drift must deny the batch write");

    assert_workspace_write_error(
        error,
        "signal invalidation routing batch width drifted from write batch",
    );
    assert!(runtime
        .drain_patches(&view)
        .query_delivery_batches
        .is_empty());
}

fn assert_workspace_write_error(error: ForgeQueryRuntimeError, expected_message_fragment: &str) {
    match error {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains(expected_message_fragment));
        }
        other => panic!("expected workspace write denial, got {other:?}"),
    }
}
