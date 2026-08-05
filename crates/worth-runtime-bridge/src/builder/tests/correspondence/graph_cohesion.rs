use super::*;

#[test]
fn one_bridge_runtime_cannot_own_two_executable_signal_graphs() {
    let mut first_graph = SignalGraph::new();
    let first_node = first_graph.node().build();
    let mut second_graph = SignalGraph::new();
    let second_node = second_graph.node().build();
    let mapping = exact_mapping();
    let aspect_mapping = aspect_mapping(&mapping);
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_mapping(mapping)
        .register_aspect_mapping(aspect_mapping)
        .register_semantic_correspondence(registration(
            dependency("query:first"),
            vec![target(&first_graph, first_node)],
        ))
        .register_semantic_correspondence(registration(
            dependency("query:second"),
            vec![target(&second_graph, second_node)],
        ))
        .build()
        .expect_err("one Bridge runtime must represent one executable Signal graph");
    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::MixedSemanticCorrespondenceSignalGraphs
    );
}

#[test]
fn graph_binding_rejects_a_cloned_signal_graph_at_the_seam() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );
    let mut cloned_graph = graph.clone();
    assert!(matches!(
        runtime.bind_signal_graph(&mut cloned_graph),
        Err(crate::facade::BridgeCorrespondenceRebindRequired::SignalGraphGeneration)
    ));
}

#[test]
fn one_signal_graph_rejects_a_second_bridge_allocation_authority() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let registration = || registration(dependency("query:one"), vec![target(&graph, node)]);
    let first = runtime(exact_mapping(), vec![registration()]);
    let first_clone = first.clone();
    let second = runtime(exact_mapping(), vec![registration()]);

    {
        let _binding = first.bind_signal_graph(&mut graph).unwrap();
    }
    assert!(matches!(
        second.bind_signal_graph(&mut graph),
        Err(crate::facade::BridgeCorrespondenceRebindRequired::SignalGraphLoweringOwner)
    ));
    assert!(first_clone.bind_signal_graph(&mut graph).is_ok());
}

#[test]
fn bridge_without_correspondence_allocations_cannot_monopolize_a_signal_graph() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let empty = runtime(exact_mapping(), Vec::new());
    let owner = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );

    {
        let _binding = empty.bind_signal_graph(&mut graph).unwrap();
    }
    assert!(owner.bind_signal_graph(&mut graph).is_ok());
}
