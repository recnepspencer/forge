use super::super::support::*;
use super::super::*;

pub(in crate::tests::resource_runtime) fn resource_malformed_completion_report(
) -> ResourceCompletionAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted.handle();
    runtime.admit_resource_completion(RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        admitted.attempt(),
        ResourcePayloadContractDigest::new("payload-contract:999:1024"),
        64,
    ))
}

pub(in crate::tests::resource_runtime) fn resource_diagnostics_summary_for_budget(
    budget: ResourceDiagnosticsExpansionBudget,
) -> ResourceDiagnosticsSummary {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    runtime
        .try_resource_diagnostics_summary(budget)
        .expect("budget should admit descriptor plus lifecycle reconstruction")
}

pub(in crate::tests::resource_runtime) fn resource_diagnostics_summary_for_unknown_completion(
    request_id: ResourceRequestId,
) -> ResourceDiagnosticsSummary {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            request_id,
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown completion should retain denial provenance");

    runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction()
}
