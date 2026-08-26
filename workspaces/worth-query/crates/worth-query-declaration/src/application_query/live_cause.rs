use crate::application_schema::{
    ApplicationEffectPayload, ApplicationEffectRef, ApplicationFieldUnit, EqualityPredicate,
    ReadOnly, TypedApplicationValue,
};
use crate::portable_identity::{WorthQueryPortableType, WorthQueryPortableTypeIdentity};

use super::{ApplicationQueryMarkerIdentity, ApplicationQueryResultFieldRef};

/// Domain-owned interpretation of one committed effect as a live-query cause.
///
/// The binding type is identity-bearing query meaning. Query invokes these
/// functions only after installation has matched the exact binding, effect,
/// payload, scope selector, and target selector.
pub trait ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>:
    WorthQueryPortableType + 'static
{
    type Effect;
    type Payload: ApplicationEffectPayload + WorthQueryPortableType + Clone;
    type ScopeIdentity: TypedApplicationValue + WorthQueryPortableType;
    type TargetIdentity: TypedApplicationValue + WorthQueryPortableType;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload>;
    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity;
    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryLiveResourceContract {
    maximum_buffered_causes: u64,
    maximum_work_per_delivery: u64,
    maximum_retained_payload_bytes: u64,
}

impl ApplicationQueryLiveResourceContract {
    pub const fn bounded(
        maximum_buffered_causes: u64,
        maximum_work_per_delivery: u64,
        maximum_retained_payload_bytes: u64,
    ) -> Self {
        Self {
            maximum_buffered_causes,
            maximum_work_per_delivery,
            maximum_retained_payload_bytes,
        }
    }

    pub const fn maximum_buffered_causes(self) -> u64 {
        self.maximum_buffered_causes
    }

    pub const fn maximum_work_per_delivery(self) -> u64 {
        self.maximum_work_per_delivery
    }

    pub const fn maximum_retained_payload_bytes(self) -> u64 {
        self.maximum_retained_payload_bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryLiveCauseContract {
    binding_type: WorthQueryPortableTypeIdentity,
    effect: &'static str,
    payload_type: WorthQueryPortableTypeIdentity,
    scope_slot_type: WorthQueryPortableTypeIdentity,
    scope_field: (&'static str, &'static str, &'static str),
    scope_value_type: WorthQueryPortableTypeIdentity,
    target_slot_type: WorthQueryPortableTypeIdentity,
    target_field: (&'static str, &'static str, &'static str),
    target_value_type: WorthQueryPortableTypeIdentity,
    resources: ApplicationQueryLiveResourceContract,
}

impl ApplicationQueryLiveCauseContract {
    pub const fn binding_type(&self) -> &'static str {
        self.binding_type.as_str()
    }

    pub const fn binding_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.binding_type
    }

    pub const fn effect(&self) -> &'static str {
        self.effect
    }

    pub const fn payload_type(&self) -> &'static str {
        self.payload_type.as_str()
    }

    pub const fn payload_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.payload_type
    }

    pub const fn scope_slot_type(&self) -> &'static str {
        self.scope_slot_type.as_str()
    }

    pub const fn scope_slot_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.scope_slot_type
    }

    pub const fn scope_field(&self) -> (&'static str, &'static str, &'static str) {
        self.scope_field
    }

    pub const fn scope_value_type(&self) -> &'static str {
        self.scope_value_type.as_str()
    }

    pub const fn target_slot_type(&self) -> &'static str {
        self.target_slot_type.as_str()
    }

    pub const fn target_slot_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.target_slot_type
    }

    pub const fn target_field(&self) -> (&'static str, &'static str, &'static str) {
        self.target_field
    }

    pub const fn target_value_type(&self) -> &'static str {
        self.target_value_type.as_str()
    }

    pub const fn resources(&self) -> ApplicationQueryLiveResourceContract {
        self.resources
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn typed<
        Schema,
        Query,
        Scope,
        Target,
        Binding,
        ScopeSlot,
        ScopeAspect,
        ScopeField,
        ScopeUnit,
        TargetSlot,
        TargetAspect,
        TargetField,
        TargetUnit,
    >(
        scope_identity: ApplicationQueryResultFieldRef<
            Query,
            ScopeSlot,
            Schema,
            Scope,
            ScopeAspect,
            ScopeField,
            Binding::ScopeIdentity,
            ReadOnly,
            EqualityPredicate,
            ScopeUnit,
        >,
        target_identity: ApplicationQueryResultFieldRef<
            Query,
            TargetSlot,
            Schema,
            Target,
            TargetAspect,
            TargetField,
            Binding::TargetIdentity,
            ReadOnly,
            EqualityPredicate,
            TargetUnit,
        >,
        resources: ApplicationQueryLiveResourceContract,
    ) -> Self
    where
        Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
        ScopeUnit: ApplicationFieldUnit,
        TargetUnit: ApplicationFieldUnit,
        Query: ApplicationQueryMarkerIdentity,
        ScopeSlot: WorthQueryPortableType,
        TargetSlot: WorthQueryPortableType,
    {
        Self {
            binding_type: Binding::PORTABLE_TYPE_IDENTITY,
            effect: Binding::effect().name(),
            payload_type: Binding::Payload::PORTABLE_TYPE_IDENTITY,
            scope_slot_type: scope_identity.slot_key().slot_identity(),
            scope_field: (
                scope_identity.entity(),
                scope_identity.aspect(),
                scope_identity.field(),
            ),
            scope_value_type: Binding::ScopeIdentity::PORTABLE_TYPE_IDENTITY,
            target_slot_type: target_identity.slot_key().slot_identity(),
            target_field: (
                target_identity.entity(),
                target_identity.aspect(),
                target_identity.field(),
            ),
            target_value_type: Binding::TargetIdentity::PORTABLE_TYPE_IDENTITY,
            resources,
        }
    }
}
