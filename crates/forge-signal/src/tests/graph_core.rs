use crate::facade::*;
use crate::tests::support::*;

#[test]
fn create_node_returns_valid_handle() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    assert!(graph.is_alive(node));
    assert_eq!(graph.active_node_count(), 1);
}

#[test]
fn new_node_starts_dirty() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let state = graph.get_state(node).unwrap();
    assert_eq!(state, NodeState::Dirty);
}

#[test]
fn add_dependency_wires_both_directions() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().build();
    let downstream = graph.node().build();
    graph
        .add_dependency(downstream, upstream, ASPECT_A)
        .unwrap();

    let deps = graph.get_entry(downstream).unwrap().get_dependencies();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].source(), upstream);

    let subs = graph.get_entry(upstream).unwrap().get_subscribers();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0], downstream);
}

#[test]
fn dirty_direct_dependent() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .add_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let state = graph.get_state(dependent).unwrap();
    assert_eq!(state, NodeState::Dirty);
}

#[test]
fn maybe_stale_transitive_dependent() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();

    graph.add_dependency(b, a, ASPECT_B).unwrap();
    graph.add_dependency(c, b, ASPECT_B).unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, a, &mut compute).unwrap();
    evaluate(&mut graph, b, &mut compute).unwrap();
    evaluate(&mut graph, c, &mut compute).unwrap();

    mark_dirty(&mut graph, a, ASPECT_B).unwrap();

    let state_b = graph.get_state(b).unwrap();
    let state_c = graph.get_state(c).unwrap();
    assert_eq!(state_b, NodeState::Dirty);
    assert_eq!(state_c, NodeState::MaybeStale);
}

#[test]
fn clean_version_skip_on_unchanged_upstream() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();

    graph.add_dependency(b, a, ASPECT_A).unwrap();
    graph.add_dependency(c, b, ASPECT_A).unwrap();

    let mut eval_count = 0u32;

    let mut compute_a = |_id, _g: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, a, &mut compute_a).unwrap();

    let mut compute_b = |_id, _g: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, b, &mut compute_b).unwrap();

    let mut compute_c = |_id, _g: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, c, &mut compute_c).unwrap();

    mark_dirty(&mut graph, a, ASPECT_B).unwrap();

    let mut recompute = |_id, _g: &SignalGraph| {
        eval_count += 1;
        Ok(version_ab(1, 0))
    };
    evaluate(&mut graph, a, &mut recompute).unwrap();
    evaluate(&mut graph, b, &mut recompute).unwrap();
    evaluate(&mut graph, c, &mut recompute).unwrap();

    assert!(
        eval_count <= 2,
        "node c should have skipped recomputation (MaybeStale with unchanged topo)"
    );
}
