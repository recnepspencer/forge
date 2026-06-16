use forge_foundational::facade::CanonicalDerivedDigest;
use forge_runtime_bridge::facade::{
    bridge_truth_digest_identity_evidence_from_external_token,
    bridge_truth_external_identity_token, bridge_truth_projection_identity_from_external_token,
    BridgeIdentityEvidence,
};
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
    pub(crate) fn compose(scope: ForgeQueryEvidenceScope) -> ForgeQueryEvidenceIdentityEncoder {
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

    pub(crate) fn as_str(&self) -> &str {
        &self.digest_token
    }

    pub(crate) fn reporting_projection(&self) -> &str {
        self.as_str()
    }

    pub(crate) fn terminal_projection_for_reporting(&self) -> &str {
        self.reporting_projection()
    }

    pub(crate) fn bridge_evidence_identity(&self) -> BridgeIdentityEvidence {
        let token = bridge_truth_external_identity_token(self.as_str());
        let scope = bridge_truth_projection_identity_from_external_token(
            token.clone(),
            self.scope.as_str(),
        );
        let identity_token = bridge_truth_digest_identity_evidence_from_external_token(
            token,
            self.canonical_digest.clone(),
        );
        BridgeIdentityEvidence::from_query_evidence_identity(scope, identity_token)
    }

    pub(crate) fn bridge_external_identity_evidence(&self) -> BridgeIdentityEvidence {
        BridgeIdentityEvidence::from_external_authority(bridge_truth_external_identity_token(
            self.as_str(),
        ))
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
