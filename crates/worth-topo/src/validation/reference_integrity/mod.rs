mod loop_wiring;
mod naming;
mod ownership;
mod radial_rings;
mod shared;
mod shared_queries;
mod shell_closure;
#[cfg(test)]
mod tests;
mod vertex_disks;
mod wire_connectivity;

use std::fmt;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, RelationalRuntimeApi,
    RelationalRuntimeBuilder,
};
use forge_relational::facade::schema::SchemaRegistryError;
use schema::facade::bootstrap_schema_registry;

#[derive(Debug)]
pub enum MilestoneOneRuntimeSetupError {
    SchemaRegistry(SchemaRegistryError),
    CustomInvariantRegistration(CustomInvariantRegistrationError),
}

impl From<SchemaRegistryError> for MilestoneOneRuntimeSetupError {
    fn from(value: SchemaRegistryError) -> Self {
        Self::SchemaRegistry(value)
    }
}

impl From<CustomInvariantRegistrationError> for MilestoneOneRuntimeSetupError {
    fn from(value: CustomInvariantRegistrationError) -> Self {
        Self::CustomInvariantRegistration(value)
    }
}

impl fmt::Display for MilestoneOneRuntimeSetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaRegistry(error) => write!(f, "{error:?}"),
            Self::CustomInvariantRegistration(error) => write!(f, "{error:?}"),
        }
    }
}

pub fn milestone_one_invariant_registrations(
) -> Result<Vec<CustomInvariantRegistration>, CustomInvariantRegistrationError> {
    Ok(vec![
        ownership::graph_composition_registration()?,
        ownership::commit_backstop_registration()?,
        loop_wiring::graph_composition_registration()?,
        loop_wiring::commit_backstop_registration()?,
        radial_rings::graph_composition_registration()?,
        radial_rings::commit_backstop_registration()?,
        wire_connectivity::graph_composition_registration()?,
        wire_connectivity::commit_backstop_registration()?,
        vertex_disks::graph_composition_registration()?,
        vertex_disks::commit_backstop_registration()?,
        shell_closure::graph_composition_registration()?,
        shell_closure::commit_backstop_registration()?,
        naming::graph_composition_registration()?,
        naming::commit_backstop_registration()?,
    ])
}

pub fn configure_milestone_one_runtime_builder(
    builder: RelationalRuntimeBuilder,
) -> Result<RelationalRuntimeBuilder, MilestoneOneRuntimeSetupError> {
    let builder = builder.schema_registry(bootstrap_schema_registry()?);
    let registrations = milestone_one_invariant_registrations()?;
    Ok(registrations
        .into_iter()
        .fold(builder, |builder, registration| {
            builder.custom_invariant(registration)
        }))
}

pub fn milestone_one_runtime_builder(
) -> Result<RelationalRuntimeBuilder, MilestoneOneRuntimeSetupError> {
    configure_milestone_one_runtime_builder(RelationalRuntimeApi::builder())
}

pub fn build_milestone_one_runtime() -> Result<RelationalRuntime, MilestoneOneRuntimeSetupError> {
    Ok(milestone_one_runtime_builder()?.build())
}
