use std::marker::PhantomData;

use crate::application_schema::{ApplicationEntityRef, ApplicationOperationRef};

use super::{
    ApplicationCapabilityComposition, ApplicationCapabilityConstraintDefinition,
    ApplicationCapabilityDelegationDefinition, ApplicationCapabilityRef,
    ApplicationCapabilityTargetDefinition,
};

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

/// Typestate authoring progression for one complete capability contract.
///
/// A descriptive name and operation are not a complete contract:
///
/// ```compile_fail
/// use worth_query_declaration::facade::{
///     application_capability::{
///         ApplicationCapabilityContractBuilder, ApplicationCapabilityRef,
///     },
///     application_schema::{ApplicationEntityRef, ApplicationOperationRef},
/// };
/// struct Schema;
/// struct Capability;
/// struct Operation;
/// struct Grant;
///
/// let builder = ApplicationCapabilityContractBuilder::<
///     Schema,
///     Capability,
///     Operation,
///     (),
/// >::new(
///     ApplicationCapabilityRef::from_schema_identifier("Capability"),
///     ApplicationOperationRef::from_schema_identifier("Operation"),
///     ApplicationEntityRef::<Schema, Grant>::from_schema_identifier("Grant"),
/// );
/// let _ = builder.build();
/// ```
pub struct ApplicationCapabilityContractBuilder<
    Schema,
    Capability,
    Operation,
    Input,
    Target = Missing,
    Constraints = Missing,
    Delegation = Missing,
    Composition = Missing,
> {
    name: &'static str,
    capability_type: &'static str,
    operation: &'static str,
    operation_type: &'static str,
    grant_entity: &'static str,
    target: Target,
    constraints: Constraints,
    delegation: Delegation,
    composition: Composition,
    _marker: PhantomData<fn(Input) -> (Schema, Capability, Operation)>,
}

impl<Schema, Capability, Operation, Input>
    ApplicationCapabilityContractBuilder<Schema, Capability, Operation, Input>
{
    pub fn new<Grant>(
        capability: ApplicationCapabilityRef<Schema, Capability>,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
        grant: ApplicationEntityRef<Schema, Grant>,
    ) -> Self {
        Self {
            name: capability.name(),
            capability_type: std::any::type_name::<Capability>(),
            operation: operation.name(),
            operation_type: std::any::type_name::<Operation>(),
            grant_entity: grant.name(),
            target: Missing,
            constraints: Missing,
            delegation: Missing,
            composition: Missing,
            _marker: PhantomData,
        }
    }
}

impl<Schema, Capability, Operation, Input, Constraints, Delegation, Composition>
    ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Missing,
        Constraints,
        Delegation,
        Composition,
    >
{
    pub fn target(
        self,
        target: ApplicationCapabilityTargetDefinition,
    ) -> ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Present<ApplicationCapabilityTargetDefinition>,
        Constraints,
        Delegation,
        Composition,
    > {
        ApplicationCapabilityContractBuilder {
            name: self.name,
            capability_type: self.capability_type,
            operation: self.operation,
            operation_type: self.operation_type,
            grant_entity: self.grant_entity,
            target: Present(target),
            constraints: self.constraints,
            delegation: self.delegation,
            composition: self.composition,
            _marker: PhantomData,
        }
    }
}

impl<Schema, Capability, Operation, Input, Target, Delegation, Composition>
    ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Target,
        Missing,
        Delegation,
        Composition,
    >
{
    pub fn constraints(
        self,
        constraints: ApplicationCapabilityConstraintDefinition,
    ) -> ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Target,
        Present<ApplicationCapabilityConstraintDefinition>,
        Delegation,
        Composition,
    > {
        ApplicationCapabilityContractBuilder {
            name: self.name,
            capability_type: self.capability_type,
            operation: self.operation,
            operation_type: self.operation_type,
            grant_entity: self.grant_entity,
            target: self.target,
            constraints: Present(constraints),
            delegation: self.delegation,
            composition: self.composition,
            _marker: PhantomData,
        }
    }
}

impl<Schema, Capability, Operation, Input, Target, Constraints, Composition>
    ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Target,
        Constraints,
        Missing,
        Composition,
    >
{
    pub fn delegation(
        self,
        delegation: ApplicationCapabilityDelegationDefinition,
    ) -> ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Target,
        Constraints,
        Present<ApplicationCapabilityDelegationDefinition>,
        Composition,
    > {
        ApplicationCapabilityContractBuilder {
            name: self.name,
            capability_type: self.capability_type,
            operation: self.operation,
            operation_type: self.operation_type,
            grant_entity: self.grant_entity,
            target: self.target,
            constraints: self.constraints,
            delegation: Present(delegation),
            composition: self.composition,
            _marker: PhantomData,
        }
    }
}

impl<Schema, Capability, Operation, Input, Target, Constraints, Delegation>
    ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Target,
        Constraints,
        Delegation,
        Missing,
    >
{
    pub fn composition(
        self,
        composition: ApplicationCapabilityComposition,
    ) -> ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Target,
        Constraints,
        Delegation,
        Present<ApplicationCapabilityComposition>,
    > {
        ApplicationCapabilityContractBuilder {
            name: self.name,
            capability_type: self.capability_type,
            operation: self.operation,
            operation_type: self.operation_type,
            grant_entity: self.grant_entity,
            target: self.target,
            constraints: self.constraints,
            delegation: self.delegation,
            composition: Present(composition),
            _marker: PhantomData,
        }
    }
}

impl<Schema, Capability, Operation, Input>
    ApplicationCapabilityContractBuilder<
        Schema,
        Capability,
        Operation,
        Input,
        Present<ApplicationCapabilityTargetDefinition>,
        Present<ApplicationCapabilityConstraintDefinition>,
        Present<ApplicationCapabilityDelegationDefinition>,
        Present<ApplicationCapabilityComposition>,
    >
{
    pub fn build(self) -> ApplicationCapabilityContract<Schema, Capability, Operation, Input> {
        ApplicationCapabilityContract {
            erased: ErasedApplicationCapabilityContract {
                name: self.name.to_string(),
                capability_type: self.capability_type.to_string(),
                operation: self.operation.to_string(),
                operation_type: self.operation_type.to_string(),
                input_type: std::any::type_name::<Input>().to_string(),
                grant_entity: self.grant_entity.to_string(),
                target: self.target.0,
                constraints: self.constraints.0,
                delegation: self.delegation.0,
                composition: self.composition.0,
            },
            _marker: PhantomData,
        }
    }
}
