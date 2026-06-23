use worth_ui::facade::{
    AppearanceTokenId, CapabilityDiagnosticCode, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface, DensityTokenId,
    ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    WorthUi, WorthUiDropdownAppearanceRequest, WorthUiDropdownProjectionPlanDenial,
    WorthUiHeaderAppearancePlan, WorthUiHeaderAppearancePlanDenial, WorthUiHeaderAppearanceRequest,
    WorthUiHeaderMenuPlan, WorthUiHeaderMenuPlanDenial, WorthUiHeaderMenuProjectionRequest,
    WorthUiHeaderThemePlan, WorthUiHeaderThemePlanDenial, WorthUiHeaderThemeTokenRequest,
    WorthUiRuntimeFactId,
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

    assert_eq!(theme.panel_fill(), "#1F2933");
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
fn header_appearance_rejects_missing_appearance_and_density_tokens() {
    let missing_appearance = WorthUiHeaderAppearancePlan::from_snapshot(
        validation_worth_ui_app().capabilities(),
        header_appearance_request(),
    )
    .expect("baseline appearance plan builds");

    assert!(missing_appearance.dependencies().contains_exact(
        &WorthUiRuntimeFactId::appearance_token(
            &AppearanceTokenId::new("validation.appearance.header.font_size").expect("valid id"),
        )
    ));

    let missing_appearance_token = WorthUiHeaderAppearancePlan::from_snapshot(
        validation_worth_ui_app().capabilities(),
        WorthUiHeaderAppearanceRequest::new(
            AppearanceTokenId::new("validation.appearance.header.missing").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.menu_min_width")
                .expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.border_width").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.panel_shadow").expect("valid id"),
            DensityTokenId::new("validation.density.header.row_padding").expect("valid id"),
            DensityTokenId::new("validation.density.header.container_padding").expect("valid id"),
            DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
        ),
    )
    .expect_err("missing appearance token must fail before renderer execution");

    assert_eq!(
        missing_appearance_token,
        WorthUiHeaderAppearancePlanDenial::MissingAppearanceToken(
            "validation.appearance.header.missing".to_owned(),
        )
    );

    let missing_density_token = WorthUiHeaderAppearancePlan::from_snapshot(
        validation_worth_ui_app().capabilities(),
        WorthUiHeaderAppearanceRequest::new(
            AppearanceTokenId::new("validation.appearance.header.font_size").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.menu_min_width")
                .expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.border_width").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.panel_shadow").expect("valid id"),
            DensityTokenId::new("validation.density.header.missing").expect("valid id"),
            DensityTokenId::new("validation.density.header.container_padding").expect("valid id"),
            DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
        ),
    )
    .expect_err("missing density token must fail before renderer execution");

    assert_eq!(
        missing_density_token,
        WorthUiHeaderAppearancePlanDenial::MissingDensityToken(
            "validation.density.header.missing".to_owned(),
        )
    );
}

#[test]
fn header_appearance_rejects_wrong_appearance_and_density_value_kinds() {
    let app = validation_worth_ui_app();

    let wrong_appearance = WorthUiHeaderAppearancePlan::from_snapshot(
        app.capabilities(),
        WorthUiHeaderAppearanceRequest::new(
            AppearanceTokenId::new("validation.appearance.header.menu_min_width")
                .expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.menu_min_width")
                .expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.border_width").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.panel_shadow").expect("valid id"),
            DensityTokenId::new("validation.density.header.row_padding").expect("valid id"),
            DensityTokenId::new("validation.density.header.container_padding").expect("valid id"),
            DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
        ),
    )
    .expect_err("length token must not masquerade as the header font-size token");

    assert_eq!(
        wrong_appearance,
        WorthUiHeaderAppearancePlanDenial::WrongAppearanceValue {
            id: "validation.appearance.header.menu_min_width".to_owned(),
            expected: "FontSize",
        }
    );

    let wrong_density = WorthUiHeaderAppearancePlan::from_snapshot(
        app.capabilities(),
        WorthUiHeaderAppearanceRequest::new(
            AppearanceTokenId::new("validation.appearance.header.font_size").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.menu_min_width")
                .expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.border_width").expect("valid id"),
            AppearanceTokenId::new("validation.appearance.header.panel_shadow").expect("valid id"),
            DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
            DensityTokenId::new("validation.density.header.container_padding").expect("valid id"),
            DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
        ),
    )
    .expect_err("spacing token must not masquerade as padding authority");

    assert_eq!(
        wrong_density,
        WorthUiHeaderAppearancePlanDenial::WrongDensityValue {
            id: "validation.density.header.control_spacing".to_owned(),
            expected: "Padding",
        }
    );
}

