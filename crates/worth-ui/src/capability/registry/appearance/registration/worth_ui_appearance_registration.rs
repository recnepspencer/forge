use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, WorthUiAppearanceTokenDescriptor,
    APPEARANCE_TOKEN_FAMILY_NAME,
};

impl WorthUiAppearanceTokenDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        RegistrationCandidate::new(
            APPEARANCE_TOKEN_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        )
    }
}
