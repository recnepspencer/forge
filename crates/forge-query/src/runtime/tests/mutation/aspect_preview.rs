use super::super::support::*;

#[test]
fn preview_insert_uses_aspect_native_authoring_and_stays_preview_local() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-aspect-insert")
        .expect("task runtime should open a named workspace");
    let mut preview = workspace
        .preview_with_options(
            test_session_label("task-preview"),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");

    let receipt = preview
        .insert("Task", |task| {
            task.aspect(
                test_aspect_touch("identity.id"),
                test_string_aspect_value("preview-task-1"),
            )
            .aspect(
                test_aspect_touch("title.value"),
                test_string_aspect_value("Preview title"),
            )
        })
        .expect("preview insert should stage");
    let outcome = preview.discard();

    assert_eq!(
        receipt.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(receipt.basis_lane(), ForgeQueryAuthorityLane::PreviewTruth);
    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Insert);
    assert_eq!(
        receipt.terminal_declared_collection_projection(),
        Some("Task")
    );
    assert_eq!(
        receipt.terminal_target_collection_projection(),
        Some("Task")
    );
    assert_eq!(
        receipt.target_evidence().declared().target_class(),
        ForgeQueryMutationTargetClass::Collection
    );
    assert_eq!(
        receipt.target_evidence().resolved().target_class(),
        ForgeQueryMutationTargetClass::Collection
    );
    assert!(receipt.causality_evidence().is_none());
    assert!(receipt.provenance_evidence().is_none());
    assert_eq!(
        receipt.deltas()[0].admitted_touched_aspects(),
        test_aspect_touches(["identity.id", "title.value"]).as_slice()
    );
    assert_eq!(outcome.authoritative_residue_count(), 0);
}
