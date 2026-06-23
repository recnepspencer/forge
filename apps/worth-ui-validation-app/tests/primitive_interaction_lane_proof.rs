mod primitive_interaction_support;
mod validation_app_reload_fixture;

use worth_ui::facade::{WorthUiInteractionKind, WorthUiInteractionTarget};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

use primitive_interaction_support::{
    assert_rebinds_primitive_interaction_fact, launch_interaction_workbench,
    prepare_interaction_reload, submit_centered_primitive, PRIMITIVE_SURFACE,
};

#[test]
fn click_emits_generic_submit_receipt_with_active_authored_payload() {
    let mut workbench = launch_interaction_workbench();
    let receipt = submit_centered_primitive(&mut workbench);

    assert_eq!(receipt.kind(), WorthUiInteractionKind::Submit);
    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(
        receipt
            .payload()
            .field("payload")
            .expect("submit receipt carries payload")
            .as_text(),
        "submit.secondary"
    );
}

#[test]
fn payload_edit_changes_next_receipt_and_rebinds_interaction_fact() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_payload",
            "\"submit.changed\"",
        )],
    );
    assert_rebinds_primitive_interaction_fact(&prepared);

    workbench
        .activate_reload(prepared)
        .expect("interaction payload reload activates");
    let receipt = submit_centered_primitive(&mut workbench);

    assert_eq!(
        receipt
            .payload()
            .field("payload")
            .expect("submit receipt carries payload")
            .as_text(),
        "submit.changed"
    );
}

#[test]
fn target_edit_changes_next_receipt_without_component_code_rebuild() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_target",
            "worth.surface.preview.primitive.alternate",
        )],
    );
    assert_rebinds_primitive_interaction_fact(&prepared);

    workbench
        .activate_reload(prepared)
        .expect("interaction target reload activates");
    let receipt = submit_centered_primitive(&mut workbench);

    assert_eq!(
        receipt.target(),
        &WorthUiInteractionTarget::Surface("worth.surface.preview.primitive.alternate".to_owned())
    );
}
