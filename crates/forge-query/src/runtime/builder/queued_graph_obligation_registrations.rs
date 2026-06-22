use super::*;

#[derive(Default)]
pub(super) struct QueuedGraphObligationRegistrations {
    explicit_registrations: Vec<ForgeQueryGraphObligationRegistration>,
}

impl QueuedGraphObligationRegistrations {
    pub(super) fn is_empty(&self) -> bool {
        self.explicit_registrations.is_empty()
    }

    pub(super) fn push(&mut self, registration: ForgeQueryGraphObligationRegistration) {
        self.explicit_registrations.push(registration);
    }

    pub(super) fn extend(
        &mut self,
        registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
    ) {
        self.explicit_registrations.extend(registrations);
    }

    pub(super) fn into_explicit_registrations(self) -> Vec<ForgeQueryGraphObligationRegistration> {
        self.explicit_registrations
    }
}
