use crate::{WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiQueryViewDefinition};

/// Installed snapshot view used to register and resolve the stable operation.
/// Execution starts from its exact installed binding reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledSnapshotQueryView {
    registration: WorthUiInstalledQueryView,
}

impl WorthUiInstalledSnapshotQueryView {
    pub(super) fn from_registration(registration: WorthUiInstalledQueryView) -> Self {
        debug_assert_eq!(
            registration.definition().lifecycle(),
            crate::WorthUiQueryViewLifecycle::Snapshot
        );
        Self { registration }
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        self.registration.definition()
    }

    pub fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        self.registration.installed_domain()
    }
}

impl From<WorthUiInstalledSnapshotQueryView> for WorthUiInstalledQueryView {
    fn from(view: WorthUiInstalledSnapshotQueryView) -> Self {
        view.registration
    }
}
