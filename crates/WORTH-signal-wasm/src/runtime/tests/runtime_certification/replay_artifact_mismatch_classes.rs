use super::super::support::*;

#[test]
fn replay_artifact_proof_reports_typed_mismatch_classes() {
    let (mut runtime, main_branch_id, feature_branch_id, _) =
        build_adversarial_merge_runtime(RuntimePolicySpec::default());

    let expected_plan = runtime
        .plan_merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    let expected_result = runtime
        .merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    let expected_state = runtime.branch_state_proof(main_branch_id).unwrap();

    let replayed_branch = runtime
        .create_branch("replayed-divergent".to_owned())
        .unwrap();
    runtime.switch_branch(replayed_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(7.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let report = runtime
        .replay_artifact_proof(
            worth_signal::facade::adapters::ReplayArtifactProofInput {
                proof_schema_version: expected_result.proof.proof_schema_version.clone(),
                registry_bundle_digest: Some(expected_result.proof.registry_bundle_digest.clone()),
                lowered_strategy_bundle_digest: Some(
                    expected_result.proof.lowered_strategy_bundle_digest.clone(),
                ),
                merge_plan_digest: Some(expected_plan.proof.plan_digest.clone()),
                merge_result_digest: Some(expected_result.proof.result_digest.clone()),
                lineage_digest: Some(expected_result.proof.lineage_digest.clone()),
                branch_state_digest: expected_state.state_digest.clone(),
            },
            replayed_branch.id.0,
        )
        .unwrap();

    assert!(!report.parity);
    assert!(report
        .mismatch_classes
        .contains(&worth_signal::facade::adapters::ReplayMismatchClass::BranchStateDigestMismatch));
}
