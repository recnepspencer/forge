use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport,
    ComponentId, ComponentPropSchema, ComponentStateOwnership,
};

pub(super) fn text_input_component() -> ComponentDescriptor {
    live_view_control_component(
        "worth.component.text_input",
        "worth.control.text_input.props",
    )
}

pub(super) fn dropdown_input_component() -> ComponentDescriptor {
    live_view_control_component(
        "worth.component.dropdown_input",
        "worth.control.dropdown_input.props",
    )
}

fn live_view_control_component(component_id: &str, prop_schema: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(component_id).expect("valid component id"),
        ComponentPropSchema::named(prop_schema),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_focus(ComponentFocusSupport::focusable())
    .with_execution_lane(ComponentExecutionLane::Interactive)
}
