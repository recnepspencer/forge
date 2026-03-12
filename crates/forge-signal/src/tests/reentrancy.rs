use crate::data::graph::ScratchLeaseKind;
use crate::facade::*;
use crate::tests::support::*;

#[test]
fn nested_scratch_acquire_returns_structured_error() {
    let mut graph = SignalGraph::new();
    let scratch = graph
        .acquire_scratch(ScratchLeaseKind::Evaluation)
        .expect("first scratch lease should succeed");

    let err = graph
        .acquire_scratch(ScratchLeaseKind::Invalidation)
        .expect_err("nested scratch lease must fail");
    assert!(matches!(
        err,
        SignalError::ScratchReentry {
            active: ScratchLeaseKind::Evaluation,
            attempted: ScratchLeaseKind::Invalidation,
        }
    ));
    assert_eq!(graph.telemetry().storage.scratch_reentry_error_count, 1);

    graph
        .restore_scratch(ScratchLeaseKind::Evaluation, scratch)
        .expect("scratch restore should succeed");
}

#[test]
fn scratch_reentry_failure_leaves_graph_reusable() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    graph.add_dependency(b, a, ASPECT_B).unwrap();

    let scratch = graph.acquire_scratch(ScratchLeaseKind::Evaluation).unwrap();
    assert!(graph.acquire_scratch(ScratchLeaseKind::Gc).is_err());
    graph
        .restore_scratch(ScratchLeaseKind::Evaluation, scratch)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    evaluate(&mut graph, a, &mut compute).unwrap();
    mark_dirty(&mut graph, a, ASPECT_B).unwrap();
}