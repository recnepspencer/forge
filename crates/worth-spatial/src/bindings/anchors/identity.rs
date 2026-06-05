use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::anchors::identity_basis::SpatialAnchorIdentityBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialAnchorIdentity(String);

impl SpatialAnchorIdentity {
    pub(crate) fn from_basis(basis: SpatialAnchorIdentityBasis) -> Self {
        Self::from_digest_parts(&basis.digest_parts())
    }

    pub(crate) fn from_digest_parts(parts: &[String]) -> Self {
        Self(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            parts,
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
