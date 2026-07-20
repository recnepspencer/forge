use super::support::*;

#[test]
fn preview_and_branch_intents_dispatch_declared_touch_descriptors() {
    let mut preview_runtime = intent_runtime_with_obligation(
        "preview-intent",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::PreviewIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = preview_runtime
        .preview_with_options(
            test_session_label("preview obligation intent"),
            WorthQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should open");
    let preview_receipt = preview
        .execute_intent(touch_bearing_intent_declaration("preview-intent"))
        .expect("preview intent should be admitted");

    let preview_dispatch = preview_receipt
        .obligation_dispatch()
        .expect("preview intent should dispatch declared touch");
    assert_eq!(
        preview_dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::PreviewIntent
    );
    assert_eq!(
        preview_dispatch.execution_inputs()[0]
            .executor_contract()
            .support_lane(),
        WorthQueryGraphObligationSupportLane::PreviewIntent
    );

    let mut branch_runtime = intent_runtime_with_obligation(
        "branch-intent",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::BranchIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::branch(),
    );
    let mut branch = branch_runtime
        .branch_with_options(
            test_session_label("branch obligation intent"),
            WorthQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session should open");
    let branch_receipt = branch
        .execute_intent(touch_bearing_intent_declaration("branch-intent"))
        .expect("branch intent should be admitted");
    let branch_dispatch = branch_receipt
        .obligation_dispatch()
        .expect("branch intent should dispatch declared touch");

    assert_eq!(
        branch_dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::BranchIntent
    );
    assert_eq!(
        branch_dispatch
            .envelope()
            .unwrap()
            .context()
            .operating_world_digest(),
        WorthQueryGraphObligationOperatingWorldDescriptor::branch().descriptor_digest()
    );
}

#[test]
fn preview_and_branch_intents_cannot_omit_touch_when_obligations_exist() {
    let mut preview_runtime = intent_runtime_with_obligation(
        "preview-intent-missing-touch",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::PreviewIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );
    let mut preview = preview_runtime
        .preview_with_options(
            test_session_label("preview missing touch"),
            WorthQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview session should open");
    let preview_error = preview
        .execute_intent(plain_intent("preview-missing-touch"))
        .expect_err("preview intent must declare touch when obligations exist");

    assert!(matches!(
        preview_error,
        WorthQueryRuntimeError::GraphObligationIntentTouchDescriptorMissing { .. }
    ));
    assert!(preview.preview_intent_receipts().is_empty());

    let mut branch_runtime = intent_runtime_with_obligation(
        "branch-intent-missing-touch",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::BranchIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::branch(),
    );
    let mut branch = branch_runtime
        .branch_with_options(
            test_session_label("branch missing touch"),
            WorthQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session should open");
    let branch_error = branch
        .execute_intent(plain_intent("branch-missing-touch"))
        .expect_err("branch intent must declare touch when obligations exist");

    assert!(matches!(
        branch_error,
        WorthQueryRuntimeError::GraphObligationIntentTouchDescriptorMissing { .. }
    ));
    assert!(branch.branch_intent_receipts().is_empty());
}

#[test]
fn preview_and_branch_intent_no_match_keeps_zero_selection_dispatch() {
    let descriptor = task_touch_descriptor("intent-no-match");
    let touch_intent = touch_bearing_intent("intent-no-match", descriptor);
    assert!(touch_intent
        .declaration()
        .graph_touch_descriptor()
        .is_some());
    assert_eq!(
        touch_intent.graph_touch_descriptor().descriptor_digest(),
        touch_intent
            .declaration()
            .graph_touch_descriptor()
            .expect("touch-bearing intent should expose descriptor")
            .descriptor_digest()
    );
    assert_eq!(
        touch_intent.input_digest(),
        touch_intent.declaration().input_digest()
    );

    let mut preview_runtime = runtime_with_registration(collection_registration(
        "Other",
        "preview-no-match",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::PreviewIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    ));
    let mut preview = preview_runtime
        .preview_with_options(
            test_session_label("preview intent no match"),
            WorthQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");
    let preview_receipt = preview
        .execute_intent(touch_intent.clone().into_declaration())
        .expect("no-match preview intent should not deny");
    assert_zero_selection_dispatch(
        preview_receipt
            .obligation_dispatch()
            .expect("no-match preview intent still carries dispatch counters"),
    );

    let mut branch_runtime = runtime_with_registration(collection_registration(
        "Other",
        "branch-no-match",
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::BranchIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::branch(),
    ));
    let mut branch = branch_runtime
        .branch_with_options(
            test_session_label("branch intent no match"),
            WorthQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch should open");
    let branch_receipt = branch
        .execute_intent(touch_intent.into_declaration())
        .expect("no-match branch intent should not deny");
    assert_zero_selection_dispatch(
        branch_receipt
            .obligation_dispatch()
            .expect("no-match branch intent still carries dispatch counters"),
    );
}

#[test]
fn branch_intent_unsupported_posture_enforces_branch_policy() {
    let mut runtime = runtime_with_obligation(
        "branch-unsupported",
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::BranchIntent,
        ),
        WorthQueryGraphObligationOperatingWorldSelector::branch(),
    );
    let mut branch = runtime
        .branch_with_options(
            test_session_label("branch unsupported posture"),
            WorthQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch should open");

    match branch.execute_intent(touch_bearing_intent_declaration("branch-unsupported")) {
        Err(WorthQueryRuntimeError::GraphObligationDenied(denial)) => {
            assert_eq!(denial.blocking_count(), 1);
        }
        other => panic!("expected branch obligation denial, got {other:?}"),
    }
    assert!(branch.branch_intent_receipts().is_empty());
}
