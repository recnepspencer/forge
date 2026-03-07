use crate::facade::*;

#[test]
fn chain_1000_minimal_recomputation() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<crate::facade::NodeId> = Vec::with_capacity(1000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..1000 {
        let node = graph.create_node();
        graph
            .add_dependency(node, chain[i - 1], Aspect::Geometry)
            .unwrap();
        chain.push(node);
    }

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(0, 1));
    for node in &chain {
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }

    mark_dirty(&mut graph, chain[0], Aspect::Geometry).unwrap();

    let state_first = graph.get_state(chain[0]).unwrap();
    assert_eq!(state_first, NodeState::Dirty);

    let state_second = graph.get_state(chain[1]).unwrap();
    assert_eq!(state_second, NodeState::Dirty);

    let state_last = graph.get_state(chain[999]).unwrap();
    assert_eq!(state_last, NodeState::MaybeStale);
}

#[test]
fn push_perf_10k_nodes() {
    let mut graph = SignalGraph::new();
    let mut chain: Vec<crate::facade::NodeId> = Vec::with_capacity(10_000);

    let first = graph.create_node();
    chain.push(first);

    for i in 1..10_000 {
        let node = graph.create_node();
        graph
            .add_dependency(node, chain[i - 1], Aspect::Geometry)
            .unwrap();
        chain.push(node);
    }

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(0, 1));
    for node in &chain {
        evaluate(&mut graph, *node, &mut compute).unwrap();
    }

    let start = std::time::Instant::now();
    mark_dirty(&mut graph, chain[0], Aspect::Geometry).unwrap();
    let elapsed = start.elapsed();

    let max_push_ms: u128 = 500;
    assert!(
        elapsed.as_millis() < max_push_ms,
        "Push propagation took {}ms, expected < {}ms",
        elapsed.as_millis(),
        max_push_ms
    );
}
