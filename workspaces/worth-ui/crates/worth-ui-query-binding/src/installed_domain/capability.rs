use worth_query::facade::domain::{
    WorthQueryDomainHandleDenial, WorthQueryDomainHandleDenialKind, WorthQueryInstalledDomainHandle,
};

use crate::WorthUiDomainEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryInstallationDenialKind {
    DomainNotInstalled,
    ForeignRuntime,
    StaleInstallationGeneration,
    PackageIdentityChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryInstallationDenial {
    kind: WorthUiQueryInstallationDenialKind,
}

impl WorthUiQueryInstallationDenial {
    pub fn kind(&self) -> WorthUiQueryInstallationDenialKind {
        self.kind
    }
}

impl From<WorthQueryDomainHandleDenial> for WorthUiQueryInstallationDenial {
    fn from(denial: WorthQueryDomainHandleDenial) -> Self {
        let kind = match denial.kind() {
            WorthQueryDomainHandleDenialKind::DomainNotInstalled => {
                WorthUiQueryInstallationDenialKind::DomainNotInstalled
            }
            WorthQueryDomainHandleDenialKind::ForeignRuntime => {
                WorthUiQueryInstallationDenialKind::ForeignRuntime
            }
            WorthQueryDomainHandleDenialKind::StaleInstallationGeneration => {
                WorthUiQueryInstallationDenialKind::StaleInstallationGeneration
            }
            WorthQueryDomainHandleDenialKind::PackageIdentityChanged => {
                WorthUiQueryInstallationDenialKind::PackageIdentityChanged
            }
        };
        Self { kind }
    }
}

/// Runtime-affine Worth UI domain authority resolved by the owning Query
/// workspace. Consumers can clone the capability but cannot mint or rebuild it
/// from installation fields, receipts, or reporting projections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledQueryDomain {
    pub(crate) handle: WorthQueryInstalledDomainHandle<WorthUiDomainEntry>,
}

impl WorthUiInstalledQueryDomain {
    pub(crate) fn from_handle(handle: WorthQueryInstalledDomainHandle<WorthUiDomainEntry>) -> Self {
        Self { handle }
    }

    pub fn shares_authority_with(&self, other: &Self) -> bool {
        std::ptr::eq(self.handle.authority(), other.handle.authority())
    }

    pub(crate) fn handle(&self) -> &WorthQueryInstalledDomainHandle<WorthUiDomainEntry> {
        &self.handle
    }
}
