use super::{
    WorthQueryInstallationAdmissionDenial, WorthQueryInstallationAdmissionDenialKind,
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationSupportStatus,
};
use crate::package::WorthQueryValidatedPortableDomainPackage;

impl WorthQueryInstallationAdmissionProfile {
    pub(super) fn validate_profile_identity_and_conflicts(
        &self,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        if self.support_identity.trim().is_empty() {
            return Err(denial(
                WorthQueryInstallationAdmissionDenialKind::InvalidSupportProfileIdentity,
                &self.support_identity,
            ));
        }
        if self.configuration_identity.trim().is_empty() {
            return Err(denial(
                WorthQueryInstallationAdmissionDenialKind::InvalidConfigurationProfileIdentity,
                &self.configuration_identity,
            ));
        }
        if let Some(conflict) = self
            .conflicting_rows
            .iter()
            .find(|conflict| conflict.starts_with("artifact-version:"))
        {
            return Err(denial(
                WorthQueryInstallationAdmissionDenialKind::AmbiguousArtifactMigration,
                conflict,
            ));
        }
        if let Some(conflict) = self.conflicting_rows.first() {
            return Err(denial(
                WorthQueryInstallationAdmissionDenialKind::ConflictingProfileRow,
                conflict,
            ));
        }
        Ok(())
    }

    pub(super) fn admit_declared_requirements(
        &self,
        package: &WorthQueryValidatedPortableDomainPackage,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        self.admit_capabilities(package)?;
        self.admit_configuration(package)?;
        self.admit_operating_requirements(package)
    }

    fn admit_capabilities(
        &self,
        package: &WorthQueryValidatedPortableDomainPackage,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        for capability in package.capabilities() {
            match self
                .capability_statuses
                .get(capability.as_str())
                .copied()
                .unwrap_or(WorthQueryInstallationSupportStatus::Unsupported)
            {
                WorthQueryInstallationSupportStatus::Admitted => {}
                WorthQueryInstallationSupportStatus::Deferred => {
                    return Err(denial(
                        WorthQueryInstallationAdmissionDenialKind::DeferredCapability,
                        capability.as_str(),
                    ));
                }
                WorthQueryInstallationSupportStatus::Unsupported => {
                    return Err(denial(
                        WorthQueryInstallationAdmissionDenialKind::UnsupportedCapability,
                        capability.as_str(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn admit_configuration(
        &self,
        package: &WorthQueryValidatedPortableDomainPackage,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        for section in package.configuration() {
            if !self
                .configuration_statuses
                .get(section.as_str())
                .copied()
                .unwrap_or(false)
            {
                return Err(denial(
                    WorthQueryInstallationAdmissionDenialKind::DisabledConfiguration,
                    section.as_str(),
                ));
            }
        }
        Ok(())
    }

    fn admit_operating_requirements(
        &self,
        package: &WorthQueryValidatedPortableDomainPackage,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        for requirement in package.operating_requirements() {
            match self
                .operating_statuses
                .get(requirement.as_str())
                .copied()
                .unwrap_or(WorthQueryInstallationSupportStatus::Unsupported)
            {
                WorthQueryInstallationSupportStatus::Admitted => {}
                WorthQueryInstallationSupportStatus::Deferred => {
                    return Err(denial(
                        WorthQueryInstallationAdmissionDenialKind::DeferredOperatingRequirement,
                        requirement.as_str(),
                    ));
                }
                WorthQueryInstallationSupportStatus::Unsupported => {
                    return Err(denial(
                        WorthQueryInstallationAdmissionDenialKind::UnsupportedOperatingRequirement,
                        requirement.as_str(),
                    ));
                }
            }
        }
        Ok(())
    }
}

fn denial(
    kind: WorthQueryInstallationAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryInstallationAdmissionDenial {
    WorthQueryInstallationAdmissionDenial {
        kind,
        subject: subject.into(),
    }
}
