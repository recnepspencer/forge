use worth_ui::facade::app::WorthUi;

use super::snapshot_fixtures::{command_icon, command_with_icon, component, theme_token};

#[test]
fn snapshot_digest_stable_under_registration_permutation() {
    let first = WorthUi::app()
        .register_theme_token(theme_token("theme.text.primary"))
        .register_icon(command_icon("icon.save"))
        .register_command(command_with_icon("command.save", "icon.save"))
        .register_component(
            component("component.editor")
                .with_command_binding_slot(super::snapshot_fixtures::command_id("command.save"))
                .with_theme_token_dependency(super::snapshot_fixtures::theme_token_id(
                    "theme.text.primary",
                )),
        )
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_component(
            component("component.editor")
                .with_theme_token_dependency(super::snapshot_fixtures::theme_token_id(
                    "theme.text.primary",
                ))
                .with_command_binding_slot(super::snapshot_fixtures::command_id("command.save")),
        )
        .register_command(command_with_icon("command.save", "icon.save"))
        .register_icon(command_icon("icon.save"))
        .register_theme_token(theme_token("theme.text.primary"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_eq!(first.capabilities(), second.capabilities());
    assert_eq!(
        first.capabilities().freeze_report().family_width("command"),
        Some(1)
    );
    assert_eq!(
        first
            .capabilities()
            .freeze_report()
            .family_width("component"),
        Some(1)
    );
    assert_eq!(
        first.capabilities().freeze_report().family_width("icon"),
        Some(1)
    );
}
