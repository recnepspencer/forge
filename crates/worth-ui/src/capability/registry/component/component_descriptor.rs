use crate::capability::{CommandId, ComponentId, ThemeTokenId};

use super::{
    ComponentAccessibilitySupport, ComponentChildPolicy, ComponentExecutionLane,
    ComponentFocusSupport, ComponentPropSchema, ComponentStateOwnership,
};

/// Declarative renderable component capability supplied by an application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentDescriptor {
    id: ComponentId,
    prop_schema: Option<ComponentPropSchema>,
    child_policy: ComponentChildPolicy,
    state_ownership: Option<ComponentStateOwnership>,
    accessibility: ComponentAccessibilitySupport,
    focus: ComponentFocusSupport,
    theme_token_dependencies: Vec<ThemeTokenId>,
    command_binding_slots: Vec<CommandId>,
    execution_lane: ComponentExecutionLane,
}

impl ComponentDescriptor {
    pub fn new(
        id: ComponentId,
        prop_schema: ComponentPropSchema,
        child_policy: ComponentChildPolicy,
        state_ownership: ComponentStateOwnership,
    ) -> Self {
        Self {
            id,
            prop_schema: Some(prop_schema),
            child_policy,
            state_ownership: Some(state_ownership),
            accessibility: ComponentAccessibilitySupport::semantic(),
            focus: ComponentFocusSupport::not_focusable(),
            theme_token_dependencies: Vec::new(),
            command_binding_slots: Vec::new(),
            execution_lane: ComponentExecutionLane::Passive,
        }
    }

    pub fn without_prop_schema_for_diagnostics(
        id: ComponentId,
        child_policy: ComponentChildPolicy,
        state_ownership: ComponentStateOwnership,
    ) -> Self {
        Self {
            id,
            prop_schema: None,
            child_policy,
            state_ownership: Some(state_ownership),
            accessibility: ComponentAccessibilitySupport::semantic(),
            focus: ComponentFocusSupport::not_focusable(),
            theme_token_dependencies: Vec::new(),
            command_binding_slots: Vec::new(),
            execution_lane: ComponentExecutionLane::Passive,
        }
    }

    pub fn without_state_ownership_for_diagnostics(
        id: ComponentId,
        prop_schema: ComponentPropSchema,
        child_policy: ComponentChildPolicy,
    ) -> Self {
        Self {
            id,
            prop_schema: Some(prop_schema),
            child_policy,
            state_ownership: None,
            accessibility: ComponentAccessibilitySupport::semantic(),
            focus: ComponentFocusSupport::not_focusable(),
            theme_token_dependencies: Vec::new(),
            command_binding_slots: Vec::new(),
            execution_lane: ComponentExecutionLane::Passive,
        }
    }

    pub fn with_accessibility(mut self, accessibility: ComponentAccessibilitySupport) -> Self {
        self.accessibility = accessibility;
        self
    }

    pub fn with_focus(mut self, focus: ComponentFocusSupport) -> Self {
        self.focus = focus;
        self
    }

    pub fn with_theme_token_dependency(mut self, token_id: ThemeTokenId) -> Self {
        self.theme_token_dependencies.push(token_id);
        self
    }

    pub fn with_command_binding_slot(mut self, command_id: CommandId) -> Self {
        self.command_binding_slots.push(command_id);
        self
    }

    pub fn with_execution_lane(mut self, execution_lane: ComponentExecutionLane) -> Self {
        self.execution_lane = execution_lane;
        self
    }

    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    pub fn prop_schema(&self) -> Option<&ComponentPropSchema> {
        self.prop_schema.as_ref()
    }

    pub fn child_policy(&self) -> ComponentChildPolicy {
        self.child_policy
    }

    pub fn state_ownership(&self) -> Option<ComponentStateOwnership> {
        self.state_ownership
    }

    pub fn accessibility(&self) -> ComponentAccessibilitySupport {
        self.accessibility
    }

    pub fn focus(&self) -> ComponentFocusSupport {
        self.focus
    }

    pub fn theme_token_dependencies(&self) -> &[ThemeTokenId] {
        &self.theme_token_dependencies
    }

    pub fn command_binding_slots(&self) -> &[CommandId] {
        &self.command_binding_slots
    }

    pub fn execution_lane(&self) -> ComponentExecutionLane {
        self.execution_lane
    }
}
