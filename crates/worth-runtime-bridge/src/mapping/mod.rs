//! Frozen bridge mapping registration and lookup surfaces.

pub(crate) mod aspects;
pub(crate) mod freezing;
pub(crate) mod lookup;
pub(crate) mod registration;
pub(crate) mod subscriptions;
pub(crate) mod widening;

pub(crate) use aspects::FrozenAspectMappingRegistry;
pub use aspects::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeAuthoritativeSourcePrecisionPolicy,
    SliceWideningPolicy, TruthDeltaSurfaceKind,
};
pub(crate) use freezing::{FrozenBridgeMappingRegistration, FrozenMappingRegistry};
pub(crate) use lookup::BridgeMappingLookup;
pub(crate) use registration::TruthPatchTargetView;
pub use registration::{
    AspectKeySelector, BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode,
    MappingSelector, SignalInvalidationScope, TruthPatchScope, TruthPatchTargetSelector,
};
pub use subscriptions::SubscriptionSliceKind;
pub use widening::BridgeMappingWideningClass;
