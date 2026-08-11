use super::*;

#[test]
fn resource_replay_availability_budget_history_denial_is_distinct_from_restore_compatibility_denial(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut strict_runtime = TestRuntime::build(graph.clone());
    strict_runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = strict_runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();
    for request_id in [ResourceRequestId::new(930), ResourceRequestId::new(931)] {
        strict_runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(strict_runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }
    strict_runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );
    let budget_history_denied = strict_runtime
        .resource_replay_availability(&deny_on_unknown_or_missing_replay_resource_declaration(
            node,
        ))
        .expect("strict replay availability should classify");

    let mut restore_runtime = TestRuntime::build(graph);
    restore_runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("timeout declaration should lower");
    let restore_denied = restore_runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &timeout_resource_declaration(node, 9),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("restore-denied replay availability should classify");

    assert_eq!(
        budget_history_denied.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert_eq!(
        restore_denied.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::RestoreCompatibilityDenied)
    );
    assert_ne!(
        budget_history_denied.availability_digest(),
        restore_denied.availability_digest(),
        "budget-history denial and restore-compatibility denial must remain distinct"
    );
}
