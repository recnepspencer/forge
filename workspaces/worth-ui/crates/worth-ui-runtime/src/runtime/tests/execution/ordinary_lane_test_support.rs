use super::allocation_planning_test_support::allocation_planning;
use super::durable_state_inventory_test_support::platform_inventory;
use super::replacement_impact_test_support::{
    admitted_candidate, artifact_from_modules, impact_test_app, launch_runtime,
};
use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiExecutionLane, WorthUiExecutionLaneSupport,
    WorthUiExecutionPlan, WorthUiExecutionPlanInput, WorthUiLaneAdmission, WorthUiOrdinaryLanePlan,
    WorthUiOrdinaryLanePlanDenial, WorthUiPlanNodeInputFamily, WorthUiRuntime,
    WorthUiRuntimeHandleAllocation,
};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

pub(super) fn ordinary_lane_fixture() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiOrdinaryLanePlan,
    WorthUiRuntimeHandleAllocation,
) {
    let context = ordinary_execution_context();
    (context.runtime, context.ordinary_plan, context.allocation)
}

pub(super) fn ordinary_lane_denial_for_missing_family(
    removed_family: WorthUiPlanNodeInputFamily,
    removed_lane: WorthUiExecutionLane,
) -> WorthUiOrdinaryLanePlanDenial {
    let context = ordinary_execution_context();
    let admission_context = ordinary_execution_context();
    let narrower_plan_input =
        plan_input_without_family(&admission_context.plan_input, removed_family);
    let receipt_runtime = fresh_ordinary_runtime();
    let narrower_planning = allocation_planning(
        &receipt_runtime,
        &narrower_plan_input,
        "ordinary-lane.missing-family",
    );
    let narrower_admission = receipt_runtime
        .admit_execution_lanes(
            &receipt_runtime.detached_allocation_receipt_for_test(&narrower_planning),
            &WorthUiExecutionLaneSupport::without_lane_for_test(removed_lane),
        )
        .expect("narrower input can be admitted without removed lane");

    context
        .runtime
        .prepare_ordinary_lane_plan(&context.execution_plan, &narrower_admission)
        .expect_err("ordinary plan refuses unrelated narrower admission")
}

fn fresh_ordinary_runtime() -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    let app = impact_test_app();
    launch_runtime(&app, ordinary_source_artifact(&app))
}

struct OrdinaryExecutionContext {
    runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    plan_input: WorthUiExecutionPlanInput,
    allocation: WorthUiRuntimeHandleAllocation,
    execution_plan: WorthUiExecutionPlan,
    ordinary_plan: WorthUiOrdinaryLanePlan,
}

fn ordinary_execution_context() -> OrdinaryExecutionContext {
    let app = impact_test_app();
    let active = ordinary_source_artifact(&app);
    let candidate = ordinary_source_artifact(&app);
    let runtime = launch_runtime(&app, active);
    let admitted = admitted_candidate(&app, &runtime, candidate);
    let comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity matching succeeds");
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan succeeds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation_plan = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind_plan = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("query rebind planning succeeds");
    let pending_execution_plan_lowering_input = runtime
        .prepare_pending_execution_plan_lowering_input(
            &node_plan,
            &reconciliation_plan,
            &query_rebind_plan,
        );
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation_plan),
            Some(&query_rebind_plan),
            Some(&pending_execution_plan_lowering_input),
        )
        .expect("activation staging succeeds");
    let plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &ordinary_component_hooks(),
        )
        .expect("execution plan input prepares");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(
            &allocation_planning(&runtime, &plan_input, "ordinary-lane.fixture"),
        ))
        .expect("handle allocation succeeds");
    let planning = allocation_planning(&runtime, &plan_input, "ordinary-lane.fixture");
    let lane_admission = ordinary_lane_admission(&runtime, &planning);
    let execution_plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &runtime.detached_allocation_receipt_for_test(&planning),
            &allocation,
            &lane_admission,
        )
        .expect("execution plan topology assembles");
    let ordinary_plan = runtime
        .prepare_ordinary_lane_plan(&execution_plan, &lane_admission)
        .expect("ordinary lane plan prepares");

    assert!(ordinary_plan.counters().ordinary_plan_row_count() > 0);
    assert!(
        ordinary_plan
            .counters()
            .skipped_nonordinary_plan_row_count()
            > 0
    );

    OrdinaryExecutionContext {
        runtime,
        plan_input,
        allocation,
        execution_plan,
        ordinary_plan,
    }
}

fn ordinary_source_artifact(app: &crate::facade::WorthUiApp) -> crate::source::WorthUiArtifact {
    artifact_from_modules(app, [ordinary_source_module()])
}

fn ordinary_source_module() -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component("workspace.component.dashboard")
        .with_surface("workspace.surface.command_save")
        .with_token("theme.text.primary", "#101820")
}

fn ordinary_component_hooks() -> [WorthUiComponentLoweringHook; 2] {
    [
        WorthUiComponentLoweringHook::registered(
            "ordinary.fixture.command_surface",
            WorthUiPlanNodeInputFamily::Command,
        ),
        WorthUiComponentLoweringHook::registered(
            "ordinary.fixture.diagnostics_skip",
            WorthUiPlanNodeInputFamily::DiagnosticsRef,
        ),
    ]
}

fn ordinary_lane_admission(
    runtime: &WorthUiRuntime,
    planning: &crate::runtime::UiAllocationCandidate,
) -> WorthUiLaneAdmission {
    let receipt = runtime.detached_allocation_receipt_for_test(planning);
    runtime
        .admit_execution_lanes(&receipt, &WorthUiExecutionLaneSupport::platform_default())
        .expect("lane admission succeeds")
}

fn plan_input_without_family(
    plan_input: &WorthUiExecutionPlanInput,
    family: WorthUiPlanNodeInputFamily,
) -> WorthUiExecutionPlanInput {
    let node_inputs = plan_input
        .node_inputs()
        .iter()
        .filter(|node_input| node_input.family() != family)
        .cloned()
        .collect::<Vec<_>>();
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}
