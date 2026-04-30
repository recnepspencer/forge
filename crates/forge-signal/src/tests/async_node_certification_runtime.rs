use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, async_node_capability_with_dependents,
    AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::{define_keyed_computation, evaluate, version_ab};
use serde_json::{json, Value};

struct NoopAsyncNodeObservationListener;

impl ObservationListener<(), (), (), (), ()> for NoopAsyncNodeObservationListener {
    fn on_observation(
        &self,
        _ctx: ObservationReadContext<'_, (), (), (), (), ()>,
        _notice: &ObservationNotice<'_>,
    ) {
    }
}

#[test]
fn async_node_milestone_d_certification_run_builds_from_real_async_capability_reports() {
    let attachment_equivalence = attachment_equivalence_report();
    let condition_blocked_request = condition_blocked_request_report();
    let (aspect_keyed_historical, aspect_keyed_equivalence) = keyed_closeout_reports();
    let previous_value_blocked_request = previous_value_blocked_request_report();
    let temporal_blocked_request = temporal_blocked_request_report();
    let (gate_state, gate_historical_parity) = gate_closeout_reports();
    let (hierarchy_replay, hierarchy_cancellation, hierarchy_historical_parity) =
        hierarchy_closeout_reports();
    let compile_time_boundary =
        async_node_compile_time_boundary_proof(REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES)
            .expect("compile-time proof should accept the required async-node fixtures");

    let matrix = async_node_milestone_d_scenario_matrix(AsyncNodeMilestoneDScenarioInputs {
        attachment_equivalence: &attachment_equivalence,
        condition_blocked_request: &condition_blocked_request,
        aspect_keyed_historical: &aspect_keyed_historical,
        aspect_keyed_equivalence: &aspect_keyed_equivalence,
        previous_value_blocked_request: &previous_value_blocked_request,
        temporal_blocked_request: &temporal_blocked_request,
        gate_state: &gate_state,
        gate_historical_parity: &gate_historical_parity,
        hierarchy_replay: &hierarchy_replay,
        hierarchy_cancellation: &hierarchy_cancellation,
        hierarchy_historical_parity: &hierarchy_historical_parity,
        compile_time_boundary: &compile_time_boundary,
    })
    .expect("milestone D scenario matrix should build from real runtime reports");
    let closeout = async_node_milestone_d_performance_closeout(&matrix)
        .expect("milestone D performance closeout should build");
    let run = async_node_milestone_d_certification_run(matrix, closeout)
        .expect("milestone D certification run should build");

    assert_eq!(
        run.scenario_matrix().rows().len(),
        REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS.len()
    );
    assert_eq!(
        run.performance_closeout().rows().len(),
        REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS.len()
    );
    assert_eq!(run.summary().direct_blocking_count(), 3);
    assert_eq!(run.summary().combined_suite_count(), 4);
    assert_eq!(
        run.summary().compile_time_fixture_count(),
        REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES.len() as u32
    );
}

#[test]
fn async_node_milestone_d_compile_time_boundary_proof_rejects_missing_required_fixture() {
    let err = async_node_compile_time_boundary_proof([
        "validated_async_node_capability_declaration_fields_are_private",
        "async_capable_node_fields_are_private",
    ])
    .expect_err("compile-time proof must reject missing required fixtures");

    assert!(err
        .to_string()
        .contains("missing required async-node compile-time fixtures"));
}

#[test]
fn async_node_milestone_d_scenario_matrix_rejects_gate_historical_node_mismatch() {
    let attachment_equivalence = attachment_equivalence_report();
    let condition_blocked_request = condition_blocked_request_report();
    let (aspect_keyed_historical, aspect_keyed_equivalence) = keyed_closeout_reports();
    let previous_value_blocked_request = previous_value_blocked_request_report();
    let temporal_blocked_request = temporal_blocked_request_report();
    let (gate_state, gate_historical_parity) = mismatched_gate_reports();
    let (hierarchy_replay, hierarchy_cancellation, hierarchy_historical_parity) =
        hierarchy_closeout_reports();
    let compile_time_boundary =
        async_node_compile_time_boundary_proof(REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES)
            .expect("compile-time proof should accept the required async-node fixtures");

    let err = async_node_milestone_d_scenario_matrix(AsyncNodeMilestoneDScenarioInputs {
        attachment_equivalence: &attachment_equivalence,
        condition_blocked_request: &condition_blocked_request,
        aspect_keyed_historical: &aspect_keyed_historical,
        aspect_keyed_equivalence: &aspect_keyed_equivalence,
        previous_value_blocked_request: &previous_value_blocked_request,
        temporal_blocked_request: &temporal_blocked_request,
        gate_state: &gate_state,
        gate_historical_parity: &gate_historical_parity,
        hierarchy_replay: &hierarchy_replay,
        hierarchy_cancellation: &hierarchy_cancellation,
        hierarchy_historical_parity: &hierarchy_historical_parity,
        compile_time_boundary: &compile_time_boundary,
    })
    .expect_err("matrix must reject mismatched gate/historical lineage");

    assert!(err
        .to_string()
        .contains("matching gate and historical parity nodes"));
}

