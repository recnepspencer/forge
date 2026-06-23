use std::fs;

mod validation_reload_loop_support;

use worth_ui::facade::WorthUiHeaderFrameRebindStatus;
use worth_ui_validation_app::reload::{
    ValidationReloadInput, ValidationReloadInputDenial, ValidationReloadTick,
    ValidationRuntimeReloadTickOutcome,
};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE, VALIDATION_SAMPLE_SOURCE,
};

use validation_reload_loop_support::{
    meaningfully_changed_source, packaged_validation_source_path, runtime_workbench,
    ReloadLoopFixture, SAMPLE_MODULE_PATH,
};

#[test]
fn reload_loop_ignores_unchanged_source_and_theme_inputs() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();

    let first_tick = reload_loop.poll_inputs();
    let second_tick = reload_loop.poll_inputs();

    assert!(matches!(first_tick, ValidationReloadTick::Unchanged(_)));
    assert!(matches!(second_tick, ValidationReloadTick::Unchanged(_)));
}

#[test]
fn reload_loop_reports_source_and_theme_changes_as_one_typed_tick() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fixture.write_source(&meaningfully_changed_source());
    fixture.write_theme(
        "# Worth UI validation header theme.\nvalidation.theme.header.panel = #102030\n",
    );

    let tick = reload_loop.poll_inputs();

    let ValidationReloadTick::Changed(ValidationReloadInput::ObservedAuthoredBatch(batch)) = tick
    else {
        panic!("source and theme edits should be represented as one observed authored batch");
    };
    let source = batch.source();
    let theme = batch
        .theme()
        .expect("theme should be present in the authored batch");
    assert_eq!(source.module_path(), SAMPLE_MODULE_PATH);
    assert_ne!(source.source_text(), VALIDATION_SAMPLE_SOURCE);
    assert!(theme.source_text().contains("#102030"));
}

#[test]
fn reload_loop_reports_simultaneous_appearance_and_density_changes_as_one_typed_tick() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fs::write(
        &fixture.appearance_path,
        "validation.appearance.header.menu_min_width = 260px\n",
    )
    .unwrap();
    fs::write(
        &fixture.density_path,
        "validation.density.header.control_spacing = 12px\n",
    )
    .unwrap();

    let ValidationReloadTick::Changed(ValidationReloadInput::HeaderAppearanceAndDensity {
        appearance,
        density,
    }) = reload_loop.poll_inputs()
    else {
        panic!("simultaneous appearance+density edits should stay atomic at the reload boundary");
    };
    assert_eq!(appearance.source_path(), fixture.appearance_path.as_path());
    assert_eq!(density.source_path(), fixture.density_path.as_path());
}

#[test]
fn reload_loop_batches_source_and_appearance_edits_from_one_observation() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fixture.write_source(&meaningfully_changed_source());
    fs::write(
        &fixture.appearance_path,
        "validation.appearance.header.menu_min_width = 260px\n",
    )
    .unwrap();

    let ValidationReloadTick::Changed(ValidationReloadInput::ObservedAuthoredBatch(batch)) =
        reload_loop.poll_inputs()
    else {
        panic!("source and appearance should be observed as one authored batch");
    };
    let source = batch.source();
    let appearance = batch
        .appearance()
        .expect("appearance should be present in the authored batch");
    assert_eq!(source.module_path(), SAMPLE_MODULE_PATH);
    assert_ne!(source.source_text(), VALIDATION_SAMPLE_SOURCE);
    assert_eq!(appearance.source_path(), fixture.appearance_path.as_path());
}

