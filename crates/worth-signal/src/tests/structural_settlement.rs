use crate::facade::*;
use crate::tests::support::*;

#[test]
fn clean_dependency_rewire_forces_one_structural_recompute() {
    let mut graph = SignalGraph::new();
    let old_source = graph.node().build();
    let new_source = graph.node().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(old_source, ASPECT_A)])
        .unwrap();

    let mut source_compute = |_id, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, old_source, &mut source_compute).unwrap();
    evaluate(&mut graph, new_source, &mut source_compute).unwrap();
    let mut baseline_calls = 0_u32;
    evaluate(&mut graph, consumer, &mut |_id, _graph| {
        baseline_calls += 1;
        Ok(version_ab(1, 0))
    })
    .unwrap();

    graph
        .set_dependencies(consumer, [DependencyEdge::new(new_source, ASPECT_A)])
        .unwrap();
    assert_eq!(graph.get_state(consumer).unwrap(), NodeState::MaybeStale);
    let pending = graph
        .pending_dependency_revalidation(consumer)
        .unwrap()
        .expect("rewire must establish structural authority");
    assert!(pending.requires_structural_recompute());
    assert!(pending.is_resolved());

    let mut rewire_calls = 0_u32;
    evaluate(&mut graph, consumer, &mut |_id, _graph| {
        rewire_calls += 1;
        Ok(version_ab(2, 0))
    })
    .unwrap();

    assert_eq!(baseline_calls, 1);
    assert_eq!(rewire_calls, 1);
    assert_eq!(graph.get_state(consumer).unwrap(), NodeState::Clean);
}

#[test]
fn deferred_rewired_producer_does_not_resolve_structural_consumer() {
    let mut graph = SignalGraph::new();
    let old_source = graph.node().build();
    let gated_source = graph.node().on_demand().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(old_source, ASPECT_A)])
        .unwrap();

    let mut source_compute = |_id, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, old_source, &mut source_compute).unwrap();
    evaluate(&mut graph, consumer, &mut source_compute).unwrap();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(gated_source, ASPECT_A)])
        .unwrap();

    let mut consumer_calls = 0_u32;
    evaluate(&mut graph, consumer, &mut |node, _graph| {
        if node == consumer {
            consumer_calls += 1;
        }
        Ok(version_ab(2, 0))
    })
    .unwrap();

    assert_eq!(consumer_calls, 0);
    assert_eq!(
        graph.get_state(gated_source).unwrap(),
        NodeState::MaybeStale
    );
    assert_eq!(graph.get_state(consumer).unwrap(), NodeState::MaybeStale);
    let pending = graph
        .pending_dependency_revalidation(consumer)
        .unwrap()
        .expect("deferred producer must remain pending");
    assert_eq!(pending.unresolved_producers(), &[gated_source]);
}

#[test]
fn structural_recompute_precedes_the_consumers_ordinary_gate() {
    let mut graph = SignalGraph::new();
    let old_source = graph.node().build();
    let new_source = graph.node().build();
    let consumer = graph.node().on_demand().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(old_source, ASPECT_A)])
        .unwrap();

    let mut source_compute = |_id, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, old_source, &mut source_compute).unwrap();
    evaluate(&mut graph, new_source, &mut source_compute).unwrap();
    evaluate_on_demand(&mut graph, consumer, &mut source_compute).unwrap();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(new_source, ASPECT_A)])
        .unwrap();

    let mut consumer_calls = 0_u32;
    evaluate(&mut graph, consumer, &mut |_id, _graph| {
        consumer_calls += 1;
        Ok(version_ab(2, 0))
    })
    .unwrap();

    assert_eq!(consumer_calls, 1);
    assert_eq!(graph.get_state(consumer).unwrap(), NodeState::Clean);
}

#[test]
fn stable_producer_settlement_resolves_but_does_not_erase_structural_work() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| Ok(version_ab(0, 0))).unwrap();

    let pending = graph
        .pending_dependency_revalidation(consumer)
        .unwrap()
        .expect("structural posture must survive stable predecessor settlement");
    assert!(pending.is_resolved());
    assert!(pending.requires_structural_recompute());
    assert_eq!(graph.get_state(consumer).unwrap(), NodeState::Dirty);
    let mut calls = 0;
    evaluate(&mut graph, consumer, &mut |_id, _graph| {
        calls += 1;
        Ok(version_ab(1, 0))
    })
    .unwrap();
    assert_eq!(calls, 1);
}

#[test]
fn unrelated_aspect_commit_still_resolves_the_immediate_structural_waiter() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let consumer = graph.node().build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, ASPECT_A)])
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| Ok(version_ab(0, 1))).unwrap();

    assert!(graph.pending_causes(consumer).unwrap().is_empty());
    let pending = graph
        .pending_dependency_revalidation(consumer)
        .unwrap()
        .expect("producer settlement is independent from aspect cause admission");
    assert!(pending.is_resolved());
    assert!(pending.requires_structural_recompute());
    assert_eq!(graph.get_state(consumer).unwrap(), NodeState::Dirty);
}