#[test]
fn async_node_milestone_d_certification_run_rejects_duplicate_scenario_coverage() {
    let run_inputs = milestone_d_certification_inputs();
    let matrix = async_node_milestone_d_scenario_matrix(run_inputs.scenario_inputs())
        .expect("milestone D scenario matrix should build from real runtime reports");
    let closeout = async_node_milestone_d_performance_closeout(&matrix)
        .expect("milestone D performance closeout should build");

    let mut forged_matrix: Value =
        serde_json::to_value(&matrix).expect("matrix should serialize for hostile forgery");
    let rows = forged_matrix["rows"]
        .as_array_mut()
        .expect("matrix rows should serialize as an array");
    rows[0]["scenarioId"] = json!("conditionGatedAsyncAdmissionParity");
    forged_matrix["summary"]["directBlockingCount"] = json!(2);
    forged_matrix["summary"]["combinedSuiteCount"] = json!(5);
    let forged_matrix: AsyncNodeMilestoneDScenarioMatrix =
        serde_json::from_value(forged_matrix).expect("forged matrix should deserialize");

    let err = async_node_milestone_d_certification_run(forged_matrix, closeout)
        .expect_err("certification run must reject duplicate/missing scenario coverage");

    assert!(err.to_string().contains("exact required scenario coverage"));
}

#[test]
fn async_node_milestone_d_certification_run_rejects_forged_performance_envelope() {
    let run_inputs = milestone_d_certification_inputs();
    let matrix = async_node_milestone_d_scenario_matrix(run_inputs.scenario_inputs())
        .expect("milestone D scenario matrix should build from real runtime reports");
    let closeout = async_node_milestone_d_performance_closeout(&matrix)
        .expect("milestone D performance closeout should build");

    let mut forged_closeout: Value =
        serde_json::to_value(&closeout).expect("closeout should serialize for hostile forgery");
    forged_closeout["rows"][0]["boundary"] = json!("SummaryRead");
    forged_closeout["rows"][0]["performance"]["boundary"] = json!("SummaryRead");
    let forged_closeout: AsyncNodeMilestoneDPerformanceCloseout =
        serde_json::from_value(forged_closeout).expect("forged closeout should deserialize");

    let err = async_node_milestone_d_certification_run(matrix, forged_closeout)
        .expect_err("certification run must reject forged performance envelope drift");

    assert!(err
        .to_string()
        .contains("preserve the scenario boundary envelope"));
}

struct MilestoneDCertificationInputs {
    attachment_equivalence: AsyncNodeCapabilityEquivalenceReport,
    condition_blocked_request: AsyncNodeRequestAdmissionReport,
    aspect_keyed_historical: AsyncKeyedNodeHistoricalParityReport,
    aspect_keyed_equivalence: AsyncKeyedNodeCapabilityEquivalenceReport,
    previous_value_blocked_request: AsyncNodeRequestAdmissionReport,
    temporal_blocked_request: AsyncNodeRequestAdmissionReport,
    gate_state: AsyncNodeGateStateReport,
    gate_historical_parity: AsyncNodeHistoricalParityReport,
    hierarchy_replay: AsyncNodeHierarchyReplaySummary,
    hierarchy_cancellation: AsyncNodeHierarchyCancellationReport,
    hierarchy_historical_parity: AsyncNodeHierarchyHistoricalParityReport,
    compile_time_boundary: AsyncNodeCompileTimeBoundaryProof,
}

