use crate::domain_installation::{
    WorthQueryAdmittedDomainPackage, WorthQueryDomainInstallationDenial,
};

use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn domain_package<D: 'static>(
        mut self,
        package: WorthQueryAdmittedDomainPackage<D>,
    ) -> Result<Self, WorthQueryDomainInstallationDenial> {
        let compiled = self.pending_domain_installations.install(package)?;
        self.queued_invariant_registrations
            .custom_invariants
            .extend(compiled.custom_invariants);
        self.queued_graph_obligation_registrations
            .extend(compiled.graph_obligations);
        Ok(self)
    }
}