#[test]
fn reload_loop_observes_command_projection_and_component_files_through_real_paths() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fixture.write_command("validation.command.file.new = Create File\n");
    fixture.write_command_projection(&multi_select_projection_source());
    fixture.write_component(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = background",
    );

    let first_tick = reload_loop.poll_inputs();
    let second_tick = reload_loop.poll_inputs();
    let third_tick = reload_loop.poll_inputs();

    assert!(matches!(
        first_tick,
        ValidationReloadTick::Changed(ValidationReloadInput::HeaderCommands(_))
    ));
    assert!(matches!(
        second_tick,
        ValidationReloadTick::Changed(ValidationReloadInput::HeaderCommandProjections(_))
    ));
    assert!(matches!(
        third_tick,
        ValidationReloadTick::Changed(ValidationReloadInput::HeaderComponents(_))
    ));
}

#[test]
fn unreadable_source_input_is_reported_as_typed_denial() {
    let fixture = ReloadLoopFixture::new();
    let mut reload_loop = fixture.start_loop();
    fixture.delete_source();

    let tick = reload_loop.poll_inputs();

    let ValidationReloadTick::Unreadable(denial) = tick else {
        panic!("missing source must be reported as a typed unreadable input denial");
    };
    assert_eq!(denial.path(), &fixture.source_path);
    assert!(!denial.reason().is_empty());
}

#[test]
fn unreadable_tick_reaches_runtime_outcome_without_mutating_header() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let denial = ValidationReloadInputDenial::unreadable(
        "missing-header.wui",
        &std::io::Error::new(std::io::ErrorKind::NotFound, "missing source"),
    );

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Unreadable(denial.clone()));

    assert_eq!(
        outcome,
        ValidationRuntimeReloadTickOutcome::InputUnreadable(denial)
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_digest);
}

#[test]
fn invalid_source_change_preserves_active_runtime_and_header_frame() {
    let mut workbench = runtime_workbench();
    let before_active = workbench.runtime().inspect_active();
    let before_header = workbench.header_frame_plan().frame_digest();
    let source = worth_ui_validation_app::reload::ValidationSourcePackage::new(
        SAMPLE_MODULE_PATH,
        "app Broken { workspace Missing",
    );

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::SourcePackage(source),
    ));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded {
        evidence,
        phase_execution,
        ..
    } = outcome
    else {
        panic!("invalid source still returns runtime reload evidence");
    };
    let phase_execution =
        phase_execution.expect("denied source reload should still emit phase execution");
    assert!(matches!(
        evidence.status(),
        worth_ui_validation_app::reload::ValidationReloadStatus::Denied(_)
    ));
    assert_eq!(workbench.runtime().inspect_active(), before_active);
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        phase_execution.header_rebind().status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
}

#[test]
fn source_change_activates_then_rebinds_header_through_workbench() {
    let mut workbench = runtime_workbench();
    let before_active = workbench.runtime().inspect_active();
    let source = worth_ui_validation_app::reload::ValidationSourcePackage::new(
        SAMPLE_MODULE_PATH,
        meaningfully_changed_source(),
    );

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::SourcePackage(source),
    ));

    let ValidationRuntimeReloadTickOutcome::SourceReloaded {
        evidence,
        phase_execution,
        ..
    } = outcome
    else {
        panic!("meaningful source change should activate through workbench");
    };
    let phase_execution =
        phase_execution.expect("activated source reload should emit phase execution");
    assert_eq!(
        evidence.status(),
        worth_ui_validation_app::reload::ValidationReloadStatus::Activated
    );
    assert_ne!(workbench.runtime().inspect_active(), before_active);
    let receipt = phase_execution.header_rebind();
    assert_eq!(
        receipt.status(),
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
    );
    assert_eq!(receipt.source_parse_count(), 0);
    assert_eq!(receipt.registry_lookup_count(), 0);
    assert_eq!(receipt.artifact_tree_scan_count(), 0);
}

#[test]
fn packaged_validation_source_matches_embedded_launch_source() {
    let packaged_source = fs::read_to_string(packaged_validation_source_path())
        .expect("packaged source should be readable");

    assert_eq!(packaged_source, VALIDATION_SAMPLE_SOURCE);
}

fn multi_select_projection_source() -> String {
    VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE.replace(
        "validation.header.menu.file = single",
        "validation.header.menu.file = multi",
    )
}
