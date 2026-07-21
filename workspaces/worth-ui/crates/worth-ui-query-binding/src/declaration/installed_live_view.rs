use worth_query::facade::{read, runtime};

use crate::{
    WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiQueryExt,
    WorthUiQueryLiveOpenError, WorthUiQueryLiveOpenOutcome, WorthUiQueryViewDefinition,
    WorthUiQueryViewLifecycle,
};

/// Installed live view. Query-owned managed-resource operations are added on
/// this lifecycle type rather than on the registration envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledLiveQueryView {
    registration: WorthUiInstalledQueryView,
}

impl WorthUiInstalledLiveQueryView {
    pub(super) fn from_registration(registration: WorthUiInstalledQueryView) -> Self {
        debug_assert_eq!(
            registration.definition().lifecycle(),
            WorthUiQueryViewLifecycle::Live
        );
        Self { registration }
    }

    pub fn definition(&self) -> &WorthUiQueryViewDefinition {
        self.registration.definition()
    }

    pub fn installed_domain(&self) -> &WorthUiInstalledQueryDomain {
        self.registration.installed_domain()
    }

    pub fn open_using(
        &self,
        context: impl Into<read::WorthQueryReadContextDeclaration>,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<WorthUiQueryLiveOpenOutcome, WorthUiQueryLiveOpenError> {
        let declaration = self
            .installed_domain()
            .handle()
            .live_measurements(self.definition().identity().as_str())
            .map_err(WorthUiQueryLiveOpenError::Declaration)?;
        declaration
            .using(context)
            .open(workspace)
            .map(|outcome| {
                WorthUiQueryLiveOpenOutcome::from_query(self.definition().clone(), outcome)
            })
            .map_err(Box::new)
            .map_err(WorthUiQueryLiveOpenError::InstalledAuthority)
    }
}

impl From<WorthUiInstalledLiveQueryView> for WorthUiInstalledQueryView {
    fn from(view: WorthUiInstalledLiveQueryView) -> Self {
        view.registration
    }
}
