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

#[cfg(test)]
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, RelationalRuntimeApi,
    RelationalRuntimeBuilder,
};
use forge_relational::facade::schema::SchemaRegistryError;
use schema::facade::bootstrap_schema_registry;

#[derive(Debug)]
pub(crate) enum MilestoneOneRuntimeSetupError {
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
        ownership::registration()?,
        loop_wiring::registration()?,
        radial_rings::registration()?,
        wire_connectivity::registration()?,
        vertex_disks::registration()?,
        shell_closure::registration()?,
        naming::registration()?,
    ])
}

pub(crate) fn configure_milestone_one_runtime_builder(
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

pub(crate) fn milestone_one_runtime_builder(
) -> Result<RelationalRuntimeBuilder, MilestoneOneRuntimeSetupError> {
    configure_milestone_one_runtime_builder(RelationalRuntimeApi::builder())
}

#[cfg(test)]
pub(crate) fn build_milestone_one_runtime(
) -> Result<RelationalRuntime, MilestoneOneRuntimeSetupError> {
    Ok(milestone_one_runtime_builder()?.build())
}
