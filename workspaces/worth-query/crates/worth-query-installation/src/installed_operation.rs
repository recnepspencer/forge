use crate::admission::WorthQueryInstallationAdmissionIdentity;
use crate::authority_cryptography::PackageAuthorityKey;
use crate::generation::WorthQueryInstallationGeneration;
use crate::package::WorthQueryPortableDomainPackageIdentity;

/// Opaque proof that an exact portable operation belongs to one installed
/// package, runtime, and generation.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledOperationAuthority {
    pub(crate) runtime_ordinal: u64,
    pub(crate) generation: WorthQueryInstallationGeneration,
    pub(crate) owner: String,
    pub(crate) package_identity: WorthQueryPortableDomainPackageIdentity,
    pub(crate) admission_identity: WorthQueryInstallationAdmissionIdentity,
    pub(crate) package_authority_key: PackageAuthorityKey,
    pub(crate) operation_slot: String,
    pub(crate) operation_semantics: String,
}

impl WorthQueryInstalledOperationAuthority {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn operation_slot(&self) -> &str {
        &self.operation_slot
    }

    pub fn package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.package_identity
    }

    pub fn admission_identity(&self) -> &WorthQueryInstallationAdmissionIdentity {
        &self.admission_identity
    }
}
