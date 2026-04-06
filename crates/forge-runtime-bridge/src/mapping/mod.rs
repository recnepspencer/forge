//! Frozen bridge mapping registration and lookup surfaces.

pub(crate) mod aspects;
pub(crate) mod fallback;
pub(crate) mod freezing;
pub(crate) mod lookup;
pub(crate) mod registration;
pub(crate) mod subscriptions;

pub use aspects::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, SliceFallbackPolicy, TruthDeltaSurfaceKind,
};
pub use fallback::BridgeMappingFallbackClass;
pub(crate) use aspects::FrozenAspectMappingRegistry;
pub(crate) use freezing::{FrozenBridgeMappingRegistration, FrozenMappingRegistry};
pub(crate) use lookup::BridgeMappingLookup;
pub use registration::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    SignalInvalidationScope, TruthPatchScope,
};
pub use subscriptions::SubscriptionSliceKind;
