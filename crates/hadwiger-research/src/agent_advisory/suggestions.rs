use crate::domain_artifacts::core_artifact::{
    require_non_empty, HadwigerArtifactReference, HadwigerArtifactShapeError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AgentAdvisoryKind {
    MotifSuggestion,
    InvariantHypothesisSuggestion,
    ExperimentProposal,
    RepairSuggestion,
    AdmissionCaution,
    AdmissionViolation,
}

impl AgentAdvisoryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MotifSuggestion => "motif_suggestion",
            Self::InvariantHypothesisSuggestion => "invariant_hypothesis_suggestion",
            Self::ExperimentProposal => "experiment_proposal",
            Self::RepairSuggestion => "repair_suggestion",
            Self::AdmissionCaution => "admission_caution",
            Self::AdmissionViolation => "admission_violation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AgentPromotionPathDescriptor {
    NoDirectPromotion,
    ExactGeometryVerification,
    SatColorabilityVerification,
    ProofClaimAdmission,
    QueryInvariantRegistration,
}

impl AgentPromotionPathDescriptor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoDirectPromotion => "no_direct_promotion",
            Self::ExactGeometryVerification => "exact_geometry_verification",
            Self::SatColorabilityVerification => "sat_colorability_verification",
            Self::ProofClaimAdmission => "proof_claim_admission",
            Self::QueryInvariantRegistration => "query_invariant_registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentMotifSuggestion {
    suggestion_id: String,
    cited_evidence: HadwigerArtifactReference,
    observation: String,
    promotion_path: AgentPromotionPathDescriptor,
}

