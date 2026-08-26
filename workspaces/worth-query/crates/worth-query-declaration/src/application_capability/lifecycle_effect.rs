use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::sync::Arc;

use crate::application_schema::{ApplicationEffectPayload, ApplicationEffectRef, OperationEmits};
use crate::portable_identity::{WorthQueryPortableType, WorthQueryPortableTypeIdentity};

/// Typed derivation of the one effect caused by an elevation lifecycle input.
///
/// Query fixes the effect target at declaration and invokes this trait on the
/// exact input retained by capability admission. The caller cannot append an
/// independently authored emission to the framework-owned lifecycle program.
pub trait ApplicationCapabilityLifecycleEffect<Schema, Operation>: 'static {
    type Effect: OperationEmits<Operation>;
    type Payload: ApplicationEffectPayload + WorthQueryPortableType;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload>;

    fn lifecycle_effect(&self) -> Option<Self::Payload>;
}

#[derive(Clone)]
pub struct ApplicationCapabilityLifecycleEffectBinding {
    effect: String,
    effect_type: String,
    payload_type: WorthQueryPortableTypeIdentity,
    derive: fn(&dyn Any) -> Option<DerivedApplicationCapabilityLifecycleEffect>,
}

impl ApplicationCapabilityLifecycleEffectBinding {
    pub(super) fn from_input<Schema, Operation, Input>() -> Self
    where
        Input: ApplicationCapabilityLifecycleEffect<Schema, Operation>,
    {
        Self {
            effect: Input::effect().name().to_string(),
            effect_type: Input::effect().name().to_string(),
            payload_type: Input::Payload::PORTABLE_TYPE_IDENTITY,
            derive: derive_from_input::<Schema, Operation, Input>,
        }
    }

    pub fn effect(&self) -> &str {
        &self.effect
    }

    pub fn effect_type(&self) -> &str {
        &self.effect_type
    }

    pub fn payload_type(&self) -> &str {
        self.payload_type.as_str()
    }

    pub(crate) fn derive_from_retained_input(
        &self,
        input: &dyn Any,
    ) -> Option<DerivedApplicationCapabilityLifecycleEffect> {
        (self.derive)(input).filter(|derived| {
            derived.effect() == self.effect && derived.payload_type() == self.payload_type.as_str()
        })
    }

    fn meaning(&self) -> (&str, &str, WorthQueryPortableTypeIdentity) {
        (&self.effect, &self.effect_type, self.payload_type)
    }
}

impl std::fmt::Debug for ApplicationCapabilityLifecycleEffectBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationCapabilityLifecycleEffectBinding")
            .field("effect", &self.effect)
            .field("effect_type", &self.effect_type)
            .field("payload_type", &self.payload_type)
            .finish_non_exhaustive()
    }
}

impl PartialEq for ApplicationCapabilityLifecycleEffectBinding {
    fn eq(&self, other: &Self) -> bool {
        self.meaning() == other.meaning()
    }
}

impl Eq for ApplicationCapabilityLifecycleEffectBinding {}

impl PartialOrd for ApplicationCapabilityLifecycleEffectBinding {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ApplicationCapabilityLifecycleEffectBinding {
    fn cmp(&self, other: &Self) -> Ordering {
        self.meaning().cmp(&other.meaning())
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct DerivedApplicationCapabilityLifecycleEffect {
    effect: &'static str,
    payload_type: WorthQueryPortableTypeIdentity,
    payload_type_id: TypeId,
    payload: Arc<dyn Any + Send + Sync>,
    retained_bytes: u64,
    measure_retained_bytes: fn(&(dyn Any + Send + Sync)) -> Option<u64>,
}

impl DerivedApplicationCapabilityLifecycleEffect {
    pub fn effect(&self) -> &'static str {
        self.effect
    }

    pub fn payload_type(&self) -> &'static str {
        self.payload_type.as_str()
    }

    pub const fn payload_identity(&self) -> WorthQueryPortableTypeIdentity {
        self.payload_type
    }

    pub fn payload_type_id(&self) -> TypeId {
        self.payload_type_id
    }

    pub fn payload(&self) -> Arc<dyn Any + Send + Sync> {
        Arc::clone(&self.payload)
    }

    pub const fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    pub fn measure_retained_bytes(&self) -> fn(&(dyn Any + Send + Sync)) -> Option<u64> {
        self.measure_retained_bytes
    }

    pub fn payload_is(&self, payload: &Arc<dyn Any + Send + Sync>) -> bool {
        Arc::ptr_eq(&self.payload, payload)
    }
}

impl std::fmt::Debug for DerivedApplicationCapabilityLifecycleEffect {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DerivedApplicationCapabilityLifecycleEffect")
            .field("effect", &self.effect)
            .field("payload_type", &self.payload_type)
            .field("retained_bytes", &self.retained_bytes)
            .finish_non_exhaustive()
    }
}

fn derive_from_input<Schema, Operation, Input>(
    input: &dyn Any,
) -> Option<DerivedApplicationCapabilityLifecycleEffect>
where
    Input: ApplicationCapabilityLifecycleEffect<Schema, Operation>,
{
    let input = input.downcast_ref::<Input>()?;
    let payload = input.lifecycle_effect()?;
    let retained_bytes = payload.retained_bytes();
    Some(DerivedApplicationCapabilityLifecycleEffect {
        effect: Input::effect().name(),
        payload_type: Input::Payload::PORTABLE_TYPE_IDENTITY,
        payload_type_id: TypeId::of::<Input::Payload>(),
        payload: Arc::new(payload),
        retained_bytes,
        measure_retained_bytes: measure_retained_bytes::<Input::Payload>,
    })
}

fn measure_retained_bytes<Payload>(payload: &(dyn Any + Send + Sync)) -> Option<u64>
where
    Payload: ApplicationEffectPayload,
{
    payload
        .downcast_ref::<Payload>()
        .map(ApplicationEffectPayload::retained_bytes)
}
