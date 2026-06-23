use crate::reload::ValidationManualReloadEdit;

use super::{ValidationManualAppAction, ValidationManualFlowId};

pub(crate) fn actions_for_flow(flow_id: ValidationManualFlowId) -> Vec<ValidationManualAppAction> {
    let mut actions = vec![ValidationManualAppAction::ResetToBaseline];
    match flow_id {
        ValidationManualFlowId::HeaderText => {
            actions.extend(reload_step(ValidationManualReloadEdit::command_file(
                "header.commands",
                command_reload_source(),
            )));
        }
        ValidationManualFlowId::HeaderColor => {
            actions.extend(reload_step(ValidationManualReloadEdit::theme_file(
                "header.theme",
                "validation.theme.header.panel = #102030\n",
            )));
        }
        ValidationManualFlowId::HeaderFontSize => {
            actions.extend(reload_step(ValidationManualReloadEdit::appearance_file(
                "header.appearance",
                "\
validation.appearance.header.menu_min_width = 220px
validation.appearance.header.panel_shadow = #00000066 0px 1px 3px 0px
validation.appearance.header.font_size = 15px
validation.appearance.header.border_width = 1px
",
            )));
        }
        ValidationManualFlowId::DropdownRowPadding => {
            actions.extend(reload_step(ValidationManualReloadEdit::density_file(
                "header.density",
                "\
validation.density.header.container_padding = 4px 8px
validation.density.header.control_spacing = 8px
validation.density.header.row_padding = 3px 10px
",
            )));
        }
        ValidationManualFlowId::DropdownContainerPadding => {
            actions.extend(reload_step(ValidationManualReloadEdit::density_file(
                "header.density",
                "\
validation.density.header.container_padding = 10px 14px
validation.density.header.control_spacing = 8px
validation.density.header.row_padding = 1px 6px
",
            )));
        }
        ValidationManualFlowId::DropdownShadow => {
            actions.extend(reload_step(ValidationManualReloadEdit::appearance_file(
                "header.appearance",
                "\
validation.appearance.header.menu_min_width = 220px
validation.appearance.header.panel_shadow = #102030ff 2px 3px 5px 1px
validation.appearance.header.font_size = 13px
validation.appearance.header.border_width = 1px
",
            )));
        }
        ValidationManualFlowId::SingleToMultiMode => {
            actions.extend(reload_step(
                ValidationManualReloadEdit::command_projection_file(
                    "header.projections",
                    single_select_projection_source(),
                ),
            ));
            actions.push(ValidationManualAppAction::select_dropdown_command(
                "validation.header.menu.file",
                "validation.command.file.new",
            ));
            actions.extend(reload_step(
                ValidationManualReloadEdit::command_projection_file(
                    "header.projections",
                    single_to_multi_projection_source(),
                ),
            ));
        }
        ValidationManualFlowId::MultiToSingleReconciliation => {
            actions.extend(reload_step(
                ValidationManualReloadEdit::command_projection_file(
                    "header.projections",
                    single_to_multi_projection_source(),
                ),
            ));
            actions.push(ValidationManualAppAction::select_dropdown_command(
                "validation.header.menu.file",
                "validation.command.file.new",
            ));
            actions.push(ValidationManualAppAction::select_dropdown_command(
                "validation.header.menu.file",
                "validation.command.file.open",
            ));
            actions.extend(reload_step(
                ValidationManualReloadEdit::command_projection_file(
                    "header.projections",
                    single_select_projection_source(),
                ),
            ));
        }
        ValidationManualFlowId::ComponentDescriptor => {
            actions.extend(reload_step(ValidationManualReloadEdit::component_file(
                "header.components",
                "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive
command_binding_slots = validation.command.header.refresh
",
            )));
        }
        ValidationManualFlowId::PageSlotReassignment => {
            actions.extend(reload_step(ValidationManualReloadEdit::source_file(
                "header.wui",
                proof_component_selection_source(),
            )));
        }
        ValidationManualFlowId::LayoutGap => {
            actions.extend(reload_step(ValidationManualReloadEdit::source_file(
                "header.wui",
                layout_gap_source(),
            )));
        }
        ValidationManualFlowId::ThreadInset => {
            actions.extend(reload_step(ValidationManualReloadEdit::source_file(
                "header.wui",
                thread_inset_source(),
            )));
        }
        ValidationManualFlowId::InvalidAppearanceDenial => {
            actions.extend(reload_step(ValidationManualReloadEdit::appearance_file(
                "header.appearance",
                "validation.appearance.header.font_size = #102030\n",
            )));
        }
        ValidationManualFlowId::EquivalentCanonicalAppearance => {
            actions.extend(reload_step(ValidationManualReloadEdit::appearance_file(
                "header.appearance",
                "validation.appearance.header.menu_min_width = 220.0px\n",
            )));
        }
        ValidationManualFlowId::MixedProductStorm => {
            actions.extend(reload_step(ValidationManualReloadEdit::source_file(
                "header.wui",
                alternate_surface_source(),
            )));
            actions.extend(reload_step(ValidationManualReloadEdit::command_file(
                "header.commands",
                command_reload_source(),
            )));
            actions.extend(reload_step(
                ValidationManualReloadEdit::command_projection_file(
                    "header.projections",
                    single_to_multi_projection_source(),
                ),
            ));
            actions.extend(reload_step(ValidationManualReloadEdit::component_file(
                "header.components",
                "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive
command_binding_slots = validation.command.header.refresh
",
            )));
            actions.extend(reload_step(
                ValidationManualReloadEdit::appearance_and_density_files(
                    "header.appearance",
                    "\
validation.appearance.header.menu_min_width = 260px
validation.appearance.header.panel_shadow = #00000066 0px 1px 3px 0px
validation.appearance.header.font_size = 13px
validation.appearance.header.border_width = 1px
",
                    "header.density",
                    "\
validation.density.header.container_padding = 4.0px 8.0px 4.0px 8.0px
validation.density.header.control_spacing = 8.0px
validation.density.header.row_padding = 1.0px 6.0px
",
                ),
            ));
            actions.extend(reload_step(ValidationManualReloadEdit::appearance_file(
                "header.appearance",
                "validation.appearance.header.font_size = #102030\n",
            )));
        }
    }
    actions
}

