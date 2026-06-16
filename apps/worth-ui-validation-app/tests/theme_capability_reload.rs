use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus,
    WorthUiHeaderFrameRebindStatus,
};
use worth_ui_validation_app::reload::{
    ValidationCommandProjectionSource, ValidationCommandSource, ValidationReloadInput,
    ValidationReloadTick, ValidationRuntimeReloadTickOutcome, ValidationThemeSource,
};
use worth_ui_validation_app::{ValidationRuntimeWorkbench, ValidationWorkbenchLaunch};

#[test]
fn theme_change_activates_runtime_capability_snapshot_and_rebinds_header() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();

    let outcome = apply_theme_source(&mut workbench, "validation.theme.header.panel = #102030");

    let ValidationRuntimeReloadTickOutcome::ThemeReloaded {
        evidence,
        header_receipt,
    } = outcome
    else {
        panic!("theme-only reload should activate through runtime capability reload");
    };
    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.touched_theme_token_count(), 1);
    assert_eq!(evidence.registry_lookup_count(), 1);
    assert!(
        evidence.theme_token_family_entry_count() >= evidence.touched_theme_token_count(),
        "theme reload evidence must expose full rebuilt family breadth separately from edited delta"
    );
    assert_ne!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_ne!(workbench.header_frame_plan().frame_digest(), before_digest);
    assert_eq!(
        header_receipt
            .expect("activated theme reload should rebind header")
            .status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
}

#[test]
fn multi_token_theme_change_reports_delta_width_without_inflating_lookup_count() {
    let mut workbench = runtime_workbench();

    let outcome = apply_theme_source(
        &mut workbench,
        "\
validation.theme.header.panel = #102030
validation.theme.header.menu = #203040
validation.theme.header.text = #A0B0C0",
    );

    let ValidationRuntimeReloadTickOutcome::ThemeReloaded { evidence, .. } = outcome else {
        panic!("multi-token theme reload should still return runtime evidence");
    };
    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.touched_theme_token_count(), 3);
    assert_eq!(
        evidence.registry_lookup_count(),
        3,
        "indexed admission should lookup each edited token once, not scan all tokens per edit"
    );
    assert_eq!(
        evidence.theme_token_family_entry_count(),
        6,
        "validation app registers six header theme tokens and rebuild breadth should be explicit"
    );
    assert_eq!(evidence.artifact_tree_scan_count(), 0);
}

#[test]
fn malformed_theme_change_is_runtime_denial_without_header_frame_mutation() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = apply_theme_source(
        &mut workbench,
        "validation.theme.header.panel = not-a-color",
    );

    assert_denied_theme_reload_preserves_runtime_and_header(
        outcome,
        &workbench,
        before_snapshot,
        before_header,
        WorthUiCapabilityReloadStage::ThemeTokenSourceParse,
    );
}

#[test]
fn unknown_theme_token_is_admission_denial_without_header_frame_mutation() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = apply_theme_source(&mut workbench, "validation.theme.header.unknown = #102030");

    assert_denied_theme_reload_preserves_runtime_and_header(
        outcome,
        &workbench,
        before_snapshot,
        before_header,
        WorthUiCapabilityReloadStage::ThemeTokenAdmission,
    );
}

#[test]
fn duplicate_theme_token_is_parse_denial_without_header_frame_mutation() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = apply_theme_source(
        &mut workbench,
        "\
validation.theme.header.panel = #102030
validation.theme.header.panel = #203040",
    );

    assert_denied_theme_reload_preserves_runtime_and_header(
        outcome,
        &workbench,
        before_snapshot,
        before_header,
        WorthUiCapabilityReloadStage::ThemeTokenSourceParse,
    );
}

#[test]
fn stale_prepared_theme_reload_cannot_overwrite_newer_active_snapshot() {
    let mut workbench = runtime_workbench();
    let stale_theme = ValidationThemeSource::new("validation.theme.header.panel = #102030");
    let stale_reload = workbench.prepare_theme_capability_reload(&stale_theme);

    assert_eq!(
        stale_reload.evidence().status(),
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
    );

    let newer_outcome =
        apply_theme_source(&mut workbench, "validation.theme.header.panel = #203040");
    let ValidationRuntimeReloadTickOutcome::ThemeReloaded {
        evidence: newer_evidence,
        ..
    } = newer_outcome
    else {
        panic!("newer theme reload should activate before stale prepared reload is retried");
    };
    assert_eq!(
        newer_evidence.status(),
        WorthUiCapabilityReloadStatus::Activated
    );
    let active_after_newer_theme = workbench.runtime().inspect_active();

    let denial = workbench
        .activate_theme_capability_reload(stale_reload)
        .expect_err("stale prepared theme reload must not overwrite newer snapshot truth");

    assert_eq!(denial, WorthUiCapabilityReloadStage::ActiveSnapshotDrift);
    assert_eq!(
        workbench.runtime().inspect_active(),
        active_after_newer_theme
    );
}

