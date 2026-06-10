use crate::capability::{
    CapabilitySupportKind, CommandDescriptor, RegistrationCandidate, RegistrationDependency,
    COMMAND_FAMILY_NAME, COMMAND_PROJECTION_FAMILY_NAME,
};

impl CommandDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            COMMAND_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );

        match self.projection_eligibility() {
            Some(projection_id) => candidate.with_dependency(RegistrationDependency::new(
                COMMAND_PROJECTION_FAMILY_NAME,
                COMMAND_PROJECTION_FAMILY_NAME,
                projection_id.as_str(),
            )),
            None => candidate,
        }
    }
}
