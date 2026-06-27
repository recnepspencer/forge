use worth_ui::facade::{
    ComponentAccessibilitySupport, ComponentChildPolicy, ComponentDescriptor,
    ComponentExecutionLane, ComponentFocusSupport, ComponentId, ComponentPropSchema,
    ComponentStateOwnership,
};

fn main() {
    let _ = ComponentDescriptor {
        id: ComponentId::new("workspace.component.editor").expect("valid component id"),
        prop_schema: Some(ComponentPropSchema::named("workspace.editor.props")),
        child_policy: ComponentChildPolicy::no_children(),
        state_ownership: Some(ComponentStateOwnership::runtime_owned()),
        accessibility: ComponentAccessibilitySupport::semantic(),
        focus: ComponentFocusSupport::not_focusable(),
        theme_token_dependencies: Vec::new(),
        command_binding_slots: Vec::new(),
        execution_lane: ComponentExecutionLane::Passive,
    };
}
