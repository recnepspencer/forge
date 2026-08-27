/// Scroll-owned state axis. Offset persistence remains a classified future
/// candidate, never persistence authority.
pub(in crate::runtime) struct UiScrollRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
}

impl UiScrollRuntimeState {
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
