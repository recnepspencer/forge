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
    pub(crate) admission_identity: String,
    pub(crate) package_authority_nonce: [u8; 32],
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

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }
}
