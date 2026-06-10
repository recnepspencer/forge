use super::{CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture};

/// Proof that a capability is runtime-owned and not public admitted support.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlatformInternalCapability<T: CapabilitySupportId> {
    id: T,
}

impl<T: CapabilitySupportId> PlatformInternalCapability<T> {
    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn kind(&self) -> CapabilitySupportKind {
        CapabilitySupportKind::PlatformInternal
    }
}

impl<T: CapabilitySupportId + Clone> PlatformInternalCapability<T> {
    pub fn posture(&self) -> CapabilitySupportPosture<T> {
        CapabilitySupportPosture::platform_internal(self.id.clone())
    }
}
