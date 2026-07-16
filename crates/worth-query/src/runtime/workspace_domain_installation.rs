use super::WorthQueryWorkspace;

impl WorthQueryWorkspace {
    pub fn domain<D: 'static>(
        &self,
        marker: D,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledDomainHandle<D>,
        crate::domain_installation::WorthQueryDomainHandleDenial,
    > {
        self.runtime.domain(marker)
    }

    pub fn domain_installation_receipt<D: 'static>(
        &self,
        marker: D,
    ) -> Option<&crate::domain_installation::WorthQueryDomainInstallationReceipt> {
        self.runtime.domain_installation_receipt(marker)
    }

    pub(crate) fn validate_installed_domain_witness<D: 'static>(
        &self,
        witness: &crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness,
    ) -> Result<(), crate::domain_installation::WorthQueryDomainHandleDenial> {
        self.runtime.validate_installed_domain_witness::<D>(witness)
    }

    #[cfg(test)]
    pub(crate) fn replace_domain_installation_with_successor_generation(&mut self) {
        self.runtime
            .replace_domain_installation_with_successor_generation();
    }
}
