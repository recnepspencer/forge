mod loop_wiring;
mod naming;
mod ownership;
mod radial;
mod shared;
mod shell_closure;
#[cfg(test)]
mod tests;
mod vertex_branching;
mod wire_connectivity;

use forge_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRegistrationError, RelationalRuntime,
    RelationalRuntimeApi, RelationalRuntimeBuilder,
};
use forge_relational::facade::schema::SchemaRegistryError;
use worth_schema::facade::{worth_bootstrap_runtime_invariant_plan, worth_bootstrap_schema_registry};

#[derive(Debug)]
pub enum WorthMilestoneOneRuntimeSetupError {
    SchemaRegistry(SchemaRegistryError),
    CustomInvariantRegistration(CustomInvariantRegistrationError),
}

impl From<SchemaRegistryError> for WorthMilestoneOneRuntimeSetupError {
    fn from(value: SchemaRegistryError) -> Self {
        Self::SchemaRegistry(value)
    }
}

impl From<CustomInvariantRegistrationError> for WorthMilestoneOneRuntimeSetupError {
    fn from(value: CustomInvariantRegistrationError) -> Self {
        Self::CustomInvariantRegistration(value)
    }
}

pub fn worth_milestone_one_runtime_invariants(
) -> Result<Vec<CustomInvariantRegistration>, CustomInvariantRegistrationError> {
    let _declared = worth_bootstrap_runtime_invariant_plan();
    Ok(vec![
        ownership::registration()?,
        loop_wiring::registration()?,
        radial::registration()?,
        wire_connectivity::registration()?,
        vertex_branching::registration()?,
        shell_closure::registration()?,
        naming::registration()?,
    ])
}

pub fn configure_worth_milestone_one_runtime_builder(
    builder: RelationalRuntimeBuilder,
) -> Result<RelationalRuntimeBuilder, WorthMilestoneOneRuntimeSetupError> {
    let builder = builder.schema_registry(worth_bootstrap_schema_registry()?);
    let registrations = worth_milestone_one_runtime_invariants()?;
    Ok(registrations
        .into_iter()
        .fold(builder, |builder, registration| builder.custom_invariant(registration)))
}

pub fn worth_milestone_one_runtime_builder(
) -> Result<RelationalRuntimeBuilder, WorthMilestoneOneRuntimeSetupError> {
    configure_worth_milestone_one_runtime_builder(RelationalRuntimeApi::builder())
}

pub fn build_worth_milestone_one_runtime(
) -> Result<RelationalRuntime, WorthMilestoneOneRuntimeSetupError> {
    Ok(worth_milestone_one_runtime_builder()?.build())
}
