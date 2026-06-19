use super::support::*;

#[test]
fn preview_selection_matches_authoritative_selection_for_same_touch() {
    let command = task_insert_command("selection-equivalence");
    let mut authoritative_runtime = runtime_with_registration(collection_registration(
        "Task",
        "equivalent-rule",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ScalarMutation,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    ));
    let authoritative_receipt = authoritative_runtime
        .write(command.clone())
        .expect("authoritative write should execute");
    let authoritative_dispatch = authoritative_receipt
        .obligation_dispatch()
        .expect("authoritative write should carry dispatch");

    let mut preview_runtime = runtime_with_registration(collection_registration(
        "Task",
        "equivalent-rule",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::PreviewMutation,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    ));
    let mut preview = preview_runtime
        .preview(test_session_label("preview selection equivalence"))
        .expect("preview should open");
    let preview_receipt = preview
        .write(command)
        .expect("preview write should execute");
    let preview_dispatch = preview_receipt
        .obligation_dispatch()
        .expect("preview write should carry dispatch");

    assert_eq!(
        selected_rule_identity_digests(authoritative_dispatch),
        selected_rule_identity_digests(preview_dispatch)
    );
    assert_eq!(
        authoritative_dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
    assert_eq!(
        preview_dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
    assert_eq!(
        authoritative_dispatch
            .selection()
            .matched_obligation_count(),
        preview_dispatch.selection().matched_obligation_count()
    );
}
