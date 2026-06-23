use worth_ui::facade::{
    WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadFamilyStatus,
    WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus,
};

mod capability_reload_family_batch_support;

use capability_reload_family_batch_support::{command_package, runtime_workbench, theme_package};

#[test]
fn duplicate_family_in_batch_is_denied_before_a_candidate_can_activate() {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let prepared =
        workbench
            .runtime()
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::batch([
                WorthUiCapabilityReloadRequest::from_theme_tokens(theme_package(
                    "validation.theme.header.panel = #102030",
                )),
                WorthUiCapabilityReloadRequest::from_theme_tokens(theme_package(
                    "validation.theme.header.text = #A0B0C0",
                )),
            ]));

    let evidence = prepared.evidence();
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(
            WorthUiCapabilityReloadStage::DuplicateCapabilityFamily
        )
    );
    assert_eq!(evidence.family_rows().len(), 1);
    assert_eq!(
        evidence.source_parse_count(),
        0,
        "duplicate-family preflight must deny before deriving any family candidate"
    );
    assert_eq!(
        evidence.family_rows()[0].family(),
        WorthUiCapabilityReloadFamilyKind::ThemeTokens
    );
    assert_eq!(
        evidence.family_rows()[0].status(),
        WorthUiCapabilityReloadFamilyStatus::Denied
    );
    assert!(evidence.changed_facts().is_empty());

    assert_eq!(
        workbench
            .activate_capability_reload(prepared)
            .expect_err("denied duplicate batch cannot be activated"),
        WorthUiCapabilityReloadStage::MissingReadyActivation
    );
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
}

#[test]
fn unknown_command_in_batch_is_denied_without_runtime_mutation() {
    let workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let prepared =
        workbench
            .runtime()
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::batch([
                WorthUiCapabilityReloadRequest::from_theme_tokens(theme_package(
                    "validation.theme.header.panel = #102030",
                )),
                WorthUiCapabilityReloadRequest::from_commands(command_package(
                    "validation.command.file.unknown = Surprise",
                )),
            ]));

    let evidence = prepared.evidence();
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::CommandAdmission)
    );
    assert_eq!(
        evidence.family_rows()[0].family(),
        WorthUiCapabilityReloadFamilyKind::Commands
    );
    assert_eq!(evidence.edited_delta_width(), 0);
    assert!(evidence.changed_facts().is_empty());
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
}

#[test]
fn stale_prepared_command_family_cannot_overwrite_newer_capability_snapshot() {
    let mut workbench = runtime_workbench();
    let stale = workbench.runtime().prepare_capability_reload(
        WorthUiCapabilityReloadRequest::from_commands(command_package(
            "validation.command.file.new = Create File",
        )),
    );
    assert_eq!(
        stale.evidence().status(),
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
    );

    let newer = workbench.runtime().prepare_capability_reload(
        WorthUiCapabilityReloadRequest::from_theme_tokens(theme_package(
            "validation.theme.header.panel = #102030",
        )),
    );
    workbench
        .activate_capability_reload(newer)
        .expect("newer theme reload activates first");
    let active_after_newer = workbench.runtime().inspect_active();

    assert_eq!(
        workbench
            .activate_capability_reload(stale)
            .expect_err("stale command reload cannot overwrite newer active snapshot"),
        WorthUiCapabilityReloadStage::ActiveSnapshotDrift
    );
    assert_eq!(workbench.runtime().inspect_active(), active_after_newer);
}

#[test]
fn duplicate_command_edit_in_batch_is_parse_denied_without_command_candidate_width() {
    let workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();
    let prepared = workbench.runtime().prepare_capability_reload(
        WorthUiCapabilityReloadRequest::from_commands(command_package(
            "\
validation.command.file.new = Create File
validation.command.file.new = Create Again",
        )),
    );

    let evidence = prepared.evidence();
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::CommandSourceParse)
    );
    assert_eq!(
        evidence.family_rows()[0].family(),
        WorthUiCapabilityReloadFamilyKind::Commands
    );
    assert_eq!(evidence.edited_delta_width(), 0);
    assert_eq!(evidence.registry_lookup_count(), 0);
    assert!(evidence.changed_facts().is_empty());
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
}
