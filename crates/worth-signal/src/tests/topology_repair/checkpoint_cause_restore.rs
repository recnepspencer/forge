use crate::data::proof::invalidation::binding::{OutputCommitOrdinal, ResolvedDependencyCause};
use crate::data::proof::PartitionScopeSet;
use crate::facade::*;
use crate::tests::support::ASPECT_A;

#[test]
fn checkpoint_dead_edge_repair_with_quarantined_causes_returns_without_panicking() {
    let mut graph = SignalGraph::new();
    let retired_source = graph.node().build();
    let consumer = graph.node().build();
    graph.unregister_node(retired_source).unwrap();
    graph
        .inject_retired_dependency_for_test(consumer, retired_source, ASPECT_A)
        .unwrap();
    graph
        .inject_pending_causes_unchecked_for_test(
            consumer,
            [ResolvedDependencyCause::new(
                graph.runtime_instance_id(),
                consumer,
                graph.dependency_revision(consumer).unwrap(),
                retired_source,
                ASPECT_A,
                None,
                0,
                OutputCommitOrdinal(1),
                1,
                PartitionScopeSet::default(),
            )],
        )
        .unwrap();
    let authority = graph.capture_checkpoint_authority();

    let restored = SignalGraph::restore_from_checkpoint_authority(&authority)
        .expect("dead-edge repair must return typed success or failure, never panic");

    assert!(restored.dependencies_of(consumer).unwrap().is_empty());
    assert!(restored.pending_causes(consumer).unwrap().is_empty());
}
