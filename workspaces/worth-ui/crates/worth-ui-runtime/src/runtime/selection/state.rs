/// UI-selection-owned state axis. The persistence posture is classification,
/// not durability or undo authority.
pub(in crate::runtime) struct UiSelectionRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
}

impl UiSelectionRuntimeState {
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
