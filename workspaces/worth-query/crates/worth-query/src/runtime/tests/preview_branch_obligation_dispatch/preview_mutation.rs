use super::support::*;

#[test]
fn preview_write_dispatches_preview_mutation_obligation() {
    let mut runtime = runtime_with_obligation(
        "preview-write",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label("preview obligation write"))
        .expect("preview session should open");

    let receipt = preview
        .write(task_insert_command("preview-write"))
        .expect("preview write should be admitted");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("preview write should carry obligation dispatch");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::PreviewMutation
    );
    assert_eq!(
        dispatch
            .envelope()
            .unwrap()
            .context()
            .operating_world_digest(),
        WorthQueryGraphObligationOperatingWorldDescriptor::preview().descriptor_digest()
    );
    assert_eq!(
        dispatch.execution_inputs()[0]
            .executor_contract()
            .support_lane(),
        WorthQueryGraphObligationSupportLane::PreviewMutation
    );
}

#[test]
fn preview_batch_dispatches_without_inventing_authoritative_batch_context() {
    let mut runtime = runtime_with_obligation(
        "preview-batch",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label("preview obligation batch"))
        .expect("preview session should open");

    let receipt = preview
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("preview-batch-a"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Preview A"),
                    )
                })
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("preview-batch-b"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Preview B"),
                    )
                })
        })
        .expect("preview batch should be admitted");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("preview batch should carry obligation dispatch");

    assert_eq!(receipt.write_count(), 2);
    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::PreviewMutation
    );
}

#[test]
fn preview_mutation_support_postures_keep_distinct_verdict_meaning() {
    assert_preview_posture(
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        "allow",
        Some("selected-for-execution"),
    );
    assert_preview_posture(
        WorthQueryGraphObligationSupportPosture::diagnostic_only(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        "advise",
        Some("selected-diagnostic-only"),
    );
    assert_preview_posture(
        WorthQueryGraphObligationSupportPosture::deferred_to_backstop(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        "allow",
        Some("selected-deferred-to-backstop"),
    );
    assert_preview_posture(
        WorthQueryGraphObligationSupportPosture::not_applicable(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        "allow",
        Some("not-applicable-at-selection"),
    );

    let mut runtime = runtime_with_obligation(
        "preview-unsupported",
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label("preview unsupported posture"))
        .expect("preview should open");
    match preview.write(task_insert_command("preview-unsupported")) {
        Err(WorthQueryRuntimeError::GraphObligationDenied(denial)) => {
            assert_eq!(denial.blocking_count(), 1);
        }
        other => panic!("expected unsupported posture denial, got {other:?}"),
    }
}

#[test]
fn preview_obligation_denial_leaves_no_preview_write_residue() {
    let mut runtime = runtime_with_obligation(
        "preview-denial",
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::PreviewMutation,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label("preview obligation denial"))
        .expect("preview session should open");

    let error = preview
        .write(task_insert_command("preview-denied"))
        .expect_err("unsupported preview obligation should deny");

    assert!(matches!(
        error,
        WorthQueryRuntimeError::GraphObligationDenied(_)
    ));
    let outcome = preview.discard();
    assert_eq!(outcome.write_count(), 0);
    assert_eq!(outcome.closeout_evidence().preview_write_staging_count(), 0);
}

fn assert_preview_posture(
    support_posture: WorthQueryGraphObligationSupportPosture,
    expected_verdict: &str,
    expected_context: Option<&str>,
) {
    let mut runtime = runtime_with_obligation(
        support_posture.status().as_str(),
        support_posture,
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = runtime
        .preview(test_session_label("preview posture"))
        .expect("preview should open");
    let receipt = preview
        .write(task_insert_command("preview-posture"))
        .expect("posture should not block preview write");
    let row = only_projection_row(
        receipt
            .obligation_dispatch()
            .expect("preview dispatch should exist"),
    );
    assert_eq!(row.verdict(), expected_verdict);
    assert_eq!(row.verdict_context(), expected_context);
}
