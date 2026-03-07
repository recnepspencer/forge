use crate::facade::*;
use crate::tests::support::*;

#[test]
fn kv61_add_delete_10k_flat_memory() {
    let mut graph = SignalGraph::with_gc_threshold(100);

    for _ in 0..10_000 {
        let node = graph.create_node();
        graph.unregister_node(node).unwrap();

        if graph.should_gc() {
            graph.run_gc_epoch();
        }
    }

    assert!(
        graph.arena_capacity() <= 200,
        "Arena capacity {} should stay bounded via slot reuse",
        graph.arena_capacity()
    );
    assert_eq!(graph.active_node_count(), 0);
}

#[test]
fn kv62_delete_mid_chain_no_panic() {
    let mut graph = SignalGraph::new();
    let param = graph.create_node();
    let middle = graph.create_node();
    let feature = graph.create_node();

    graph
        .add_dependency(middle, param, ASPECT_B)
        .unwrap();
    graph
        .add_dependency(feature, middle, ASPECT_B)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, param, &mut compute).unwrap();
    evaluate(&mut graph, middle, &mut compute).unwrap();
    evaluate(&mut graph, feature, &mut compute).unwrap();

    graph.unregister_node(middle).unwrap();

    mark_dirty(&mut graph, param, ASPECT_B).unwrap();

    let feature_state = graph.get_state(feature).unwrap();
    assert_eq!(
        feature_state,
        NodeState::Dirty,
        "Feature should be Dirty after middle node was deleted"
    );
}

#[test]
fn unregister_severs_subscriptions() {
    let mut graph = SignalGraph::new();
    let upstream = graph.create_node();
    let node = graph.create_node();
    let downstream = graph.create_node();

    graph
        .add_dependency(node, upstream, ASPECT_B)
        .unwrap();
    graph
        .add_dependency(downstream, node, ASPECT_B)
        .unwrap();

    graph.unregister_node(node).unwrap();

    let upstream_subs = graph.get_entry(upstream).unwrap().get_subscribers();
    assert!(
        upstream_subs.is_empty(),
        "Upstream should have no subscribers after middle node deleted"
    );

    let downstream_deps = graph.get_entry(downstream).unwrap().get_dependencies();
    assert!(
        downstream_deps.is_empty(),
        "Downstream should have no deps on deleted node"
    );
}

#[test]
fn gc_epoch_compacts_arena() {
    let mut graph = SignalGraph::with_gc_threshold(5);

    let mut nodes = Vec::new();
    for _ in 0..10 {
        nodes.push(graph.create_node());
    }

    for node in &nodes[..5] {
        graph.unregister_node(*node).unwrap();
    }

    assert_eq!(graph.tombstone_count(), 5);
    graph.run_gc_epoch();
    assert_eq!(graph.tombstone_count(), 0);
}

#[test]
fn vacated_slot_reuse_preserves_generation_safety() {
    let mut graph = SignalGraph::new();
    let original = graph.create_node();
    graph.unregister_node(original).unwrap();
    let reused = graph.create_node();

    assert_ne!(original.generation(), reused.generation());
    assert!(!graph.is_alive(original));
    assert!(graph.is_alive(reused));
}

#[test]
fn double_unregister_is_rejected_without_free_list_corruption() {
    let mut graph = SignalGraph::new();
    let node = graph.create_node();
    graph.unregister_node(node).unwrap();
    let capacity_before = graph.arena_capacity();

    assert!(graph.unregister_node(node).is_err());

    let reused = graph.create_node();
    assert_eq!(graph.arena_capacity(), capacity_before);
    assert_ne!(reused.generation(), node.generation());
}
