use super::qualification_basis::RootProfileBinding;

/// Copyable diagnostic description of the root/profile binding observed by a
/// completed qualification.
///
/// This value can constrain a later admission, but it is deliberately not an
/// authority witness and cannot construct capabilities or a media owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootProfileQualificationReport {
    binding: RootProfileBinding,
}

impl RootProfileQualificationReport {
    pub(super) fn new(binding: RootProfileBinding) -> Self {
        Self { binding }
    }

    pub(super) fn into_binding(self) -> RootProfileBinding {
        self.binding
    }

    pub const fn contract_version(&self) -> u16 {
        self.binding.contract_version
    }

    pub const fn root_identity(&self) -> [u8; 32] {
        self.binding.root_identity
    }

    pub const fn volume_identity(&self) -> [u8; 32] {
        self.binding.volume_identity
    }

    pub const fn backend_build_identity(&self) -> [u8; 32] {
        self.binding.backend_build_identity
    }

    pub const fn profile_digest(&self) -> [u8; 32] {
        self.binding.profile_digest
    }

    /// Reports the deployment assumption bound to this qualification. It is
    /// diagnostic policy context, not proof that arbitrary peers cooperate.
    pub const fn access_contract(&self) -> super::FilesystemAccessContract {
        self.binding.access_contract
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn with_contract_version_for_certification(mut self, version: u16) -> Self {
        self.binding.contract_version = version;
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn with_volume_identity_for_certification(mut self, identity: [u8; 32]) -> Self {
        self.binding.volume_identity = identity;
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn with_profile_digest_for_certification(mut self, digest: [u8; 32]) -> Self {
        self.binding.profile_digest = digest;
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn with_backend_build_identity_for_certification(mut self, identity: [u8; 32]) -> Self {
        self.binding.backend_build_identity = identity;
        self
    }
}
