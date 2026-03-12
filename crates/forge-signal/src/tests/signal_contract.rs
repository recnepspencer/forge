use crate::facade::*;
use crate::tests::support::*;
use std::cell::Cell;

#[test]
fn add_dependency_is_idempotent_per_aspect() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().build();
    let downstream = graph.node().build();

    graph
        .add_dependency(downstream, upstream, ASPECT_A)
        .expect("initial dependency should be accepted");
    graph
        .add_dependency(downstream, upstream, ASPECT_A)
        .expect("duplicate dependency should be ignored");

    let downstream_dependencies = graph
        .dependencies_of(downstream)
        .expect("downstream dependencies should exist");
    assert_eq!(downstream_dependencies.len(), 1);

    let upstream_subscribers = graph
        .subscribers_of(upstream)
        .expect("upstream subscribers should exist");
    assert_eq!(upstream_subscribers.len(), 1);
    assert_eq!(upstream_subscribers[0], downstream);
}

#[test]
fn remove_dependency_preserves_other_aspects_between_same_nodes() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().build();
    let downstream = graph.node().build();

    graph
        .add_dependency(downstream, upstream, ASPECT_A)
        .unwrap();
    graph
        .add_dependency(downstream, upstream, ASPECT_B)
        .unwrap();

    graph
        .remove_dependency(downstream, upstream, ASPECT_A)
        .expect("aspect-specific removal should succeed");

    let downstream_dependencies = graph
        .dependencies_of(downstream)
        .expect("downstream dependencies should exist");
    assert_eq!(downstream_dependencies.len(), 1);
    assert_eq!(downstream_dependencies[0].aspect(), ASPECT_B);

    let upstream_subscribers = graph
        .subscribers_of(upstream)
        .expect("upstream subscribers should exist");
    assert_eq!(upstream_subscribers, &[downstream]);
}

#[test]
fn subscriber_fanout_does_not_duplicate_across_aspects() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().build();
    let downstream = graph.node().build();

    graph
        .add_dependency(downstream, upstream, ASPECT_A)
        .unwrap();
    graph
        .add_dependency(downstream, upstream, ASPECT_B)
        .unwrap();

    let subscribers = graph
        .subscribers_of(upstream)
        .expect("upstream subscribers should exist");
    assert_eq!(subscribers, &[downstream]);
}

#[test]
fn version_gated_skip_respects_repeated_monotonic_bumps() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();

    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut next_version = 0_u64;
    let mut compute_source = |_id, _graph: &SignalGraph| {
        next_version += 1;
        Ok(AspectVersion::zero().with(ASPECT_A, next_version))
    };
    let mut compute_dependent =
        |_id, _graph: &SignalGraph| Ok(AspectVersion::zero().with(ASPECT_A, 1));

    evaluate(&mut graph, source, &mut compute_source).unwrap();
    evaluate(&mut graph, dependent, &mut compute_dependent).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut compute_source).unwrap();

    let dependent_recomputes = Cell::new(0_u64);
    let mut recompute_dependent = |_id, _graph: &SignalGraph| {
        let next = dependent_recomputes.get() + 1;
        dependent_recomputes.set(next);
        Ok(AspectVersion::zero().with(ASPECT_A, next))
    };
    evaluate(&mut graph, dependent, &mut recompute_dependent).unwrap();
    assert_eq!(dependent_recomputes.get(), 1);

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut compute_source).unwrap();
    evaluate(&mut graph, dependent, &mut recompute_dependent).unwrap();

    assert_eq!(
        dependent_recomputes.get(),
        2,
        "meaningful upstream version bumps must force repeated dependent recomputation"
    );
}