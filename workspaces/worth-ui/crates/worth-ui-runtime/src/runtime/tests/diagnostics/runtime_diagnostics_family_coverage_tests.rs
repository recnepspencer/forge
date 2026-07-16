use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::allocation_planning;
use super::durable_state_reconciliation_test_support::{
    deterministic_reconciliation_inputs, stale_inventory_for,
};
use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, identity_match_app, runtime_and_narrowing, surface_node,
};
use super::query_binding_comparison_test_support::{
    phase11_pipeline, query_artifact, standard_query_app,
};
use super::reload_failure_test_support::missing_artifact_candidate_denial;
use super::replacement_impact_test_support::{
    admitted_candidate, artifact_from_modules, impact_test_app, launch_runtime, surface_module,
    token_module,
};
use crate::runtime::{
    WorthUiCandidateAdmissionDenial, WorthUiDiagnosticProjectionHook, WorthUiExecutionLane,
    WorthUiExecutionLaneSupport, WorthUiExecutionPlanInput, WorthUiNodeReplacementPlan,
    WorthUiPlanNodeInputFamily, WorthUiRuntimeDiagnosticFamily, WorthUiRuntimeDiagnosticReport,
};
use std::collections::BTreeSet;

#[test]
fn every_replacement_phase_denial_maps_to_specific_diagnostic_family() {
    let fixture = activation_staging_inputs();
    let admission_denial = WorthUiCandidateAdmissionDenial::SnapshotMismatch {
        candidate_snapshot_digest: 10,
        active_snapshot_digest: 20,
    };

    let invalid = fixture
        .runtime
        .diagnostics()
        .for_invalid_candidate(missing_artifact_candidate_denial())
        .materialize();
    let admission = fixture
        .runtime
        .diagnostics()
        .for_candidate_admission(&admission_denial)
        .materialize();
    let artifact = artifact_equivalence_report();
    let impact = replacement_impact_report();
    let narrowing = impact_narrowing_report();
    let identity = identity_matching_report();
    let query = query_live_rebind_report();
    let lane = lane_admission_report();
    let plan_inspection = plan_inspection_report();
    let reconciliation = reconciliation_report();
    let lowering = plan_lowering_report();
    let activation_staging = activation_staging_report();
    let activation_gate = activation_gate_report();
    let committed_activation = committed_allocation_activation_report();
    let projection = fixture
        .runtime
        .diagnostics()
        .for_projection_hook(&WorthUiDiagnosticProjectionHook::projection(
            "workspace.diagnostics.panel",
        ))
        .materialize();

    let families = [
        first_family(&invalid),
        first_family(&admission),
        first_family(&artifact),
        first_family(&impact),
        first_family(&narrowing),
        first_family(&identity),
        first_family(&reconciliation),
        first_family(&query),
        first_family(&lowering),
        first_family(&lane),
        first_family(&activation_staging),
        first_family(&activation_gate),
        first_family(&committed_activation),
        first_family(&plan_inspection),
        first_family(&projection),
    ];

    let unique_families = families.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(unique_families.len(), 15);
    assert_eq!(
        unique_families,
        BTreeSet::from([
            WorthUiRuntimeDiagnosticFamily::Reload,
            WorthUiRuntimeDiagnosticFamily::CandidateAdmission,
            WorthUiRuntimeDiagnosticFamily::ArtifactEquivalence,
            WorthUiRuntimeDiagnosticFamily::ReplacementImpact,
            WorthUiRuntimeDiagnosticFamily::ImpactNarrowing,
            WorthUiRuntimeDiagnosticFamily::IdentityMatching,
            WorthUiRuntimeDiagnosticFamily::DurableStateReconciliation,
            WorthUiRuntimeDiagnosticFamily::QueryLiveRebind,
            WorthUiRuntimeDiagnosticFamily::PlanLowering,
            WorthUiRuntimeDiagnosticFamily::LaneAdmission,
            WorthUiRuntimeDiagnosticFamily::ActivationStaging,
            WorthUiRuntimeDiagnosticFamily::ActivationGate,
            WorthUiRuntimeDiagnosticFamily::CommittedAllocationActivation,
            WorthUiRuntimeDiagnosticFamily::PlanInspection,
            WorthUiRuntimeDiagnosticFamily::DiagnosticsProjection,
        ])
    );
}

fn reconciliation_report() -> WorthUiRuntimeDiagnosticReport {
    let (runtime, node_plan, inventory) = deterministic_reconciliation_inputs();
    let stale_inventory = stale_inventory_for(&inventory);
    let denial = runtime
        .reconcile_durable_state(&node_plan, &stale_inventory)
        .expect_err("stale inventory denies");
    runtime
        .diagnostics()
        .for_reconciliation(&denial)
        .materialize()
}

fn artifact_equivalence_report() -> WorthUiRuntimeDiagnosticReport {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );
    let admitted = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );
    let stale_admitted =
        admitted.with_admitted_query_contract_for_test("stale-diagnostic-contract");
    let denial = runtime
        .compare_admitted_replacement(&stale_admitted)
        .expect_err("changed admission receipt denies before artifact comparison");
    runtime
        .diagnostics()
        .for_artifact_equivalence(&denial)
        .materialize()
}

fn replacement_impact_report() -> WorthUiRuntimeDiagnosticReport {
    let app = impact_test_app();
    let comparison_runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );
    let admission_runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [token_module("theme.text.primary")]),
    );
    let comparison_candidate = admitted_candidate(
        &app,
        &comparison_runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_open")]),
    );
    let admitted_same_artifact = admitted_candidate(
        &app,
        &admission_runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_open")]),
    );
    let comparison = comparison_runtime
        .compare_admitted_replacement(&comparison_candidate)
        .expect("comparison prepares against first active runtime");
    let denial = admission_runtime
        .classify_replacement_impact(&comparison, &admitted_same_artifact)
        .expect_err("comparison evidence cannot certify a different active basis");
    admission_runtime
        .diagnostics()
        .for_replacement_impact(&denial)
        .materialize()
}

