use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;
use crate::workload_platform::user_response::WorthPolicyDecision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionPolicyLaneChoice {
    lane: ProjectionFactParityLane,
    choices: Vec<WorthPolicyDecision>,
    parity_basis_identity: String,
    policy_outcome_evidence_digest: String,
    policy_outcome_source_identity: String,
}

impl ProjectionPolicyLaneChoice {
    pub(crate) fn new(
        lane: ProjectionFactParityLane,
        choices: Vec<WorthPolicyDecision>,
        parity_basis_identity: impl Into<String>,
        policy_outcome_evidence_digest: impl Into<String>,
        policy_outcome_source_identity: impl Into<String>,
    ) -> Self {
        Self {
            lane,
            choices,
            parity_basis_identity: parity_basis_identity.into(),
            policy_outcome_evidence_digest: policy_outcome_evidence_digest.into(),
            policy_outcome_source_identity: policy_outcome_source_identity.into(),
        }
    }

    pub fn lane(&self) -> ProjectionFactParityLane {
        self.lane
    }

    pub fn choices(&self) -> &[WorthPolicyDecision] {
        &self.choices
    }

    pub fn parity_basis_identity(&self) -> &str {
        &self.parity_basis_identity
    }

    pub fn policy_outcome_evidence_digest(&self) -> &str {
        &self.policy_outcome_evidence_digest
    }

    pub fn policy_outcome_source_identity(&self) -> &str {
        &self.policy_outcome_source_identity
    }
}
