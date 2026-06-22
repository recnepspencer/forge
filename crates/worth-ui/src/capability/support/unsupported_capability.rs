use super::{CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture};

/// Proof that a capability is intentionally unsupported.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsupportedCapability<T: CapabilitySupportId> {
    id: T,
}

impl<T: CapabilitySupportId> UnsupportedCapability<T> {
    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn kind(&self) -> CapabilitySupportKind {
        CapabilitySupportKind::Unsupported
    }
}

impl<T: CapabilitySupportId + Clone> UnsupportedCapability<T> {
    pub fn posture(&self) -> CapabilitySupportPosture<T> {
        CapabilitySupportPosture::unsupported(self.id.clone())
    }
}
