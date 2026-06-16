use worth_ui::facade::{
    CapabilityDiagnosticCode, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface, ThemeTokenAlias,
    ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, WorthUi,
    WorthUiHeaderMenuPlan, WorthUiHeaderMenuPlanDenial, WorthUiHeaderMenuProjectionRequest,
    WorthUiHeaderThemePlan, WorthUiHeaderThemePlanDenial, WorthUiHeaderThemeTokenRequest,
};
use worth_ui_validation_app::{validation_worth_ui_app, ValidationWorkbenchLaunch};

#[test]
fn header_menu_bar_is_built_from_worth_ui_command_projections() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("header validation app should launch through Worth UI");
    let receipt = launch.header_plan().execute_frame();

    assert_eq!(
        receipt
            .groups()
            .iter()
            .map(|menu| menu.title())
            .collect::<Vec<_>>(),
        ["File", "Edit", "Terminal", "Help"]
    );

    for menu in receipt.groups() {
        assert!(
            !menu.commands().is_empty(),
            "{} menu should be backed by a non-empty Worth UI projection",
            menu.title()
        );
        assert!(
            menu.commands()
                .iter()
                .all(|command| launch.has_command(command.command_id())),
            "{} menu should only contain frozen Worth UI commands",
            menu.title()
        );
    }

    assert_eq!(receipt.source_parse_count(), 0);
    assert_eq!(receipt.registry_lookup_count(), 0);
    assert_eq!(receipt.artifact_tree_scan_count(), 0);
}

#[test]
fn header_theme_is_built_from_worth_ui_theme_tokens() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("header validation app should launch through Worth UI");
    let theme = launch.header_theme_plan().execute_frame();

    assert_eq!(theme.panel_fill(), "#1E1E1E");
    assert_eq!(theme.menu_fill(), "#252526");
    assert_eq!(theme.menu_hover_fill(), "#3E3E42");
    assert_eq!(theme.menu_active_fill(), "#007ACC");
    assert_eq!(theme.text(), "#CCCCCC");
    assert_eq!(theme.border(), "#3F3F46");
    assert_eq!(theme.source_parse_count(), 0);
    assert_eq!(theme.registry_lookup_count(), 0);
    assert_eq!(theme.artifact_tree_scan_count(), 0);
}

#[test]
fn header_theme_projection_rebuild_from_frozen_snapshot_is_deterministic() {
    let app = validation_worth_ui_app();
    let left = WorthUiHeaderThemePlan::from_snapshot(app.capabilities(), header_theme_request())
        .expect("theme should build from frozen theme tokens");
    let right = WorthUiHeaderThemePlan::from_snapshot(app.capabilities(), header_theme_request())
        .expect("theme should rebuild from the same frozen theme tokens");

    assert_eq!(left, right);
}

#[test]
fn header_rebuild_from_frozen_snapshot_is_deterministic() {
    let app = validation_worth_ui_app();
    let left = WorthUiHeaderMenuPlan::from_snapshot(app.capabilities(), header_requests())
        .expect("header should build from frozen command projections");
    let right = WorthUiHeaderMenuPlan::from_snapshot(app.capabilities(), header_requests())
        .expect("header should rebuild from the same frozen command projections");

    assert_eq!(left, right);
}

#[test]
fn header_projection_rejects_missing_projection_before_frame_execution() {
    let app = validation_worth_ui_app();
    let denial = WorthUiHeaderMenuPlan::from_snapshot(
        app.capabilities(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "Ghost",
            CommandProjectionId::new("validation.header.menu.ghost").expect("valid projection id"),
        )],
    )
    .expect_err("missing projection must be rejected while preparing the header plan");

    assert_eq!(
        denial,
        WorthUiHeaderMenuPlanDenial::MissingProjection("validation.header.menu.ghost".to_owned())
    );
}

#[test]
fn header_projection_rejects_projection_that_references_missing_command() {
    let projection_id =
        CommandProjectionId::new("validation.header.menu.hostile").expect("valid projection id");
    let missing_command_id =
        CommandId::new("validation.command.hostile.missing").expect("valid command id");
    let report = WorthUi::app()
        .register_command_projection(
            CommandProjectionDescriptor::new(
                projection_id.clone(),
                CommandProjectionSurface::menu_bar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(
                missing_command_id.clone(),
            )),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_eq!(
        diagnostic_codes(report.registration_diagnostics()),
        [CapabilityDiagnosticCode::MissingDependency]
    );
    assert!(report
        .accepted_snapshot()
        .command_projections()
        .get(&projection_id)
        .is_none());

    let denial = WorthUiHeaderMenuPlan::from_snapshot(
        report.accepted_snapshot(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "Hostile",
            projection_id.clone(),
        )],
    )
    .expect_err("projection cannot smuggle a missing command into a frame receipt");

    assert_eq!(
        denial,
        WorthUiHeaderMenuPlanDenial::MissingProjection(projection_id.as_str().to_owned())
    );
}

