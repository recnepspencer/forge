use worth_spatial::facade::projection_policy_choice::ProjectionPolicyChoiceReceipt;

fn main() {
    let _receipt = ProjectionPolicyChoiceReceipt {
        policy_choice_digest: "fake".to_string(),
        lane_choices: Vec::new(),
        workload_basis_identity: "fake".to_string(),
    };
}
