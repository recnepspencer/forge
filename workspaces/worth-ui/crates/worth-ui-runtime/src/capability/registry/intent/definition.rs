use core::marker::PhantomData;

use super::{
    UiIntentAcceptedInteractions, UiIntentExecutionDestination, UiIntentId, UiIntentPayload,
    UiIntentProductOutcome, UiIntentRuntimeServiceDestination, UiIntentSchema,
    UiIntentTransitionDestination,
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
pub struct UiIntentDefinition<I: UiIntent> {
    destination: UiIntentExecutionDestination,
    intent: PhantomData<fn() -> I>,
}

impl<I: UiIntent> Clone for UiIntentDefinition<I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: UiIntent> Copy for UiIntentDefinition<I> {}

impl<I: UiIntent> UiIntentDefinition<I> {
    pub const fn application_effect() -> Self {
        Self::for_destination(UiIntentExecutionDestination::ApplicationEffect)
    }

    pub const fn ui_transition(destination: UiIntentTransitionDestination) -> Self {
        Self::for_destination(UiIntentExecutionDestination::UiTransition(destination))
    }

    pub const fn runtime_service(destination: UiIntentRuntimeServiceDestination) -> Self {
        Self::for_destination(UiIntentExecutionDestination::RuntimeService(destination))
    }

    pub const fn id(&self) -> UiIntentId {
        I::ID
    }

    pub const fn payload_schema(&self) -> UiIntentSchema {
        I::Payload::SCHEMA
    }

    pub const fn product_outcome_schema(&self) -> UiIntentSchema {
        I::ProductOutcome::SCHEMA
    }

    pub const fn accepted_interactions(&self) -> UiIntentAcceptedInteractions {
        I::ACCEPTED_INTERACTIONS
    }

    pub const fn execution_destination(&self) -> UiIntentExecutionDestination {
        self.destination
    }

    pub(crate) fn erase(self) -> IntentDefinitionDescriptor {
        IntentDefinitionDescriptor {
            id: self.id(),
            payload_schema: self.payload_schema(),
            product_outcome_schema: self.product_outcome_schema(),
            accepted_interactions: self.accepted_interactions(),
            destination: self.destination,
        }
    }

    const fn for_destination(destination: UiIntentExecutionDestination) -> Self {
        Self {
            destination,
            intent: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntentDefinitionDescriptor {
    id: UiIntentId,
    payload_schema: UiIntentSchema,
    product_outcome_schema: UiIntentSchema,
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

    pub fn product_outcome_schema(&self) -> UiIntentSchema {
        self.product_outcome_schema
    }

    pub fn accepted_interactions(&self) -> &'static [super::UiSemanticInteractionFamily] {
        self.accepted_interactions.as_slice()
    }

    pub fn execution_destination(&self) -> UiIntentExecutionDestination {
        self.destination
    }
}
