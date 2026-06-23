use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiCapabilityReloadStatus, WorthUiHeaderFrameRebindStatus,
    WorthUiPageHostRebindStatus,
};
use worth_ui_validation_app::reload::ValidationReloadEvidenceEntry;
use worth_ui_validation_app::sample_source::VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE;

mod validation_app_reload_fixture;

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn manual_appearance_edit_flows_through_public_app_api_and_runtime_receipts() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    app.apply_manual_appearance_text(
        "\
validation.appearance.header.menu_min_width = 220px
validation.appearance.header.panel_shadow = #102030ff 2px 3px 5px 1px
validation.appearance.header.font_size = 13px
validation.appearance.header.border_width = 1px
",
    );

    let proof = app.proof_snapshot();
    let shadow = proof.header().applied_style().shadow();
    let latest = proof
        .latest_evidence()
        .expect("manual appearance edit should surface runtime proof");

    assert_eq!(shadow.offset, [2, 3]);
    assert_eq!(shadow.blur, 5);
    assert_eq!(shadow.spread, 1);
    let ValidationReloadEvidenceEntry::AppearanceReload {
        status,
        header_rebind,
        page_host_rebind,
        ..
    } = latest
    else {
        panic!("manual appearance edit should remain an appearance reload lane");
    };
    assert_eq!(*status, WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(
        header_rebind
            .as_ref()
            .expect("appearance reload should rebind the header")
            .status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert_eq!(
        page_host_rebind
            .as_ref()
            .expect("appearance reload should keep the page-host receipt visible")
            .status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
}

#[test]
fn manual_command_projection_edit_preserves_runtime_owned_selection_through_public_app_api() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    app.apply_manual_command_projection_text(single_select_projection_source());
    app.apply_manual_command_projection_text(multi_select_projection_source());

    let proof = app.proof_snapshot();
    let file_menu = proof
        .header()
        .menus()
        .iter()
        .find(|menu| menu.title() == "File")
        .expect("file menu should remain visible after projection reload");
    let latest = proof
        .latest_evidence()
        .expect("projection reload should surface runtime proof");

    assert_eq!(
        file_menu.selection_mode(),
        CommandProjectionSelectionMode::MultiSelect
    );
    assert_eq!(
        file_menu.selection_reconciliation_status(),
        worth_ui::facade::WorthUiDropdownSelectionStateStatus::Empty
    );
    let ValidationReloadEvidenceEntry::CommandProjectionReload {
        status,
        header_rebind,
        page_host_rebind,
        ..
    } = latest
    else {
        panic!("manual projection edit should remain a command-projection reload lane");
    };
    assert_eq!(*status, WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(
        header_rebind
            .as_ref()
            .expect("projection reload should rebind the header")
            .status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert_eq!(
        page_host_rebind
            .as_ref()
            .expect("projection reload should preserve page-host truth")
            .status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
}

#[test]
fn multi_select_interaction_updates_visible_selection_through_public_app_api() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    fixture.write_command_projection(&multi_select_projection_source());
    app.run_one_reload_observation_cycle();
    app.select_dropdown_command("validation.header.menu.file", "validation.command.file.new");
    app.select_dropdown_command(
        "validation.header.menu.file",
        "validation.command.file.save",
    );

    let proof = app.proof_snapshot();
    let file_menu = proof
        .header()
        .menus()
        .iter()
        .find(|menu| menu.title() == "File")
        .expect("file menu should remain visible after runtime-owned interaction");

    assert_eq!(
        file_menu.selection_mode(),
        CommandProjectionSelectionMode::MultiSelect
    );
    assert_eq!(
        file_menu.selected_command_ids(),
        &[
            "validation.command.file.new".to_owned(),
            "validation.command.file.save".to_owned(),
        ]
    );
    assert_eq!(
        file_menu.selection_reconciliation_status(),
        worth_ui::facade::WorthUiDropdownSelectionStateStatus::PreservedMulti
    );
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
