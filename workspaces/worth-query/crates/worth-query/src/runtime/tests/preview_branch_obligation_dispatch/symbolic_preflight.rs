use super::support::*;

#[test]
fn preview_batch_symbolic_denial_precedes_graph_obligation_denial() {
    let mut runtime = runtime_with_obligation(
        "preview-batch-missing-symbol",
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label(
            "preview missing symbol before obligation",
        ))
        .expect("preview session should open");

    let missing_symbol = WorthQuerySymbolicTargetReference::new("missing-preview-task")
        .expect("symbolic reference should build")
        .in_target_collection("Task")
        .expect("symbolic reference collection should build");
    let error = preview
        .batch(|batch| {
            batch.update_symbolic(missing_symbol, |task| {
                task.set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("Should fail before obligation dispatch"),
                )
            })
        })
        .expect_err("missing symbolic target should deny before obligation dispatch");

    match error {
        WorthQueryRuntimeError::MutationTargetReferenceDenied(denial) => {
            assert_eq!(
                denial.kind(),
                WorthQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget
            );
        }
        other => panic!("expected symbolic target denial, got {other:?}"),
    }
    assert_eq!(preview.discard().write_count(), 0);
}

#[test]
fn preview_batch_symbolic_preflight_edges_precede_obligation_denial() {
    assert_preview_symbolic_target_denial_precedes_obligation(
        |batch| {
            batch
                .insert_symbolic("draft-task", "Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("draft-task"),
                    )
                })
                .update_symbolic(
                    WorthQuerySymbolicTargetReference::new("draft-task")
                        .expect("symbolic reference should build")
                        .in_target_collection("Project")
                        .expect("symbolic collection should build"),
                    |task| {
                        task.set_aspect(
                            test_aspect_touch("title.value"),
                            test_authored_string_aspect_value("wrong collection"),
                        )
                    },
                )
        },
        WorthQuerySymbolicTargetReferenceDenialKind::CollectionMismatch,
    );

    assert_preview_symbolic_target_denial_precedes_obligation(
        |batch| {
            batch.insert("TaskEdge", |edge| {
                edge.set_aspect(
                    test_aspect_touch("edge.kind"),
                    test_authored_string_aspect_value("depends_on"),
                )
                .symbolic_entity_identity(
                    test_aspect_touch("edge.source_identity"),
                    WorthQuerySymbolicTargetReference::new("missing-task")
                        .expect("symbolic reference should build"),
                )
                .set_aspect(
                    test_aspect_touch("edge.target_identity"),
                    test_authored_string_aspect_value("task-existing"),
                )
            })
        },
        WorthQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
    );
}

fn assert_preview_symbolic_target_denial_precedes_obligation(
    declaration: impl FnOnce(WorthQueryMutationBatchBuilder) -> WorthQueryMutationBatchBuilder,
    expected_kind: WorthQuerySymbolicTargetReferenceDenialKind,
) {
    let mut runtime = runtime_with_obligation(
        "symbolic-preflight-blocker",
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label("preview symbolic preflight"))
        .expect("preview should open");
    match preview.batch(declaration) {
        Err(WorthQueryRuntimeError::MutationTargetReferenceDenied(denial)) => {
            assert_eq!(denial.kind(), expected_kind);
        }
        other => panic!("expected symbolic target denial, got {other:?}"),
    }
    assert_eq!(preview.discard().write_count(), 0);
}
