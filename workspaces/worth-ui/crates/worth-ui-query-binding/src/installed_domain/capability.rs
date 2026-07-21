use worth_query::facade::domain::WorthQueryInstalledDomainHandle;
pub use worth_query::facade::domain::{
    WorthQueryDomainHandleDenial as WorthUiQueryInstallationDenial,
    WorthQueryDomainHandleDenialKind as WorthUiQueryInstallationDenialKind,
};

use crate::WorthUiDomainEntry;

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
