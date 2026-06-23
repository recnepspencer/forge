use eframe::egui::Color32;
use worth_ui::facade::{
    AppearanceTokenId, CommandProjectionSelectionMode, DensityTokenId,
    WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus, WorthUiRuntimeFactId,
};
use worth_ui_validation_app::reload::{ValidationReloadEvidenceEntry, ValidationReloadStatus};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE, VALIDATION_SAMPLE_SOURCE,
};

mod validation_app_reload_fixture;

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn file_backed_source_edit_updates_app_proof_through_real_reload_loop() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    fixture.write_source(&alternate_surface_source());
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();
    let visible_entry = proof
        .visible_evidence_panel()
        .entries()
        .first()
        .expect("source edit should project visible reload evidence");

    assert_eq!(
        proof.product_summary().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(visible_entry.heading(), "Reload status");
    assert_eq!(
        proof.page_slot_interaction().slots()[0].component_id(),
        "worth.component.primitive_proof"
    );
    assert_eq!(
        proof
            .page_slot_interaction()
            .latest_rebind()
            .expect("source edit should capture page-host rebind proof")
            .status(),
        worth_ui::facade::WorthUiPageHostRebindStatus::ReboundAfterActivation
    );
    let Some(ValidationReloadEvidenceEntry::RuntimeReload {
        status,
        changed_facts,
        ..
    }) = proof.latest_evidence()
    else {
        panic!("source edit should surface runtime reload evidence through the app");
    };
    assert_eq!(*status, ValidationReloadStatus::Activated);
    assert!(
        changed_facts.contains(&WorthUiRuntimeFactId::primitive_interaction(
            "worth.surface.preview.primitive.proof"
        ))
    );
    let phase_execution = proof
        .latest_phase_execution()
        .expect("source edit should preserve aggregate phase execution proof");
    assert_eq!(phase_execution.phase_row_count(), 2);
    assert_eq!(phase_execution.rebuild_attempt_count(), 1);
    assert_eq!(
        phase_execution.page_host_rebind_status(),
        worth_ui::facade::WorthUiPageHostRebindStatus::ReboundAfterActivation
    );
}

#[test]
fn file_backed_theme_edit_updates_visible_header_color() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    let before = app.proof_snapshot().header().applied_style().panel_fill();

    fixture.write_theme("validation.theme.header.panel = #102030\n");
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();

    assert_ne!(before, Color32::from_rgb(16, 32, 48));
    assert_eq!(
        proof.header().applied_style().panel_fill(),
        Color32::from_rgb(16, 32, 48)
    );
    let Some(ValidationReloadEvidenceEntry::ThemeReload { status, .. }) = proof.latest_evidence()
    else {
        panic!("theme file edit should surface theme reload evidence through the app");
    };
    assert_eq!(*status, WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(
        proof
            .latest_phase_execution()
            .expect("theme edit should project phase execution proof")
            .header_rebind_status(),
        worth_ui::facade::WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
}

#[test]
fn file_backed_appearance_edit_projects_typed_shadow_dependency_proof() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    let before = app.proof_snapshot().header().applied_style().shadow();

    fixture.write_appearance(
        "\
validation.appearance.header.menu_min_width = 220px
validation.appearance.header.panel_shadow = #102030ff 2px 3px 5px 1px
validation.appearance.header.font_size = 13px
validation.appearance.header.border_width = 1px
",
    );
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();
    let after = proof.header().applied_style().shadow();
    let shadow = proof.page_slot_interaction().shadow_dependency();

    assert_eq!(before.offset, [0, 1]);
    assert_eq!(after.offset, [2, 3]);
    assert_eq!(after.blur, 5);
    assert_eq!(after.spread, 1);
    assert_eq!(
        shadow.token_id(),
        "validation.appearance.header.panel_shadow"
    );
    assert_eq!(shadow.offset_x_points(), 2);
    assert_eq!(shadow.offset_y_points(), 3);
    assert_eq!(shadow.blur_points(), 5);
    assert_eq!(shadow.spread_points(), 1);
    let Some(ValidationReloadEvidenceEntry::AppearanceReload {
        status,
        changed_facts,
        ..
    }) = proof.latest_evidence()
    else {
        panic!("appearance file edit should surface appearance reload evidence through the app");
    };
    assert_eq!(*status, WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(
        changed_facts,
        &vec![WorthUiRuntimeFactId::appearance_token(
            &AppearanceTokenId::new("validation.appearance.header.panel_shadow").unwrap(),
        )]
    );
}

#[test]
fn file_backed_density_edit_projects_typed_padding_dependency_proof() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    let before = app
        .proof_snapshot()
        .header()
        .applied_style()
        .container_margin();

    fixture.write_density(
        "\
validation.density.header.container_padding = 10px 14px
validation.density.header.control_spacing = 8px
validation.density.header.row_padding = 1px 6px
",
    );
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();
    let after = proof.header().applied_style().container_margin();
    let padding = proof.page_slot_interaction().padding_dependency();

    assert_eq!(before.left, 8);
    assert_eq!(before.top, 4);
    assert_eq!(after.left, 14);
    assert_eq!(after.right, 14);
    assert_eq!(after.top, 10);
    assert_eq!(after.bottom, 10);
    assert_eq!(
        padding.token_id(),
        "validation.density.header.container_padding"
    );
    assert_eq!(padding.top_points(), 10);
    assert_eq!(padding.right_points(), 14);
    assert_eq!(padding.bottom_points(), 10);
    assert_eq!(padding.left_points(), 14);
    let Some(ValidationReloadEvidenceEntry::DensityReload { changed_facts, .. }) =
        proof.latest_evidence()
    else {
        panic!("density file edit should surface density reload evidence through the app");
    };
    assert_eq!(
        changed_facts,
        &vec![WorthUiRuntimeFactId::density_token(
            &DensityTokenId::new("validation.density.header.container_padding").unwrap(),
        )]
    );
}

