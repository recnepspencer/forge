use std::marker::PhantomData;

use crate::application_schema::{
    ApplicationAuthorizationTraversalDirection, ApplicationEntityRef, ApplicationRelationRef,
};

use super::{ApplicationCapabilityContextRef, ApplicationCapabilityRelationBinding};

pub struct ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity> {
    context: &'static str,
    slot: &'static str,
    entity: &'static str,
    _marker: PhantomData<fn() -> (Schema, Context, Slot, Entity)>,
}

impl<Schema, Context, Slot, Entity> Copy
    for ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>
{
}

impl<Schema, Context, Slot, Entity> Clone
    for ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Context, Slot, Entity>
    ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>
{
    #[doc(hidden)]
    pub const fn from_schema_identifiers(
        context: ApplicationCapabilityContextRef<Schema, Context>,
        slot: &'static str,
        entity: ApplicationEntityRef<Schema, Entity>,
    ) -> Self {
        Self {
            context: context.name(),
            slot,
            entity: entity.name(),
            _marker: PhantomData,
        }
    }

    pub const fn context(self) -> &'static str {
        self.context
    }

    pub fn context_type(self) -> &'static str {
        std::any::type_name::<Context>()
    }

    pub const fn slot(self) -> &'static str {
        self.slot
    }

    pub fn slot_type(self) -> &'static str {
        std::any::type_name::<Slot>()
    }

    pub const fn entity(self) -> &'static str {
        self.entity
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityContextEntitySlotBinding {
    context: String,
    context_type: String,
    slot: String,
    slot_type: String,
    entity: String,
}

impl ApplicationCapabilityContextEntitySlotBinding {
    pub fn from_reference<Schema, Context, Slot, Entity>(
        slot: ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>,
    ) -> Self {
        Self {
            context: slot.context().to_string(),
            context_type: slot.context_type().to_string(),
            slot: slot.slot().to_string(),
            slot_type: slot.slot_type().to_string(),
            entity: slot.entity().to_string(),
        }
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn context_type(&self) -> &str {
        &self.context_type
    }

    pub fn slot(&self) -> &str {
        &self.slot
    }

    pub fn slot_type(&self) -> &str {
        &self.slot_type
    }

    pub fn entity(&self) -> &str {
        &self.entity
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityPathContextAnchor {
    relation: ApplicationCapabilityRelationBinding,
    direction: ApplicationAuthorizationTraversalDirection,
    slot: ApplicationCapabilityContextEntitySlotBinding,
}

impl ApplicationCapabilityPathContextAnchor {
    pub fn after_forward<Schema, Relation, From, Entity, Context, Slot>(
        relation: ApplicationRelationRef<Schema, Relation, From, Entity>,
        slot: ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>,
    ) -> Self {
        Self {
            relation: ApplicationCapabilityRelationBinding::from_reference(relation),
            direction: ApplicationAuthorizationTraversalDirection::Forward,
            slot: ApplicationCapabilityContextEntitySlotBinding::from_reference(slot),
        }
    }

    pub fn after_reverse<Schema, Relation, Entity, To, Context, Slot>(
        relation: ApplicationRelationRef<Schema, Relation, Entity, To>,
        slot: ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>,
    ) -> Self {
        Self {
            relation: ApplicationCapabilityRelationBinding::from_reference(relation),
            direction: ApplicationAuthorizationTraversalDirection::Reverse,
            slot: ApplicationCapabilityContextEntitySlotBinding::from_reference(slot),
        }
    }

    pub const fn relation(&self) -> &ApplicationCapabilityRelationBinding {
        &self.relation
    }

    pub const fn direction(&self) -> ApplicationAuthorizationTraversalDirection {
        self.direction
    }

    pub const fn slot(&self) -> &ApplicationCapabilityContextEntitySlotBinding {
        &self.slot
    }
}
