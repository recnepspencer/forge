use forge_foundational::facade::CanonicalDerivedDigest;

use super::encoder::ForgeQueryEvidenceIdentityEncoder;
use super::scheme::ForgeQueryEvidenceIdentityScheme;
use super::scope::ForgeQueryEvidenceScope;
use super::sealed::SealedForgeQueryEvidenceIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEvidenceIdentityComparisonError {
    SchemeMismatch {
        left: ForgeQueryEvidenceIdentityScheme,
        right: ForgeQueryEvidenceIdentityScheme,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEvidenceIdentity {
    scope: ForgeQueryEvidenceScope,
    scheme: ForgeQueryEvidenceIdentityScheme,
    digest_token: String,
    canonical_digest: CanonicalDerivedDigest,
}

impl ForgeQueryEvidenceIdentity {
    pub fn compose(scope: ForgeQueryEvidenceScope) -> ForgeQueryEvidenceIdentityEncoder {
        ForgeQueryEvidenceIdentityEncoder::new(scope, ForgeQueryEvidenceIdentityScheme::V1)
    }

    #[cfg(test)]
    pub(crate) fn compose_with_scheme(
        scope: ForgeQueryEvidenceScope,
        scheme: ForgeQueryEvidenceIdentityScheme,
    ) -> ForgeQueryEvidenceIdentityEncoder {
        ForgeQueryEvidenceIdentityEncoder::new(scope, scheme)
    }

    pub(crate) fn new(sealed: SealedForgeQueryEvidenceIdentity) -> Self {
        Self {
            scope: sealed.scope,
            scheme: sealed.scheme,
            digest_token: sealed.digest_token,
            canonical_digest: sealed.canonical_digest,
        }
    }

    pub fn scope(&self) -> ForgeQueryEvidenceScope {
        self.scope
    }

    pub fn scheme(&self) -> ForgeQueryEvidenceIdentityScheme {
        self.scheme
    }

    pub fn as_str(&self) -> &str {
        &self.digest_token
    }

    pub fn eq_same_scheme(
        &self,
        other: &Self,
    ) -> Result<bool, ForgeQueryEvidenceIdentityComparisonError> {
        self.same_scheme_as(other)?;
        Ok(self.digest_token == other.digest_token)
    }

    pub fn same_scheme_as(
        &self,
        other: &Self,
    ) -> Result<(), ForgeQueryEvidenceIdentityComparisonError> {
        if self.scheme == other.scheme {
            Ok(())
        } else {
            Err(ForgeQueryEvidenceIdentityComparisonError::SchemeMismatch {
                left: self.scheme,
                right: other.scheme,
            })
        }
    }

    pub fn compare_same_scheme(
        &self,
        other: &Self,
    ) -> Result<std::cmp::Ordering, ForgeQueryEvidenceIdentityComparisonError> {
        self.same_scheme_as(other)?;
        Ok(self.digest_token.cmp(&other.digest_token))
    }

    #[cfg(test)]
    pub(crate) fn canonical_digest(&self) -> &CanonicalDerivedDigest {
        &self.canonical_digest
    }
}

impl std::fmt::Display for ForgeQueryEvidenceIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for ForgeQueryEvidenceIdentity {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
