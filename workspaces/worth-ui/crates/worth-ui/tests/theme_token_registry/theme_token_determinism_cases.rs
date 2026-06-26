use worth_ui::facade::{ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenSource, WorthUi};

use super::theme_token_assertions::assert_registered_theme_token_ids;
use super::theme_token_fixtures::{
    alias_theme_token, color_theme_token, color_value, theme_token_id,
};

#[test]
fn equivalent_token_graphs_resolve_equivalent_entries() {
    let left = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .register_theme_token(alias_theme_token(
            "theme.text.default",
            "theme.text.primary",
        ))
        .freeze();
    let right = WorthUi::app()
        .register_theme_token(alias_theme_token(
            "theme.text.default",
            "theme.text.primary",
        ))
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .freeze();

    let token_id = theme_token_id("theme.text.default");
    let left_entry = left
        .capabilities()
        .theme_tokens()
        .get_entry(&token_id)
        .unwrap();
    let right_entry = right
        .capabilities()
        .theme_tokens()
        .get_entry(&token_id)
        .unwrap();

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left_entry.resolved_target_id(),
        right_entry.resolved_target_id()
    );
    assert_eq!(
        left_entry.key().projection_basis(),
        right_entry.key().projection_basis()
    );
}

#[test]
fn multi_hop_alias_graph_resolves_to_terminal_token_definition() {
    let app = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .register_theme_token(alias_theme_token(
            "theme.text.default",
            "theme.text.primary",
        ))
        .register_theme_token(alias_theme_token(
            "theme.text.control",
            "theme.text.default",
        ))
        .freeze();

    let control_entry = app
        .capabilities()
        .theme_tokens()
        .get_entry(&theme_token_id("theme.text.control"))
        .unwrap();

    assert_eq!(
        control_entry.resolved_target_id(),
        &theme_token_id("theme.text.primary")
    );
}

#[test]
fn accepted_theme_tokens_are_canonically_ordered_and_inspectable() {
    let app = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.secondary", "#506070"))
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .freeze();

    assert_registered_theme_token_ids(
        app.capabilities().theme_tokens(),
        &["theme.text.primary", "theme.text.secondary"],
    );
    assert!(app
        .capabilities()
        .theme_tokens()
        .get(&theme_token_id("theme.text.primary"))
        .is_some());
}

#[test]
fn all_builtin_theme_token_families_are_admitted() {
    let app = ThemeTokenFamily::all_built_in_for_tests()
        .into_iter()
        .enumerate()
        .fold(WorthUi::app(), |builder, (index, family)| {
            builder.register_theme_token(ThemeTokenDescriptor::define(
                theme_token_id(&format!("theme.family.family_{index}")),
                family,
                ThemeTokenSource::application(),
                color_value("#112233"),
            ))
        })
        .freeze();

    assert_eq!(app.capabilities().theme_tokens().len(), 17);
}

#[test]
fn theme_token_value_change_changes_snapshot_digest() {
    let light = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.primary", "#101820"))
        .freeze();
    let dark = WorthUi::app()
        .register_theme_token(color_theme_token("theme.text.primary", "#f6f7f9"))
        .freeze();

    assert_ne!(light.capabilities().digest(), dark.capabilities().digest());
}

trait BuiltInThemeTokenFamilies {
    fn all_built_in_for_tests() -> Vec<ThemeTokenFamily>;
}

impl BuiltInThemeTokenFamilies for ThemeTokenFamily {
    fn all_built_in_for_tests() -> Vec<ThemeTokenFamily> {
        vec![
            ThemeTokenFamily::surface(),
            ThemeTokenFamily::elevated_surface(),
            ThemeTokenFamily::text(),
            ThemeTokenFamily::muted_text(),
            ThemeTokenFamily::border(),
            ThemeTokenFamily::accent(),
            ThemeTokenFamily::selection(),
            ThemeTokenFamily::focus(),
            ThemeTokenFamily::danger(),
            ThemeTokenFamily::warning(),
            ThemeTokenFamily::success(),
            ThemeTokenFamily::advisory(),
            ThemeTokenFamily::disabled(),
            ThemeTokenFamily::overlay(),
            ThemeTokenFamily::shadow(),
            ThemeTokenFamily::chart_series(),
            ThemeTokenFamily::runtime_state(),
        ]
    }
}
