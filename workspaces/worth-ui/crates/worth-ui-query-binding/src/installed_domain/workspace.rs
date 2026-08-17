use worth_query::facade::runtime::WorthQueryWorkspace;

use super::{WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial};
use crate::WorthUiDomainEntry;

/// Certification and test discovery over the legacy raw Query workspace.
#[cfg(any(test, feature = "certification-construction"))]
pub trait WorthUiQueryWorkspaceExt {
    fn worth_ui(&self) -> Result<WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial>;
}

#[cfg(any(test, feature = "certification-construction"))]
impl WorthUiQueryWorkspaceExt for WorthQueryWorkspace {
    fn worth_ui(&self) -> Result<WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial> {
        WorthUiQueryHost::from_workspace(self).installed_domain()
    }
}

/// Crate-private bridge while the legacy workspace owner is migrated to
/// Query's decomposed host-audience runtime.
pub(crate) struct WorthUiQueryHost<'workspace> {
    workspace: &'workspace WorthQueryWorkspace,
}

impl<'workspace> WorthUiQueryHost<'workspace> {
    pub(crate) fn from_workspace(workspace: &'workspace WorthQueryWorkspace) -> Self {
        Self { workspace }
    }

    pub(crate) fn installed_domain(
        &self,
    ) -> Result<WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial> {
        self.workspace
            .domain(WorthUiDomainEntry)
            .map(WorthUiInstalledQueryDomain::from_handle)
    }
}
