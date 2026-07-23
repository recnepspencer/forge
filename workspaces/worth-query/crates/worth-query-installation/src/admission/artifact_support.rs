use super::{
    retain_profile_row, WorthQueryInstallationAdmissionDenial,
    WorthQueryInstallationAdmissionDenialKind, WorthQueryInstallationAdmissionProfile,
    WorthQueryInstallationSupportStatus,
};
use crate::package::WorthQueryValidatedPortableDomainPackage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactVersionSupport {
    Admitted,
    Unsupported,
    Retired,
    MigrationRequired {
        target_schema_version: u32,
        migration_owner: String,
    },
}

impl WorthQueryArtifactVersionSupport {
    pub(super) fn canonical_part(&self) -> String {
        match self {
            Self::Admitted => "admitted".into(),
            Self::Unsupported => "unsupported".into(),
            Self::Retired => "retired".into(),
            Self::MigrationRequired {
                target_schema_version,
                migration_owner,
            } => format!("migration-required:{target_schema_version}:{migration_owner}"),
        }
    }
}

impl WorthQueryInstallationAdmissionProfile {
    pub fn artifact_version<F: crate::domain_computation::WorthQueryArtifactFamily>(
        mut self,
        schema_version: crate::domain_computation::WorthQueryArtifactSchemaVersion,
        protocol_version: crate::domain_computation::WorthQueryArtifactProtocolVersion,
        status: WorthQueryArtifactVersionSupport,
    ) -> Self {
        let key = (
            F::SEMANTIC_FAMILY.to_string(),
            schema_version.get(),
            protocol_version.get(),
        );
        retain_profile_row(
            &mut self.artifact_version_statuses,
            &mut self.conflicting_rows,
            "artifact-version",
            key,
            status,
        );
        self
    }

    pub fn artifact_comparator<F: crate::domain_computation::WorthQueryArtifactComparatorFamily>(
        mut self,
        status: WorthQueryInstallationSupportStatus,
    ) -> Self {
        retain_profile_row(
            &mut self.artifact_comparator_statuses,
            &mut self.conflicting_rows,
            "artifact-comparator",
            F::SEMANTIC_FAMILY.to_string(),
            status,
        );
        self
    }

    pub(super) fn admit_artifact_contracts(
        &self,
        package: &WorthQueryValidatedPortableDomainPackage,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        for contract in package.artifact_contracts() {
            self.admit_artifact_version(contract)?;
            self.admit_artifact_comparator(contract)?;
        }
        Ok(())
    }

    fn admit_artifact_version(
        &self,
        contract: &crate::domain_computation::WorthQueryPortableArtifactContract,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        let key = (
            contract.family().as_str().to_string(),
            contract.schema_version().get(),
            contract.protocol_version().get(),
        );
        let status = self
            .artifact_version_statuses
            .get(&key)
            .cloned()
            .unwrap_or(WorthQueryArtifactVersionSupport::Unsupported);
        let kind = match status {
            WorthQueryArtifactVersionSupport::Admitted => return Ok(()),
            WorthQueryArtifactVersionSupport::Unsupported => {
                WorthQueryInstallationAdmissionDenialKind::UnsupportedArtifactVersion
            }
            WorthQueryArtifactVersionSupport::Retired => {
                WorthQueryInstallationAdmissionDenialKind::RetiredArtifactVersion
            }
            WorthQueryArtifactVersionSupport::MigrationRequired { .. } => {
                WorthQueryInstallationAdmissionDenialKind::ArtifactMigrationRequired
            }
        };
        Err(WorthQueryInstallationAdmissionDenial {
            kind,
            subject: format!("{}:{}:{}", key.0, key.1, key.2),
        })
    }

    fn admit_artifact_comparator(
        &self,
        contract: &crate::domain_computation::WorthQueryPortableArtifactContract,
    ) -> Result<(), WorthQueryInstallationAdmissionDenial> {
        let Some(comparator) = contract.reproducibility().comparison().registered_family() else {
            return Ok(());
        };
        let kind = match self
            .artifact_comparator_statuses
            .get(comparator)
            .copied()
            .unwrap_or(WorthQueryInstallationSupportStatus::Unsupported)
        {
            WorthQueryInstallationSupportStatus::Admitted => return Ok(()),
            WorthQueryInstallationSupportStatus::Deferred => {
                WorthQueryInstallationAdmissionDenialKind::DeferredArtifactComparator
            }
            WorthQueryInstallationSupportStatus::Unsupported => {
                WorthQueryInstallationAdmissionDenialKind::UnsupportedArtifactComparator
            }
        };
        Err(WorthQueryInstallationAdmissionDenial {
            kind,
            subject: comparator.to_string(),
        })
    }
}
