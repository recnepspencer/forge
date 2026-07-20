use super::*;

#[test]
fn denied_correspondence_batch_commits_no_earlier_dependency() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let first = dependency("query:first");
    let second = dependency("query:second");
    let runtime = runtime(
        exact_mapping(),
        vec![
            registration(
                first.clone(),
                vec![exact_target(&graph, node, Aspect::new(0))],
            ),
            registration(
                second.clone(),
                vec![exact_target(&graph, node, Aspect::new(0))],
            ),
        ],
    );

    let denied = crate::correspondence::prepare_correspondence_batch(
        &runtime,
        vec![first.clone(), second],
        &graph,
    );
    assert!(matches!(
        denied,
        Err(TransitionOutcome::Denied(ref denial))
            if denial.kind()
                == crate::facade::BridgeCorrespondenceDenialKind::SharedSlotRequiresDeclaredWidening
    ));
    let registry = runtime.correspondence_allocations.read().unwrap();
    assert!(registry.is_empty());
    drop(registry);

    assert!(runtime
        .install_semantic_correspondence(first, &graph)
        .is_success());
}
