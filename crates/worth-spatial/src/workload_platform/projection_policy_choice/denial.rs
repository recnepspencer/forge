use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionPolicyChoiceDenialKind {
    EmptyPolicyMatrix,
    DuplicatePolicyLane,
    OutcomeWasNotPolicyRequired,
    MissingPolicyLane,
    MissingUserChoices,
    MismatchedPolicyOutcomeEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionPolicyChoiceDenial {
    kind: ProjectionPolicyChoiceDenialKind,
    lane: Option<ProjectionFactParityLane>,
    human_reason: String,
}

impl ProjectionPolicyChoiceDenial {
    pub(crate) fn new(
        kind: ProjectionPolicyChoiceDenialKind,
        lane: Option<ProjectionFactParityLane>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            lane,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> ProjectionPolicyChoiceDenialKind {
        self.kind
    }

    pub fn lane(&self) -> Option<ProjectionFactParityLane> {
        self.lane
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
