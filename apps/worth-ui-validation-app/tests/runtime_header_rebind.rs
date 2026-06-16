use worth_ui::facade::{
    CommandDescriptor, CommandId, CommandProjectionCommandReference, CommandProjectionDescriptor,
    CommandProjectionId, CommandProjectionSurface, ThemeColorValue, ThemeTokenDescriptor,
    ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue, WorthUi, WorthUiApp,
    WorthUiHeaderFrameRebindDenial, WorthUiHeaderFrameRebindRequest,
    WorthUiHeaderFrameRebindStatus, WorthUiHeaderMenuProjectionRequest,
    WorthUiHeaderThemeTokenRequest,
};
use worth_ui_validation_app::reload::{ValidationReloadRequest, ValidationReloadStatus};
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
};
use worth_ui_validation_app::ValidationWorkbenchLaunch;

#[test]
fn equivalent_reload_preserves_header_frame_without_projection_rebuild() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let reload = workbench.prepare_reload(reload_request(VALIDATION_SAMPLE_SOURCE));

    assert_eq!(
        reload.evidence().status(),
        ValidationReloadStatus::EquivalentNoOp
    );
    let receipt = workbench
        .rebind_header_after_reload(reload.evidence())
        .expect("equivalent reload evidence preserves header frame");

    assert_eq!(
        receipt.status(),
        WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload
    );
    assert_eq!(receipt.previous_frame_digest(), before_digest);
    assert_eq!(receipt.rebound_frame_digest(), before_digest);
    assert_eq!(receipt.projection_rebuild_count(), 0);
    assert_eq!(receipt.source_parse_count(), 0);
    assert_eq!(receipt.registry_lookup_count(), 0);
    assert_eq!(receipt.artifact_tree_scan_count(), 0);
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_digest);
}

#[test]
fn denied_reload_preserves_header_frame_without_projection_rebuild() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let reload = workbench.prepare_reload(reload_request("app Broken { workspace Missing"));

    assert!(matches!(
        reload.evidence().status(),
        ValidationReloadStatus::Denied(_)
    ));
    let receipt = workbench
        .rebind_header_after_reload(reload.evidence())
        .expect("denied reload evidence preserves previous header frame");

    assert_eq!(
        receipt.status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );
    assert_eq!(receipt.previous_frame_digest(), before_digest);
    assert_eq!(receipt.rebound_frame_digest(), before_digest);
    assert_eq!(receipt.projection_rebuild_count(), 0);
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_digest);
}

#[test]
fn ready_but_unactivated_reload_cannot_rebind_header_frame() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let reload = workbench.prepare_reload(reload_request(&meaningfully_changed_source()));

    assert_eq!(
        reload.evidence().status(),
        ValidationReloadStatus::ReadyForFrameBoundary,
        "reload denied with detail: {:?}",
        reload.evidence().denial_detail()
    );
    let denial = workbench
        .rebind_header_after_reload(reload.evidence())
        .expect_err("header rebind must wait for runtime activation evidence");

    assert_eq!(denial, WorthUiHeaderFrameRebindDenial::ReloadNotActivated);
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_digest);
}

#[test]
fn activated_reload_preserves_header_when_dependencies_do_not_change() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let reload = workbench.prepare_reload(reload_request(&meaningfully_changed_source()));
    let evidence = workbench
        .activate_reload(reload)
        .expect("ready reload activates through runtime");

    assert_eq!(evidence.status(), ValidationReloadStatus::Activated);
    let receipt = workbench
        .rebind_header_after_reload(&evidence)
        .expect("activated reload evidence is valid for header dependency rebind");

    assert_eq!(
        receipt.status(),
        WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
    );
    assert_eq!(receipt.previous_frame_digest(), before_digest);
    assert_eq!(receipt.rebound_frame_digest(), before_digest);
    assert_eq!(receipt.projection_rebuild_count(), 0);
    assert_eq!(receipt.source_parse_count(), 0);
    assert_eq!(receipt.registry_lookup_count(), 0);
    assert_eq!(receipt.artifact_tree_scan_count(), 0);
}

