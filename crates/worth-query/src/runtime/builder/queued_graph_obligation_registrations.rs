use super::*;

#[derive(Default)]
pub(super) struct QueuedGraphObligationRegistrations {
    explicit_registrations: Vec<WorthQueryGraphObligationRegistration>,
}

impl QueuedGraphObligationRegistrations {
    pub(super) fn push(&mut self, registration: WorthQueryGraphObligationRegistration) {
        self.explicit_registrations.push(registration);
    }

    pub(super) fn extend(
        &mut self,
        registrations: impl IntoIterator<Item = WorthQueryGraphObligationRegistration>,
    ) {
        self.explicit_registrations.extend(registrations);
    }

    pub(super) fn into_explicit_registrations(self) -> Vec<WorthQueryGraphObligationRegistration> {
        self.explicit_registrations
    }
}

pub(super) fn graph_obligation_registration_error(
    stage: &'static str,
    error: WorthQueryGraphObligationRegistrationDenial,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::InvariantRegistration {
        stage,
        message: format!("{error}"),
    }
}
