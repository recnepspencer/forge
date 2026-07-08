//! Registry support inventory — admitted, deferred, and unsupported capability posture.

pub use crate::capability::{
    AdmittedCapability, AmbientHostCheck, ArbitraryKeyValueSettingBag, CapabilitySupportId,
    CapabilitySupportKind, CapabilitySupportPosture, CapabilitySupportRejection, DeferredCapability,
    RegistryFamily, RegistryFamilyFacadeExposure, RegistryFamilyInventoryAudit,
    RegistryFamilyLifecyclePropagation, SupportRequirement, SupportSnapshot, UnsupportedCapability,
};