#[test]
fn command_label_change_activates_runtime_snapshot_and_rebinds_header_menu() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = apply_command_source(
        &mut workbench,
        "\
validation.command.file.new = Create File
validation.command.file.open = Open File
validation.command.file.save = Save
validation.command.file.exit = Exit
validation.command.edit.undo = Undo
validation.command.edit.redo = Redo
validation.command.edit.cut = Cut
validation.command.edit.copy = Copy
validation.command.edit.paste = Paste
validation.command.terminal.new = New Terminal
validation.command.terminal.split = Split Terminal
validation.command.terminal.clear = Clear Terminal
validation.command.help.palette = Command Palette
validation.command.help.docs = Worth UI Docs
validation.command.help.about = About Worth UI",
    );

    let ValidationRuntimeReloadTickOutcome::CommandReloaded {
        evidence,
        header_receipt,
    } = outcome
    else {
        panic!("command source reload should activate through runtime capability reload");
    };
    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.touched_theme_token_count(), 15);
    assert_ne!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_ne!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        header_receipt
            .expect("command reload should rebind dependent header")
            .status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    let file_menu = workbench
        .header_frame_plan()
        .menu_plan()
        .execute_frame()
        .groups()
        .iter()
        .find(|group| group.title() == "File")
        .expect("file menu remains projected")
        .clone();
    assert_eq!(file_menu.commands()[0].label(), "Create File");
}

#[test]
fn command_projection_policy_change_rebinds_header_without_local_style_state() {
    let mut workbench = runtime_workbench();
    let before_header = workbench.header_frame_plan().frame_digest();

    let outcome = apply_command_projection_source(
        &mut workbench,
        "\
validation.header.menu.file = multi
validation.header.menu.edit = single
validation.header.menu.terminal = single
validation.header.menu.help = single",
    );

    let ValidationRuntimeReloadTickOutcome::CommandProjectionReloaded {
        evidence,
        header_receipt,
    } = outcome
    else {
        panic!("projection source reload should activate through runtime capability reload");
    };
    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(evidence.touched_theme_token_count(), 4);
    assert_ne!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        header_receipt
            .expect("projection reload should rebind dependent header")
            .status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    let file_menu = workbench
        .header_frame_plan()
        .menu_plan()
        .execute_frame()
        .groups()
        .iter()
        .find(|group| group.title() == "File")
        .expect("file menu remains projected")
        .clone();
    assert_eq!(
        file_menu.selection_mode(),
        CommandProjectionSelectionMode::MultiSelect
    );
}

fn apply_theme_source(
    workbench: &mut ValidationRuntimeWorkbench,
    source_text: &str,
) -> ValidationRuntimeReloadTickOutcome {
    workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderTheme(ValidationThemeSource::new(source_text)),
    ))
}

fn apply_command_source(
    workbench: &mut ValidationRuntimeWorkbench,
    source_text: &str,
) -> ValidationRuntimeReloadTickOutcome {
    workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderCommands(ValidationCommandSource::new(source_text)),
    ))
}

fn apply_command_projection_source(
    workbench: &mut ValidationRuntimeWorkbench,
    source_text: &str,
) -> ValidationRuntimeReloadTickOutcome {
    workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderCommandProjections(ValidationCommandProjectionSource::new(
            source_text,
        )),
    ))
}

fn assert_denied_theme_reload_preserves_runtime_and_header(
    outcome: ValidationRuntimeReloadTickOutcome,
    workbench: &ValidationRuntimeWorkbench,
    before_snapshot: u64,
    before_header: u64,
    expected_stage: WorthUiCapabilityReloadStage,
) {
    let ValidationRuntimeReloadTickOutcome::ThemeReloaded {
        evidence,
        header_receipt,
    } = outcome
    else {
        panic!("denied theme-only reload should return typed capability evidence");
    };
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(expected_stage)
    );
    assert_eq!(evidence.touched_theme_token_count(), 0);
    assert_eq!(evidence.registry_lookup_count(), 0);
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_header);
    assert_eq!(
        header_receipt
            .expect("denied theme reload should preserve header")
            .status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
}

fn runtime_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}
