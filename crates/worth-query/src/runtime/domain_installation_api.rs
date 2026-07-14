use crate::domain_installation::{
    WorthQueryDomainExecutionIndexRebuildReport, WorthQueryDomainHandleDenial,
    WorthQueryDomainInstallationLookupCounters, WorthQueryDomainInstallationReceipt,
    WorthQueryDomainRebindDenial, WorthQueryDomainRebindReceipt, WorthQueryDomainRebindRequest,
    WorthQueryInstalledDomainAuthorityWitness, WorthQueryInstalledDomainHandle,
    WorthQueryReboundDomainHandle,
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

    pub(crate) fn validate_installed_domain_witness<D: 'static>(
        &self,
        witness: &WorthQueryInstalledDomainAuthorityWitness,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.domain_installation_registry
            .validate_authority::<D>(witness.authority())
    }

    pub(crate) fn validate_installed_domain_authority(
        &self,
        witness: &WorthQueryInstalledDomainAuthorityWitness,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.domain_installation_registry
            .validate_erased_authority(witness.authority())
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

    pub fn rebind_domain<D: 'static>(
        &self,
        request: WorthQueryDomainRebindRequest<D>,
    ) -> Result<WorthQueryReboundDomainHandle<D>, WorthQueryDomainRebindDenial> {
        let prior = request.into_prior();
        let current = self
            .domain_installation_registry
            .domain::<D>()
            .map_err(|_| WorthQueryDomainRebindDenial::domain_not_installed(&prior))?;
        if prior.package_identity() != current.package_identity() {
            return Err(WorthQueryDomainRebindDenial::package_meaning_changed(
                &prior,
                current.authority(),
            ));
        }
        let current_witness = current.authority_witness();
        let receipt = WorthQueryDomainRebindReceipt::new(&prior, &current_witness);
        Ok(WorthQueryReboundDomainHandle::new(current, receipt))
    }

    pub(crate) fn installed_domain_execution_index(
        &self,
    ) -> &crate::domain_installation::WorthQueryInstalledDomainExecutionIndex {
        self.domain_installation_registry.execution_index()
    }
}
