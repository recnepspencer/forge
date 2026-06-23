mod validation_app_reload_fixture;

use eframe::egui::Color32;
use worth_ui_validation_app::launch::ValidationObservedStartupFileKind;
use worth_ui_validation_app::ValidationWorkbenchLaunch;

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn observed_startup_applies_current_theme_file_on_first_frame() {
    let fixture = ValidationAppReloadFixture::new();
    fixture.write_theme(
        "\
validation.theme.header.panel = #102030
validation.theme.header.menu = #203040
validation.theme.header.menu.hover = #304050
validation.theme.header.menu.active = #405060
validation.theme.header.text = #A0B0C0
validation.theme.header.border = #506070
",
    );
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_workspace_root(fixture.workspace_root())
        .expect("startup should load authored inputs from observed files");
    let theme = launch.header_theme_plan().execute_frame();

    assert_eq!(theme.panel_fill(), "#102030");
    assert_eq!(theme.menu_fill(), "#203040");
    assert_eq!(theme.menu_hover_fill(), "#304050");
    assert_eq!(theme.menu_active_fill(), "#405060");
    assert_eq!(theme.text(), "#A0B0C0");
    assert_eq!(theme.border(), "#506070");
    assert_eq!(
        launch
            .authored_inputs()
            .theme()
            .expect("theme file should be present at startup")
            .source_path(),
        fixture.theme_path.as_path()
    );
    let observed = launch
        .observed_startup()
        .expect("observed startup should carry file parity evidence");
    assert_eq!(observed.rows().len(), 7);
    assert!(observed.rows().iter().any(|row| {
        row.kind() == ValidationObservedStartupFileKind::Theme
            && row.path() == fixture.theme_path.as_path()
            && row.source_digest()
                == launch
                    .authored_inputs()
                    .theme()
                    .expect("theme file should be loaded")
                    .source_digest()
    }));
}

#[test]
fn startup_seeded_reload_loop_does_not_reapply_already_loaded_theme() {
    let fixture = ValidationAppReloadFixture::new();
    fixture.write_theme("validation.theme.header.panel = #102030\n");

    let mut app = fixture.build_app();
    let before = app.proof_snapshot();

    assert_eq!(
        before.header().applied_style().panel_fill(),
        Color32::from_rgb(0x10, 0x20, 0x30)
    );
    assert!(before.latest_evidence().is_none());

    app.run_one_reload_observation_cycle();

    let after = app.proof_snapshot();
    assert_eq!(
        after.header().applied_style().panel_fill(),
        Color32::from_rgb(0x10, 0x20, 0x30)
    );
    assert_eq!(
        after.header().frame_digest(),
        before.header().frame_digest()
    );
    assert!(after.latest_evidence().is_none());
    assert!(after.latest_phase_execution().is_none());
    assert_eq!(
        after
            .observed_startup()
            .expect("startup app proof should preserve observed file parity")
            .rows()
            .len(),
        7
    );
}
