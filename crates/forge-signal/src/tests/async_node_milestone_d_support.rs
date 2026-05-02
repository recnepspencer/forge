use crate::facade::*;
use crate::tests::async_node_nightmare_support::milestone_d_combined_workload;
use crate::tests::async_node_public_hierarchy_branch_support::public_hierarchy_branch_workload;
use crate::tests::async_node_support::{
    async_node_capability_declaration, async_node_capability_with_dependents,
    AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt};

pub(crate) struct MilestoneDCertificationInputs {
    pub(crate) attachment_equivalence: AsyncNodeCapabilityEquivalenceReport,
    pub(crate) condition_blocked_request: AsyncNodeRequestAdmissionReport,
    pub(crate) aspect_keyed_historical: AsyncKeyedNodeHistoricalParityReport,
    pub(crate) aspect_keyed_equivalence: AsyncKeyedNodeCapabilityEquivalenceReport,
    pub(crate) previous_value_blocked_request: AsyncNodeRequestAdmissionReport,
    pub(crate) temporal_blocked_request: AsyncNodeRequestAdmissionReport,
    pub(crate) gate_state: AsyncNodeGateStateReport,
    pub(crate) gate_historical_parity: AsyncNodeHistoricalParityReport,
    pub(crate) hierarchy_replay: AsyncNodeHierarchyReplaySummary,
    pub(crate) hierarchy_cancellation: AsyncNodeHierarchyCancellationReport,
    pub(crate) hierarchy_historical_parity: AsyncNodeHierarchyHistoricalParityReport,
    pub(crate) compile_time_boundary: AsyncNodeCompileTimeBoundaryProof,
}

impl MilestoneDCertificationInputs {
    pub(crate) fn scenario_inputs(&self) -> AsyncNodeMilestoneDScenarioInputs<'_> {
        AsyncNodeMilestoneDScenarioInputs {
            attachment_equivalence: &self.attachment_equivalence,
            condition_blocked_request: &self.condition_blocked_request,
            aspect_keyed_historical: &self.aspect_keyed_historical,
            aspect_keyed_equivalence: &self.aspect_keyed_equivalence,
            previous_value_blocked_request: &self.previous_value_blocked_request,
            temporal_blocked_request: &self.temporal_blocked_request,
            gate_state: &self.gate_state,
            gate_historical_parity: &self.gate_historical_parity,
            hierarchy_replay: &self.hierarchy_replay,
            hierarchy_cancellation: &self.hierarchy_cancellation,
            hierarchy_historical_parity: &self.hierarchy_historical_parity,
            compile_time_boundary: &self.compile_time_boundary,
        }
    }
}

pub(crate) fn milestone_d_certification_inputs() -> MilestoneDCertificationInputs {
    let combined = milestone_d_combined_workload();
    let public_hierarchy = public_hierarchy_branch_workload();
    MilestoneDCertificationInputs {
        attachment_equivalence: combined.attachment_equivalence_after_restore,
        condition_blocked_request: condition_blocked_request_report(),
        aspect_keyed_historical: combined.aspect_keyed_historical,
        aspect_keyed_equivalence: combined.aspect_keyed_equivalence,
        previous_value_blocked_request: previous_value_blocked_request_report(),
        temporal_blocked_request: temporal_blocked_request_report(),
        gate_state: public_hierarchy.feature_gate_restored_state,
        gate_historical_parity: public_hierarchy.feature_gate_restored_history,
        hierarchy_replay: combined.hierarchy_replay_after_cancellation,
        hierarchy_cancellation: combined.hierarchy_cancellation,
        hierarchy_historical_parity: combined.hierarchy_historical_after_cancellation,
        compile_time_boundary: async_node_compile_time_boundary_proof(
            REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES,
        )
        .expect("compile-time proof should accept the required async-node fixtures"),
    }
}

pub(crate) fn mismatched_gate_reports(
) -> (AsyncNodeGateStateReport, AsyncNodeHistoricalParityReport) {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    graph
        .append_dependency(child, parent, Aspect::new(0))
        .expect("dependency should wire");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .attach_async_capability(async_node_capability_with_dependents(parent, [child]))
        .expect("parent capability should attach");
    let child_handle = runtime
        .attach_async_capability(async_node_capability_declaration(child))
        .expect("child capability should attach");
    let gate = runtime
        .async_node_gate_state_report(parent)
        .expect("gate report should materialize");
    let child_history = runtime
        .async_node_historical_parity_report(
            &child_handle,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("child history should materialize");
    (gate, child_history)
}

fn condition_blocked_request_report() -> AsyncNodeRequestAdmissionReport {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph
        .node()
        .aspect_filter(AspectMask::from_aspect(Aspect::new(0)))
        .build();
    graph
        .append_dependency(node, source, Aspect::new(1))
        .expect("dependency should wire");
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut source_v1).expect("source should evaluate");
    evaluate(&mut graph, node, &mut node_v1).expect("node should evaluate");
    mark_dirty(&mut graph, source, Aspect::new(1)).expect("dirty source should propagate");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("capability should attach");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("blocked request should still produce a report")
}

fn temporal_blocked_request_report() -> AsyncNodeRequestAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .after(10)
        .expect("valid after condition")
        .build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("capability should attach");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("temporal block should still produce a report")
}

fn previous_value_blocked_request_report() -> AsyncNodeRequestAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut node_v1).expect("node should evaluate");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("capability should attach");
    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
            TemporalCondition::after(1).expect("valid delay"),
            ClockTick::new(1),
        )
        .expect("wake should schedule");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should advance");
    runtime
        .promote_temporal_wake_ready(wake.id())
        .expect("wake should become ready");
    let access = runtime
        .grant_temporal_previous_value_access(wake.id())
        .expect("ready wake should grant previous-value access");
    let reference = runtime
        .previous_temporal_value(&access, node)
        .expect("previous-value reference should capture");
    mark_dirty(runtime.graph_mut(), node, Aspect::new(0)).expect("dirty node should change");
    let mut node_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    evaluate(runtime.graph_mut(), node, &mut node_v2).expect("node should drift");
    runtime
        .admit_async_node_request(
            AsyncNodeRequestIntent::new(node).with_previous_value_reference(reference),
        )
        .expect("previous-value drift should still produce a report")
}
