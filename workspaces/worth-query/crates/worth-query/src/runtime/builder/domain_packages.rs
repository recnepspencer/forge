use crate::application::WorthQueryDomainEntryMarker;
use crate::domain_installation::{
    WorthQueryDomainPackage, WorthQueryDomainPackageInstallationError,
};

use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn domain_package<D: WorthQueryDomainEntryMarker + 'static>(
        self,
        package: WorthQueryDomainPackage<D>,
    ) -> Result<Self, WorthQueryDomainPackageInstallationError> {
        self.domain_package_with_artifact_support(
            package,
            crate::domain_installation::WorthQueryArtifactInstallationSupport::default(),
        )
    }

    pub fn domain_package_with_artifact_support<D: WorthQueryDomainEntryMarker + 'static>(
        mut self,
        package: WorthQueryDomainPackage<D>,
        artifact_support: crate::domain_installation::WorthQueryArtifactInstallationSupport,
    ) -> Result<Self, WorthQueryDomainPackageInstallationError> {
        let validated = package
            .validate()
            .map_err(WorthQueryDomainPackageInstallationError::Validation)?;
        let admitted = crate::domain_installation::admit_domain_package_with_artifact_support(
            validated,
            &artifact_support,
        )
        .map_err(WorthQueryDomainPackageInstallationError::Admission)?;
        self.pending_domain_installations
            .install(admitted)
            .map_err(WorthQueryDomainPackageInstallationError::Installation)?;
        Ok(self)
    }

    pub(super) fn queue_installed_domain_substrates(&mut self) {
        let compiled = self.pending_domain_installations.take_compiled_substrates();
        self.queued_invariant_registrations
            .custom_invariants
            .extend(compiled.custom_invariants);
    }

    pub(crate) fn with_precompiled_domain_installations(
        mut self,
        installations: crate::domain_installation::WorthQueryPendingDomainInstallations,
    ) -> Self {
        debug_assert!(installations.compiled_substrates_are_empty());
        self.pending_domain_installations = installations;
        self
    }
}