fn impact_narrowing_report() -> WorthUiRuntimeDiagnosticReport {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_save")]),
    );
    let command_candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_open")]),
    );
    let token_candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [token_module("theme.text.secondary")]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&command_candidate)
        .expect("command candidate compares");
    let classification = runtime
        .classify_replacement_impact(&comparison, &command_candidate)
        .expect("command candidate classifies");
    let denial = runtime
        .narrow_replacement_impact(&classification, &token_candidate)
        .expect_err("classification evidence cannot certify a different candidate");
    runtime
        .diagnostics()
        .for_impact_narrowing(&denial)
        .materialize()
}

fn identity_matching_report() -> WorthUiRuntimeDiagnosticReport {
    let app = identity_match_app();
    let active =
        artifact_from_nodes([("app/main.wui", vec![component_node("identity:shared", 0)])]);
    let candidate = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("identity:shared", "workspace.surface.main", 0)],
    )]);
    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let denial = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect_err("same identity basis across node kinds denies");
    runtime
        .diagnostics()
        .for_identity_matching(&denial)
        .materialize()
}

fn query_live_rebind_report() -> WorthUiRuntimeDiagnosticReport {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");
    let stale_plan = WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest() + 1,
        plan.classifications().to_vec(),
        plan.counters(),
    );
    let denial = runtime
        .plan_query_live_rebinds(&comparison, &stale_plan, &narrowing, &admitted)
        .expect_err("stale replacement plan denies live rebind planning");
    runtime
        .diagnostics()
        .for_query_live_rebind(&denial)
        .materialize()
}

fn plan_lowering_report() -> WorthUiRuntimeDiagnosticReport {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    runtime.advance_frame_epoch_for_test();
    let denial = runtime
        .prepare_execution_plan_input(&pending)
        .expect_err("stale pending activation denies lowering");
    runtime
        .diagnostics()
        .for_plan_lowering(&denial)
        .materialize()
}

fn plan_inspection_report() -> WorthUiRuntimeDiagnosticReport {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let planning =
        allocation_planning(&runtime, &plan_input, "runtime-diagnostics.plan-inspection");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("runtime handles allocate");
    let plan = runtime
        .assemble_execution_plan_topology(
            &runtime.detached_allocation_receipt_for_test(&planning),
            &allocation,
        )
        .expect("execution plan topology assembles");
    let wrong_receipt_input = plan_input_with_first_family_changed(plan_input);
    let wrong_planning = allocation_planning(
        &runtime,
        &wrong_receipt_input,
        "runtime-diagnostics.plan-inspection.wrong",
    );
    let denial = runtime
        .inspect_execution_plan(&plan, wrong_planning.planning())
        .expect_err("wrong plan input receipt denies inspection");
    runtime
        .diagnostics()
        .for_plan_inspection(&denial)
        .materialize()
}

fn plan_input_with_first_family_changed(
    plan_input: WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    node_inputs[0] = node_inputs[0]
        .clone()
        .with_family_for_test(alternate_plan_input_family(node_inputs[0].family()));
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn alternate_plan_input_family(family: WorthUiPlanNodeInputFamily) -> WorthUiPlanNodeInputFamily {
    match family {
        WorthUiPlanNodeInputFamily::TokenStyle => WorthUiPlanNodeInputFamily::DiagnosticsRef,
        _ => WorthUiPlanNodeInputFamily::TokenStyle,
    }
}

fn lane_admission_report() -> WorthUiRuntimeDiagnosticReport {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "runtime-diagnostics.lane-admission");
    let support_without_query =
        WorthUiExecutionLaneSupport::without_lane_for_test(WorthUiExecutionLane::QueryBound);
    let denial = runtime
        .admit_execution_lanes(
            &runtime.detached_allocation_receipt_for_test(&planning),
            &support_without_query,
        )
        .expect_err("unsupported Query lane denies");
    runtime
        .diagnostics()
        .for_lane_admission(&denial)
        .materialize()
}

fn activation_staging_report() -> WorthUiRuntimeDiagnosticReport {
    let denial = activation_staging_inputs().stage_without_reconciliation();
    activation_staging_inputs()
        .runtime
        .diagnostics()
        .for_activation_staging(&denial)
        .materialize()
}

fn activation_gate_report() -> WorthUiRuntimeDiagnosticReport {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "diagnostics.activation-gate",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete diagnostic catalog");
    let boundary = runtime.traversal_frame_boundary_for_test();
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect_err("unsafe boundary denies");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial carries canonical evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate) =
        denial.reason()
    else {
        panic!("canonical denial retains frame reason")
    };
    runtime
        .diagnostics()
        .for_activation_gate(gate)
        .materialize()
}

fn committed_allocation_activation_report() -> WorthUiRuntimeDiagnosticReport {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "diagnostics.committed-allocation",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete diagnostic catalog");
    let boundary = runtime.traversal_frame_boundary_for_test();
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(pending, admitted, boundary, None)
        .expect_err("unsafe boundary denies canonical activation");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial carries canonical evidence")
    };
    runtime
        .diagnostics()
        .for_committed_allocation_denial(&denial)
        .materialize()
}

fn first_family(report: &WorthUiRuntimeDiagnosticReport) -> WorthUiRuntimeDiagnosticFamily {
    report.rows()[0].family()
}
