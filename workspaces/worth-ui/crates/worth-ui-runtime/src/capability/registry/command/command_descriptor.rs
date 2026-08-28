use crate::capability::{CommandId, CommandProjectionId, IconId, UiIntent};

use super::{
    CommandCategory, UiCommandRouteDeclaration, UiCommandRouteDestination,
    UiCommandShortcutSequence,
};

/// Declarative command capability supplied by an application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandDescriptor {
    id: CommandId,
    label: String,
    description: Option<String>,
    icon: Option<IconId>,
    default_shortcut: Option<UiCommandShortcutSequence>,
    route: Option<UiCommandRouteDeclaration>,
    category: CommandCategory,
    projection_eligibility: Option<CommandProjectionId>,
}

impl CommandDescriptor {
    pub fn new(id: CommandId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            description: None,
            icon: None,
            default_shortcut: None,
            route: None,
            category: CommandCategory::Application,
            projection_eligibility: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_icon(mut self, icon: IconId) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_default_shortcut(mut self, default_shortcut: UiCommandShortcutSequence) -> Self {
        self.default_shortcut = Some(default_shortcut);
        self
    }

    pub fn with_route(mut self, route: UiCommandRouteDeclaration) -> Self {
        self.route = Some(route);
        self
    }

    pub fn with_intent_destination<I: UiIntent>(mut self) -> Self {
        self.route = Some(UiCommandRouteDeclaration::new(
            UiCommandRouteDestination::for_intent::<I>(),
        ));
        self
    }

    pub fn with_category(mut self, category: CommandCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_projection_eligibility(
        mut self,
        projection_eligibility: CommandProjectionId,
    ) -> Self {
        self.projection_eligibility = Some(projection_eligibility);
        self
    }

    pub fn id(&self) -> &CommandId {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn icon(&self) -> Option<&IconId> {
        self.icon.as_ref()
    }

    pub fn default_shortcut(&self) -> Option<UiCommandShortcutSequence> {
        self.default_shortcut
    }

    pub fn route(&self) -> Option<UiCommandRouteDeclaration> {
        self.route
    }

    pub fn category(&self) -> CommandCategory {
        self.category
    }

    pub fn projection_eligibility(&self) -> Option<&CommandProjectionId> {
        self.projection_eligibility.as_ref()
    }
}
