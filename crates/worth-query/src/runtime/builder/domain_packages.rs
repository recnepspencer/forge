use crate::application::WorthQueryDomainEntryMarker;
use crate::domain_installation::{
    WorthQueryDomainPackage, WorthQueryDomainPackageInstallationError,
};

use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn domain_package<D: WorthQueryDomainEntryMarker + 'static>(
        mut self,
        package: WorthQueryDomainPackage<D>,
    ) -> Result<Self, WorthQueryDomainPackageInstallationError> {
        let validated = package
            .validate()
            .map_err(WorthQueryDomainPackageInstallationError::Validation)?;
        let admitted = crate::domain_installation::admit_domain_package(validated)
            .map_err(WorthQueryDomainPackageInstallationError::Admission)?;
        let compiled = self
            .pending_domain_installations
            .install(admitted)
            .map_err(WorthQueryDomainPackageInstallationError::Installation)?;
        self.queued_invariant_registrations
            .custom_invariants
            .extend(compiled.custom_invariants);
        self.queued_graph_obligation_registrations
            .extend(compiled.graph_obligations);
        Ok(self)
    }
}
