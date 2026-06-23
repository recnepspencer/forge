mod primitive_interaction_support;
mod validation_app_reload_fixture;

use worth_ui::facade::{WorthUiInteractionKind, WorthUiInteractionTarget};

use primitive_interaction_support::assert_mounted_kind_target_emits;

#[test]
fn command_toggle_open_and_focus_share_the_generic_interaction_lane() {
    assert_mounted_kind_target_emits(
        WorthUiInteractionKind::Command,
        "interaction_command",
        "validation.command.file.save",
        WorthUiInteractionTarget::Command("validation.command.file.save".to_owned()),
    );
    assert_mounted_kind_target_emits(
        WorthUiInteractionKind::Toggle,
        "interaction_toggle_value",
        "worth.toggle.validation.enabled",
        WorthUiInteractionTarget::Toggle("worth.toggle.validation.enabled".to_owned()),
    );
    assert_mounted_kind_target_emits(
        WorthUiInteractionKind::Open,
        "interaction_open_target",
        "worth.overlay.validation.details",
        WorthUiInteractionTarget::Open("worth.overlay.validation.details".to_owned()),
    );
    assert_mounted_kind_target_emits(
        WorthUiInteractionKind::Focus,
        "interaction_focus_target",
        "worth.focus.validation.primary",
        WorthUiInteractionTarget::Focus("worth.focus.validation.primary".to_owned()),
    );
}
