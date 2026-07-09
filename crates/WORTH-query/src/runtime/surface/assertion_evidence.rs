use crate::evidence_identity::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExistingTruthAssertionEvidence {
    mode: crate::runtime::WorthQueryExistingTruthAssertionMode,
    asserted_aspect_count: usize,
    verification_digest: WorthQueryEvidenceIdentity,
    verified_assumption_set: Option<crate::runtime::WorthQueryVerifiedAssumptionSet>,
}

impl WorthQueryExistingTruthAssertionEvidence {
    pub(in crate::runtime) fn retained_assertion(
        asserted_aspect_count: usize,
        verification_digest: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            mode:
                crate::runtime::WorthQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion,
            asserted_aspect_count,
            verification_digest,
            verified_assumption_set: None,
        }
    }

    pub(in crate::runtime) fn backend_verified(
        verification: &crate::runtime::WorthQueryVerifiedExistingTruthAssertion,
    ) -> Self {
        Self {
            mode: crate::runtime::WorthQueryExistingTruthAssertionMode::BackendVerifiedAssertion,
            asserted_aspect_count: verification.asserted_aspect_count(),
            verification_digest: verification.verification_evidence_identity().clone(),
            verified_assumption_set: Some(verification.verified_assumption_set().clone()),
        }
    }

    pub fn mode(&self) -> crate::runtime::WorthQueryExistingTruthAssertionMode {
        self.mode
    }

    pub fn asserted_aspect_count(&self) -> usize {
        self.asserted_aspect_count
    }

    pub fn verification_digest(&self) -> &str {
        self.verification_digest.as_str()
    }

    pub fn verification_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.verification_digest
    }

    pub fn verified_assumption_set(
        &self,
    ) -> Option<&crate::runtime::WorthQueryVerifiedAssumptionSet> {
        self.verified_assumption_set.as_ref()
    }

    pub fn assumption_snapshot_digest(&self) -> Option<&str> {
        self.verified_assumption_set
            .as_ref()
            .map(crate::runtime::WorthQueryVerifiedAssumptionSet::assumption_snapshot_digest)
    }

    pub fn assumption_snapshot_evidence_digest(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.verified_assumption_set.as_ref().map(
            crate::runtime::WorthQueryVerifiedAssumptionSet::assumption_snapshot_evidence_digest,
        )
    }

    pub fn verified_precondition_digest(&self) -> Option<&str> {
        self.verified_assumption_set
            .as_ref()
            .map(crate::runtime::WorthQueryVerifiedAssumptionSet::verified_precondition_digest)
    }

    pub fn verified_precondition_evidence_digest(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.verified_assumption_set.as_ref().map(
            crate::runtime::WorthQueryVerifiedAssumptionSet::verified_precondition_evidence_digest,
        )
    }

    pub fn verification_read_set_breadth(
        &self,
    ) -> Option<&crate::runtime::WorthQueryVerificationReadSetBreadth> {
        self.verified_assumption_set
            .as_ref()
            .map(crate::runtime::WorthQueryVerifiedAssumptionSet::verification_read_set_breadth)
    }
}
