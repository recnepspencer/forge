use super::super::support::*;

#[test]
fn replay_parity_proof_distinguishes_equivalent_and_divergent_branch_states() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(16.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.42),
            produces_aspects: None,
        })
        .unwrap();

    let main = runtime.current_branch();
    let twin = runtime.create_branch("twin".to_owned()).unwrap();

    let parity = runtime.replay_parity_proof(main.id.0, twin.id.0).unwrap();
    assert_eq!(
        parity.proof_schema_version,
        worth_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION
    );
    assert!(parity.parity);
    assert_eq!(parity.expected_state_digest, parity.replayed_state_digest);

    runtime.switch_branch(twin.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let divergent = runtime.replay_parity_proof(main.id.0, twin.id.0).unwrap();
    assert!(!divergent.parity);
    assert_ne!(
        divergent.expected_state_digest,
        divergent.replayed_state_digest
    );
}

#[test]
fn adversarial_merge_proof_envelopes_and_rebuild_state_remain_consistent() {
    let (mut runtime, main_branch_id, feature_branch_id, main_branch_name) =
        build_adversarial_merge_runtime(RuntimePolicySpec::default());

    let plan_envelope = runtime
        .plan_merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    assert_eq!(
        plan_envelope.proof.proof_schema_version,
        worth_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION
    );
    assert_eq!(plan_envelope.plan.source_branch_id, feature_branch_id);
    assert_eq!(plan_envelope.plan.target_branch_id, main_branch_id);
    assert!(!plan_envelope
        .plan
        .selected_semantics
        .strategy_name
        .is_empty());
    assert!(plan_envelope
        .plan
        .node_map
        .iter()
        .all(|entry| entry.source_node.contains(':') && entry.target_node.contains(':')));
    assert!(plan_envelope
        .plan
        .node_plan
        .iter()
        .all(|entry| !entry.decision.is_empty()));
    assert!(plan_envelope
        .plan
        .adoption_core
        .iter()
        .all(|entry| entry.source_node.contains(':')));
    assert!(plan_envelope
        .plan
        .adoption_policy
        .iter()
        .all(|entry| !entry.runtime_artifact.is_empty()));

    let result_envelope = runtime
        .merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    assert_eq!(
        result_envelope.proof.proof_schema_version,
        worth_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION
    );
    assert_eq!(
        plan_envelope.proof.selected_strategy_digest,
        result_envelope.proof.selected_strategy_digest
    );
    assert_eq!(
        plan_envelope.proof.selected_merge_base_digest,
        result_envelope.proof.selected_merge_base_digest
    );
    assert_eq!(
        plan_envelope.proof.selected_conflict_policy_digest,
        result_envelope.proof.selected_conflict_policy_digest
    );
    assert_eq!(
        plan_envelope.proof.selected_conflict_isolation_digest,
        result_envelope.proof.selected_conflict_isolation_digest
    );
    assert_eq!(result_envelope.result.source_branch, feature_branch_id);
    assert_eq!(result_envelope.result.target_branch, main_branch_id);
    assert!(result_envelope.result.counters.replay_event_count >= 1);
    assert!(result_envelope
        .result
        .records
        .iter()
        .all(|record| record.source_node.contains(':')));

    runtime.switch_branch(main_branch_id).unwrap();
    assert_eq!(
        runtime.read_value("gearTeeth").unwrap(),
        SignalValue::Number(22.0)
    );
    assert_eq!(
        runtime.read_value("gearThickness").unwrap(),
        SignalValue::Number(0.42)
    );
    assert_eq!(
        runtime.read_value("gearInnerRadius").unwrap(),
        SignalValue::Number(0.36)
    );
    assert_eq!(
        runtime.read_value("lightIntensity").unwrap(),
        SignalValue::Number(1.78)
    );

    let merged_proof = runtime.branch_state_proof(main_branch_id).unwrap();
    let envelope = runtime.export_runtime_envelope().unwrap();
    let mut rebuilt = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    rebuilt.replace_runtime_envelope(envelope).unwrap();
    let rebuilt_main_branch = rebuilt
        .branches()
        .into_iter()
        .find(|branch| branch.name == main_branch_name)
        .expect("rebuilt runtime should preserve the merged target branch by name");
    rebuilt.switch_branch(rebuilt_main_branch.id.0).unwrap();
    let rebuilt_proof = rebuilt
        .branch_state_proof(rebuilt_main_branch.id.0)
        .unwrap();

    assert_eq!(
        merged_proof.proof_schema_version,
        rebuilt_proof.proof_schema_version
    );
    assert_eq!(merged_proof.state_digest, rebuilt_proof.state_digest);
    assert_eq!(
        rebuilt.read_value("gearTopologyModel").unwrap(),
        SignalValue::Object(vec![
            ("teeth".to_owned(), SignalValue::Number(22.0)),
            ("thickness".to_owned(), SignalValue::Number(0.42)),
            ("innerRadius".to_owned(), SignalValue::Number(0.36)),
        ])
    );
    assert_eq!(
        rebuilt.read_value("hudModel").unwrap(),
        SignalValue::Object(vec![
            (
                "gear".to_owned(),
                SignalValue::Object(vec![
                    ("teeth".to_owned(), SignalValue::Number(22.0)),
                    ("thickness".to_owned(), SignalValue::Number(0.42)),
                    ("innerRadius".to_owned(), SignalValue::Number(0.36)),
                ]),
            ),
            ("light".to_owned(), SignalValue::Number(1.78)),
        ])
    );
}
