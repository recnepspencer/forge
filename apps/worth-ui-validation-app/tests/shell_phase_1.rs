use egui::Color32;
use worth_ui::facade::{
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
};
use worth_ui_harness::facade::{
    HarnessDensity, HarnessVisualFoundationBundle, HarnessVisualTokenRole,
};
use worth_ui_validation_app::shell::{
    StableShellSurfaceId, StableShellSurfaceManifest, StableShellSurfacePlacement, ValidationPageId,
};
use worth_ui_validation_app::theme::ValidationWorkbenchTheme;
use worth_ui_validation_app::{ValidationWorkbenchApp, ValidationWorkbenchLaunch};

#[test]
fn validation_app_workbench_app_loads_first_page_with_all_shell_surfaces() {
    let app = prepared_app();
    let snapshot = app.snapshot();

    for surface in StableShellSurfaceManifest::REQUIRED.surfaces() {
        assert!(
            snapshot.contains_surface(surface.id()),
            "missing required shell surface {}",
            surface.label()
        );
    }
    assert_eq!(snapshot.active_page_label(), "Surface atlas");
    assert_eq!(
        snapshot.selected_scenario(),
        "validation.scenario.surface-atlas"
    );
}

#[test]
fn validation_app_page_navigation_preserves_active_run_context() {
    let mut app = prepared_app();
    let before = app.snapshot();
    let before_observation = app.launch().runtime().inspect_active();
    let before_density = app.launch().density();
    let before_theme = app.launch().render_theme().clone();
    let before_latest_receipt = app.launch().latest_run_receipt();

    app.launch_mut()
        .navigation_mut()
        .select_page(ValidationPageId::Evidence);

    let after = app.snapshot();
    let after_observation = app.launch().runtime().inspect_active();

    assert_eq!(after.active_page_label(), "Evidence");
    assert_eq!(before.selected_scenario(), after.selected_scenario());
    assert_eq!(before.active_plan_digest(), after.active_plan_digest());
    assert_eq!(before_observation, after_observation);
    assert_eq!(before_density, app.launch().density());
    assert_eq!(&before_theme, app.launch().render_theme());
    assert_eq!(before_latest_receipt, app.launch().latest_run_receipt());
}

#[test]
fn validation_app_shell_surface_ids_are_stable_across_restart() {
    let before_restart = StableShellSurfaceManifest::REQUIRED
        .surfaces()
        .iter()
        .map(|surface| surface.id().as_str())
        .collect::<Vec<_>>();
    let after_restart = prepared_app()
        .snapshot()
        .rendered_surface_ids()
        .iter()
        .map(|surface| surface.as_str())
        .collect::<Vec<_>>();

    assert_eq!(before_restart, after_restart);
}

#[test]
fn validation_app_embedded_surfaces_have_declared_render_parents() {
    assert_eq!(
        placement_for(StableShellSurfaceId::COMMAND_PALETTE),
        StableShellSurfacePlacement::Embedded {
            parent: StableShellSurfaceId::TOOLBAR,
        }
    );
    assert_eq!(
        placement_for(StableShellSurfaceId::EDITOR_TABS),
        StableShellSurfacePlacement::Embedded {
            parent: StableShellSurfaceId::PAGE_HOST,
        }
    );
}

#[test]
fn validation_app_shell_does_not_use_marketing_or_demo_only_routes() {
    let app = prepared_app();
    let snapshot = app.snapshot();

    assert_ne!(snapshot.active_page_label(), "Landing");
    assert_ne!(snapshot.active_page_label(), "Hero");
    assert!(snapshot.contains_surface(StableShellSurfaceId::PAGE_HOST));
    assert!(snapshot.contains_surface(StableShellSurfaceId::INSPECTOR));
    assert!(snapshot.contains_surface(StableShellSurfaceId::BOTTOM_TIMELINE));
}

#[test]
fn validation_app_theme_is_derived_from_registered_token_descriptors() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let mut descriptors = prepared.theme_tokens().to_vec();
    replace_token_color(
        &mut descriptors,
        HarnessVisualTokenRole::EditorCanvas,
        "#102030",
    );

    let theme = ValidationWorkbenchTheme::from_theme_tokens(&descriptors)
        .expect("altered token descriptors should still define a valid theme");

    assert_eq!(theme.editor_canvas(), Color32::from_rgb(0x10, 0x20, 0x30));
    assert_eq!(
        prepared.receipt().theme().density(),
        HarnessDensity::DEFAULT
    );
}

fn prepared_app() -> ValidationWorkbenchApp {
    ValidationWorkbenchApp::new(
        ValidationWorkbenchLaunch::new()
            .prepare()
            .expect("validation launch should prepare"),
    )
}

fn placement_for(surface_id: StableShellSurfaceId) -> StableShellSurfacePlacement {
    StableShellSurfaceManifest::REQUIRED
        .surface(surface_id)
        .expect("required surface should exist")
        .placement()
}

fn replace_token_color(
    descriptors: &mut [ThemeTokenDescriptor],
    role: HarnessVisualTokenRole,
    hex: &'static str,
) {
    let original = descriptors
        .iter()
        .find(|descriptor| descriptor.id().as_str() == role.token_id_text())
        .expect("default foundation should include role token");
    let replacement = ThemeTokenDescriptor::define(
        ThemeTokenId::new(role.token_id_text()).expect("valid theme token id"),
        original.family().clone(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(ThemeColorValue::hex(hex).expect("valid replacement color")),
    );
    let slot = descriptors
        .iter_mut()
        .find(|descriptor| descriptor.id().as_str() == role.token_id_text())
        .expect("default foundation should include role token");
    *slot = replacement;
}
