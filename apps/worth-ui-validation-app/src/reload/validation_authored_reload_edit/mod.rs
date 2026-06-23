mod source_text_surface_component_edit;
mod source_text_surface_prop_edit;

use source_text_surface_component_edit::repoint_surface_component;
use source_text_surface_prop_edit::{remove_surface_prop, set_surface_prop};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationAuthoredReloadEdit {
    RepointSurfaceComponent {
        surface_id: String,
        component_id: String,
    },
    SetSurfaceProp {
        surface_id: String,
        prop_key: String,
        authored_value: String,
    },
    RemoveSurfaceProp {
        surface_id: String,
        prop_key: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationAuthoredReloadEditDenial {
    SurfaceDeclarationNotFound {
        surface_id: String,
    },
    SurfaceComponentLineNotFound {
        surface_id: String,
    },
    SurfacePropLineNotFound {
        surface_id: String,
        prop_key: String,
    },
    SurfaceBlockMalformed {
        surface_id: String,
    },
}

impl ValidationAuthoredReloadEdit {
    pub fn repoint_surface_component(
        surface_id: impl Into<String>,
        component_id: impl Into<String>,
    ) -> Self {
        Self::RepointSurfaceComponent {
            surface_id: surface_id.into(),
            component_id: component_id.into(),
        }
    }

    pub fn set_surface_prop(
        surface_id: impl Into<String>,
        prop_key: impl Into<String>,
        authored_value: impl Into<String>,
    ) -> Self {
        Self::SetSurfaceProp {
            surface_id: surface_id.into(),
            prop_key: prop_key.into(),
            authored_value: authored_value.into(),
        }
    }

    pub fn remove_surface_prop(surface_id: impl Into<String>, prop_key: impl Into<String>) -> Self {
        Self::RemoveSurfaceProp {
            surface_id: surface_id.into(),
            prop_key: prop_key.into(),
        }
    }

    pub fn apply_to_source_text(
        &self,
        source_text: &str,
    ) -> Result<String, ValidationAuthoredReloadEditDenial> {
        match self {
            Self::RepointSurfaceComponent {
                surface_id,
                component_id,
            } => repoint_surface_component(source_text, surface_id, component_id),
            Self::SetSurfaceProp {
                surface_id,
                prop_key,
                authored_value,
            } => set_surface_prop(source_text, surface_id, prop_key, authored_value),
            Self::RemoveSurfaceProp {
                surface_id,
                prop_key,
            } => remove_surface_prop(source_text, surface_id, prop_key),
        }
    }
}
