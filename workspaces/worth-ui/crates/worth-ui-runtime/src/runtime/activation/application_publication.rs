/// Prepared application ownership transition committed inside the runtime's
/// complete publication transaction.
pub(crate) struct WorthUiPreparedApplicationPublication<'application> {
    active: &'application mut crate::facade::WorthUiApp,
    successor: crate::facade::WorthUiApp,
}

impl<'application> WorthUiPreparedApplicationPublication<'application> {
    pub(crate) fn new(
        active: &'application mut crate::facade::WorthUiApp,
        successor: crate::facade::WorthUiApp,
    ) -> Self {
        Self { active, successor }
    }

    pub(super) fn commit_once(self) {
        *self.active = self.successor;
    }
}
