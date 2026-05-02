#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthAssertionEvidence {
    mode: crate::runtime::ForgeQueryExistingTruthAssertionMode,
    asserted_aspect_count: usize,
    verification_digest: String,
    verified_assumption_set: Option<crate::runtime::ForgeQueryVerifiedAssumptionSet>,
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
            verified_assumption_set: None,
        }
    }

    pub(in crate::runtime) fn backend_verified(
        verification: &crate::runtime::ForgeQueryVerifiedExistingTruthAssertion,
    ) -> Self {
        Self {
            mode: crate::runtime::ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
            asserted_aspect_count: verification.asserted_aspect_count(),
            verification_digest: verification.verification_digest().to_string(),
            verified_assumption_set: Some(verification.verified_assumption_set().clone()),
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

    pub fn verified_assumption_set(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryVerifiedAssumptionSet> {
        self.verified_assumption_set.as_ref()
    }

    pub fn assumption_snapshot_digest(&self) -> Option<&str> {
        self.verified_assumption_set
            .as_ref()
            .map(crate::runtime::ForgeQueryVerifiedAssumptionSet::assumption_snapshot_digest)
    }

    pub fn verified_precondition_digest(&self) -> Option<&str> {
        self.verified_assumption_set
            .as_ref()
            .map(crate::runtime::ForgeQueryVerifiedAssumptionSet::verified_precondition_digest)
    }

    pub fn verification_read_set_breadth(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryVerificationReadSetBreadth> {
        self.verified_assumption_set
            .as_ref()
            .map(crate::runtime::ForgeQueryVerifiedAssumptionSet::verification_read_set_breadth)
    }
}
