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

use forge_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, RelationalRuntime,
    RelationalRuntimeApi, RelationalRuntimeBuilder,
};
use forge_relational::facade::schema::SchemaRegistryError;
use schema::facade::{bootstrap_runtime_invariant_plan, bootstrap_schema_registry};

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

pub fn milestone_one_runtime_invariants(
) -> Result<Vec<CustomInvariantRegistration>, CustomInvariantRegistrationError> {
    let _declared = bootstrap_runtime_invariant_plan();
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

pub fn configure_milestone_one_runtime_builder(
    builder: RelationalRuntimeBuilder,
) -> Result<RelationalRuntimeBuilder, MilestoneOneRuntimeSetupError> {
    let builder = builder.schema_registry(bootstrap_schema_registry()?);
    let registrations = milestone_one_runtime_invariants()?;
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