impl MilestoneDCertificationInputs {
    fn scenario_inputs(&self) -> AsyncNodeMilestoneDScenarioInputs<'_> {
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

fn milestone_d_certification_inputs() -> MilestoneDCertificationInputs {
    let attachment_equivalence = attachment_equivalence_report();
    let condition_blocked_request = condition_blocked_request_report();
    let (aspect_keyed_historical, aspect_keyed_equivalence) = keyed_closeout_reports();
    let previous_value_blocked_request = previous_value_blocked_request_report();
    let temporal_blocked_request = temporal_blocked_request_report();
    let (gate_state, gate_historical_parity) = gate_closeout_reports();
    let (hierarchy_replay, hierarchy_cancellation, hierarchy_historical_parity) =
        hierarchy_closeout_reports();
    let compile_time_boundary =
        async_node_compile_time_boundary_proof(REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES)
            .expect("compile-time proof should accept the required async-node fixtures");

    MilestoneDCertificationInputs {
        attachment_equivalence,
        condition_blocked_request,
        aspect_keyed_historical,
        aspect_keyed_equivalence,
        previous_value_blocked_request,
        temporal_blocked_request,
        gate_state,
        gate_historical_parity,
        hierarchy_replay,
        hierarchy_cancellation,
        hierarchy_historical_parity,
        compile_time_boundary,
    }
}

fn attachment_equivalence_report() -> AsyncNodeCapabilityEquivalenceReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration = async_node_capability_declaration(node);
    let handle = runtime
        .attach_async_capability(declaration.clone())
        .expect("capability should attach");
    runtime
        .async_node_capability_equivalence_report(
            &handle,
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("leaf equivalence should materialize")
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

fn keyed_closeout_reports() -> (
    AsyncKeyedNodeHistoricalParityReport,
    AsyncKeyedNodeCapabilityEquivalenceReport,
) {
    let mut runtime = TestRuntime::build(SignalGraph::new());
    let family = define_keyed_computation(&mut runtime, "async-cert", ());
    let keyed = family.keyed("left");
    let payload = AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(77));
    let declaration = keyed.async_capability_declaration(&mut runtime, payload.clone());
    let binding = keyed
        .declare_async_capability(&mut runtime, payload)
        .expect("keyed capability should attach");
    let handle = keyed
        .async_capable_node(&mut runtime)
        .expect("keyed handle should exist");
    let historical = runtime
        .async_keyed_node_historical_parity_report(
            &binding,
            &handle,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("keyed historical parity should materialize");
    let equivalence = runtime
        .async_keyed_node_capability_equivalence_report(
            &binding,
            &handle,
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("keyed equivalence should materialize");
    (historical, equivalence)
}

fn gate_closeout_reports() -> (AsyncNodeGateStateReport, AsyncNodeHistoricalParityReport) {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    graph
        .append_dependency(child, parent, Aspect::new(0))
        .expect("dependency should wire");
    let mut runtime = TestRuntime::build(graph);
    let handle = runtime
        .attach_async_capability(async_node_capability_with_dependents(parent, [child]))
        .expect("capability should attach");
    let gate = runtime
        .async_node_gate_state_report(parent)
        .expect("gate report should materialize");
    let history = runtime
        .async_node_historical_parity_report(
            &handle,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("historical parity should materialize");
    (gate, history)
}

fn mismatched_gate_reports() -> (AsyncNodeGateStateReport, AsyncNodeHistoricalParityReport) {
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

fn hierarchy_closeout_reports() -> (
    AsyncNodeHierarchyReplaySummary,
    AsyncNodeHierarchyCancellationReport,
    AsyncNodeHierarchyHistoricalParityReport,
) {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let grandchild = graph.node().build();
    graph
        .append_dependency(child, parent, Aspect::new(0))
        .expect("child should depend on parent");
    graph
        .append_dependency(grandchild, child, Aspect::new(0))
        .expect("grandchild should depend on child");
    let mut runtime = TestRuntime::build(graph);
    let parent_handle = runtime
        .attach_async_capability(async_node_capability_with_dependents(parent, [child]))
        .expect("parent capability should attach");
    let child_handle = runtime
        .attach_async_capability(async_node_capability_with_dependents(child, [grandchild]))
        .expect("child capability should attach");
    runtime
        .attach_async_capability(async_node_capability_declaration(grandchild))
        .expect("grandchild capability should attach");
    runtime.observe_nodes(
        ObservationPolicy::touched(),
        [parent],
        Box::new(NoopAsyncNodeObservationListener),
    );
    let parent_admitted = runtime
        .admit_async_node_request(parent_handle.request_intent())
        .expect("parent request should admit")
        .resource_admission()
        .expect("parent request should lower into resource admission")
        .admitted_request()
        .clone();
    let _ = runtime
        .admit_async_node_request(child_handle.request_intent())
        .expect("child request should admit")
        .resource_admission()
        .expect("child request should lower into resource admission")
        .admitted_request()
        .clone();
    let cancellation = runtime
        .cancel_async_node_request(
            parent_admitted.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("hierarchy cancellation should materialize");
    let replay = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("post-cancellation hierarchy replay should materialize");
    let history = runtime
        .async_node_hierarchy_historical_parity_report(
            &parent_handle,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("hierarchy historical parity should materialize");
    (replay, cancellation, history)
}
