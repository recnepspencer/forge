#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRebindPhaseSelectionStatus {
    PreservedEquivalentReload,
    PreservedDeniedReload,
    PreservedWithoutIntersection,
    RebuildScheduled,
}

impl WorthUiRebindPhaseSelectionStatus {
    pub(crate) fn skipped_phase(self) -> bool {
        matches!(
            self,
            Self::PreservedEquivalentReload
                | Self::PreservedDeniedReload
                | Self::PreservedWithoutIntersection
        )
    }

    pub(crate) fn rebuild_attempt(self) -> bool {
        matches!(self, Self::RebuildScheduled)
    }

    pub(crate) fn preserved_projection(self) -> bool {
        !self.rebuild_attempt()
    }

    pub(crate) fn rebuilt_projection(self) -> bool {
        self.rebuild_attempt()
    }
}
