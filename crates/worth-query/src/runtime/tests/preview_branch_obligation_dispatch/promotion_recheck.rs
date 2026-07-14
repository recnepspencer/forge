use super::support::*;

#[test]
fn promotion_rechecks_authoritative_obligations_before_committed_truth() {
    let mut runtime = complete_backend_from_parts_builder()
        .graph_obligation(collection_registration(
            "Task",
            "preview-promote",
            WorthQueryGraphObligationSupportPosture::supported(
                WorthQueryGraphObligationSupportLane::PreviewMutation,
            ),
            WorthQueryGraphObligationOperatingWorldSelector::preview(),
        ))
        .graph_obligation(collection_registration(
            "Task",
            "authoritative-promote",
            WorthQueryGraphObligationSupportPosture::unsupported(
                WorthQueryGraphObligationSupportLane::ScalarMutation,
            ),
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ))
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with preview and authoritative obligations");
    let mut preview = runtime
        .preview(test_session_label("preview promotion recheck"))
        .expect("preview session should open");
    let receipt = preview
        .write(task_insert_command("promotion-recheck"))
        .expect("preview-local write should be admitted");

    assert_eq!(
        receipt
            .obligation_dispatch()
            .unwrap()
            .envelope()
            .unwrap()
            .context()
            .kind(),
        WorthQueryGraphObligationDispatchContextKind::PreviewMutation
    );

    let error = preview
        .promote()
        .expect_err("promotion should re-enter authoritative obligation dispatch");
    match error {
        WorthQueryRuntimeError::PreviewPromotionWriteFailed { evidence } => {
            assert_eq!(evidence.failed_write_sequence(), Some(1));
            assert_eq!(evidence.promoted_write_count(), 0);
            let projection = evidence
                .graph_obligation_denial_projection()
                .expect("promotion failure should retain graph obligation denial projection");
            assert_eq!(projection.blocking_count(), 1);
            assert!(!projection.projection_digest().is_empty());
        }
        other => panic!("expected promotion write failure, got {other:?}"),
    }
}
