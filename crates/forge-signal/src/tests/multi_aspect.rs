use crate::facade::*;
use crate::tests::support::*;

#[test]
fn kv60_geometry_change_skips_topo_dependents() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let topo_sub = graph.node().build();

    graph
        .add_dependency(topo_sub, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, topo_sub, &mut compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let state = graph.get_state(topo_sub).unwrap();
    assert_eq!(
        state,
        NodeState::MaybeStale,
        "Topo-only subscriber should be MaybeStale (not Dirty) on geometry change"
    );

    let mut same_version_compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 2));
    evaluate(&mut graph, source, &mut same_version_compute).unwrap();

    let mut eval_ran = false;
    let mut topo_sub_compute = |_id, _g: &SignalGraph| {
        eval_ran = true;
        Ok(version_ab(1, 1))
    };
    evaluate(&mut graph, topo_sub, &mut topo_sub_compute).unwrap();

    assert!(
        !eval_ran,
        "Topo subscriber should NOT have re-evaluated when only geometry changed"
    );
}

#[test]
fn kv60_topology_change_triggers_topo_dependents() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let topo_sub = graph.node().build();

    graph
        .add_dependency(topo_sub, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id, _g: &SignalGraph| Ok(version_ab(1, 1));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, topo_sub, &mut compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    let state = graph.get_state(topo_sub).unwrap();
    assert_eq!(
        state,
        NodeState::Dirty,
        "Topo subscriber must be Dirty on topology change"
    );
}

#[test]
fn aspect_independent_versioning() {
    let ver = version_ab(5, 10);
    let bumped_topo = ver.bump(ASPECT_A);
    let bumped_geom = ver.bump(ASPECT_B);

    assert_eq!(bumped_topo.get(ASPECT_A), 6);
    assert_eq!(bumped_topo.get(ASPECT_B), 10);

    assert_eq!(bumped_geom.get(ASPECT_A), 5);
    assert_eq!(bumped_geom.get(ASPECT_B), 11);
}
