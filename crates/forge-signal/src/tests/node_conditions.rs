use crate::facade::*;

#[test]
fn node_entry_stores_evaluation_condition_config() {
    let mut graph = SignalGraph::new();
    let node = graph.create_node();

    let cfg = NodeEvaluationConfig {
        condition: EvaluationCondition::Debounce(2_000),
        ..NodeEvaluationConfig::default()
    };
    graph.get_entry_mut(node).unwrap().set_eval_config(cfg.clone());

    let stored = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(stored, cfg);
}

#[test]
fn create_node_with_config_sets_condition() {
    let mut graph = SignalGraph::new();
    let node = graph.create_node_with_config(NodeEvaluationConfig {
        condition: EvaluationCondition::OnDemand,
        ..NodeEvaluationConfig::default()
    });
    assert!(matches!(
        graph.get_entry(node).unwrap().get_eval_config().condition,
        EvaluationCondition::OnDemand
    ));
}