#[test]
fn header_projection_rejects_empty_projection_before_renderer_boundary() {
    let projection_id =
        CommandProjectionId::new("validation.header.menu.empty").expect("valid projection id");
    let report = WorthUi::app()
        .register_command_projection(CommandProjectionDescriptor::new(
            projection_id.clone(),
            CommandProjectionSurface::menu_bar(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_eq!(
        diagnostic_codes(report.registration_diagnostics()),
        [CapabilityDiagnosticCode::MissingCommandProjectionEligibility]
    );
    assert!(report
        .accepted_snapshot()
        .command_projections()
        .get(&projection_id)
        .is_none());

    let denial = WorthUiHeaderMenuPlan::from_snapshot(
        report.accepted_snapshot(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "Empty",
            projection_id.clone(),
        )],
    )
    .expect_err("empty projections cannot become visual-only header menus");

    assert_eq!(
        denial,
        WorthUiHeaderMenuPlanDenial::MissingProjection(projection_id.as_str().to_owned())
    );
}

#[test]
fn header_theme_rejects_missing_and_non_color_theme_tokens() {
    let missing_theme = WorthUiHeaderThemePlan::from_snapshot(
        validation_worth_ui_app().capabilities(),
        WorthUiHeaderThemeTokenRequest::new(
            ThemeTokenId::new("validation.theme.header.missing").expect("valid theme token id"),
            ThemeTokenId::new("validation.theme.header.menu").expect("valid theme token id"),
            ThemeTokenId::new("validation.theme.header.menu.hover").expect("valid theme token id"),
            ThemeTokenId::new("validation.theme.header.menu.active").expect("valid theme token id"),
            ThemeTokenId::new("validation.theme.header.text").expect("valid theme token id"),
            ThemeTokenId::new("validation.theme.header.border").expect("valid theme token id"),
        ),
    )
    .expect_err("missing theme token must fail before renderer fallback");

    assert_eq!(
        missing_theme,
        WorthUiHeaderThemePlanDenial::MissingToken("validation.theme.header.missing".to_owned())
    );

    let alias_id =
        ThemeTokenId::new("validation.theme.header.alias").expect("valid theme token id");
    let target_id =
        ThemeTokenId::new("validation.theme.header.target").expect("valid theme token id");
    let app = WorthUi::app()
        .register_theme_token(ThemeTokenDescriptor::alias(
            alias_id.clone(),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(target_id.clone()),
        ))
        .register_theme_token(ThemeTokenDescriptor::define(
            target_id,
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            worth_ui::facade::ThemeTokenValue::color(
                worth_ui::facade::ThemeColorValue::hex("#101820").expect("valid theme color"),
            ),
        ))
        .freeze();

    let non_color_theme = WorthUiHeaderThemePlan::from_snapshot(
        app.capabilities(),
        WorthUiHeaderThemeTokenRequest::new(
            alias_id.clone(),
            alias_id.clone(),
            alias_id.clone(),
            alias_id.clone(),
            alias_id.clone(),
            alias_id.clone(),
        ),
    )
    .expect_err("alias tokens must not masquerade as resolved frame colors");

    assert_eq!(
        non_color_theme,
        WorthUiHeaderThemePlanDenial::NonColorToken(alias_id.as_str().to_owned())
    );
}

fn header_requests() -> Vec<WorthUiHeaderMenuProjectionRequest> {
    [
        ("File", "validation.header.menu.file"),
        ("Edit", "validation.header.menu.edit"),
        ("Terminal", "validation.header.menu.terminal"),
        ("Help", "validation.header.menu.help"),
    ]
    .into_iter()
    .map(|(title, projection_id)| {
        WorthUiHeaderMenuProjectionRequest::new(
            title,
            CommandProjectionId::new(projection_id).expect("valid projection id"),
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

fn diagnostic_codes(
    diagnostics: &[worth_ui::facade::CapabilityRegistrationDiagnostic],
) -> Vec<CapabilityDiagnosticCode> {
    diagnostics
        .iter()
        .map(worth_ui::facade::CapabilityRegistrationDiagnostic::code)
        .collect()
}
