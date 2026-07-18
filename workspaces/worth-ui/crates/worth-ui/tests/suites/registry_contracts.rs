//! Semantic integration suite. Individual responsibilities remain in named child modules.

#[path = "../command_projection_registry.rs"]
mod command_projection_registry;
#[path = "../command_registry.rs"]
mod command_registry;
#[path = "../component_registry.rs"]
mod component_registry;
#[path = "../icon_registry.rs"]
mod icon_registry;
#[path = "../mosaic_registry.rs"]
mod mosaic_registry;
#[path = "../native_capability_registry.rs"]
mod native_capability_registry;
#[path = "../plugin_slot_registry.rs"]
mod plugin_slot_registry;
#[path = "../runtime_outcome_projection_registry.rs"]
mod runtime_outcome_projection_registry;
#[path = "../settings_registry.rs"]
mod settings_registry;
#[path = "../surface_registry.rs"]
mod surface_registry;
#[path = "../task_presentation_registry.rs"]
mod task_presentation_registry;
#[path = "../theme_token_registry.rs"]
mod theme_token_registry;
#[path = "../view_binding_registry.rs"]
mod view_binding_registry;
