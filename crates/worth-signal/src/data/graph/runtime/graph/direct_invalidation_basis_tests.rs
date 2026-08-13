use super::cause_sets_tests::{graph_with_edge, publish_delta};
use super::SignalGraph;
use crate::data::aspect::{Aspect, AspectMask, AspectVersion};
use crate::data::node::NodeState;
use crate::data::output::PartitionSubscription;
use crate::data::proof::invalidation::source_seed::DirectInvalidationBasis;
use crate::facade::mark_dirty;
use crate::tests::support::evaluate;

#[test]
fn resolved_invalidation_rejects_cache_drift_and_derives_from_causes() {
    let (mut graph, producer, consumer, aspect) = graph_with_edge();
    publish_delta(&mut graph, producer, aspect, 0, 1, 1);
    graph
        .get_entry_mut(consumer)
        .unwrap()
        .set_dirty_aspects(AspectMask::EMPTY);

    assert!(graph.node_invalidation_input(consumer).is_err());
    graph
        .rebuild_dirty_caches_from_pending_causes(consumer)
        .unwrap();
    assert_eq!(
        graph
            .node_invalidation_input(consumer)
            .unwrap()
            .resolved_dirty_aspects(),
        Some(AspectMask::from_aspect(aspect))
    );
}

#[test]
fn checkpoint_readmission_preserves_unrelated_direct_recompute_basis() {
    let (mut graph, producer, consumer, aspect) = graph_with_edge();
    publish_delta(&mut graph, producer, aspect, 0, 1, 1);
    let source = graph.node().produces_aspects(Aspect::new(5)).build();
    let mut baseline = |_id, _graph: &SignalGraph| Ok(AspectVersion::zero());
    evaluate(&mut graph, source, &mut baseline).unwrap();
    mark_dirty(&mut graph, source, Aspect::new(5)).unwrap();

    let restored = SignalGraph::restore_from_checkpoint_image(&checkpoint_image(&graph)).unwrap();

    assert_eq!(restored.pending_causes(consumer).unwrap().len(), 1);
    assert_eq!(
        restored
            .node_invalidation_input(source)
            .unwrap()
            .resolved_dirty_aspects(),
        Some(AspectMask::from_aspect(Aspect::new(5)))
    );
}

#[test]
fn cause_free_dirty_cache_cannot_mint_direct_recompute_authority() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut baseline = |_id, _graph: &SignalGraph| Ok(AspectVersion::zero());
    evaluate(&mut graph, node, &mut baseline).unwrap();
    graph
        .get_entry_mut(node)
        .unwrap()
        .set_state(NodeState::Dirty);
    graph
        .get_entry_mut(node)
        .unwrap()
        .set_dirty_aspects(AspectMask::from_aspect(Aspect::new(3)));

    assert!(graph.node_invalidation_input(node).is_err());
    assert!(SignalGraph::restore_from_checkpoint_image(&checkpoint_image(&graph)).is_err());
}

#[test]
fn checkpoint_rejects_aspect_scope_pair_forgery() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut baseline = |_id, _graph: &SignalGraph| Ok(AspectVersion::zero());
    evaluate(&mut graph, node, &mut baseline).unwrap();
    let aspect_a = Aspect::new(1);
    let aspect_b = Aspect::new(2);
    let scope_y = PartitionSubscription::whole_partition("curve-y");
    graph
        .transition_node_dirty(
            node,
            aspect_a,
            &[PartitionSubscription::whole_partition("curve-x")],
        )
        .unwrap();

    let mut image = checkpoint_image(&graph);
    let slot = &mut image.authority.arena.slots[node.index() as usize];
    let mut parts = slot.node.take().unwrap().into_parts();
    parts.dirty_aspects = AspectMask::from_aspect(aspect_a);
    parts.dirty_partition_scopes = vec![(aspect_b, scope_y.clone())];
    parts.direct_invalidation_basis = Some(DirectInvalidationBasis::SourceRecompute {
        dirty_aspects: AspectMask::from_aspect(aspect_a),
        scoped_aspects: vec![(aspect_b, scope_y)],
    });
    slot.node = Some(crate::data::node::CheckpointNodeImage::from_parts(parts));

    assert!(SignalGraph::restore_from_checkpoint_image(&image).is_err());
}

