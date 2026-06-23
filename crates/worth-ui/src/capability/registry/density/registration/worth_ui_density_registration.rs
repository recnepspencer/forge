use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, WorthUiDensityTokenDescriptor,
    DENSITY_TOKEN_FAMILY_NAME,
};

impl WorthUiDensityTokenDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        RegistrationCandidate::new(
            DENSITY_TOKEN_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        )
    }
}