#[test]
fn file_backed_command_projection_edit_preserves_page_host_proof() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    fixture.write_command_projection(&single_select_projection_source());
    app.run_one_reload_observation_cycle();
    fixture.write_command_projection(&multi_select_projection_source());
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();
    let file_menu = proof
        .header()
        .menus()
        .iter()
        .find(|menu| menu.title() == "File")
        .expect("file menu should remain visible");

    assert_eq!(
        file_menu.selection_mode(),
        CommandProjectionSelectionMode::MultiSelect
    );
    assert_eq!(
        file_menu.component_id(),
        "validation.component.header.multi_select_dropdown"
    );
    let rebind = proof
        .header()
        .latest_rebind()
        .expect("projection edit should preserve header rebind proof");
    assert!(rebind.rows().iter().any(|row| {
        row.projection_identity() == "worth-ui.dropdown:validation.header.menu.file"
    }));
    assert_eq!(
        proof
            .page_slot_interaction()
            .latest_rebind()
            .expect("projection edit should preserve page-host proof")
            .status(),
        worth_ui::facade::WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
}

#[test]
fn file_backed_invalid_appearance_edit_preserves_visible_truth_and_surfaces_denial() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    let before = app
        .proof_snapshot()
        .header()
        .applied_style()
        .font_size_points();

    fixture.write_appearance("validation.appearance.header.font_size = #102030\n");
    app.run_one_reload_observation_cycle();

    let proof = app.proof_snapshot();

    assert_eq!(proof.header().applied_style().font_size_points(), before);
    let Some(ValidationReloadEvidenceEntry::AppearanceReload {
        status,
        header_rebind,
        ..
    }) = proof.latest_evidence()
    else {
        panic!("invalid appearance edit should still surface typed appearance evidence");
    };
    assert_eq!(
        *status,
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::AppearanceSourceParse)
    );
    assert_eq!(
        header_rebind
            .as_ref()
            .expect("denied appearance edit should preserve header rebind proof")
            .status(),
        worth_ui::facade::WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
}

fn alternate_surface_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.from_source_reload\"",
    )
}

fn multi_select_projection_source() -> String {
    VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE.replace(
        "validation.header.menu.file = single",
        "validation.header.menu.file = multi",
    )
}

fn single_select_projection_source() -> String {
    VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE.replace(
        "validation.header.menu.file = multi",
        "validation.header.menu.file = single",
    )
}