#[test]
fn checkpoint_rejects_direct_basis_on_clean_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph
        .transition_node_dirty(node, Aspect::new(1), &[])
        .unwrap();
    let mut image = checkpoint_image(&graph);
    let slot = &mut image.authority.arena.slots[node.index() as usize];
    let mut parts = slot.node.take().unwrap().into_parts();
    parts.state = NodeState::Clean;
    slot.node = Some(crate::data::node::CheckpointNodeImage::from_parts(parts));

    assert!(SignalGraph::restore_from_checkpoint_image(&image).is_err());
}

#[test]
fn checkpoint_rejects_empty_source_recompute_basis() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph
        .transition_node_dirty(node, Aspect::new(1), &[])
        .unwrap();
    let mut image = checkpoint_image(&graph);
    let slot = &mut image.authority.arena.slots[node.index() as usize];
    let mut parts = slot.node.take().unwrap().into_parts();
    parts.dirty_aspects = AspectMask::EMPTY;
    parts.dirty_partition_scopes.clear();
    parts.direct_invalidation_basis = Some(DirectInvalidationBasis::SourceRecompute {
        dirty_aspects: AspectMask::EMPTY,
        scoped_aspects: Vec::new(),
    });
    slot.node = Some(crate::data::node::CheckpointNodeImage::from_parts(parts));

    assert!(SignalGraph::restore_from_checkpoint_image(&image).is_err());
}

#[test]
fn checkpoint_rejects_valid_dependency_cause_on_clean_consumer() {
    let (mut graph, producer, consumer, aspect) = graph_with_edge();
    publish_delta(&mut graph, producer, aspect, 0, 1, 1);
    let mut image = checkpoint_image(&graph);
    let slot = &mut image.authority.arena.slots[consumer.index() as usize];
    let mut parts = slot.node.take().unwrap().into_parts();
    parts.state = NodeState::Clean;
    slot.node = Some(crate::data::node::CheckpointNodeImage::from_parts(parts));

    assert!(SignalGraph::restore_from_checkpoint_image(&image).is_err());
}

#[test]
fn locality_admission_does_not_cross_aspects_with_unrelated_scopes() {
    let aspect_a = Aspect::new(1);
    let aspect_b = Aspect::new(2);
    let scope_x = PartitionSubscription::whole_partition("curve-x");
    let scope_y = PartitionSubscription::whole_partition("curve-y");
    let contract = crate::data::node::NodeContract::reads(AspectMask::from_aspect(aspect_a))
        .with_partition_scope(scope_y.clone());
    let changes = vec![(aspect_a, scope_x), (aspect_b, scope_y.clone())];

    assert!(
        !contract.cares_about_correlated_change(AspectMask::from([aspect_a, aspect_b]), &changes,)
    );
    assert!(contract.cares_about_correlated_change(
        AspectMask::from([aspect_a, aspect_b]),
        &[(aspect_a, scope_y)],
    ));
}

#[test]
fn whole_aspect_change_remains_stronger_than_scoped_contract_locality() {
    let aspect_a = Aspect::new(1);
    let aspect_b = Aspect::new(2);
    let contract = crate::data::node::NodeContract::reads(AspectMask::from_aspect(aspect_a))
        .with_partition_scope(PartitionSubscription::whole_partition("curve-y"));

    assert!(contract.cares_about_correlated_change(
        AspectMask::from([aspect_a, aspect_b]),
        &[(aspect_b, PartitionSubscription::whole_partition("curve-y"),)],
    ));
}

fn checkpoint_image(graph: &SignalGraph) -> crate::state::SignalCheckpointImage {
    crate::state::SignalCheckpointImage {
        authority: graph.capture_checkpoint_authority(),
        dependency_snapshot_batch: graph.capture_checkpoint_dependency_snapshot_batch(),
        graph_telemetry: *graph.telemetry(),
    }
}
