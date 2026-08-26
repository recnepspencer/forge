use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryPackageArchiveChecksum([u8; 32]);

impl WorthQueryPackageArchiveChecksum {
    pub(crate) fn derive(archive: &[u8]) -> Self {
        Self(Sha256::digest(archive).into())
    }

    pub(crate) const fn from_untrusted_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub(crate) const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub(crate) fn matches(self, archive: &[u8]) -> bool {
        self == Self::derive(archive)
    }
}
