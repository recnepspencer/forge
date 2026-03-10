use crate::facade::*;
use crate::tests::support::*;

#[test]
fn evaluation_context_tracks_deps() {
    let mut graph = SignalGraph::new();
    let upstream_a = graph.node().build();
    let upstream_b = graph.node().build();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, upstream_a, &mut compute).unwrap();
    evaluate(&mut graph, upstream_b, &mut compute).unwrap();

    let evaluating = graph.node().build();
    let mut ctx = EvaluationContext::new(evaluating);

    let ver_a = ctx.read(&graph, upstream_a, ASPECT_A).unwrap();
    assert_eq!(ver_a, 1);

    let ver_b = ctx.read(&graph, upstream_b, ASPECT_B).unwrap();
    assert_eq!(ver_b, 1);

    ctx.read(&graph, upstream_a, ASPECT_A).unwrap();

    let deps = ctx.finalize();
    assert_eq!(
        deps.len(),
        2,
        "Duplicate reads should not create duplicate deps"
    );
    assert_eq!(deps[0].source(), upstream_a);
    assert_eq!(deps[0].aspect(), ASPECT_A);
    assert_eq!(deps[1].source(), upstream_b);
    assert_eq!(deps[1].aspect(), ASPECT_B);
}

#[test]
fn evaluation_context_read_is_aspect_specific() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().build();

    evaluate(&mut graph, upstream, &mut |_id, _g: &SignalGraph| {
        Ok(version_ab(7, 13))
    })
    .unwrap();

    let evaluating = graph.node().build();
    let mut ctx = EvaluationContext::new(evaluating);

    let version_a = ctx.read(&graph, upstream, ASPECT_A).unwrap();
    let version_b = ctx.read(&graph, upstream, ASPECT_B).unwrap();

    assert_eq!(version_a, 7);
    assert_eq!(version_b, 13);
}
