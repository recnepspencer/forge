use worth_ui::facade::{
    CommandCategory, CommandProjectionIconLabelPolicy, CommandProjectionOverflowBehavior,
    CommandProjectionReadinessDisplayPolicy, CommandProjectionShortcutVisibility,
    CommandProjectionSurface, IconFamily, WorthUi,
};
use worth_ui_harness::facade::{
    HarnessCommandProjectionVisualRole, HarnessRuntimeOutcomeVisualRole,
    HarnessVisualFoundationBundle, HarnessVisualFoundationRegistration,
};

#[test]
fn visual_foundation_installs_icons_sizing_and_runtime_outcomes_through_public_facade() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let receipt = prepared.receipt().clone();

    let app = WorthUi::app()
        .install_harness_visual_foundation(prepared)
        .freeze();

    assert_eq!(app.capabilities().icons().len(), receipt.icon_count());
    assert_registered_icon_contracts(&app);
    assert_eq!(
        app.capabilities().command_projections().len(),
        receipt.command_projection_count()
    );
    assert_eq!(
        app.capabilities().mosaic_sizing_contracts().len(),
        receipt.theme().sizing_contract_count()
    );
    assert_eq!(
        app.capabilities().runtime_outcome_projections().len(),
        HarnessRuntimeOutcomeVisualRole::REQUIRED.len()
    );
    assert_registered_command_projection_contracts(&app, &receipt);
}

fn assert_registered_icon_contracts(app: &worth_ui::facade::WorthUiApp) {
    for (icon_id, icon_family) in expected_icon_contracts().iter() {
        let icon = app
            .capabilities()
            .icons()
            .get(&worth_ui::facade::IconId::new(*icon_id).unwrap())
            .unwrap_or_else(|| panic!("harness icon {icon_id} was not registered"));
        assert_eq!(icon.family(), icon_family);
        let source = icon.source().expect("harness icon source");
        assert_eq!(source.provider(), "symbol");
        assert_eq!(source.source_key(), *icon_id);
    }
}

fn assert_registered_command_projection_contracts(
    app: &worth_ui::facade::WorthUiApp,
    receipt: &worth_ui_harness::facade::HarnessVisualFoundationReceipt,
) {
    for role in HarnessCommandProjectionVisualRole::REQUIRED {
        assert!(
            receipt.covers_command_projection_role(role),
            "missing command projection role {role:?}"
        );
        let projection = app
            .capabilities()
            .command_projections()
            .get(&worth_ui::facade::CommandProjectionId::new(role.projection_id_text()).unwrap())
            .unwrap_or_else(|| panic!("command projection role {role:?} was not registered"));
        assert_eq!(projection.surface(), &expected_command_surface(role));
        assert_eq!(
            projection.eligible_categories(),
            expected_command_categories(role)
        );
        assert_eq!(
            projection.shortcut_visibility(),
            expected_shortcut_visibility(role)
        );
        assert_eq!(
            projection.readiness_display_policy(),
            expected_readiness_policy(role)
        );
        assert_eq!(
            projection.icon_label_policy(),
            CommandProjectionIconLabelPolicy::PreferCommandIconAndLabel
        );
        assert_eq!(
            projection.overflow_behavior(),
            expected_overflow_behavior(role)
        );
    }
}

fn expected_icon_contracts() -> &'static [(&'static str, IconFamily)] {
    &[
        ("harness.icon.command.palette", IconFamily::Command),
        ("harness.icon.command.run", IconFamily::Command),
        ("harness.icon.surface.sidebar", IconFamily::Surface),
        ("harness.icon.surface.panel", IconFamily::Surface),
        ("harness.icon.surface.overlay", IconFamily::Surface),
        ("harness.icon.runtime.success", IconFamily::RuntimeOutcome),
        ("harness.icon.runtime.warning", IconFamily::RuntimeOutcome),
        ("harness.icon.runtime.danger", IconFamily::RuntimeOutcome),
        ("harness.icon.runtime.disabled", IconFamily::RuntimeOutcome),
        ("harness.icon.runtime.active", IconFamily::RuntimeOutcome),
    ]
}

