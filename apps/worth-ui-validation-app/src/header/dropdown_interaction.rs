use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiHeaderMenuCommand, WorthUiHeaderMenuGroup,
};

use super::ValidationHeaderSelectionAction;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ValidationDropdownControlKind {
    SingleSelectButton,
    MultiSelectCheckbox,
}

pub(super) fn dropdown_control_kind(
    menu: &WorthUiHeaderMenuGroup,
) -> ValidationDropdownControlKind {
    match menu.selection_mode() {
        CommandProjectionSelectionMode::SingleSelect => {
            ValidationDropdownControlKind::SingleSelectButton
        }
        CommandProjectionSelectionMode::MultiSelect => {
            ValidationDropdownControlKind::MultiSelectCheckbox
        }
    }
}

pub(super) fn selection_action_for_response(
    menu: &WorthUiHeaderMenuGroup,
    command: &WorthUiHeaderMenuCommand,
    activated: bool,
) -> Option<ValidationHeaderSelectionAction> {
    activated
        .then(|| ValidationHeaderSelectionAction::new(menu.projection_id(), command.command_id()))
}

#[cfg(test)]
mod tests {
    use worth_ui::facade::{
        CommandDescriptor, CommandId, CommandProjectionCommandReference,
        CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSelectionMode,
        CommandProjectionSurface, ComponentChildPolicy, ComponentDescriptor, ComponentId,
        ComponentPropSchema, ComponentStateOwnership, WorthUi, WorthUiDropdownAppearanceRequest,
        WorthUiHeaderMenuGroup, WorthUiHeaderMenuPlan, WorthUiHeaderMenuProjectionRequest,
    };

    use super::{
        dropdown_control_kind, selection_action_for_response, ValidationDropdownControlKind,
    };
    use crate::header::ValidationHeaderSelectionAction;

    #[test]
    fn multi_select_dropdown_uses_checkbox_paint_even_without_selected_commands() {
        let menu = file_menu_group(CommandProjectionSelectionMode::MultiSelect);

        assert!(menu.selection_state().selected_command_ids().is_empty());
        assert_eq!(
            dropdown_control_kind(&menu),
            ValidationDropdownControlKind::MultiSelectCheckbox
        );
    }

    #[test]
    fn activated_menu_interaction_emits_runtime_selection_action() {
        let menu = file_menu_group(CommandProjectionSelectionMode::MultiSelect);
        let command = &menu.commands()[0];

        assert_eq!(
            selection_action_for_response(&menu, command, true),
            Some(ValidationHeaderSelectionAction::new(
                "validation.header.menu.file",
                "validation.command.file.new"
            ))
        );
        assert_eq!(selection_action_for_response(&menu, command, false), None);
    }

    fn file_menu_group(mode: CommandProjectionSelectionMode) -> WorthUiHeaderMenuGroup {
        let app = WorthUi::app()
            .register_command(CommandDescriptor::new(
                CommandId::new("validation.command.file.new").unwrap(),
                "New",
            ))
            .register_command_projection(
                CommandProjectionDescriptor::new(
                    CommandProjectionId::new("validation.header.menu.file").unwrap(),
                    CommandProjectionSurface::menu_bar(),
                )
                .with_selection_mode(mode)
                .with_command_reference(
                    CommandProjectionCommandReference::command(
                        CommandId::new("validation.command.file.new").unwrap(),
                    ),
                ),
            )
            .register_component(ComponentDescriptor::new(
                ComponentId::new("validation.component.header.dropdown").unwrap(),
                ComponentPropSchema::named("validation.header.dropdown.props"),
                ComponentChildPolicy::no_children(),
                ComponentStateOwnership::runtime_owned(),
            ))
            .register_component(ComponentDescriptor::new(
                ComponentId::new("validation.component.header.multi_select_dropdown").unwrap(),
                ComponentPropSchema::named("validation.header.multi_select_dropdown.props"),
                ComponentChildPolicy::no_children(),
                ComponentStateOwnership::runtime_owned(),
            ))
            .register_appearance_token(worth_ui::facade::WorthUiAppearanceTokenDescriptor::define(
                worth_ui::facade::AppearanceTokenId::new(
                    "validation.appearance.header.menu_min_width",
                )
                .unwrap(),
                worth_ui::facade::WorthUiAppearanceFamily::Layout,
                worth_ui::facade::WorthUiAppearanceTokenSource::Application,
                worth_ui::facade::WorthUiAppearanceValue::Length(
                    worth_ui::facade::WorthUiLengthValue::from_px("220px").unwrap(),
                ),
            ))
            .register_density_token(worth_ui::facade::WorthUiDensityTokenDescriptor::define(
                worth_ui::facade::DensityTokenId::new("validation.density.header.row_padding")
                    .unwrap(),
                worth_ui::facade::WorthUiDensityFamily::RowPadding,
                worth_ui::facade::WorthUiDensityValue::Padding(
                    worth_ui::facade::WorthUiPaddingValue::from_shorthand_px("1px 6px").unwrap(),
                ),
            ))
            .register_density_token(worth_ui::facade::WorthUiDensityTokenDescriptor::define(
                worth_ui::facade::DensityTokenId::new("validation.density.header.control_spacing")
                    .unwrap(),
                worth_ui::facade::WorthUiDensityFamily::ControlSpacing,
                worth_ui::facade::WorthUiDensityValue::Spacing(
                    worth_ui::facade::WorthUiSpacingValue::from_px("8px").unwrap(),
                ),
            ))
            .freeze();
        let header_plan = WorthUiHeaderMenuPlan::from_snapshot(
            app.capabilities(),
            [WorthUiHeaderMenuProjectionRequest::new(
                "File",
                CommandProjectionId::new("validation.header.menu.file").unwrap(),
                ComponentId::new("validation.component.header.dropdown").unwrap(),
                ComponentId::new("validation.component.header.multi_select_dropdown").unwrap(),
            )],
            WorthUiDropdownAppearanceRequest::new(
                worth_ui::facade::AppearanceTokenId::new(
                    "validation.appearance.header.menu_min_width",
                )
                .unwrap(),
                worth_ui::facade::DensityTokenId::new("validation.density.header.row_padding")
                    .unwrap(),
                worth_ui::facade::DensityTokenId::new("validation.density.header.control_spacing")
                    .unwrap(),
            ),
        )
        .expect("header plan should build");
        header_plan.groups()[0].clone()
    }
}
