use super::contracts::StructuralCandidateBudget;
use super::cost::{StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract};
#[cfg(test)]
use super::error::CorrespondenceEvaluationError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CorrespondenceFamilyIntent {
    LineageOnly,
    StructuralOnly,
    MixedEvidenceAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LineageEvidenceInput {
    AuthoritativeContinuity {
        canonical_subject: String,
        authoritative_counterpart: String,
    },
    #[cfg(test)]
    UnsupportedTopology { topology: &'static str },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum StructuralEvidenceInput {
    CandidateSet {
        candidates: Vec<String>,
        ordering_contract: StructuralCandidateOrderingContract,
    },
    #[cfg(test)]
    UnsupportedFamily { family: &'static str },
    #[cfg(test)]
    LineageConflict { structural_counterpart: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceEvaluationRequest {
    lineage_evidence: Option<LineageEvidenceInput>,
    structural_evidence: Option<StructuralEvidenceInput>,
    discovery_plan: StructuralCandidateDiscoveryPlan,
    budget: StructuralCandidateBudget,
    intent: CorrespondenceFamilyIntent,
}

impl CorrespondenceEvaluationRequest {
    pub fn lineage_only(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
    ) -> Self {
        Self {
            lineage_evidence: Some(LineageEvidenceInput::AuthoritativeContinuity {
                canonical_subject: canonical_subject.into(),
                authoritative_counterpart: authoritative_counterpart.into(),
            }),
            structural_evidence: None,
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            intent: CorrespondenceFamilyIntent::LineageOnly,
        }
    }

    pub fn structural_only(
        candidates: Vec<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        ordering_contract: StructuralCandidateOrderingContract,
    ) -> Self {
        Self {
            lineage_evidence: None,
            structural_evidence: Some(StructuralEvidenceInput::CandidateSet {
                candidates,
                ordering_contract,
            }),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            intent: CorrespondenceFamilyIntent::StructuralOnly,
        }
    }

    pub fn mixed(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        candidates: Vec<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
        ordering_contract: StructuralCandidateOrderingContract,
    ) -> Self {
        Self {
            lineage_evidence: Some(LineageEvidenceInput::AuthoritativeContinuity {
                canonical_subject: canonical_subject.into(),
                authoritative_counterpart: authoritative_counterpart.into(),
            }),
            structural_evidence: Some(StructuralEvidenceInput::CandidateSet {
                candidates,
                ordering_contract,
            }),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            intent: CorrespondenceFamilyIntent::MixedEvidenceAdmitted,
        }
    }

    #[cfg(test)]
    pub(crate) fn unsupported_structural_family(
        family: &'static str,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
    ) -> Self {
        Self {
            lineage_evidence: None,
            structural_evidence: Some(StructuralEvidenceInput::UnsupportedFamily { family }),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            intent: CorrespondenceFamilyIntent::StructuralOnly,
        }
    }

    #[cfg(test)]
    pub(crate) fn unsupported_lineage_topology(
        topology: &'static str,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
    ) -> Self {
        Self {
            lineage_evidence: Some(LineageEvidenceInput::UnsupportedTopology { topology }),
            structural_evidence: None,
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            intent: CorrespondenceFamilyIntent::LineageOnly,
        }
    }

    #[cfg(test)]
    pub(crate) fn mixed_lineage_conflict(
        canonical_subject: impl Into<String>,
        authoritative_counterpart: impl Into<String>,
        structural_counterpart: impl Into<String>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: usize,
    ) -> Self {
        Self {
            lineage_evidence: Some(LineageEvidenceInput::AuthoritativeContinuity {
                canonical_subject: canonical_subject.into(),
                authoritative_counterpart: authoritative_counterpart.into(),
            }),
            structural_evidence: Some(StructuralEvidenceInput::LineageConflict {
                structural_counterpart: structural_counterpart.into(),
            }),
            discovery_plan,
            budget: StructuralCandidateBudget::bounded(budget),
            intent: CorrespondenceFamilyIntent::MixedEvidenceAdmitted,
        }
    }

    pub fn discovery_plan(&self) -> &StructuralCandidateDiscoveryPlan {
        &self.discovery_plan
    }

    pub fn budget(&self) -> &StructuralCandidateBudget {
        &self.budget
    }

    pub(crate) fn lineage_evidence(&self) -> Option<&LineageEvidenceInput> {
        self.lineage_evidence.as_ref()
    }

    pub(crate) fn structural_evidence(&self) -> Option<&StructuralEvidenceInput> {
        self.structural_evidence.as_ref()
    }

    pub(crate) fn intent(&self) -> &CorrespondenceFamilyIntent {
        &self.intent
    }

    #[cfg(test)]
    pub(crate) fn from_inputs(
        lineage_evidence: Option<LineageEvidenceInput>,
        structural_evidence: Option<StructuralEvidenceInput>,
        discovery_plan: StructuralCandidateDiscoveryPlan,
        budget: StructuralCandidateBudget,
    ) -> Result<Self, CorrespondenceEvaluationError> {
        if lineage_evidence.is_none() && structural_evidence.is_none() {
            return Err(CorrespondenceEvaluationError::MissingEvidence);
        }

        let intent = match (lineage_evidence.is_some(), structural_evidence.is_some()) {
            (true, false) => CorrespondenceFamilyIntent::LineageOnly,
            (false, true) => CorrespondenceFamilyIntent::StructuralOnly,
            (true, true) => CorrespondenceFamilyIntent::MixedEvidenceAdmitted,
            (false, false) => return Err(CorrespondenceEvaluationError::MissingEvidence),
        };

        Ok(Self {
            lineage_evidence,
            structural_evidence,
            discovery_plan,
            budget,
            intent,
        })
    }
}
