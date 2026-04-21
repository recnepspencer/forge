use forge_query::facade::{PolicyCostPosture, PolicyEpoch, PolicyRuleSnapshot, PolicyWorkBudget};

fn main() {
    let _ = PolicyRuleSnapshot {
        policy_basis_label: "policy".to_string(),
        rule_set_digest: "rules".to_string(),
        policy_epoch: PolicyEpoch::Synthetic(1),
        admits_query_family: true,
        narrows_projection: false,
        admits_non_disclosing_use: false,
        cost_posture: PolicyCostPosture::ConstantProof,
        work_budget: Some(PolicyWorkBudget::bounded(1, 1, 1)),
        digest: "digest".to_string(),
    };
}
