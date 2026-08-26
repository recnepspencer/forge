use std::marker::PhantomData;

#[cfg(test)]
use crate::application_schema::ApplicationEntityRef;
use crate::application_schema::{
    ApplicationAuthorizationTraversalDirection, ApplicationRelationRef,
};
use crate::portable_identity::WorthQueryPortableTypeIdentity;

#[cfg(test)]
use super::ApplicationCapabilityContextRef;
use super::ApplicationCapabilityRelationBinding;

pub struct ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity> {
    context: &'static str,
    context_type: WorthQueryPortableTypeIdentity,
    slot: &'static str,
    slot_type: WorthQueryPortableTypeIdentity,
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
    #[cfg(test)]
    pub(crate) const fn from_schema_identifiers(
        context: ApplicationCapabilityContextRef<Schema, Context>,
        slot: &'static str,
        entity: ApplicationEntityRef<Schema, Entity>,
    ) -> Self {
        Self::from_test_declaration(
            context,
            slot,
            WorthQueryPortableTypeIdentity::declared(slot),
            entity.name(),
        )
    }

    #[doc(hidden)]
    #[cfg(test)]
    pub(crate) const fn from_test_declaration(
        context: ApplicationCapabilityContextRef<Schema, Context>,
        slot: &'static str,
        slot_identity: WorthQueryPortableTypeIdentity,
        entity: &'static str,
    ) -> Self {
        Self {
            context: context.name(),
            context_type: context.marker_identity(),
            slot,
            slot_type: slot_identity,
            entity,
            _marker: PhantomData,
        }
    }

    pub const fn context_identity(self) -> WorthQueryPortableTypeIdentity {
        self.context_type
    }

    pub const fn slot_identity(self) -> WorthQueryPortableTypeIdentity {
        self.slot_type
    }

    pub const fn context(self) -> &'static str {
        self.context
    }

    pub const fn slot(self) -> &'static str {
        self.slot
    }

    pub const fn entity(self) -> &'static str {
        self.entity
    }
}

impl<Schema, Context, Slot, Entity>
    ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>
where
    Context: super::ApplicationCapabilityContextMarkerIdentity<Schema = Schema>,
    Entity: crate::application_schema::ApplicationEntityMarkerIdentity<Schema = Schema>,
    Slot: super::ApplicationCapabilityContextEntitySlotMarkerIdentity<
        Schema = Schema,
        Context = Context,
        Entity = Entity,
    >,
{
    #[doc(hidden)]
    pub const fn from_declaration() -> Self {
        Self {
            context: Context::IDENTIFIER,
            context_type: Context::PORTABLE_TYPE_IDENTITY,
            slot: Slot::IDENTIFIER,
            slot_type: Slot::PORTABLE_TYPE_IDENTITY,
            entity: Entity::IDENTIFIER,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityContextEntitySlotBinding {
    context: String,
    context_identity: WorthQueryPortableTypeIdentity,
    slot: String,
    slot_identity: WorthQueryPortableTypeIdentity,
    entity: String,
}

impl ApplicationCapabilityContextEntitySlotBinding {
    pub fn from_reference<Schema, Context, Slot, Entity>(
        slot: ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>,
    ) -> Self {
        Self {
            context: slot.context().to_string(),
            context_identity: slot.context_identity(),
            slot: slot.slot().to_string(),
            slot_identity: slot.slot_identity(),
            entity: slot.entity().to_string(),
        }
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub const fn context_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.context_identity
    }

    pub fn slot(&self) -> &str {
        &self.slot
    }

    pub const fn slot_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.slot_identity
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
