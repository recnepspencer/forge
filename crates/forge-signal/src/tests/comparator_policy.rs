use crate::facade::*;
use crate::tests::support::*;

#[test]
fn exact_comparator_detects_any_change() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    graph.add_dependency(b, a, ASPECT_B).unwrap();

    let mut compute = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 1));
    evaluate(&mut graph, a, &mut compute).unwrap();
    evaluate(&mut graph, b, &mut compute).unwrap();

    mark_dirty(&mut graph, a, ASPECT_B).unwrap();
    evaluate(&mut graph, b, &mut compute).unwrap();
    assert_eq!(graph.get_state(b).unwrap(), NodeState::Clean);
}

#[test]
fn tolerance_comparator_skips_small_version_delta() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().tolerance(2).build();
    graph.add_dependency(b, a, ASPECT_B).unwrap();
    graph.add_dependency(c, b, ASPECT_B).unwrap();

    let mut compute_a_v10 = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 10));
    let mut compute_a_v12 = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 12));
    let mut compute_b = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 100));

    evaluate(&mut graph, a, &mut compute_a_v10).unwrap();
    evaluate(&mut graph, b, &mut compute_b).unwrap();
    let mut compute_c = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 1_000));
    evaluate(&mut graph, c, &mut compute_c).unwrap();

    mark_dirty(&mut graph, a, ASPECT_B).unwrap();
    evaluate(&mut graph, a, &mut compute_a_v12).unwrap();
    evaluate(&mut graph, b, &mut compute_b).unwrap();
    evaluate(&mut graph, c, &mut compute_c).unwrap();

    assert!(graph.telemetry().evaluation.skipped_by_comparator >= 1);
}

struct ForceChangeResolver;
impl VersionComparatorResolver for ForceChangeResolver {
    fn resolve(
        &mut self,
        _key: &str,
        _aspect: Aspect,
        _cached: u64,
        _current: u64,
    ) -> Result<bool, SignalError> {
        Ok(true)
    }
}

#[test]
fn custom_comparator_uses_resolver() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph
        .node()
        .comparator(VersionComparatorPolicy::Custom {
            key: "force-change".to_string(),
        })
        .build();
    graph.add_dependency(b, a, ASPECT_B).unwrap();

    let mut compute = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(0, 1));
    evaluate_with_resolver(&mut graph, a, &mut compute, &mut ForceChangeResolver).unwrap();
    evaluate_with_resolver(&mut graph, b, &mut compute, &mut ForceChangeResolver).unwrap();

    mark_dirty(&mut graph, a, ASPECT_B).unwrap();
    evaluate_with_resolver(&mut graph, b, &mut compute, &mut ForceChangeResolver).unwrap();
    assert_eq!(graph.get_state(b).unwrap(), NodeState::Clean);
}