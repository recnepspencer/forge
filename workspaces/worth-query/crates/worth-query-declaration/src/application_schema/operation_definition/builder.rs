use std::marker::PhantomData;

use crate::application_aftermath::DeclaredApplicationAftermathContract;

use super::super::capabilities::OperationEmits;
use super::super::{
    ApplicationEffectRef, ApplicationExternalEffectPayload, ApplicationOperationRef,
};
use super::contract_slots::DeclaredExternalEffectSlot;
use super::definition::ApplicationOperationDefinition;

/// Typestate authoring for one operation's singleton static contracts.
///
/// The two const parameters record whether external-effect and aftermath
/// posture have been decided. `finish` exists only after both decisions.
pub struct ApplicationOperationDefinitionBuilder<
    Schema,
    Operation,
    Input,
    const EXTERNAL_EFFECT_DECIDED: bool,
    const AFTERMATH_DECIDED: bool,
> {
    operation: &'static str,
    external_effect: Option<DeclaredExternalEffectSlot>,
    aftermath: Option<DeclaredApplicationAftermathContract>,
    marker: PhantomData<fn(Input) -> (Schema, Operation)>,
}

impl<Schema, Operation, Input> ApplicationOperationRef<Schema, Operation, Input> {
    /// Starts the operation definition that must explicitly select both static
    /// singleton contract slots before schema registration.
    pub fn definition(
        self,
    ) -> ApplicationOperationDefinitionBuilder<Schema, Operation, Input, false, false> {
        ApplicationOperationDefinitionBuilder {
            operation: self.name(),
            external_effect: None,
            aftermath: None,
            marker: PhantomData,
        }
    }
}

impl<Schema, Operation, Input, const AFTERMATH_DECIDED: bool>
    ApplicationOperationDefinitionBuilder<Schema, Operation, Input, false, AFTERMATH_DECIDED>
{
    /// Selects the explicit absence of an external effect.
    pub fn no_external_effect(
        self,
    ) -> ApplicationOperationDefinitionBuilder<Schema, Operation, Input, true, AFTERMATH_DECIDED>
    {
        ApplicationOperationDefinitionBuilder {
            operation: self.operation,
            external_effect: None,
            aftermath: self.aftermath,
            marker: PhantomData,
        }
    }

    /// Selects the operation's one external-effect contract.
    pub fn external_effect<Effect, Payload>(
        self,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
        correlation_family: &str,
    ) -> ApplicationOperationDefinitionBuilder<Schema, Operation, Input, true, AFTERMATH_DECIDED>
    where
        Effect: OperationEmits<Operation>,
        Payload: ApplicationExternalEffectPayload,
    {
        ApplicationOperationDefinitionBuilder {
            operation: self.operation,
            external_effect: Some(DeclaredExternalEffectSlot {
                effect: effect.name().to_string(),
                rust_payload_type: std::any::type_name::<Payload>().to_string(),
                protocol: Payload::PROTOCOL,
                maximum_payload_bytes: Payload::MAX_EXTERNAL_BYTES,
                correlation_family: correlation_family.to_string(),
            }),
            aftermath: self.aftermath,
            marker: PhantomData,
        }
    }
}

impl<Schema, Operation, Input, const EXTERNAL_EFFECT_DECIDED: bool>
    ApplicationOperationDefinitionBuilder<Schema, Operation, Input, EXTERNAL_EFFECT_DECIDED, false>
{
    /// Selects the explicit absence of an aftermath contract.
    pub fn no_aftermath(
        self,
    ) -> ApplicationOperationDefinitionBuilder<
        Schema,
        Operation,
        Input,
        EXTERNAL_EFFECT_DECIDED,
        true,
    > {
        ApplicationOperationDefinitionBuilder {
            operation: self.operation,
            external_effect: self.external_effect,
            aftermath: None,
            marker: PhantomData,
        }
    }

    /// Selects the operation's one aftermath contract.
    pub fn aftermath(
        self,
        contract: DeclaredApplicationAftermathContract,
    ) -> ApplicationOperationDefinitionBuilder<
        Schema,
        Operation,
        Input,
        EXTERNAL_EFFECT_DECIDED,
        true,
    > {
        ApplicationOperationDefinitionBuilder {
            operation: self.operation,
            external_effect: self.external_effect,
            aftermath: Some(contract),
            marker: PhantomData,
        }
    }
}

impl<Schema, Operation, Input>
    ApplicationOperationDefinitionBuilder<Schema, Operation, Input, true, true>
{
    pub fn finish(self) -> ApplicationOperationDefinition<Schema, Operation, Input> {
        ApplicationOperationDefinition {
            operation: self.operation,
            input_type: std::any::type_name::<Input>(),
            external_effect: self.external_effect,
            aftermath: self.aftermath,
            marker: PhantomData,
        }
    }
}
