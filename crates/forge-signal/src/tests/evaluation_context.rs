use crate::facade::*;

#[test]
fn evaluation_context_tracks_deps() {
    let mut graph = SignalGraph::new();
    let upstream_a = graph.create_node();
    let upstream_b = graph.create_node();

    let mut compute = |_id, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));
    evaluate(&mut graph, upstream_a, &mut compute).unwrap();
    evaluate(&mut graph, upstream_b, &mut compute).unwrap();

    let evaluating = graph.create_node();
    let mut ctx = EvaluationContext::new(evaluating);

    let ver_a = ctx.read(&graph, upstream_a, Aspect::Topology).unwrap();
    assert_eq!(ver_a.topology(), 1);

    let ver_b = ctx.read(&graph, upstream_b, Aspect::Geometry).unwrap();
    assert_eq!(ver_b.geometry(), 1);

    ctx.read(&graph, upstream_a, Aspect::Topology).unwrap();

    let deps = ctx.finalize();
    assert_eq!(
        deps.len(),
        2,
        "Duplicate reads should not create duplicate deps"
    );
    assert_eq!(deps[0].source(), upstream_a);
    assert_eq!(deps[0].aspect(), Aspect::Topology);
    assert_eq!(deps[1].source(), upstream_b);
    assert_eq!(deps[1].aspect(), Aspect::Geometry);
}
