use crate::generation::WorthQueryInstallationGeneration;
use crate::package::WorthQueryPortableDomainPackageIdentity;

/// Opaque proof that one exact portable package belongs to an installed
/// runtime generation.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledPackageAuthority {
    pub(crate) runtime_ordinal: u64,
    pub(crate) generation: WorthQueryInstallationGeneration,
    pub(crate) owner: String,
    pub(crate) package_identity: WorthQueryPortableDomainPackageIdentity,
    pub(crate) admission_identity: String,
    pub(crate) authority_nonce: [u8; 32],
}

impl WorthQueryInstalledPackageAuthority {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.package_identity
    }

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }
}
