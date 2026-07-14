use worth_query::facade::foundation::WorthQueryRecoveryBrief;

use crate::aspect_authority::UnitDistanceAspectRecord;
use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactReference, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
    UnitDistanceVerification,
};
use crate::proof_claims::HadwigerBlockedProofClaim;

use super::HadwigerRepairObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExplainRejectionRequest {
    explanation_id: String,
    graph_version: GraphVersion,
    checker_verification: UnitDistanceVerification,
    rejected_aspect: Option<UnitDistanceAspectRecord>,
    repair_obligations: Vec<HadwigerRepairObligation>,
}

impl ExplainRejectionRequest {
    pub fn for_checker_rejection(
        explanation_id: impl Into<String>,
        graph_version: &GraphVersion,
        checker_verification: &UnitDistanceVerification,
    ) -> Self {
        Self {
            explanation_id: explanation_id.into(),
            graph_version: graph_version.clone(),
            checker_verification: checker_verification.clone(),
            rejected_aspect: None,
            repair_obligations: Vec::new(),
        }
    }

    pub fn with_rejected_aspect(mut self, aspect: &UnitDistanceAspectRecord) -> Self {
        self.rejected_aspect = Some(aspect.clone());
        self
    }

    pub fn with_repair_obligation(
        mut self,
        detail: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.repair_obligations
            .push(HadwigerRepairObligation::new(detail)?);
        Ok(self)
    }

    pub(crate) fn explanation_id(&self) -> &str {
        &self.explanation_id
    }

    pub(crate) fn graph_version(&self) -> &GraphVersion {
        &self.graph_version
    }

    pub(crate) fn checker_verification(&self) -> &UnitDistanceVerification {
        &self.checker_verification
    }

    pub(crate) fn rejected_aspect(&self) -> Option<&UnitDistanceAspectRecord> {
        self.rejected_aspect.as_ref()
    }

    pub(crate) fn repair_obligations(&self) -> &[HadwigerRepairObligation] {
        &self.repair_obligations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExplainPartialAdmissionRequest {
    explanation_id: String,
    graph_version: GraphVersion,
    blocked_claim: HadwigerBlockedProofClaim,
    surviving_artifacts: Vec<HadwigerArtifactReference>,
    repair_obligations: Vec<HadwigerRepairObligation>,
}

impl ExplainPartialAdmissionRequest {
    pub fn from_blocked_proof_claim(
        explanation_id: impl Into<String>,
        graph_version: &GraphVersion,
        blocked_claim: &HadwigerBlockedProofClaim,
    ) -> Self {
        Self {
            explanation_id: explanation_id.into(),
            graph_version: graph_version.clone(),
            blocked_claim: blocked_claim.clone(),
            surviving_artifacts: Vec::new(),
            repair_obligations: Vec::new(),
        }
    }

    pub fn with_surviving_artifact(mut self, reference: HadwigerArtifactReference) -> Self {
        self.surviving_artifacts.push(reference);
        self
    }

    pub fn with_repair_obligation(
        mut self,
        detail: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.repair_obligations
            .push(HadwigerRepairObligation::new(detail)?);
        Ok(self)
    }

    pub(crate) fn explanation_id(&self) -> &str {
        &self.explanation_id
    }

    pub(crate) fn graph_version(&self) -> &GraphVersion {
        &self.graph_version
    }

    pub(crate) fn blocked_claim(&self) -> &HadwigerBlockedProofClaim {
        &self.blocked_claim
    }

    pub(crate) fn surviving_artifacts(&self) -> &[HadwigerArtifactReference] {
        &self.surviving_artifacts
    }

    pub(crate) fn repair_obligations(&self) -> &[HadwigerRepairObligation] {
        &self.repair_obligations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerQueryRecoveryExplanationRequest {
    explanation_id: String,
    recovery_brief: WorthQueryRecoveryBrief,
}

impl HadwigerQueryRecoveryExplanationRequest {
    pub fn new(explanation_id: impl Into<String>, recovery_brief: WorthQueryRecoveryBrief) -> Self {
        Self {
            explanation_id: explanation_id.into(),
            recovery_brief,
        }
    }

    pub(crate) fn explanation_id(&self) -> &str {
        &self.explanation_id
    }

    pub(crate) fn recovery_brief(&self) -> &WorthQueryRecoveryBrief {
        &self.recovery_brief
    }
}

pub(crate) fn graph_reference(graph_version: &GraphVersion) -> HadwigerArtifactReference {
    graph_version.reference()
}
