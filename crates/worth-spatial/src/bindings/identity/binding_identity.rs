use crate::bindings::identity::identity_basis::SpatialBindingIdentityBasis;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialBindingIdentity(String);

impl SpatialBindingIdentity {
    pub(crate) fn from_basis(basis: SpatialBindingIdentityBasis) -> Self {
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
