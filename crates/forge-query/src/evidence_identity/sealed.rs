use forge_foundational::facade::CanonicalDerivedDigest;

use super::scheme::ForgeQueryEvidenceIdentityScheme;
use super::scope::ForgeQueryEvidenceScope;

pub(crate) struct SealedForgeQueryEvidenceIdentity {
    pub(crate) scope: ForgeQueryEvidenceScope,
    pub(crate) scheme: ForgeQueryEvidenceIdentityScheme,
    pub(crate) digest_token: String,
    pub(crate) canonical_digest: CanonicalDerivedDigest,
}

impl SealedForgeQueryEvidenceIdentity {
    pub(crate) fn new(
        scope: ForgeQueryEvidenceScope,
        scheme: ForgeQueryEvidenceIdentityScheme,
        digest_token: String,
        canonical_digest: CanonicalDerivedDigest,
    ) -> Self {
        Self {
            scope,
            scheme,
            digest_token,
            canonical_digest,
        }
    }
}
