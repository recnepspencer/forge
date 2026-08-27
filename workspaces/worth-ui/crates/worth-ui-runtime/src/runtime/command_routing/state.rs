/// Command-routing-owned state axis. Prefix occupancy may be ephemeral; this
/// posture grants no command history or inverse-operation authority.
pub(in crate::runtime) struct UiCommandRoutingRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
}

impl UiCommandRoutingRuntimeState {
    pub(in crate::runtime) const fn new(
        persistence: crate::runtime::UiServiceStatePersistencePosture,
    ) -> Self {
        Self { persistence }
    }

    pub(in crate::runtime) const fn persistence(
        &self,
    ) -> crate::runtime::UiServiceStatePersistencePosture {
        self.persistence
    }
}
