use super::super::*;

pub(super) struct ResourceMilestoneCRetentionReplayEvidence {
    pub(super) retention_report: ResourceLifecycleRetentionCompactionReport,
    pub(super) replay_availability: ResourceReplayAvailabilityReport,
    pub(super) diagnostics_denial: ResourceDiagnosticsExpansionDenial,
}

pub(super) fn resource_milestone_c_retention_replay_evidence(
) -> ResourceMilestoneCRetentionReplayEvidence {
    let mut replay_graph = SignalGraph::new();
    let first_replay_node = replay_graph.node().build();
    let second_replay_node = replay_graph.node().build();
    let mut replay_runtime = TestRuntime::build(replay_graph);
    replay_runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(
            first_replay_node,
        ))
        .expect("first replay declaration should lower");
    replay_runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(
            second_replay_node,
        ))
        .expect("second replay declaration should lower");
    let first_replay = replay_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_replay_node,
        )))
        .expect("first replay request should admit")
        .admitted_request();
    let second_replay = replay_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_replay_node,
        )))
        .expect("second replay request should admit")
        .admitted_request();
    replay_runtime
        .cancel_resource_request(
            first_replay.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("first replay cancellation should admit");
    replay_runtime
        .cancel_resource_request(
            second_replay.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("second replay cancellation should admit");
    let retention_report =
        replay_runtime.compact_resource_lifecycle_history_with_retained_limit(2, 1);
    let replay_availability = replay_runtime
        .resource_replay_availability(&resource_declaration(first_replay_node))
        .expect("default replay availability should classify");
    let diagnostics_denial = replay_runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-summary-only diagnostics budget should deny cold reconstruction");

    ResourceMilestoneCRetentionReplayEvidence {
        retention_report,
        replay_availability,
        diagnostics_denial,
    }
}
