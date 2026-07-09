use super::super::support::*;

#[test]
fn merge_plan_and_result_are_available_through_history_surface() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "counter".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let plan = runtime
        .plan_merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();
    assert_eq!(plan.source_branch_id, feature_branch.id.0);
    assert_eq!(plan.target_branch_id, main_branch.id.0);
    assert!(!plan.selected_semantics.strategy_name.is_empty());
    assert!(plan
        .node_map
        .iter()
        .all(|entry| entry.source_node.contains(':') && entry.target_node.contains(':')));
    assert!(plan
        .node_plan
        .iter()
        .all(|entry| !entry.decision.is_empty()));
    assert!(plan
        .adoption_core
        .iter()
        .all(|entry| entry.source_node.contains(':')));
    assert!(plan
        .adoption_policy
        .iter()
        .all(|entry| !entry.runtime_artifact.is_empty()));

    let result = runtime
        .merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();
    assert_eq!(result.source_branch, feature_branch.id.0);
    assert_eq!(result.target_branch, main_branch.id.0);
    assert!(result.counters.replay_event_count >= 1);
}
