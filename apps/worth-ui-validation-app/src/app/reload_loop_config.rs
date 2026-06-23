use crate::reload::ValidationReloadLoopConfig;
use crate::ValidationWorkbenchAuthoredInputs;

use super::support::{
    default_header_appearance_path, default_header_command_path,
    default_header_command_projection_path, default_header_component_path,
    default_header_density_path, default_header_theme_path, default_validation_source_path,
};

pub fn default_reload_loop_config() -> ValidationReloadLoopConfig {
    default_reload_loop_config_from_authored_inputs(None)
}

pub fn default_reload_loop_config_from_authored_inputs(
    authored_inputs: Option<&ValidationWorkbenchAuthoredInputs>,
) -> ValidationReloadLoopConfig {
    let mut config = ValidationReloadLoopConfig::new(default_header_theme_path())
        .with_source_path(default_validation_source_path())
        .with_command_path(default_header_command_path())
        .with_command_projection_path(default_header_command_projection_path())
        .with_component_path(default_header_component_path())
        .with_appearance_path(default_header_appearance_path())
        .with_density_path(default_header_density_path());
    if let Some(authored_inputs) = authored_inputs {
        config = config.with_initial_source(authored_inputs.source().clone());
        if let Some(theme) = authored_inputs.theme() {
            config = config.with_initial_theme(theme.clone());
        }
        if let Some(commands) = authored_inputs.commands() {
            config = config.with_initial_command(commands.clone());
        }
        if let Some(command_projections) = authored_inputs.command_projections() {
            config = config.with_initial_command_projection(command_projections.clone());
        }
        if let Some(component) = authored_inputs.component() {
            config = config.with_initial_component(component.clone());
        }
        if let Some(appearance) = authored_inputs.appearance() {
            config = config.with_initial_appearance(appearance.clone());
        }
        if let Some(density) = authored_inputs.density() {
            config = config.with_initial_density(density.clone());
        }
    }
    config
}
