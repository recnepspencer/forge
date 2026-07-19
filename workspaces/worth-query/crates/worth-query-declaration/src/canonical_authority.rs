use crate::identity::CanonicalQueryDigest;

/// Typed carrier for canonical Query identity.
///
/// Canonicalization is the only minting path. The compatibility edge may carry
/// this proof downstream, but a digest or reporting artifact cannot reconstruct
/// it. This type is not installation or execution authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCanonicalAuthority {
    digest: CanonicalQueryDigest,
}

impl QueryCanonicalAuthority {
    pub fn digest(&self) -> &CanonicalQueryDigest {
        &self.digest
    }

    pub(crate) fn mint(digest: CanonicalQueryDigest) -> Self {
        Self { digest }
    }
}
