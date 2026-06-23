#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionRebindStatus {
    PreservedEquivalentReload,
    PreservedDeniedReload,
    DeniedReloadNotActivated,
    EquivalentAfterActivation,
    ReboundAfterActivation,
}

impl WorthUiProjectionRebindStatus {
    pub(crate) fn preserves_frame(self) -> bool {
        matches!(
            self,
            Self::PreservedEquivalentReload
                | Self::PreservedDeniedReload
                | Self::DeniedReloadNotActivated
                | Self::EquivalentAfterActivation
        )
    }

    pub(crate) fn denied_frame(self) -> bool {
        matches!(self, Self::DeniedReloadNotActivated)
    }
}
