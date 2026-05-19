use super::support::*;

#[test]
fn runtime_live_view_denies_when_schema_boundary_receipt_drifts_from_request() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(DriftingSchemaReceiptAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend should build with drifting schema boundary receipt");

    let error = runtime
        .declare_live_view::<serde_json::Value>(
            "external.tasks",
            task_live_request(),
            task_schema(),
        )
        .expect_err("drifted live admission boundary must deny installation");

    assert_live_installation_error(error, "backend-live-admission-receipt");
}

#[test]
fn runtime_write_denies_when_write_authority_route_receipt_drifts_from_command() {
    struct DriftingWriteAuthority;

    impl ForgeQueryRuntimeWriteAuthorityAdapter for DriftingWriteAuthority {
        fn write(
            &mut self,
            bridge: &RuntimeBridge,
            relational_runtime: Option<&mut RelationalRuntime>,
            command: ForgeQueryWriteCommand,
        ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
            let mut authority = TestWriteAuthority;
            let honest = authority.write(bridge, relational_runtime, command.clone())?;
            Ok(WriteAuthorityExecutionReceipt::from_command(
                &ForgeQueryWriteCommand::Delete {
                    entity_identity: "drifted-entity".to_string(),
                },
                honest.mutation_receipt().clone(),
            ))
        }
    }

    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(DriftingWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("backend should build with drifting write route receipt");

    let error = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", serde_json::json!("")),
                ("title.value", serde_json::json!("x")),
            ],
        ))
        .expect_err("drifted write route receipt must deny the write");

    assert_workspace_write_error(
        error,
        "lower-runtime capability request subject digest drifted",
    );
}

fn assert_live_installation_error(error: ForgeQueryRuntimeError, expected_stage: &str) {
    match error {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation { stage, .. } => {
            assert_eq!(stage, expected_stage);
        }
        other => panic!("expected live installation denial, got {other:?}"),
    }
}

fn assert_workspace_write_error(error: ForgeQueryRuntimeError, expected_message: &str) {
    match error {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert_eq!(error.to_string(), expected_message);
        }
        other => panic!("expected workspace denial, got {other:?}"),
    }
}