#[test]
fn activated_reload_cannot_rebind_against_an_unowned_header_theme_snapshot() {
    let mut workbench = runtime_workbench();
    let before_digest = workbench.header_frame_plan().frame_digest();
    let reload = workbench.prepare_reload(reload_request(&meaningfully_changed_source()));
    let evidence = workbench
        .activate_reload(reload)
        .expect("source reload activates before header frame rebind");
    let alternate_app = alternate_header_theme_app("#102030");

    let denial = workbench
        .runtime()
        .rebind_header_frame_after_reload(
            alternate_app.capabilities(),
            workbench.header_frame_plan(),
            header_frame_rebind_request(),
            &evidence,
        )
        .expect_err("activated evidence must not authorize an unrelated theme snapshot");

    assert!(matches!(
        denial,
        WorthUiHeaderFrameRebindDenial::CapabilitySnapshotMismatch { .. }
    ));
    assert_eq!(workbench.header_frame_plan().frame_digest(), before_digest);
}

#[test]
fn foreign_activated_reload_evidence_cannot_rebind_header_frame() {
    let mut source = runtime_workbench();
    let mut target = runtime_workbench();
    let target_before = target.header_frame_plan().frame_digest();
    let reload = source.prepare_reload(reload_request(&meaningfully_changed_source()));
    let foreign_evidence = source
        .activate_reload(reload)
        .expect("source runtime activates reload");

    let denial = target
        .rebind_header_after_reload(&foreign_evidence)
        .expect_err("foreign runtime evidence must not drive target header rebind");

    assert_eq!(
        denial,
        WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch
    );
    assert_eq!(target.header_frame_plan().frame_digest(), target_before);
}

fn runtime_workbench() -> worth_ui_validation_app::ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}

fn reload_request(source: &str) -> ValidationReloadRequest {
    ValidationReloadRequest::from_source_module(VALIDATION_SAMPLE_MODULE_PATH, source)
}

fn header_frame_rebind_request() -> WorthUiHeaderFrameRebindRequest {
    WorthUiHeaderFrameRebindRequest::new(header_requests(), header_theme_request())
}

fn header_requests() -> Vec<WorthUiHeaderMenuProjectionRequest> {
    HEADER_MENUS
        .iter()
        .map(|(title, projection_id, _)| {
            WorthUiHeaderMenuProjectionRequest::new(
                *title,
                CommandProjectionId::new(*projection_id).expect("valid projection id"),
            )
        })
        .collect()
}

fn header_theme_request() -> WorthUiHeaderThemeTokenRequest {
    WorthUiHeaderThemeTokenRequest::new(
        ThemeTokenId::new("validation.theme.header.panel").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.menu").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.menu.hover").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.menu.active").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.text").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.border").expect("valid theme token id"),
    )
}

fn alternate_header_theme_app(panel_fill: &str) -> WorthUiApp {
    let mut builder = WorthUi::app();
    for (_, _, command_id) in HEADER_MENUS {
        builder = builder.register_command(CommandDescriptor::new(
            CommandId::new(*command_id).expect("valid command id"),
            *command_id,
        ));
    }
    for (_title, projection_id, command_id) in HEADER_MENUS {
        builder = builder.register_command_projection(
            CommandProjectionDescriptor::new(
                CommandProjectionId::new(*projection_id).expect("valid projection id"),
                CommandProjectionSurface::menu_bar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(
                CommandId::new(*command_id).expect("valid command id"),
            ))
            .show_shortcuts(),
        );
    }
    for (token_id, color) in header_theme_tokens(panel_fill) {
        builder = builder.register_theme_token(ThemeTokenDescriptor::define(
            ThemeTokenId::new(token_id).expect("valid theme token id"),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex(color).expect("valid theme color")),
        ));
    }
    builder.freeze()
}

fn header_theme_tokens(panel_fill: &str) -> Vec<(&'static str, &str)> {
    vec![
        ("validation.theme.header.panel", panel_fill),
        ("validation.theme.header.menu", "#252526"),
        ("validation.theme.header.menu.hover", "#3E3E42"),
        ("validation.theme.header.menu.active", "#007ACC"),
        ("validation.theme.header.text", "#CCCCCC"),
        ("validation.theme.header.border", "#3F3F46"),
    ]
}

fn meaningfully_changed_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "proof -> validation.surface.header.proof",
        "proof -> validation.surface.header.proof.alt",
    )
}

const HEADER_MENUS: &[(&str, &str, &str)] = &[
    (
        "File",
        "validation.header.menu.file",
        "validation.command.file.new",
    ),
    (
        "Edit",
        "validation.header.menu.edit",
        "validation.command.edit.undo",
    ),
    (
        "Terminal",
        "validation.header.menu.terminal",
        "validation.command.terminal.new",
    ),
    (
        "Help",
        "validation.header.menu.help",
        "validation.command.help.palette",
    ),
];
