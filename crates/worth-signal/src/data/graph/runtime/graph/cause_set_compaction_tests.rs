use super::cause_sets_tests::{graph_with_edge, publish_delta};
use super::SignalGraph;
use crate::data::proof::invalidation::binding::OutputCommitOrdinal;

#[test]
fn sparse_large_graph_reclaims_last_cause_without_an_arena_census() {
    let (mut graph, producer, consumer, aspect) = graph_with_edge();
    for _ in 0..100_000 {
        graph.create_node();
    }

    publish_delta(&mut graph, producer, aspect, 0, 1, 1);
    assert_eq!(graph.cause_sets.allocated_slot_count(), 1);
    assert_eq!(graph.cause_sets.occupied_slot_count(), 1);

    graph.release_pending_causes(consumer).unwrap();

    assert_eq!(graph.cause_sets.allocated_slot_count(), 0);
    assert_eq!(graph.cause_sets.occupied_slot_count(), 0);
    assert_eq!(graph.cause_sets.last_compaction_slot_visits(), 1);
}

#[test]
fn retiring_dirty_consumer_releases_cause_authority_before_checkpoint() {
    let (mut graph, producer, consumer, aspect) = graph_with_edge();
    publish_delta(&mut graph, producer, aspect, 0, 1, 1);
    let ordinal = graph.cause_sets.output_commit_ordinal_for_test();

    assert_eq!(graph.cause_sets.occupied_slot_count(), 1);
    assert!(graph
        .cause_sets
        .published_output_commit(OutputCommitOrdinal(ordinal))
        .is_some());

    graph.unregister_node(consumer).unwrap();

    assert_eq!(graph.cause_sets.occupied_slot_count(), 0);
    assert_eq!(graph.cause_sets.allocated_slot_count(), 0);
    assert!(graph
        .cause_sets
        .published_output_commit(OutputCommitOrdinal(ordinal))
        .is_none());
    let authority = graph.capture_checkpoint_authority();
    SignalGraph::restore_from_checkpoint_authority(&authority).unwrap();
}
