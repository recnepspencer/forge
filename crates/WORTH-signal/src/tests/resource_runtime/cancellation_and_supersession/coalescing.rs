use super::*;

#[test]
fn resource_intent_equivalence_coalescing_preserves_winner_and_loser_lineage() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(intent_equivalent_coalescing_resource_declaration(node))
        .expect("coalescing declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let winner = first.admitted_request();
    let coalesced = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("equivalent second request should coalesce");
    let record = coalesced
        .intent_equivalence_coalescing()
        .expect("coalescing policy should retain explicit winner/loser evidence");
    let loser = record.coalesced_request();

    assert_eq!(coalesced.admitted_request(), winner);
    assert!(coalesced.supersession_record().is_none());
    assert_eq!(record.winner(), winner.handle());
    assert_ne!(loser.handle(), winner.handle());
    assert_eq!(
        record.lifecycle_transition().kind(),
        ResourceLifecycleTransitionKind::RequestSuperseded
    );
    assert_eq!(
        record.policy_decision_digest().as_str(),
        runtime
            .resource_descriptor_for_node(ResourceNodeId::from_node(node))
            .expect("descriptor should remain declared")
            .supersession_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert!(
        record
            .intent_digest()
            .as_str()
            .starts_with("resource-intent:"),
        "coalescing evidence should retain canonical intent digest truth"
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(loser.handle())
            .expect("coalesced loser should remain retained for late-completion denial")
            .status(),
        ResourceInFlightStatus::Superseded
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_intent_equivalence_coalescing_count,
        1
    );
}

#[test]
fn resource_intent_equivalence_coalescing_denies_late_completion_for_coalesced_loser() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(intent_equivalent_coalescing_resource_declaration(node))
        .expect("coalescing declaration should lower");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("winner request should admit");
    let coalesced = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("equivalent loser should coalesce");
    let loser = coalesced
        .intent_equivalence_coalescing()
        .expect("coalescing evidence should exist")
        .coalesced_request();
    let late = raw_completion(&runtime, node, loser.handle(), loser.attempt(), 64);

    let report = runtime.admit_resource_completion(late);
    let denied = report
        .denied_completion()
        .expect("late completion for coalesced loser should be denied");

    assert_eq!(denied.class(), CompletionDenialClass::Superseded);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_superseded_completion_denial_count,
        1
    );
}
