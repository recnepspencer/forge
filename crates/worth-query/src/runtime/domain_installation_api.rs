use crate::domain_installation::{
    WorthQueryDomainExecutionIndexRebuildReport, WorthQueryDomainHandleDenial,
    WorthQueryDomainInstallationLookupCounters, WorthQueryDomainInstallationReceipt,
    WorthQueryInstalledDomainHandle,
};

use super::WorthQueryRuntime;

impl WorthQueryRuntime {
    pub fn domain<D: 'static>(
        &self,
        _marker: D,
    ) -> Result<WorthQueryInstalledDomainHandle<D>, WorthQueryDomainHandleDenial> {
        self.domain_installation_registry.domain::<D>()
    }

    pub fn domain_installation_receipt<D: 'static>(
        &self,
        _marker: D,
    ) -> Option<&WorthQueryDomainInstallationReceipt> {
        self.domain_installation_registry.receipt::<D>()
    }

    pub fn domain_installation_receipts(
        &self,
    ) -> impl ExactSizeIterator<Item = &WorthQueryDomainInstallationReceipt> {
        self.domain_installation_registry.receipts()
    }

    pub fn validate_installed_domain_handle<D: 'static>(
        &self,
        handle: &WorthQueryInstalledDomainHandle<D>,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.domain_installation_registry.validate(handle)
    }

    pub fn domain_installation_lookup_counters(
        &self,
    ) -> WorthQueryDomainInstallationLookupCounters {
        self.domain_installation_registry.lookup_counters()
    }

    pub fn verify_domain_execution_index_rebuild(
        &self,
    ) -> WorthQueryDomainExecutionIndexRebuildReport {
        self.domain_installation_registry
            .rebuild_execution_index_report()
    }

    pub(crate) fn installed_domain_execution_index(
        &self,
    ) -> &crate::domain_installation::WorthQueryInstalledDomainExecutionIndex {
        self.domain_installation_registry.execution_index()
    }
}
