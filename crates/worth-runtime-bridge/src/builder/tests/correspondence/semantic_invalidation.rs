use super::*;

#[test]
fn whole_aspect_set_and_clear_invalidate_a_field_dependency() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("field dependency correspondence should install")
    };
    let signal_target = correspondence.targets().next().unwrap();
    let aspect = signal_target.aspect();

    for kind in [
        AuthoritativeAspectChangeKind::WholeAspectSet,
        AuthoritativeAspectChangeKind::WholeAspectClear,
    ] {
        let before = graph.node_aspect_version(node).unwrap().get(aspect);
        let TransitionOutcome::Success(counters) = runtime
            .deliver_installed_correspondence_envelope(
                &correspondence,
                &mut graph,
                &whole_aspect_change_envelope(kind),
            )
        else {
            panic!("whole-aspect mutation should invalidate a field dependency")
        };
        assert_eq!(counters.truth_targets_admitted(), 1);
        assert_eq!(counters.signal_seeds_emitted(), 1);
        assert_eq!(
            graph.node_aspect_version(node).unwrap().get(aspect),
            before + 1
        );
    }
}

#[test]
fn record_local_dependency_does_not_widen_an_unidentified_item() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let TransitionOutcome::Success(correspondence) =
        runtime.install_semantic_correspondence(dependency("query:one"), &graph)
    else {
        panic!("record-local dependency correspondence should install")
    };
    let aspect = correspondence.targets().next().unwrap().aspect();
    let before = graph.node_aspect_version(node).unwrap().get(aspect);
    let TransitionOutcome::Success(counters) = runtime.deliver_installed_correspondence_envelope(
        &correspondence,
        &mut graph,
        &unidentified_whole_aspect_envelope(),
    ) else {
        panic!("unmatched descriptive items are a successful no-op")
    };
    assert_eq!(counters.truth_targets_admitted(), 0);
    assert_eq!(counters.signal_seeds_emitted(), 0);
    assert_eq!(graph.node_aspect_version(node).unwrap().get(aspect), before);
}