fn reload_step(edit: ValidationManualReloadEdit) -> [ValidationManualAppAction; 2] {
    [
        ValidationManualAppAction::StageReloadEdit(edit),
        ValidationManualAppAction::SubmitStagedReloadEdit,
    ]
}

fn command_reload_source() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/theme/header.commands"
    ))
    .replace(
        "validation.command.file.save = Save All",
        "validation.command.file.save = Save Everything",
    )
}

fn single_to_multi_projection_source() -> &'static str {
    "\
validation.header.menu.file = multi
validation.header.menu.edit = single
validation.header.menu.terminal = single
validation.header.menu.help = single
"
}

fn single_select_projection_source() -> &'static str {
    "\
validation.header.menu.file = single
validation.header.menu.edit = single
validation.header.menu.terminal = single
validation.header.menu.help = single
"
}

fn alternate_surface_source() -> String {
    crate::sample_source::VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.storm\"",
    )
}

fn proof_component_selection_source() -> String {
    crate::sample_source::VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_target worth.surface.preview.primitive.proof",
        "interaction_target worth.surface.preview.primitive.proof.alt",
    )
}

fn layout_gap_source() -> String {
    crate::sample_source::VALIDATION_SAMPLE_SOURCE
        .replace("column gap(0) padding(0) {", "column gap(30) padding(0) {")
}

fn thread_inset_source() -> String {
    crate::sample_source::VALIDATION_SAMPLE_SOURCE
        .replace("column gap(0) padding(0) {", "column gap(0) padding(24) {")
}
