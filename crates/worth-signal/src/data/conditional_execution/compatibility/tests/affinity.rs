use crate::data::graph::SignalGraph;

use super::super::super::{
    SignalConditionalArtifactReuse, SignalConditionalCondition,
    SignalConditionalContractDefinition, SignalConditionalVersionComparator,
};
use super::super::{
    SignalConditionalConditionClass, SignalConditionalExecutionAffinityMismatch,
    SignalConditionalSemanticMismatch,
};
use super::support::{
    base_definition, claim_owner, install_at, install_fresh, mask, portable_definition,
    with_condition,
};

#[test]
fn execution_affinity_rejects_semantic_drift_before_graph_identity() {
    let current = install_fresh(base_definition());
    let candidate = install_fresh(with_condition(SignalConditionalCondition::TemporalWake));

    let denial = current.compare_execution_affinity(&candidate).unwrap_err();
    assert!(matches!(
        denial.mismatch(),
        SignalConditionalExecutionAffinityMismatch::Semantic(
            SignalConditionalSemanticMismatch::ConditionClass {
                current: SignalConditionalConditionClass::DeltaThreshold,
                candidate: SignalConditionalConditionClass::TemporalWake,
            }
        )
    ));
    assert_eq!(denial.work().semantic_dimensions_inspected(), 1);
    assert_eq!(denial.work().affinity_dimensions_inspected(), 0);
}

#[test]
fn execution_affinity_rejects_foreign_graph_before_node_dimensions() {
    let current = install_fresh(portable_definition());
    let candidate = install_fresh(portable_definition());

    let denial = current.compare_execution_affinity(&candidate).unwrap_err();
    assert!(matches!(
        denial.mismatch(),
        SignalConditionalExecutionAffinityMismatch::GraphInstance { .. }
    ));
    assert_eq!(denial.work().semantic_dimensions_inspected(), 7);
    assert_eq!(denial.work().affinity_dimensions_inspected(), 1);
}

#[test]
fn execution_affinity_rejects_same_graph_different_node_index() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let owner = claim_owner(&mut graph);
    let current = install_at(&mut graph, &owner, first_node, portable_definition());
    let candidate = install_at(&mut graph, &owner, second_node, portable_definition());

    assert!(matches!(
        current
            .compare_execution_affinity(&candidate)
            .unwrap_err()
            .mismatch(),
        SignalConditionalExecutionAffinityMismatch::NodeIndex { .. }
    ));
}

#[test]
fn execution_affinity_rejects_same_slot_after_generation_turnover() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let owner = claim_owner(&mut graph);
    let current = install_at(&mut graph, &owner, first_node, portable_definition());
    graph.unregister_node(first_node).unwrap();
    let successor_node = graph.node().build();
    assert_eq!(first_node.index(), successor_node.index());
    assert_ne!(first_node.generation(), successor_node.generation());
    let candidate = install_at(&mut graph, &owner, successor_node, portable_definition());

    assert!(matches!(
        current
            .compare_execution_affinity(&candidate)
            .unwrap_err()
            .mismatch(),
        SignalConditionalExecutionAffinityMismatch::NodeGeneration { .. }
    ));
}

#[test]
fn reinstalling_exact_runtime_resolved_contract_preserves_execution_affinity() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let owner = claim_owner(&mut graph);
    let definition = SignalConditionalContractDefinition {
        condition: SignalConditionalCondition::RuntimePredicate,
        dependency_aspects: mask(1),
        trigger_aspects: mask(2),
        dependency_comparator: SignalConditionalVersionComparator::RuntimeResolved,
        output_comparator: SignalConditionalVersionComparator::RuntimeResolved,
        artifact_reuse: SignalConditionalArtifactReuse::RuntimeResolved,
    };
    let current = install_at(&mut graph, &owner, node, definition.clone());
    let candidate = install_at(&mut graph, &owner, node, definition);

    let affinity = current.compare_execution_affinity(&candidate).unwrap();
    let _semantic_proof = affinity.semantic_continuity();
    assert_eq!(affinity.work().semantic_dimensions_inspected(), 10);
    assert_eq!(affinity.work().affinity_dimensions_inspected(), 7);
}
