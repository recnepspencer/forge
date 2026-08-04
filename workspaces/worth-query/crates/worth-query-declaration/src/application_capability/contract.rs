mod builder;

use std::marker::PhantomData;

use super::{
    ApplicationCapabilityComposition, ApplicationCapabilityConstraintDefinition,
    ApplicationCapabilityDelegationDefinition, ApplicationCapabilityElevationRule,
    ApplicationCapabilityTargetDefinition,
};

pub use builder::ApplicationCapabilityContractBuilder;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ErasedApplicationCapabilityContract {
    name: String,
    capability_type: String,
    operation: String,
    operation_type: String,
    input_type: String,
    grant_entity: String,
    target: ApplicationCapabilityTargetDefinition,
    constraints: ApplicationCapabilityConstraintDefinition,
    delegation: ApplicationCapabilityDelegationDefinition,
    composition: ApplicationCapabilityComposition,
    elevation: ApplicationCapabilityElevationRule,
}

impl ErasedApplicationCapabilityContract {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn capability_type(&self) -> &str {
        &self.capability_type
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn operation_type(&self) -> &str {
        &self.operation_type
    }

    pub fn input_type(&self) -> &str {
        &self.input_type
    }

    pub fn grant_entity(&self) -> &str {
        &self.grant_entity
    }

    pub const fn target(&self) -> &ApplicationCapabilityTargetDefinition {
        &self.target
    }

    pub const fn constraints(&self) -> &ApplicationCapabilityConstraintDefinition {
        &self.constraints
    }

    pub const fn delegation(&self) -> &ApplicationCapabilityDelegationDefinition {
        &self.delegation
    }

    pub const fn composition(&self) -> &ApplicationCapabilityComposition {
        &self.composition
    }

    pub const fn elevation(&self) -> &ApplicationCapabilityElevationRule {
        &self.elevation
    }
}

pub struct ApplicationCapabilityContract<Schema, Capability, Operation, Input> {
    erased: ErasedApplicationCapabilityContract,
    _marker: PhantomData<fn(Input) -> (Schema, Capability, Operation)>,
}

impl<Schema, Capability, Operation, Input>
    ApplicationCapabilityContract<Schema, Capability, Operation, Input>
{
    pub fn erased(&self) -> &ErasedApplicationCapabilityContract {
        &self.erased
    }

    pub(crate) fn into_erased(self) -> ErasedApplicationCapabilityContract {
        self.erased
    }
}

pub struct Missing;
pub struct Present<Value>(Value);
