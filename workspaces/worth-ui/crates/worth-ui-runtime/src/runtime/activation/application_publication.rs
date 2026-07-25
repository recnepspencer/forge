/// Prepared application ownership transition committed inside the runtime's
/// complete publication transaction.
pub(crate) struct WorthUiPreparedApplicationPublication {
    successor: crate::facade::WorthUiApp,
}

impl WorthUiPreparedApplicationPublication {
    pub(crate) fn new(successor: crate::facade::WorthUiApp) -> Self {
        Self { successor }
    }

    pub(super) fn commit_once(self, active: &mut crate::facade::WorthUiApp) {
        *active = self.successor;
    }
}
