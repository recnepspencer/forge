/// Comparison-safe identity of one prepared generation executing inside one
/// exact launched application session.
///
/// Prepared generation identity remains semantic: independently prepared,
/// equivalent applications may compare equal. Operational interaction
/// authority additionally requires the host-issued active session identity,
/// so it cannot cross between those equivalent launches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActiveApplicationGenerationIdentity {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    prepared:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
}

impl WorthUiActiveApplicationGenerationIdentity {
    pub(crate) fn current(
        session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
        prepared: &crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    ) -> Self {
        Self {
            session,
            prepared: prepared.clone(),
        }
    }

    pub const fn session_identity(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn prepared_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.prepared
    }
}
