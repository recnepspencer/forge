use worth_query::facade::runtime::WorthQueryWorkspace;

use super::{WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial};
use crate::WorthUiDomainEntry;

/// Positive discovery surface for resolving Worth UI's installed Query domain.
pub trait WorthUiQueryWorkspaceExt {
    fn worth_ui(&self) -> Result<WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial>;
}

impl WorthUiQueryWorkspaceExt for WorthQueryWorkspace {
    fn worth_ui(&self) -> Result<WorthUiInstalledQueryDomain, WorthUiQueryInstallationDenial> {
        self.domain(WorthUiDomainEntry)
            .map(WorthUiInstalledQueryDomain::from_handle)
    }
}