#[test]
fn header_projection_rejects_missing_projection_before_frame_execution() {
    let app = validation_worth_ui_app();
    let denial = WorthUiHeaderMenuPlan::from_snapshot(
        app.capabilities(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "Ghost",
            CommandProjectionId::new("validation.header.menu.ghost").expect("valid projection id"),
            worth_ui::facade::ComponentId::new("validation.component.header.dropdown")
                .expect("valid component id"),
            worth_ui::facade::ComponentId::new("validation.component.header.multi_select_dropdown")
                .expect("valid component id"),
        )],
        header_dropdown_appearance_request(),
    )
    .expect_err("missing projection must be rejected while preparing the header plan");

    assert_eq!(
        denial,
        WorthUiHeaderMenuPlanDenial::Dropdown(
            WorthUiDropdownProjectionPlanDenial::MissingProjection(
                "validation.header.menu.ghost".to_owned(),
            )
        )
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
            worth_ui::facade::ComponentId::new("validation.component.header.dropdown")
                .expect("valid component id"),
            worth_ui::facade::ComponentId::new("validation.component.header.multi_select_dropdown")
                .expect("valid component id"),
        )],
        header_dropdown_appearance_request(),
    )
    .expect_err("projection cannot smuggle a missing command into a frame receipt");

    assert_eq!(
        denial,
        WorthUiHeaderMenuPlanDenial::Dropdown(
            WorthUiDropdownProjectionPlanDenial::MissingProjection(
                projection_id.as_str().to_owned(),
            )
        )
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
            worth_ui::facade::ComponentId::new("validation.component.header.dropdown")
                .expect("valid component id"),
            worth_ui::facade::ComponentId::new("validation.component.header.multi_select_dropdown")
                .expect("valid component id"),
        )],
        header_dropdown_appearance_request(),
    )
    .expect_err("empty projections cannot become visual-only header menus");

    assert_eq!(
        denial,
        WorthUiHeaderMenuPlanDenial::Dropdown(
            WorthUiDropdownProjectionPlanDenial::MissingProjection(
                projection_id.as_str().to_owned(),
            )
        )
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

fn header_appearance_request() -> WorthUiHeaderAppearanceRequest {
    WorthUiHeaderAppearanceRequest::new(
        AppearanceTokenId::new("validation.appearance.header.font_size").expect("valid id"),
        AppearanceTokenId::new("validation.appearance.header.menu_min_width").expect("valid id"),
        AppearanceTokenId::new("validation.appearance.header.border_width").expect("valid id"),
        AppearanceTokenId::new("validation.appearance.header.panel_shadow").expect("valid id"),
        DensityTokenId::new("validation.density.header.row_padding").expect("valid id"),
        DensityTokenId::new("validation.density.header.container_padding").expect("valid id"),
        DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
    )
}

fn header_dropdown_appearance_request() -> WorthUiDropdownAppearanceRequest {
    WorthUiDropdownAppearanceRequest::new(
        AppearanceTokenId::new("validation.appearance.header.menu_min_width").expect("valid id"),
        DensityTokenId::new("validation.density.header.row_padding").expect("valid id"),
        DensityTokenId::new("validation.density.header.control_spacing").expect("valid id"),
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
