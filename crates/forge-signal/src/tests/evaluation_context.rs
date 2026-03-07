use crate::facade::*;
use crate::tests::support::*;

#[test]
fn evaluation_context_tracks_deps() {
    let mut graph = SignalGraph::new();
    let upstream_a = graph.create_node();
    let upstream_b = graph.create_node();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, upstream_a, &mut compute).unwrap();
    evaluate(&mut graph, upstream_b, &mut compute).unwrap();

    let evaluating = graph.create_node();
    let mut ctx = EvaluationContext::new(evaluating);

    let ver_a = ctx.read(&graph, upstream_a, ASPECT_A).unwrap();
    assert_eq!(ver_a.get(ASPECT_A), 1);

    let ver_b = ctx.read(&graph, upstream_b, ASPECT_B).unwrap();
    assert_eq!(ver_b.get(ASPECT_B), 1);

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
