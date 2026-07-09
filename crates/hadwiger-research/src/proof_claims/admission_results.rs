use worth_query::facade::WORTHQueryRecoveryBrief;

use crate::aspect_authority::HadwigerAspectPosture;
use crate::domain_artifacts::{HadwigerArtifactShapeError, ProofClaim};

use super::authority_chain::HadwigerProofAuthorityChain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerProofClaimBlockerKind {
    MissingEvidence,
    CheckerArtifactNotAdmitted,
    AspectNotAdmitted,
    ArtifactMismatch,
    QueryDeclaration,
    BackgroundTheoremNotSupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerProofClaimBlocker {
    blocker_kind: HadwigerProofClaimBlockerKind,
    reason: String,
    observed_posture: Option<HadwigerAspectPosture>,
    query_recovery_brief: Option<WORTHQueryRecoveryBrief>,
}

impl HadwigerProofClaimBlocker {
    pub(crate) fn local(
        blocker_kind: HadwigerProofClaimBlockerKind,
        reason: impl Into<String>,
        observed_posture: Option<HadwigerAspectPosture>,
    ) -> Self {
        Self {
            blocker_kind,
            reason: reason.into(),
            observed_posture,
            query_recovery_brief: None,
        }
    }

    pub fn blocker_kind(&self) -> HadwigerProofClaimBlockerKind {
        self.blocker_kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn observed_posture(&self) -> Option<HadwigerAspectPosture> {
        self.observed_posture
    }

    pub fn query_recovery_brief(&self) -> Option<&WORTHQueryRecoveryBrief> {
        self.query_recovery_brief.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerBlockedProofClaim {
    proof_claim: ProofClaim,
    authority_chain: HadwigerProofAuthorityChain,
    blockers: Vec<HadwigerProofClaimBlocker>,
}

impl HadwigerBlockedProofClaim {
    pub(crate) fn new(
        proof_claim: ProofClaim,
        authority_chain: HadwigerProofAuthorityChain,
        blockers: Vec<HadwigerProofClaimBlocker>,
    ) -> Self {
        Self {
            proof_claim,
            authority_chain,
            blockers,
        }
    }

    pub fn proof_claim(&self) -> &ProofClaim {
        &self.proof_claim
    }

    pub fn authority_chain(&self) -> &HadwigerProofAuthorityChain {
        &self.authority_chain
    }

    pub fn blockers(&self) -> &[HadwigerProofClaimBlocker] {
        &self.blockers
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerProofClaimAdmissionChecked {
    proof_claim: ProofClaim,
    authority_chain: HadwigerProofAuthorityChain,
}

impl HadwigerProofClaimAdmissionChecked {
    pub(crate) fn new(
        proof_claim: ProofClaim,
        authority_chain: HadwigerProofAuthorityChain,
    ) -> Self {
        Self {
            proof_claim,
            authority_chain,
        }
    }

    pub fn proof_claim(&self) -> &ProofClaim {
        &self.proof_claim
    }

    pub fn authority_chain(&self) -> &HadwigerProofAuthorityChain {
        &self.authority_chain
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerProofClaimAdmissionOutcome {
    Admitted(HadwigerProofClaimAdmissionChecked),
    Blocked(HadwigerBlockedProofClaim),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerProofClaimAdmissionError {
    Shape(HadwigerArtifactShapeError),
    DeclarationShape(crate::domain_declarations::HadwigerResearchDeclarationShapeError),
    QueryDeclarationNotAdmitted { declaration_kind: &'static str },
    Blocked(HadwigerBlockedProofClaim),
}

impl From<HadwigerArtifactShapeError> for HadwigerProofClaimAdmissionError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Shape(value)
    }
}

impl From<crate::domain_declarations::HadwigerResearchDeclarationShapeError>
    for HadwigerProofClaimAdmissionError
{
    fn from(value: crate::domain_declarations::HadwigerResearchDeclarationShapeError) -> Self {
        Self::DeclarationShape(value)
    }
}