impl AgentMotifSuggestion {
    pub fn new(
        suggestion_id: impl Into<String>,
        cited_evidence: HadwigerArtifactReference,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            cited_evidence,
            observation: String::new(),
            promotion_path: AgentPromotionPathDescriptor::NoDirectPromotion,
        }
    }

    pub fn with_observation(
        mut self,
        observation: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.suggestion_id = require_non_empty(self.suggestion_id, "suggestion_id")?;
        self.observation = require_non_empty(observation, "observation")?;
        Ok(self)
    }

    pub fn with_promotion_path(mut self, value: AgentPromotionPathDescriptor) -> Self {
        self.promotion_path = value;
        self
    }

    pub(crate) fn cited_evidence(&self) -> &HadwigerArtifactReference {
        &self.cited_evidence
    }

    pub(crate) fn into_advisory_parts(
        self,
    ) -> (
        String,
        AgentAdvisoryKind,
        Vec<HadwigerArtifactReference>,
        String,
        AgentPromotionPathDescriptor,
    ) {
        (
            self.suggestion_id,
            AgentAdvisoryKind::MotifSuggestion,
            vec![self.cited_evidence],
            self.observation,
            self.promotion_path,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentInvariantHypothesisSuggestion {
    suggestion_id: String,
    cited_evidence: HadwigerArtifactReference,
    hypothesis: String,
    promotion_path: AgentPromotionPathDescriptor,
}

impl AgentInvariantHypothesisSuggestion {
    pub fn new(
        suggestion_id: impl Into<String>,
        cited_evidence: HadwigerArtifactReference,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            cited_evidence,
            hypothesis: String::new(),
            promotion_path: AgentPromotionPathDescriptor::NoDirectPromotion,
        }
    }

    pub fn with_hypothesis(
        mut self,
        hypothesis: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.suggestion_id = require_non_empty(self.suggestion_id, "suggestion_id")?;
        self.hypothesis = require_non_empty(hypothesis, "hypothesis")?;
        Ok(self)
    }

    pub fn with_promotion_path(mut self, value: AgentPromotionPathDescriptor) -> Self {
        self.promotion_path = value;
        self
    }

    pub(crate) fn cited_evidence(&self) -> &HadwigerArtifactReference {
        &self.cited_evidence
    }

    pub(crate) fn into_advisory_parts(
        self,
    ) -> (
        String,
        AgentAdvisoryKind,
        Vec<HadwigerArtifactReference>,
        String,
        AgentPromotionPathDescriptor,
    ) {
        (
            self.suggestion_id,
            AgentAdvisoryKind::InvariantHypothesisSuggestion,
            vec![self.cited_evidence],
            self.hypothesis,
            self.promotion_path,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentExperimentProposal {
    proposal_id: String,
    target_artifact: HadwigerArtifactReference,
    rationale: String,
    promotion_path: AgentPromotionPathDescriptor,
}

impl AgentExperimentProposal {
    pub fn new(proposal_id: impl Into<String>, target_artifact: HadwigerArtifactReference) -> Self {
        Self {
            proposal_id: proposal_id.into(),
            target_artifact,
            rationale: String::new(),
            promotion_path: AgentPromotionPathDescriptor::NoDirectPromotion,
        }
    }

    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.proposal_id = require_non_empty(self.proposal_id, "proposal_id")?;
        self.rationale = require_non_empty(rationale, "rationale")?;
        Ok(self)
    }

    pub fn with_promotion_path(mut self, value: AgentPromotionPathDescriptor) -> Self {
        self.promotion_path = value;
        self
    }

    pub fn proposal_id(&self) -> &str {
        &self.proposal_id
    }

    pub(crate) fn target_artifact(&self) -> &HadwigerArtifactReference {
        &self.target_artifact
    }

    pub(crate) fn into_advisory_parts(
        self,
    ) -> (
        String,
        AgentAdvisoryKind,
        Vec<HadwigerArtifactReference>,
        String,
        AgentPromotionPathDescriptor,
    ) {
        (
            self.proposal_id,
            AgentAdvisoryKind::ExperimentProposal,
            vec![self.target_artifact],
            self.rationale,
            self.promotion_path,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentRepairSuggestion {
    suggestion_id: String,
    cited_evidence: HadwigerArtifactReference,
    repair_note: String,
}

impl AgentRepairSuggestion {
    pub fn new(
        suggestion_id: impl Into<String>,
        cited_evidence: HadwigerArtifactReference,
    ) -> Self {
        Self {
            suggestion_id: suggestion_id.into(),
            cited_evidence,
            repair_note: String::new(),
        }
    }

    pub fn with_repair_note(
        mut self,
        repair_note: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.suggestion_id = require_non_empty(self.suggestion_id, "suggestion_id")?;
        self.repair_note = require_non_empty(repair_note, "repair_note")?;
        Ok(self)
    }

    pub(crate) fn cited_evidence(&self) -> &HadwigerArtifactReference {
        &self.cited_evidence
    }

    pub(crate) fn into_advisory_parts(
        self,
    ) -> (
        String,
        AgentAdvisoryKind,
        Vec<HadwigerArtifactReference>,
        String,
        AgentPromotionPathDescriptor,
    ) {
        (
            self.suggestion_id,
            AgentAdvisoryKind::RepairSuggestion,
            vec![self.cited_evidence],
            self.repair_note,
            AgentPromotionPathDescriptor::NoDirectPromotion,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentAdmissionAdvisory {
    candidate_id: String,
    detail: String,
    kind: AgentAdvisoryKind,
}

impl AgentAdmissionAdvisory {
    pub fn caution(
        candidate_id: impl Into<String>,
        detail: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            candidate_id: require_non_empty(candidate_id, "candidate_id")?,
            detail: require_non_empty(detail, "detail")?,
            kind: AgentAdvisoryKind::AdmissionCaution,
        })
    }

    pub fn violation(
        candidate_id: impl Into<String>,
        detail: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            candidate_id: require_non_empty(candidate_id, "candidate_id")?,
            detail: require_non_empty(detail, "detail")?,
            kind: AgentAdvisoryKind::AdmissionViolation,
        })
    }

    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn kind(&self) -> AgentAdvisoryKind {
        self.kind
    }
}
