use crate::capability::{CommandId, ComponentId, IconId, SurfaceId, ViewBindingId};

use super::{SurfaceKind, SurfacePlacementClass, SurfaceStateClass};

/// Declarative product-facing shell surface supplied by an application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceDescriptor {
    id: SurfaceId,
    kind: SurfaceKind,
    component_id: ComponentId,
    placement_class: SurfacePlacementClass,
    state_class: SurfaceStateClass,
    command_slots: Vec<CommandId>,
    label: Option<String>,
    icon: Option<IconId>,
    view_binding: Option<ViewBindingId>,
}

impl SurfaceDescriptor {
    pub fn new(
        id: SurfaceId,
        kind: SurfaceKind,
        component_id: ComponentId,
        placement_class: SurfacePlacementClass,
        state_class: SurfaceStateClass,
    ) -> Self {
        Self {
            id,
            kind,
            component_id,
            placement_class,
            state_class,
            command_slots: Vec::new(),
            label: None,
            icon: None,
            view_binding: None,
        }
    }

    pub fn with_command_slot(mut self, command_id: CommandId) -> Self {
        self.command_slots.push(command_id);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_icon(mut self, icon: IconId) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_view_binding(mut self, view_binding: ViewBindingId) -> Self {
        self.view_binding = Some(view_binding);
        self
    }

    pub fn id(&self) -> &SurfaceId {
        &self.id
    }

    pub fn kind(&self) -> &SurfaceKind {
        &self.kind
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    pub fn placement_class(&self) -> &SurfacePlacementClass {
        &self.placement_class
    }

    pub fn state_class(&self) -> &SurfaceStateClass {
        &self.state_class
    }

    pub fn command_slots(&self) -> &[CommandId] {
        &self.command_slots
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn icon(&self) -> Option<&IconId> {
        self.icon.as_ref()
    }

    pub fn view_binding(&self) -> Option<&ViewBindingId> {
        self.view_binding.as_ref()
    }
}
