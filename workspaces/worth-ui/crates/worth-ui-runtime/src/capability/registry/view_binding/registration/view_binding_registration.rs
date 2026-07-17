use crate::capability::{CapabilitySupportKind, RegistrationCandidate, VIEW_BINDING_FAMILY_NAME};

use super::super::ViewBindingDescriptor;

impl ViewBindingDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        RegistrationCandidate::new(
            VIEW_BINDING_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        )
    }
}
