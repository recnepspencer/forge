use worth_ui::facade::{
    IconAccessibilityPosture, IconColorSupport, IconDescriptor, IconFamily, IconSourceDescriptor,
    IconThemePosture, WorthUi,
};

use super::icon_assertions::assert_registered_icon_ids;
use super::icon_fixtures::{color_theme_token, command_icon, icon_id, theme_token_id};

#[test]
fn equivalent_icon_descriptors_produce_equivalent_entries() {
    let first = WorthUi::app()
        .register_icon(command_icon("workspace.icon.save"))
        .register_icon(command_icon("workspace.icon.open"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_icon(command_icon("workspace.icon.open"))
        .register_icon(command_icon("workspace.icon.save"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(first.capabilities().icons(), second.capabilities().icons());
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_icon_ids(
        first.capabilities().icons(),
        &["workspace.icon.open", "workspace.icon.save"],
    );
}

#[test]
fn all_domain_agnostic_builtin_icon_families_are_admitted() {
    let app = WorthUi::app()
        .register_icon(icon("workspace.icon.command", IconFamily::command()))
        .register_icon(icon("workspace.icon.surface", IconFamily::surface()))
        .register_icon(icon("workspace.icon.status", IconFamily::status()))
        .register_icon(icon(
            "workspace.icon.runtime_outcome",
            IconFamily::runtime_outcome(),
        ))
        .register_icon(icon("workspace.icon.navigation", IconFamily::navigation()))
        .register_icon(icon("workspace.icon.toolbar", IconFamily::toolbar()))
        .register_icon(icon(
            "workspace.icon.custom_admitted",
            IconFamily::custom_admitted(),
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().icons().len(), 7);
}

#[test]
fn icon_descriptor_meaning_participates_in_snapshot_digest() {
    let inherited = WorthUi::app()
        .register_icon(command_icon("workspace.icon.save"))
        .freeze()
        .expect("application preparation should succeed");
    let token_driven = WorthUi::app()
        .register_icon(
            IconDescriptor::new(
                icon_id("workspace.icon.save"),
                IconFamily::command(),
                IconSourceDescriptor::symbol("save")
                    .with_color_support(IconColorSupport::theme_token_driven())
                    .with_theme_token(theme_token_id("theme.text.primary")),
            )
            .with_theme_posture(IconThemePosture::theme_token_driven())
            .with_accessibility_posture(IconAccessibilityPosture::semantic_standalone()),
        )
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        inherited.capabilities().icons(),
        token_driven.capabilities().icons()
    );
    assert_ne!(
        inherited.capabilities().digest(),
        token_driven.capabilities().digest()
    );
}

#[test]
fn icon_theme_token_reference_participates_in_snapshot_digest() {
    let primary = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .register_icon(theme_token_icon("theme.text.primary"))
        .freeze()
        .expect("application preparation should succeed");
    let secondary = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.secondary", "#506070"))
        .register_icon(theme_token_icon("theme.text.secondary"))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        primary.capabilities().icons(),
        secondary.capabilities().icons()
    );
    assert_ne!(
        primary.capabilities().digest(),
        secondary.capabilities().digest()
    );
}

fn icon(id: &str, family: IconFamily) -> IconDescriptor {
    IconDescriptor::new(icon_id(id), family, IconSourceDescriptor::symbol(id))
}

fn theme_token_icon(token_id: &str) -> IconDescriptor {
    IconDescriptor::new(
        icon_id("workspace.icon.save"),
        IconFamily::command(),
        IconSourceDescriptor::symbol("save")
            .with_color_support(IconColorSupport::theme_token_driven())
            .with_theme_token(theme_token_id(token_id)),
    )
    .with_theme_posture(IconThemePosture::theme_token_driven())
}
