#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthAssertionEvidence {
    mode: crate::runtime::ForgeQueryExistingTruthAssertionMode,
    asserted_aspect_count: usize,
    verification_digest: String,
}

impl ForgeQueryExistingTruthAssertionEvidence {
    pub(in crate::runtime) fn retained_assertion(
        asserted_aspect_count: usize,
        verification_digest: impl Into<String>,
    ) -> Self {
        Self {
            mode:
                crate::runtime::ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion,
            asserted_aspect_count,
            verification_digest: verification_digest.into(),
        }
    }

    pub(in crate::runtime) fn backend_verified(
        verification: &crate::runtime::ForgeQueryVerifiedExistingTruthAssertion,
    ) -> Self {
        Self {
            mode: crate::runtime::ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
            asserted_aspect_count: verification.asserted_aspect_count(),
            verification_digest: verification.verification_digest().to_string(),
        }
    }

    pub fn mode(&self) -> crate::runtime::ForgeQueryExistingTruthAssertionMode {
        self.mode
    }

    pub fn asserted_aspect_count(&self) -> usize {
        self.asserted_aspect_count
    }

    pub fn verification_digest(&self) -> &str {
        &self.verification_digest
    }
}
