use super::candidate_set::CorrespondenceCandidateSet;
use super::contracts::UniqueStructuralCorrespondenceWitness;
use super::cost::CorrespondenceCostPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageContinuity {
    canonical_subject: String,
    canonical_counterpart: String,
}

impl LineageContinuity {
    pub fn canonical_subject(&self) -> &str {
        &self.canonical_subject
    }

    pub fn authoritative_counterpart(&self) -> &str {
        &self.canonical_counterpart
    }

    pub(crate) fn new(
        canonical_subject: impl Into<String>,
        canonical_counterpart: impl Into<String>,
    ) -> Self {
        Self {
            canonical_subject: canonical_subject.into(),
            canonical_counterpart: canonical_counterpart.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryStructuralUnique {
    advisory_candidate: String,
    witness: UniqueStructuralCorrespondenceWitness,
}

impl AdvisoryStructuralUnique {
    pub fn advisory_candidate(&self) -> &str {
        &self.advisory_candidate
    }

    pub fn uniqueness_witness(&self) -> &UniqueStructuralCorrespondenceWitness {
        &self.witness
    }

    pub(crate) fn new(
        advisory_candidate: impl Into<String>,
        witness: UniqueStructuralCorrespondenceWitness,
    ) -> Self {
        Self {
            advisory_candidate: advisory_candidate.into(),
            witness,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryStructuralAmbiguous {
    candidate_set: CorrespondenceCandidateSet,
}

impl AdvisoryStructuralAmbiguous {
    pub fn candidate_set(&self) -> &CorrespondenceCandidateSet {
        &self.candidate_set
    }

    pub(crate) fn new(candidate_set: CorrespondenceCandidateSet) -> Self {
        Self { candidate_set }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageStructuralDisagreement {
    lineage_counterpart: String,
    structural_counterpart: String,
}

impl LineageStructuralDisagreement {
    pub fn lineage_counterpart(&self) -> &str {
        &self.lineage_counterpart
    }

    pub fn structural_counterpart(&self) -> &str {
        &self.structural_counterpart
    }

    pub(crate) fn new(
        lineage_counterpart: impl Into<String>,
        structural_counterpart: impl Into<String>,
    ) -> Self {
        Self {
            lineage_counterpart: lineage_counterpart.into(),
            structural_counterpart: structural_counterpart.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceDenied {
    cost_posture: CorrespondenceCostPosture,
    reason: &'static str,
}

impl CorrespondenceDenied {
    pub fn cost_posture(&self) -> &CorrespondenceCostPosture {
        &self.cost_posture
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn new(cost_posture: CorrespondenceCostPosture, reason: &'static str) -> Self {
        Self {
            cost_posture,
            reason,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CorrespondenceOutcomeValue {
    LineageContinuity(LineageContinuity),
    AdvisoryStructuralUnique(AdvisoryStructuralUnique),
    AdvisoryStructuralAmbiguous(AdvisoryStructuralAmbiguous),
    LineageStructuralDisagreement(LineageStructuralDisagreement),
    CorrespondenceDenied(CorrespondenceDenied),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceOutcome {
    value: CorrespondenceOutcomeValue,
}

impl CorrespondenceOutcome {
    pub fn family_name(&self) -> &'static str {
        match &self.value {
            CorrespondenceOutcomeValue::LineageContinuity(_) => "lineage_continuity",
            CorrespondenceOutcomeValue::AdvisoryStructuralUnique(_) => "advisory_structural_unique",
            CorrespondenceOutcomeValue::AdvisoryStructuralAmbiguous(_) => {
                "advisory_structural_ambiguous"
            }
            CorrespondenceOutcomeValue::LineageStructuralDisagreement(_) => {
                "lineage_structural_disagreement"
            }
            CorrespondenceOutcomeValue::CorrespondenceDenied(_) => "correspondence_denied",
        }
    }

    pub fn as_lineage_continuity(&self) -> Option<&LineageContinuity> {
        match &self.value {
            CorrespondenceOutcomeValue::LineageContinuity(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_advisory_structural_unique(&self) -> Option<&AdvisoryStructuralUnique> {
        match &self.value {
            CorrespondenceOutcomeValue::AdvisoryStructuralUnique(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_advisory_structural_ambiguous(&self) -> Option<&AdvisoryStructuralAmbiguous> {
        match &self.value {
            CorrespondenceOutcomeValue::AdvisoryStructuralAmbiguous(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_lineage_structural_disagreement(&self) -> Option<&LineageStructuralDisagreement> {
        match &self.value {
            CorrespondenceOutcomeValue::LineageStructuralDisagreement(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_denied(&self) -> Option<&CorrespondenceDenied> {
        match &self.value {
            CorrespondenceOutcomeValue::CorrespondenceDenied(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn lineage_continuity(value: LineageContinuity) -> Self {
        Self {
            value: CorrespondenceOutcomeValue::LineageContinuity(value),
        }
    }

    pub(crate) fn advisory_structural_unique(value: AdvisoryStructuralUnique) -> Self {
        Self {
            value: CorrespondenceOutcomeValue::AdvisoryStructuralUnique(value),
        }
    }

    pub(crate) fn advisory_structural_ambiguous(value: AdvisoryStructuralAmbiguous) -> Self {
        Self {
            value: CorrespondenceOutcomeValue::AdvisoryStructuralAmbiguous(value),
        }
    }

    pub(crate) fn lineage_structural_disagreement(value: LineageStructuralDisagreement) -> Self {
        Self {
            value: CorrespondenceOutcomeValue::LineageStructuralDisagreement(value),
        }
    }

    pub(crate) fn denied(value: CorrespondenceDenied) -> Self {
        Self {
            value: CorrespondenceOutcomeValue::CorrespondenceDenied(value),
        }
    }
}
