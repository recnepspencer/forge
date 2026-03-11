use crate::facade::*;
use crate::tests::support::*;

#[test]
fn rollback_created_nodes_keeps_free_list_unique_and_bounded_across_reuse_cycles() {
    let mut graph = SignalGraph::new();
    let anchor = graph.create_node();
    let reclaimed = graph.create_node();
    graph.unregister_node(reclaimed).unwrap();

    for _ in 0..8 {
        let created = (0..6).map(|_| graph.create_node()).collect::<Vec<_>>();
        graph.rollback_created_nodes(&created);

        let free_list = graph.free_list_snapshot();
        let mut unique = free_list.clone();
        unique.sort_unstable();
        unique.dedup();

        assert_eq!(free_list.len(), unique.len());
        assert!(free_list
            .iter()
            .all(|index| (*index as usize) < graph.arena_capacity()));
        assert!(graph.is_alive(anchor));
        assert_eq!(graph.active_node_count(), 1);
    }
}

#[test]
fn slot_reuse_after_unregister_does_not_inherit_stale_subscribers() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.create_node();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    graph.unregister_node(source).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);

    let replacement = graph.create_node();
    assert_eq!(replacement.index(), source.index());
    assert!(graph.subscribers_of(replacement).unwrap().is_empty());

    mark_dirty(&mut graph, replacement, ASPECT_A).unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn rebuild_subscriber_index_after_slot_reuse_matches_live_dependencies_only() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.create_node();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    graph.unregister_node(source).unwrap();
    let replacement = graph.create_node();
    graph.rebuild_subscriber_index_from_dependencies().unwrap();

    assert_eq!(replacement.index(), source.index());
    assert!(graph.subscribers_of(replacement).unwrap().is_empty());
    assert!(graph.runtime_dependencies_of(dependent).unwrap().is_empty());
}
