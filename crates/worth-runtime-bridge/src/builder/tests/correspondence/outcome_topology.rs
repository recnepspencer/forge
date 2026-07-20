use super::*;

#[test]
fn admission_distinguishes_rebindable_and_mixed_signal_graphs() {
    let mut first_graph = SignalGraph::new();
    let first = first_graph.node().build();
    let mut second_graph = SignalGraph::new();
    let second = second_graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&first_graph, first)],
        )],
    );

    assert!(matches!(
        runtime.install_semantic_correspondence(dependency("query:one"), &second_graph),
        TransitionOutcome::RebindRequired(
            crate::facade::BridgeCorrespondenceRebindRequired::SignalGraphGeneration
        )
    ));
    let mixed = BridgeSemanticCorrespondenceRegistration::new(
        dependency("query:one"),
        vec![target(&first_graph, first), target(&second_graph, second)],
    )
    .expect_err("mixed graph registration must deny before runtime construction");
    assert_eq!(
        mixed.kind(),
        crate::facade::BridgeCorrespondenceDenialKind::MixedGraphTargetSet
    );
}

#[test]
fn admission_exposes_real_deferred_and_failed_outcomes() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let runtime = runtime(
        exact_mapping(),
        vec![registration(
            dependency("query:one"),
            vec![target(&graph, node)],
        )],
    );

    let guard = runtime.correspondence_allocations.write().unwrap();
    assert!(matches!(
        runtime.install_semantic_correspondence(dependency("query:one"), &graph),
        TransitionOutcome::Deferred(
            crate::facade::BridgeCorrespondenceDeferred::GraphMutationInProgress
        )
    ));
    drop(guard);

    let allocations = std::sync::Arc::clone(&runtime.correspondence_allocations);
    std::thread::spawn(move || {
        let _guard = allocations.write().unwrap();
        panic!("poison correspondence allocation lock");
    })
    .join()
    .expect_err("the lock-poisoning worker must panic");
    assert!(matches!(
        runtime.install_semantic_correspondence(dependency("query:one"), &graph),
        TransitionOutcome::Failed(
            crate::facade::BridgeCorrespondenceAdmissionFailure::LockPoisoned
        )
    ));
}
