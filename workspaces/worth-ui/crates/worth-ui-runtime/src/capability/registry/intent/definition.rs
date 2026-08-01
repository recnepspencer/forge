use core::marker::PhantomData;

use super::{
    UiIntentAcceptedInteractions, UiIntentExecutionDestination, UiIntentId, UiIntentPayload,
    UiIntentPayloadFieldSet, UiIntentProductOutcome, UiIntentRuntimeServiceDestination,
    UiIntentSchema, UiIntentTransitionDestination, UiIntentTransitionOutcome,
};

/// Product-defined typed intent meaning.
pub trait UiIntent: Send + Sync + 'static {
    type Payload: UiIntentPayload;
    type ProductOutcome: UiIntentProductOutcome;

    const ID: UiIntentId;
    const ACCEPTED_INTERACTIONS: UiIntentAcceptedInteractions;
}

/// Compiled capability definition preserving one concrete intent type.
#[derive(Debug)]
pub struct UiIntentDefinition<
    I: UiIntent,
    Destination: UiIntentDefinitionDestination = UiApplicationEffectDestination,
> {
    destination: Destination,
    intent: PhantomData<fn() -> I>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiApplicationEffectDestination;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiTransitionDefinitionDestination {
    destination: UiIntentTransitionDestination,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRuntimeServiceDefinitionDestination {
    destination: UiIntentRuntimeServiceDestination,
}

pub trait UiIntentDefinitionDestination: private::Sealed + Copy + Send + Sync + 'static {
    fn execution_destination(self) -> UiIntentExecutionDestination;
}

impl<I: UiIntent, D: UiIntentDefinitionDestination> Clone for UiIntentDefinition<I, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: UiIntent, D: UiIntentDefinitionDestination> Copy for UiIntentDefinition<I, D> {}

impl<I: UiIntent> UiIntentDefinition<I, UiApplicationEffectDestination> {
    pub const fn application_effect() -> Self {
        Self::for_destination(UiApplicationEffectDestination)
    }

    pub fn ui_transition(
        destination: UiIntentTransitionDestination,
    ) -> UiIntentDefinition<I, UiTransitionDefinitionDestination>
    where
        I::ProductOutcome: UiIntentTransitionOutcome,
    {
        UiIntentDefinition::for_destination(UiTransitionDefinitionDestination { destination })
    }

    pub const fn runtime_service(
        destination: UiIntentRuntimeServiceDestination,
    ) -> UiIntentDefinition<I, UiRuntimeServiceDefinitionDestination> {
        UiIntentDefinition::for_destination(UiRuntimeServiceDefinitionDestination { destination })
    }
}

impl<I: UiIntent, D: UiIntentDefinitionDestination> UiIntentDefinition<I, D> {
    pub const fn id(&self) -> UiIntentId {
        I::ID
    }

    pub const fn payload_schema(&self) -> UiIntentSchema {
        I::Payload::SCHEMA
    }

    pub const fn product_outcome_schema(&self) -> UiIntentSchema {
        I::ProductOutcome::SCHEMA
    }

    pub const fn product_consequence_families(&self) -> super::UiIntentProductConsequenceFamilies {
        I::ProductOutcome::CONSEQUENCE_FAMILIES
    }

    pub const fn accepted_interactions(&self) -> UiIntentAcceptedInteractions {
        I::ACCEPTED_INTERACTIONS
    }

    pub fn execution_destination(&self) -> UiIntentExecutionDestination {
        self.destination.execution_destination()
    }

    pub(crate) fn descriptor(&self) -> IntentDefinitionDescriptor {
        IntentDefinitionDescriptor {
            id: self.id(),
            payload_schema: self.payload_schema(),
            payload_fields: I::Payload::FIELDS,
            product_outcome_schema: self.product_outcome_schema(),
            product_consequence_families: self.product_consequence_families(),
            accepted_interactions: self.accepted_interactions(),
            destination: self.execution_destination(),
        }
    }

    pub(crate) fn erase(self) -> UiRegisteredIntentDefinition {
        UiRegisteredIntentDefinition {
            descriptor: self.descriptor(),
        }
    }

    const fn for_destination(destination: D) -> Self {
        Self {
            destination,
            intent: PhantomData,
        }
    }
}

impl UiIntentDefinitionDestination for UiApplicationEffectDestination {
    fn execution_destination(self) -> UiIntentExecutionDestination {
        UiIntentExecutionDestination::ApplicationEffect
    }
}

impl UiIntentDefinitionDestination for UiTransitionDefinitionDestination {
    fn execution_destination(self) -> UiIntentExecutionDestination {
        UiIntentExecutionDestination::UiTransition(self.destination)
    }
}

impl UiIntentDefinitionDestination for UiRuntimeServiceDefinitionDestination {
    fn execution_destination(self) -> UiIntentExecutionDestination {
        UiIntentExecutionDestination::RuntimeService(self.destination)
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::UiApplicationEffectDestination {}
    impl Sealed for super::UiTransitionDefinitionDestination {}
    impl Sealed for super::UiRuntimeServiceDefinitionDestination {}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntentDefinitionDescriptor {
    id: UiIntentId,
    payload_schema: UiIntentSchema,
    payload_fields: UiIntentPayloadFieldSet,
    product_outcome_schema: UiIntentSchema,
    product_consequence_families: super::UiIntentProductConsequenceFamilies,
    accepted_interactions: UiIntentAcceptedInteractions,
    destination: UiIntentExecutionDestination,
}

impl IntentDefinitionDescriptor {
    pub fn id(&self) -> UiIntentId {
        self.id
    }

    pub fn payload_schema(&self) -> UiIntentSchema {
        self.payload_schema
    }

    pub fn payload_fields(&self) -> UiIntentPayloadFieldSet {
        self.payload_fields
    }

    pub fn product_outcome_schema(&self) -> UiIntentSchema {
        self.product_outcome_schema
    }

    pub const fn product_consequence_families(&self) -> super::UiIntentProductConsequenceFamilies {
        self.product_consequence_families
    }

    pub fn accepted_interactions(&self) -> &'static [super::UiSemanticInteractionFamily] {
        self.accepted_interactions.as_slice()
    }

    pub fn execution_destination(&self) -> UiIntentExecutionDestination {
        self.destination
    }
}

pub(crate) struct UiRegisteredIntentDefinition {
    descriptor: IntentDefinitionDescriptor,
}

impl Clone for UiRegisteredIntentDefinition {
    fn clone(&self) -> Self {
        Self {
            descriptor: self.descriptor.clone(),
        }
    }
}

impl core::fmt::Debug for UiRegisteredIntentDefinition {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("UiRegisteredIntentDefinition")
            .field("descriptor", &self.descriptor)
            .finish()
    }
}

impl PartialEq for UiRegisteredIntentDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}

impl Eq for UiRegisteredIntentDefinition {}

impl UiRegisteredIntentDefinition {
    pub(crate) fn descriptor(&self) -> &IntentDefinitionDescriptor {
        &self.descriptor
    }

    pub(crate) fn into_descriptor(self) -> IntentDefinitionDescriptor {
        self.descriptor
    }
}
