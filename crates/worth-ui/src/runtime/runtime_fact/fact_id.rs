use crate::capability::{CommandId, CommandProjectionId, ComponentId, ThemeTokenId, ViewBindingId};

use super::WorthUiRuntimeFactFamily;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiRuntimeFactId {
    family: WorthUiRuntimeFactFamily,
    identity: String,
}

impl WorthUiRuntimeFactId {
    pub fn active_artifact() -> Self {
        Self::new(WorthUiRuntimeFactFamily::ActiveArtifact, "active")
    }

    pub fn execution_plan() -> Self {
        Self::new(WorthUiRuntimeFactFamily::ExecutionPlan, "active")
    }

    pub fn theme_token(token_id: &ThemeTokenId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ThemeToken, token_id.as_str())
    }

    pub fn command(command_id: &CommandId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::Command, command_id.as_str())
    }

    pub fn command_projection(projection_id: &CommandProjectionId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::CommandProjection,
            projection_id.as_str(),
        )
    }

    pub fn command_projection_interaction_policy(projection_id: &CommandProjectionId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::InteractionPolicy,
            projection_id.as_str(),
        )
    }

    pub fn query_binding(view_binding_id: &ViewBindingId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryBinding,
            view_binding_id.as_str(),
        )
    }

    pub fn component(component_id: &ComponentId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::Component, component_id.as_str())
    }

    pub fn layout_topology(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LayoutTopology, identity)
    }

    pub fn content_mount(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ContentMount, identity)
    }

    pub fn appearance(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::Appearance, identity)
    }

    pub fn family(&self) -> WorthUiRuntimeFactFamily {
        self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    fn new(family: WorthUiRuntimeFactFamily, identity: impl Into<String>) -> Self {
        Self {
            family,
            identity: identity.into(),
        }
    }
}
