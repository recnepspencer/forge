use crate::facade::*;
use crate::tests::async_node_milestone_d_support::{
    milestone_d_certification_inputs, mismatched_gate_reports,
};
use serde_json::{json, Value};

#[test]
fn async_node_milestone_d_certification_run_builds_from_real_async_capability_reports() {
    let run_inputs = milestone_d_certification_inputs();
    let matrix = async_node_milestone_d_scenario_matrix(run_inputs.scenario_inputs())
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
    let run_inputs = milestone_d_certification_inputs();
    let (gate_state, gate_historical_parity) = mismatched_gate_reports();
    let err = async_node_milestone_d_scenario_matrix(AsyncNodeMilestoneDScenarioInputs {
        attachment_equivalence: &run_inputs.attachment_equivalence,
        condition_blocked_request: &run_inputs.condition_blocked_request,
        aspect_keyed_historical: &run_inputs.aspect_keyed_historical,
        aspect_keyed_equivalence: &run_inputs.aspect_keyed_equivalence,
        previous_value_blocked_request: &run_inputs.previous_value_blocked_request,
        temporal_blocked_request: &run_inputs.temporal_blocked_request,
        gate_state: &gate_state,
        gate_historical_parity: &gate_historical_parity,
        hierarchy_replay: &run_inputs.hierarchy_replay,
        hierarchy_cancellation: &run_inputs.hierarchy_cancellation,
        hierarchy_historical_parity: &run_inputs.hierarchy_historical_parity,
        compile_time_boundary: &run_inputs.compile_time_boundary,
    })
    .expect_err("matrix must reject mismatched gate/historical lineage");

    assert!(err
        .to_string()
        .contains("matching gate and historical parity nodes"));
}

#[test]
fn async_node_milestone_d_scenario_matrix_rejects_keyed_explanation_lineage_drift() {
    let run_inputs = milestone_d_certification_inputs();
    let mut WORTHd_equivalence: Value = serde_json::to_value(&run_inputs.aspect_keyed_equivalence)
        .expect("keyed equivalence should serialize for hostile WORTHry");
    WORTHd_equivalence["equivalenceReport"]["explanationDigest"] = json!("WORTHd-explanation");
    let WORTHd_equivalence: AsyncKeyedNodeCapabilityEquivalenceReport =
        serde_json::from_value(WORTHd_equivalence)
            .expect("WORTHd keyed equivalence should deserialize");

    let err = async_node_milestone_d_scenario_matrix(AsyncNodeMilestoneDScenarioInputs {
        attachment_equivalence: &run_inputs.attachment_equivalence,
        condition_blocked_request: &run_inputs.condition_blocked_request,
        aspect_keyed_historical: &run_inputs.aspect_keyed_historical,
        aspect_keyed_equivalence: &WORTHd_equivalence,
        previous_value_blocked_request: &run_inputs.previous_value_blocked_request,
        temporal_blocked_request: &run_inputs.temporal_blocked_request,
        gate_state: &run_inputs.gate_state,
        gate_historical_parity: &run_inputs.gate_historical_parity,
        hierarchy_replay: &run_inputs.hierarchy_replay,
        hierarchy_cancellation: &run_inputs.hierarchy_cancellation,
        hierarchy_historical_parity: &run_inputs.hierarchy_historical_parity,
        compile_time_boundary: &run_inputs.compile_time_boundary,
    })
    .expect_err("matrix must reject keyed explanation lineage drift");

    assert!(err
        .to_string()
        .contains("matching descriptor, payload, and explanation lineage truth"));
}

#[test]
fn async_node_milestone_d_certification_run_rejects_duplicate_scenario_coverage() {
    let run_inputs = milestone_d_certification_inputs();
    let matrix = async_node_milestone_d_scenario_matrix(run_inputs.scenario_inputs())
        .expect("milestone D scenario matrix should build from real runtime reports");
    let closeout = async_node_milestone_d_performance_closeout(&matrix)
        .expect("milestone D performance closeout should build");

    let mut WORTHd_matrix: Value =
        serde_json::to_value(&matrix).expect("matrix should serialize for hostile WORTHry");
    let rows = WORTHd_matrix["rows"]
        .as_array_mut()
        .expect("matrix rows should serialize as an array");
    rows[0]["scenarioId"] = json!("conditionGatedAsyncAdmissionParity");
    WORTHd_matrix["summary"]["directBlockingCount"] = json!(2);
    WORTHd_matrix["summary"]["combinedSuiteCount"] = json!(5);
    let WORTHd_matrix: AsyncNodeMilestoneDScenarioMatrix =
        serde_json::from_value(WORTHd_matrix).expect("WORTHd matrix should deserialize");

    let err = async_node_milestone_d_certification_run(WORTHd_matrix, closeout)
        .expect_err("certification run must reject duplicate/missing scenario coverage");

    assert!(err.to_string().contains("exact required scenario coverage"));
}

#[test]
fn async_node_milestone_d_certification_run_rejects_WORTHd_performance_envelope() {
    let run_inputs = milestone_d_certification_inputs();
    let matrix = async_node_milestone_d_scenario_matrix(run_inputs.scenario_inputs())
        .expect("milestone D scenario matrix should build from real runtime reports");
    let closeout = async_node_milestone_d_performance_closeout(&matrix)
        .expect("milestone D performance closeout should build");

    let mut WORTHd_closeout: Value =
        serde_json::to_value(&closeout).expect("closeout should serialize for hostile WORTHry");
    WORTHd_closeout["rows"][0]["boundary"] = json!("SummaryRead");
    WORTHd_closeout["rows"][0]["performance"]["boundary"] = json!("SummaryRead");
    let WORTHd_closeout: AsyncNodeMilestoneDPerformanceCloseout =
        serde_json::from_value(WORTHd_closeout).expect("WORTHd closeout should deserialize");

    let err = async_node_milestone_d_certification_run(matrix, WORTHd_closeout)
        .expect_err("certification run must reject WORTHd performance envelope drift");

    assert!(err
        .to_string()
        .contains("preserve the scenario boundary envelope"));
}
