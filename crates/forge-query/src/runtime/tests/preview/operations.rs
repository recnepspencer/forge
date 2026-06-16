use super::super::support::*;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

#[test]
fn sandboxed_preview_run_operation_stages_compiled_writes_until_promote() {
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

    let preview_run = {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("draft create"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should be admitted");
        let run = preview
            .run_operation(
                operation.clone(),
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Preview-only task".to_string()),
                )],
            )
            .expect("preview operation should run");

        assert_eq!(run.write_receipts().len(), 1);
        let draft_label = test_session_label("draft create");
        let expected_commit_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::PreviewWriteReceiptIdentity,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            draft_label.identity_digest().as_str(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("sequence"), 1)
        .seal();
        assert_eq!(
            run.write_receipts()[0]
                .commit_identity()
                .evidence_identity(),
            expected_commit_identity
        );
        assert_eq!(
            run.write_receipts()[0].authority_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );
        run
    };

    assert_eq!(
        preview_run.outputs()[0].value().as_array().unwrap().len(),
        0
    );

    {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("promote create"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should be admitted");
        preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Promoted preview task".to_string()),
                )],
            )
            .expect("preview operation should stage");
        let outcome = preview.promote().expect("preview promotion should succeed");
        assert!(outcome.promoted());
        assert_eq!(outcome.write_count(), 1);
        assert_eq!(
            outcome.effect_policy(),
            ForgeQueryEffectPolicy::SandboxedWriteIntent
        );
        assert_eq!(outcome.source_lane(), ForgeQueryAuthorityLane::PreviewTruth);
        assert_eq!(
            outcome.target_lane(),
            ForgeQueryAuthorityLane::AuthoritativeTruth
        );
    }

    let view = runtime
        .declare_live_view::<Value>("tasks.after-preview", task_live_request(), task_schema())
        .expect("live view should declare");
    let rows = runtime.read_live(&view);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].external_row()["title"]["value"],
        "Promoted preview task"
    );
}

#[test]
fn preview_run_operation_discard_keeps_authoritative_state_unchanged() {
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

    {
        let mut preview = runtime
            .preview_with_options(
                test_session_label("discard create"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session should be admitted");
        preview
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Discarded preview task".to_string()),
                )],
            )
            .expect("preview operation should stage");
        let outcome = preview.discard();
        assert!(outcome.discarded());
    }

    let view = runtime
        .declare_live_view::<Value>("tasks.after-discard", task_live_request(), task_schema())
        .expect("live view should declare");
    assert!(runtime.read_live(&view).is_empty());
}

#[test]
fn preview_run_operation_rejects_declaration_effects_before_runtime_mutation() {
    let mut runtime = stateful_bridge_task_runtime();
    let program =
        ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter).expect("fake DSL should compile");
    let installed = runtime
        .install_program(program)
        .expect("program should install");
    let operation = installed
        .operation("create_task")
        .expect("operation ref should build");

    let mut preview = runtime
        .preview_with_options(
            test_session_label("deny declaration effects"),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should be admitted");
    let error = preview
        .run_operation(
            operation,
            vec![ForgeQueryOperationInput::new(
                "title",
                Value::String("Should never install runtime declarations".to_string()),
            )],
        )
        .expect_err("preview operation must deny declaration effects");

    match error {
        ForgeQueryRuntimeError::PreviewOperationEffectDenied {
            label,
            stage,
            message,
        } => {
            assert_eq!(label, test_session_label("deny declaration effects"));
            assert_eq!(stage, "effect-admission");
            assert!(message.contains("cannot install live view `tasks.table`"));
        }
        other => panic!("expected preview declaration denial, got {other:?}"),
    }

    let view = runtime
        .declare_live_view::<Value>(
            "tasks.after-denied-declaration",
            task_live_request(),
            task_schema(),
        )
        .expect("authoritative runtime should remain available after denial");
    assert!(runtime.read_live(&view).is_empty());
}
