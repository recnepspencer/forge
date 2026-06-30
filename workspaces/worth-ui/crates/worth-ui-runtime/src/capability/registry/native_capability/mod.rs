mod descriptor;
mod frozen_native_capabilities;
mod frozen_native_capability_entry;
mod native_capability_key;
mod native_capability_registry;
mod registration;

pub use descriptor::{
    AmbientHostCheck, NativeCapabilityDescriptor, NativeCapabilityFamily, NativePlatformPosture,
    NativeShellAuthorityClaim,
};
pub use frozen_native_capabilities::FrozenNativeCapabilities;
pub use frozen_native_capability_entry::FrozenNativeCapabilityEntry;
pub use native_capability_key::NativeCapabilityKey;
pub(crate) use native_capability_registry::NativeCapabilityRegistry;
pub(crate) use registration::NativeCapabilityAcceptedRegistrationProof;
