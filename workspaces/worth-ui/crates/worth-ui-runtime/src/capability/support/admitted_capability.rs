use super::{CapabilitySupportId, CapabilitySupportKind, CapabilitySupportPosture};

/// Proof that a capability satisfied an admitted-support requirement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdmittedCapability<T: CapabilitySupportId> {
    id: T,
}

impl<T: CapabilitySupportId> AdmittedCapability<T> {
    pub(crate) fn from_checked_id(id: T) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn kind(&self) -> CapabilitySupportKind {
        CapabilitySupportKind::Admitted
    }
}

impl<T: CapabilitySupportId + Clone> AdmittedCapability<T> {
    pub fn posture(&self) -> CapabilitySupportPosture<T> {
        CapabilitySupportPosture::admitted(self.id.clone())
    }
}
