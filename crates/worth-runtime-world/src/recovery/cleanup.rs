use super::ProductUnpublishedNextAction;

/// Explicit recovery cleanup permission. It cannot be inferred from a
/// product publication or from dropping a caller's inspection handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecoveryCleanupContract {
    action: ProductUnpublishedNextAction,
}

impl RecoveryCleanupContract {
    pub(crate) const fn release_obligations() -> Self {
        Self {
            action: ProductUnpublishedNextAction::ReleaseObligations,
        }
    }

    pub(crate) const fn action(self) -> ProductUnpublishedNextAction {
        self.action
    }
}
