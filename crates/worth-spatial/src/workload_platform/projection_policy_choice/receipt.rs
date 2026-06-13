use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::lane_choice::ProjectionPolicyLaneChoice;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionPolicyChoiceReceipt {
    policy_choice_digest: String,
    lane_choices: Vec<ProjectionPolicyLaneChoice>,
    workload_basis_identity: String,
}

impl ProjectionPolicyChoiceReceipt {
    pub(crate) fn new(
        lane_choices: Vec<ProjectionPolicyLaneChoice>,
        workload_basis_identity: impl Into<String>,
    ) -> Self {
        let workload_basis_identity = workload_basis_identity.into();
        let policy_choice_digest = policy_choice_digest(&lane_choices, &workload_basis_identity);
        Self {
            policy_choice_digest,
            lane_choices,
            workload_basis_identity,
        }
    }

    pub fn policy_choice_digest(&self) -> &str {
        &self.policy_choice_digest
    }

    pub fn lane_choices(&self) -> &[ProjectionPolicyLaneChoice] {
        &self.lane_choices
    }

    pub fn workload_basis_identity(&self) -> &str {
        &self.workload_basis_identity
    }
}

fn policy_choice_digest(
    lane_choices: &[ProjectionPolicyLaneChoice],
    workload_basis_identity: &str,
) -> String {
    let mut parts = vec![
        "projection-policy-choice".to_string(),
        format!("basis:{workload_basis_identity}"),
        format!("lanes:{}", lane_choices.len()),
    ];
    for choice in lane_choices {
        parts.push(format!("lane:{:?}", choice.lane()));
        parts.push(format!("basis:{}", choice.parity_basis_identity()));
        parts.push(format!(
            "outcome-evidence:{}",
            choice.policy_outcome_evidence_digest()
        ));
        parts.push(format!(
            "outcome-source:{}",
            choice.policy_outcome_source_identity()
        ));
        parts.push(format!("choices:{}", choice.choices().len()));
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
