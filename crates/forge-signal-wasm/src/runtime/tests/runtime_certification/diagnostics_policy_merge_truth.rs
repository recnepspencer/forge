use super::super::support::*;

#[test]
fn diagnostics_tier_changes_richness_only_not_merge_truth() {
    let development = RuntimePolicySpec {
        preset: RuntimePolicyPreset::WebDevelopment,
    };
    let kernel = RuntimePolicySpec {
        preset: RuntimePolicyPreset::Kernel,
    };

    let (mut development_runtime, development_main, development_feature, _) =
        build_adversarial_merge_runtime(development);
    let (mut kernel_runtime, kernel_main, kernel_feature, _) =
        build_adversarial_merge_runtime(kernel);

    let development_plan = development_runtime
        .plan_merge_branches_with_proof(development_feature, development_main)
        .unwrap();
    let kernel_plan = kernel_runtime
        .plan_merge_branches_with_proof(kernel_feature, kernel_main)
        .unwrap();
    assert_eq!(
        development_plan.proof.plan_digest,
        kernel_plan.proof.plan_digest
    );
    assert_eq!(
        development_plan.proof.semantics_digest,
        kernel_plan.proof.semantics_digest
    );

    let development_result = development_runtime
        .merge_branches_with_proof(development_feature, development_main)
        .unwrap();
    let kernel_result = kernel_runtime
        .merge_branches_with_proof(kernel_feature, kernel_main)
        .unwrap();

    assert_eq!(
        development_result.proof.result_digest,
        kernel_result.proof.result_digest
    );
    assert_eq!(
        development_result.result.selected_semantics,
        kernel_result.result.selected_semantics
    );

    let development_state = development_runtime
        .branch_state_proof(development_main)
        .unwrap();
    let kernel_state = kernel_runtime.branch_state_proof(kernel_main).unwrap();
    assert_eq!(development_state.state_digest, kernel_state.state_digest);
}
