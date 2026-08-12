use crate::application_schema::{
    ApplicationEffectPayload, ApplicationEffectRef, ApplicationFieldUnit, EqualityPredicate,
    ReadOnly, TypedApplicationValue,
};

use super::ApplicationQueryResultFieldRef;

/// Domain-owned interpretation of one committed effect as a live-query cause.
///
/// The binding type is identity-bearing query meaning. Query invokes these
/// functions only after installation has matched the exact binding, effect,
/// payload, scope selector, and target selector.
pub trait ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>: 'static {
    type Effect;
    type Payload: ApplicationEffectPayload + Clone;
    type ScopeIdentity: TypedApplicationValue;
    type TargetIdentity: TypedApplicationValue;

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
    binding_type: &'static str,
    effect: &'static str,
    payload_type: &'static str,
    scope_slot_type: &'static str,
    scope_field: (&'static str, &'static str, &'static str),
    scope_value_type: &'static str,
    target_slot_type: &'static str,
    target_field: (&'static str, &'static str, &'static str),
    target_value_type: &'static str,
    resources: ApplicationQueryLiveResourceContract,
}

impl ApplicationQueryLiveCauseContract {
    pub const fn binding_type(&self) -> &'static str {
        self.binding_type
    }

    pub const fn effect(&self) -> &'static str {
        self.effect
    }

    pub const fn payload_type(&self) -> &'static str {
        self.payload_type
    }

    pub const fn scope_slot_type(&self) -> &'static str {
        self.scope_slot_type
    }

    pub const fn scope_field(&self) -> (&'static str, &'static str, &'static str) {
        self.scope_field
    }

    pub const fn scope_value_type(&self) -> &'static str {
        self.scope_value_type
    }

    pub const fn target_slot_type(&self) -> &'static str {
        self.target_slot_type
    }

    pub const fn target_field(&self) -> (&'static str, &'static str, &'static str) {
        self.target_field
    }

    pub const fn target_value_type(&self) -> &'static str {
        self.target_value_type
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
    {
        Self {
            binding_type: std::any::type_name::<Binding>(),
            effect: Binding::effect().name(),
            payload_type: std::any::type_name::<Binding::Payload>(),
            scope_slot_type: scope_identity.slot_type(),
            scope_field: (
                scope_identity.entity(),
                scope_identity.aspect(),
                scope_identity.field(),
            ),
            scope_value_type: std::any::type_name::<Binding::ScopeIdentity>(),
            target_slot_type: target_identity.slot_type(),
            target_field: (
                target_identity.entity(),
                target_identity.aspect(),
                target_identity.field(),
            ),
            target_value_type: std::any::type_name::<Binding::TargetIdentity>(),
            resources,
        }
    }
}