fn expected_command_surface(role: HarnessCommandProjectionVisualRole) -> CommandProjectionSurface {
    match role {
        HarnessCommandProjectionVisualRole::CommandPalette => {
            CommandProjectionSurface::command_palette()
        }
        HarnessCommandProjectionVisualRole::MenuBar => CommandProjectionSurface::menu_bar(),
        HarnessCommandProjectionVisualRole::ContextMenu => CommandProjectionSurface::context_menu(),
        HarnessCommandProjectionVisualRole::Toolbar => CommandProjectionSurface::toolbar(),
        HarnessCommandProjectionVisualRole::StatusAction => {
            CommandProjectionSurface::status_action()
        }
    }
}

fn expected_command_categories(
    role: HarnessCommandProjectionVisualRole,
) -> &'static [CommandCategory] {
    match role {
        HarnessCommandProjectionVisualRole::CommandPalette => &[
            CommandCategory::Workspace,
            CommandCategory::File,
            CommandCategory::Edit,
            CommandCategory::View,
            CommandCategory::Navigate,
            CommandCategory::Tools,
        ],
        HarnessCommandProjectionVisualRole::MenuBar => &[
            CommandCategory::Application,
            CommandCategory::File,
            CommandCategory::Edit,
            CommandCategory::View,
            CommandCategory::Help,
        ],
        HarnessCommandProjectionVisualRole::ContextMenu => &[
            CommandCategory::Workspace,
            CommandCategory::Edit,
            CommandCategory::Tools,
        ],
        HarnessCommandProjectionVisualRole::Toolbar => {
            &[CommandCategory::Workspace, CommandCategory::View]
        }
        HarnessCommandProjectionVisualRole::StatusAction => {
            &[CommandCategory::Workspace, CommandCategory::Tools]
        }
    }
}

fn expected_shortcut_visibility(
    role: HarnessCommandProjectionVisualRole,
) -> CommandProjectionShortcutVisibility {
    match role {
        HarnessCommandProjectionVisualRole::CommandPalette
        | HarnessCommandProjectionVisualRole::MenuBar
        | HarnessCommandProjectionVisualRole::Toolbar => {
            CommandProjectionShortcutVisibility::VisibleWhenCommandHasShortcut
        }
        HarnessCommandProjectionVisualRole::ContextMenu
        | HarnessCommandProjectionVisualRole::StatusAction => {
            CommandProjectionShortcutVisibility::Hidden
        }
    }
}

fn expected_readiness_policy(
    role: HarnessCommandProjectionVisualRole,
) -> CommandProjectionReadinessDisplayPolicy {
    match role {
        HarnessCommandProjectionVisualRole::CommandPalette
        | HarnessCommandProjectionVisualRole::ContextMenu => {
            CommandProjectionReadinessDisplayPolicy::ShowReadiness
        }
        HarnessCommandProjectionVisualRole::Toolbar
        | HarnessCommandProjectionVisualRole::StatusAction => {
            CommandProjectionReadinessDisplayPolicy::DisableUnavailableCommands
        }
        HarnessCommandProjectionVisualRole::MenuBar => {
            CommandProjectionReadinessDisplayPolicy::HideReadiness
        }
    }
}

fn expected_overflow_behavior(
    role: HarnessCommandProjectionVisualRole,
) -> CommandProjectionOverflowBehavior {
    match role {
        HarnessCommandProjectionVisualRole::CommandPalette => {
            CommandProjectionOverflowBehavior::scroll_within_surface()
        }
        HarnessCommandProjectionVisualRole::MenuBar
        | HarnessCommandProjectionVisualRole::ContextMenu
        | HarnessCommandProjectionVisualRole::Toolbar => {
            CommandProjectionOverflowBehavior::collapse_to_more()
        }
        HarnessCommandProjectionVisualRole::StatusAction => {
            CommandProjectionOverflowBehavior::no_overflow()
        }
    }
}
