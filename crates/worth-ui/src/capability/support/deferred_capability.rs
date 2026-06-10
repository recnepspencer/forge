use super::{CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture};

/// Proof that a capability is visible but not admitted for lowering.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeferredCapability<T: CapabilitySupportId> {
    id: T,
}

impl<T: CapabilitySupportId> DeferredCapability<T> {
    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn kind(&self) -> CapabilitySupportKind {
        CapabilitySupportKind::Deferred
    }
}

impl<T: CapabilitySupportId + Clone> DeferredCapability<T> {
    pub fn posture(&self) -> CapabilitySupportPosture<T> {
        CapabilitySupportPosture::deferred(self.id.clone())
    }
}
