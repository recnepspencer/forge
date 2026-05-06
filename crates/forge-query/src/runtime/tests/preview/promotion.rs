use super::super::support::*;

#[test]
fn preview_discard_closeout_separates_temporary_writes_from_authoritative_residue() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.preview-closeout", task_live_request(), task_schema())
        .expect("live should declare");

    let outcome = {
        let mut preview = runtime
            .preview("discard closeout")
            .expect("preview session should be admitted");
        preview.use_view(&live);
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-temp-1")),
                    ("title.value", json!("Temporary one")),
                ],
            ))
            .expect("first preview write should stage");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-temp-2")),
                    ("title.value", json!("Temporary two")),
                ],
            ))
            .expect("second preview write should stage");
        preview.discard()
    };
    let closeout = outcome.closeout_evidence();

    assert!(outcome.discarded());
    assert_eq!(closeout.kind(), ForgeQueryPreviewCloseoutKind::Discarded);
    assert_eq!(closeout.preview_binding_count(), 1);
    assert_eq!(closeout.live_binding_count(), 1);
    assert_eq!(closeout.preview_write_staging_count(), 2);
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::PreviewWriteStaging),
        2
    );
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::AuthoritativeResidue),
        0
    );
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert_eq!(closeout.effect_delivery_residue_count(), 0);
    assert_eq!(closeout.pending_write_intent_residue_count(), 0);
    assert!(!closeout.closeout_digest().is_empty());
    assert!(runtime.read_live(&live).is_empty());
}

#[test]
fn preview_promotion_closeout_records_consumed_staging_without_preview_lane_mutation() {
    let mut runtime = stateful_bridge_task_runtime();
    runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare before preview-safe operation runs");
    let program = preview_safe_program();
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");
    let outcome = {
        let mut preview = runtime
            .preview_with_options(
                "promotion closeout",
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should be admitted");
        preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Promoted closeout task".to_string()),
                )],
            )
            .expect("preview operation should stage");
        preview.promote().expect("preview promotion should succeed")
    };
    let closeout = outcome.closeout_evidence();

    assert!(outcome.promoted());
    assert_eq!(outcome.write_count(), 1);
    assert_eq!(closeout.kind(), ForgeQueryPreviewCloseoutKind::Promoted);
    assert_eq!(closeout.preview_write_staging_count(), 1);
    assert_eq!(closeout.promoted_write_count(), 1);
    assert_eq!(
        closeout.class_count(ForgeQueryPreviewResidueClass::PreviewWriteStaging),
        1
    );
    assert_eq!(closeout.authoritative_residue_count(), 0);
    assert_eq!(
        closeout.effect_policy(),
        ForgeQueryEffectPolicy::SandboxedWriteIntent
    );

    let view = runtime
        .declare_live_view::<Value>(
            "tasks.after-promotion-closeout",
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    let rows = runtime.read_live(&view);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].payload["title"]["value"], "Promoted closeout task");
}

#[test]
fn preview_promotion_rejects_stale_basis_before_authority_execution() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(DriftingSnapshotSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("drifting backend should build");

    let error = {
        let mut preview = runtime
            .preview("stale basis")
            .expect("preview session should be admitted");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("stale-preview")),
                    ("title.value", json!("Should not promote")),
                ],
            ))
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("drifting authoritative basis should deny promotion")
    };

    match error {
        ForgeQueryRuntimeError::PreviewPromotionStaleBasis(evidence) => {
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::StaleBasis
            );
            assert_eq!(evidence.staged_preview_write_count(), 1);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_ne!(
                evidence.basis_snapshot_token(),
                evidence.promotion_snapshot_token()
            );
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected stale basis promotion denial, got {other:?}"),
    }
}

#[test]
fn preview_promotion_write_failure_is_typed_and_not_silently_dropped() {
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
        .expect("denying write backend should build");

    let error = {
        let mut preview = runtime
            .preview("write failure")
            .expect("preview session should be admitted");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("denied-preview")),
                    ("title.value", json!("Denied preview write")),
                ],
            ))
            .expect("preview write should stage");
        preview
            .promote()
            .expect_err("write authority denial should fail promotion")
    };

    match error {
        ForgeQueryRuntimeError::PreviewPromotionWriteFailed { evidence } => {
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::WriteFailed
            );
            assert_eq!(evidence.staged_preview_write_count(), 1);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_eq!(evidence.failed_write_sequence(), Some(1));
            assert!(evidence.reason().contains("write authority denied"));
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected write failure promotion denial, got {other:?}"),
    }
}

#[test]
fn preview_promotion_rejects_multi_write_batch_before_partial_authority_execution() {
    let attempted_writes = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(CountingWriteAuthority {
            attempted_writes: attempted_writes.clone(),
        })
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("counting write backend should build");

    let error = {
        let mut preview = runtime
            .preview("multi write promotion")
            .expect("preview session should be admitted");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-batch-1")),
                    ("title.value", json!("First staged write")),
                ],
            ))
            .expect("first preview write should stage");
        preview
            .write(insert_command(
                "Task",
                [
                    ("identity.id", json!("preview-batch-2")),
                    ("title.value", json!("Second staged write")),
                ],
            ))
            .expect("second preview write should stage");
        preview
            .promote()
            .expect_err("non-atomic multi-write promotion should deny before authority")
    };

    assert_eq!(attempted_writes.get(), 0);
    match error {
        ForgeQueryRuntimeError::PreviewPromotionAtomicBatchUnsupported(evidence) => {
            assert_eq!(
                evidence.kind(),
                ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported
            );
            assert_eq!(evidence.staged_preview_write_count(), 2);
            assert_eq!(evidence.promoted_write_count(), 0);
            assert_eq!(evidence.failed_write_sequence(), None);
            assert!(evidence.reason().contains("atomic promotion support"));
            assert!(!evidence.denial_digest().is_empty());
        }
        other => panic!("expected atomic batch promotion denial, got {other:?}"),
    }
}
