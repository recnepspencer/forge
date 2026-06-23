use crate::facade::{
    AppearanceTokenId, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSelectionMode,
    CommandProjectionSurface, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, DensityTokenId, WorthUi, WorthUiAppearanceFamily,
    WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource, WorthUiAppearanceValue,
    WorthUiDensityFamily, WorthUiDensityTokenDescriptor, WorthUiDensityValue, WorthUiLengthValue,
    WorthUiPaddingValue, WorthUiSpacingValue,
};

use super::{
    WorthUiDropdownModeTransitionDenial, WorthUiDropdownProjectionPlan,
    WorthUiDropdownProjectionRequest, WorthUiDropdownSelectionState,
    WorthUiDropdownSelectionStateStatus, WorthUiDropdownStateDropReason,
};
use crate::runtime::WorthUiRuntimeFactId;

#[test]
fn dropdown_plan_declares_projection_component_command_and_style_dependencies() {
    let app = dropdown_app(CommandProjectionSelectionMode::SingleSelect);
    let projection_id = projection_id();
    let single_component_id = single_component_id();
    let command_new = command_id("workspace.command.new");
    let command_open = command_id("workspace.command.open");

    let plan = WorthUiDropdownProjectionPlan::from_snapshot(
        app.capabilities(),
        dropdown_request(projection_id.clone()),
    )
    .expect("dropdown plan should resolve from registered command projection");

    let dependencies = plan.dependencies();
    assert!(dependencies.contains_exact(&WorthUiRuntimeFactId::command_projection(&projection_id,)));
    assert!(dependencies.contains_exact(
        &WorthUiRuntimeFactId::command_projection_interaction_policy(&projection_id),
    ));
    assert!(dependencies.contains_exact(&WorthUiRuntimeFactId::component(&single_component_id,)));
    assert!(dependencies.contains_exact(&WorthUiRuntimeFactId::command(&command_new,)));
    assert!(dependencies.contains_exact(&WorthUiRuntimeFactId::command(&command_open,)));
    assert!(
        dependencies.contains_exact(&WorthUiRuntimeFactId::appearance_token(
            &AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
        ))
    );
    assert!(
        dependencies.contains_exact(&WorthUiRuntimeFactId::density_token(
            &DensityTokenId::new("density.header.row_padding").unwrap(),
        ))
    );
    assert!(
        dependencies.contains_exact(&WorthUiRuntimeFactId::density_token(
            &DensityTokenId::new("density.header.control_spacing").unwrap(),
        ))
    );
}

#[test]
fn dropdown_plan_promotes_selected_single_command_when_mode_widens() {
    let rebuilt = WorthUiDropdownProjectionPlan::rebuild_from_snapshot(
        dropdown_app(CommandProjectionSelectionMode::MultiSelect).capabilities(),
        dropdown_request(projection_id()),
        Some(&WorthUiDropdownSelectionState::Single(
            "workspace.command.new".to_owned(),
        )),
    )
    .expect("mode widening should preserve selection truth");

    let frame = rebuilt.execute_frame();
    assert_eq!(
        frame.selection_mode(),
        CommandProjectionSelectionMode::MultiSelect
    );
    assert_eq!(
        frame.component_id(),
        multi_component_id().as_str(),
        "multi-select mode must switch to the runtime-owned multi-select component"
    );
    assert_eq!(
        frame.selection_state().selected_command_ids(),
        vec!["workspace.command.new".to_owned()]
    );
    assert_eq!(
        frame.reconciliation().status(),
        &WorthUiDropdownSelectionStateStatus::PromotedSingleToMulti
    );
}

#[test]
fn single_select_dropdown_does_not_synthesize_initial_selected_value() {
    let plan = WorthUiDropdownProjectionPlan::from_snapshot(
        dropdown_app(CommandProjectionSelectionMode::SingleSelect).capabilities(),
        dropdown_request(projection_id()),
    )
    .expect("single-select dropdown plan should build");

    assert_eq!(
        plan.execute_frame().selection_state(),
        &WorthUiDropdownSelectionState::None
    );
    assert_eq!(
        plan.execute_frame().reconciliation().status(),
        &WorthUiDropdownSelectionStateStatus::Empty
    );
}

