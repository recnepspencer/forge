use worth_foundational::facade::CanonicalDigestId;

use super::WorthQueryApplicationCanonicalArtifact;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryInstalledApplicationQueryIdentity(CanonicalDigestId);

impl WorthQueryInstalledApplicationQueryIdentity {
    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.0
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        self.0.bytes()
    }

    pub fn render_support_hex(&self) -> String {
        self.0.render_hex()
    }

    pub(super) fn from_canonical(artifact: &WorthQueryApplicationCanonicalArtifact) -> Self {
        Self(*artifact.digest())
    }
}