#[test]
fn selection_state_rejects_ambiguous_multi_to_single_narrowing() {
    let (state, receipt) = WorthUiDropdownSelectionState::reconcile(
        &WorthUiDropdownSelectionState::Multi(vec![
            "workspace.command.new".to_owned(),
            "workspace.command.open".to_owned(),
        ]),
        CommandProjectionSelectionMode::SingleSelect,
        &[
            "workspace.command.new".to_owned(),
            "workspace.command.open".to_owned(),
        ],
    );

    assert_eq!(state, WorthUiDropdownSelectionState::None);
    assert_eq!(
        receipt.status(),
        &WorthUiDropdownSelectionStateStatus::DeniedModeTransition(
            WorthUiDropdownModeTransitionDenial::AmbiguousSingleSelectNarrowing {
                surviving_command_ids: vec![
                    "workspace.command.new".to_owned(),
                    "workspace.command.open".to_owned(),
                ],
            },
        )
    );
}

#[test]
fn selection_state_drops_removed_command_instead_of_preserving_ghost_selection() {
    let (state, receipt) = WorthUiDropdownSelectionState::reconcile(
        &WorthUiDropdownSelectionState::Single("workspace.command.open".to_owned()),
        CommandProjectionSelectionMode::SingleSelect,
        &["workspace.command.new".to_owned()],
    );

    assert_eq!(state, WorthUiDropdownSelectionState::None);
    assert_eq!(
        receipt.status(),
        &WorthUiDropdownSelectionStateStatus::DroppedSelection {
            reason: WorthUiDropdownStateDropReason::SelectedCommandUnavailable,
        }
    );
}

fn dropdown_app(selection_mode: CommandProjectionSelectionMode) -> crate::facade::WorthUiApp {
    let command_new = command_id("workspace.command.new");
    let command_open = command_id("workspace.command.open");
    WorthUi::app()
        .register_command(CommandDescriptor::new(command_new.clone(), "New"))
        .register_command(CommandDescriptor::new(command_open.clone(), "Open"))
        .register_command_projection(
            CommandProjectionDescriptor::new(projection_id(), CommandProjectionSurface::menu_bar())
                .with_selection_mode(selection_mode)
                .with_command_reference(CommandProjectionCommandReference::command(command_new))
                .with_command_reference(CommandProjectionCommandReference::command(command_open)),
        )
        .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
            WorthUiAppearanceFamily::Layout,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::Length(WorthUiLengthValue::from_px("220px").unwrap()),
        ))
        .register_density_token(WorthUiDensityTokenDescriptor::define(
            DensityTokenId::new("density.header.row_padding").unwrap(),
            WorthUiDensityFamily::RowPadding,
            WorthUiDensityValue::Padding(
                WorthUiPaddingValue::from_shorthand_px("1px 6px").unwrap(),
            ),
        ))
        .register_density_token(WorthUiDensityTokenDescriptor::define(
            DensityTokenId::new("density.header.control_spacing").unwrap(),
            WorthUiDensityFamily::ControlSpacing,
            WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px("8px").unwrap()),
        ))
        .register_component(ComponentDescriptor::new(
            single_component_id(),
            ComponentPropSchema::named("validation.header.dropdown.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_component(ComponentDescriptor::new(
            multi_component_id(),
            ComponentPropSchema::named("validation.header.multi_select_dropdown.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze()
}

fn dropdown_request(projection_id: CommandProjectionId) -> WorthUiDropdownProjectionRequest {
    WorthUiDropdownProjectionRequest::for_command_projection(
        projection_id,
        single_component_id(),
        multi_component_id(),
        crate::runtime::WorthUiDropdownAppearanceRequest::new(
            AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
            DensityTokenId::new("density.header.row_padding").unwrap(),
            DensityTokenId::new("density.header.control_spacing").unwrap(),
        ),
    )
}

fn projection_id() -> CommandProjectionId {
    CommandProjectionId::new("workspace.header.file").unwrap()
}

fn single_component_id() -> ComponentId {
    ComponentId::new("validation.component.header.dropdown").unwrap()
}

fn multi_component_id() -> ComponentId {
    ComponentId::new("validation.component.header.multi_select_dropdown").unwrap()
}

fn command_id(id: &str) -> CommandId {
    CommandId::new(id).unwrap()
}
