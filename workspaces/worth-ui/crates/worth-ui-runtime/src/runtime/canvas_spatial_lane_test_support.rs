use super::durable_state_inventory_test_support::platform_inventory;
use super::replacement_impact_test_support::{
    admitted_candidate, artifact_from_modules, impact_test_app, launch_runtime,
};
use crate::runtime::{
    WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial, WorthUiComponentLoweringHook,
    WorthUiExecutionLane, WorthUiExecutionLaneSupport, WorthUiExecutionPlan,
    WorthUiExecutionPlanInput, WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook,
    WorthUiLaneAdmission, WorthUiPlanNodeInputFamily, WorthUiRuntimeHandleAllocation,
    WorthUiRuntimeHost,
};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

pub(super) fn canvas_spatial_fixture() -> CanvasSpatialFixture {
    canvas_spatial_context().into_fixture()
}

pub(super) fn canvas_spatial_denial_for_missing_hook() -> WorthUiCanvasSpatialPlanDenial {
    let context = canvas_spatial_context();
    context
        .runtime
        .prepare_canvas_spatial_plan(
            &context.execution_plan,
            &context.allocation,
            &context.lane_admission,
            &[],
        )
        .expect_err("canvas spatial plan requires admitted hook")
}

pub(super) fn canvas_spatial_denial_for_stale_lane_admission() -> WorthUiCanvasSpatialPlanDenial {
    let context = canvas_spatial_context();
    let drifted_plan_input = plan_input_with_duplicate_lane_ref(&context.plan_input);
    let stale_admission = context
        .runtime
        .admit_execution_lanes(
            &drifted_plan_input,
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("drifted lane admission succeeds");
    context
        .runtime
        .prepare_canvas_spatial_plan(
            &context.execution_plan,
            &context.allocation,
            &stale_admission,
            &context.hook_admissions,
        )
        .expect_err("canvas spatial plan rejects stale lane admission")
}

pub(super) fn canvas_spatial_denial_for_mismatched_handle_allocation(
) -> WorthUiCanvasSpatialPlanDenial {
    let context = canvas_spatial_context();
    let drifted_plan_input = plan_input_with_drifted_diagnostics_ref(&context.plan_input);
    let drifted_allocation = context
        .runtime
        .allocate_runtime_handles(&drifted_plan_input)
        .expect("drifted handle allocation succeeds");
    context
        .runtime
        .prepare_canvas_spatial_plan(
            &context.execution_plan,
            &drifted_allocation,
            &context.lane_admission,
            &context.hook_admissions,
        )
        .expect_err("canvas spatial plan rejects mismatched handle allocation")
}

pub(super) fn canvas_spatial_denial_for_missing_support() -> WorthUiCanvasSpatialPlanDenial {
    let context = canvas_spatial_context();
    let no_canvas_support = context
        .runtime
        .admit_execution_lanes(
            &context.plan_input,
            &WorthUiExecutionLaneSupport::without_lane_for_test(
                WorthUiExecutionLane::CanvasSpatial,
            ),
        )
        .expect("support without canvas still admits plan input rows");
    context
        .runtime
        .prepare_canvas_spatial_plan(
            &context.execution_plan,
            &context.allocation,
            &no_canvas_support,
            &context.hook_admissions,
        )
        .expect_err("canvas spatial plan rejects missing support")
}

pub(super) struct CanvasSpatialFixture {
    pub(super) runtime: WorthUiRuntimeHost,
    pub(super) canvas_plan: WorthUiCanvasSpatialPlan,
    pub(super) allocation: WorthUiRuntimeHandleAllocation,
}

struct CanvasSpatialContext {
    runtime: WorthUiRuntimeHost,
    plan_input: WorthUiExecutionPlanInput,
    allocation: WorthUiRuntimeHandleAllocation,
    execution_plan: WorthUiExecutionPlan,
    lane_admission: WorthUiLaneAdmission,
    hook_admissions: Vec<WorthUiExtensionHookAdmission>,
    canvas_plan: WorthUiCanvasSpatialPlan,
}

impl CanvasSpatialContext {
    fn into_fixture(self) -> CanvasSpatialFixture {
        CanvasSpatialFixture {
            runtime: self.runtime,
            canvas_plan: self.canvas_plan,
            allocation: self.allocation,
        }
    }
}

fn canvas_spatial_context() -> CanvasSpatialContext {
    let app = impact_test_app();
    let active = canvas_source_artifact(&app);
    let candidate = canvas_source_artifact(&app);
    let runtime = launch_runtime(&app, active);
    let admitted = admitted_candidate(&app, &runtime, candidate);
    let pending = pending_plan_input(&runtime, admitted);
    let plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &canvas_component_hooks(),
        )
        .expect("execution plan input prepares");
    let allocation = runtime
        .allocate_runtime_handles(&plan_input)
        .expect("handle allocation succeeds");
    let lane_admission = runtime
        .admit_execution_lanes(
            &plan_input,
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("lane admission succeeds");
    let hook_admissions = canvas_hook_admissions(&runtime, &lane_admission);
    let execution_plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &plan_input,
            &allocation,
            &lane_admission,
        )
        .expect("execution plan topology assembles");
    let canvas_plan = runtime
        .prepare_canvas_spatial_plan(
            &execution_plan,
            &allocation,
            &lane_admission,
            &hook_admissions,
        )
        .expect("canvas spatial plan prepares");

    assert!(!canvas_plan.rows().is_empty());
    assert!(!canvas_plan.draw_hooks().is_empty());

    CanvasSpatialContext {
        runtime,
        plan_input,
        allocation,
        execution_plan,
        lane_admission,
        hook_admissions,
        canvas_plan,
    }
}

fn pending_plan_input(
    runtime: &WorthUiRuntimeHost,
    admitted: crate::runtime::WorthUiAdmittedReplacementCandidate,
) -> crate::runtime::WorthUiPendingActivation {
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
    let inventory = platform_inventory(runtime)
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
    let pending_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation_plan,
        &query_rebind_plan,
    );
    runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation_plan),
            Some(&query_rebind_plan),
            Some(&pending_input),
        )
        .expect("activation staging succeeds")
}

fn canvas_source_artifact(app: &crate::facade::WorthUiApp) -> crate::source::WorthUiArtifact {
    artifact_from_modules(
        app,
        [
            WorthUiRustAuthoredArtifactInputModule::new("app/canvas.wui")
                .with_component("workspace.component.dashboard")
                .with_surface("workspace.surface.command_save")
                .with_token("theme.text.primary", "#202830"),
        ],
    )
}

fn canvas_component_hooks() -> [WorthUiComponentLoweringHook; 4] {
    [
        WorthUiComponentLoweringHook::registered(
            "canvas.fixture.lane",
            WorthUiPlanNodeInputFamily::LanePartitionRef,
        ),
        WorthUiComponentLoweringHook::registered(
            "canvas.fixture.command",
            WorthUiPlanNodeInputFamily::Command,
        ),
        WorthUiComponentLoweringHook::registered(
            "canvas.fixture.diagnostics",
            WorthUiPlanNodeInputFamily::DiagnosticsRef,
        ),
        WorthUiComponentLoweringHook::registered(
            "canvas.fixture.render_ref",
            WorthUiPlanNodeInputFamily::RenderResourceRef,
        ),
    ]
}

fn canvas_hook_admissions(
    runtime: &WorthUiRuntimeHost,
    lane_admission: &WorthUiLaneAdmission,
) -> Vec<WorthUiExtensionHookAdmission> {
    [WorthUiLaneAdapterHook::canvas_spatial_draw_and_hit_test(
        "canvas.draw.hit_test.tool_state",
    )]
    .into_iter()
    .map(|hook| {
        runtime
            .admit_extension_hook(lane_admission, hook)
            .expect("canvas hook admits against canvas lane support")
    })
    .collect()
}

fn plan_input_with_duplicate_lane_ref(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let duplicated = plan_input
        .node_inputs()
        .iter()
        .find(|input| input.family() == WorthUiPlanNodeInputFamily::LanePartitionRef)
        .expect("fixture has lane partition ref")
        .clone();
    let mut node_inputs = plan_input.node_inputs().to_vec();
    node_inputs.push(duplicated);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn plan_input_with_drifted_diagnostics_ref(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let diagnostics_index = node_inputs
        .iter()
        .position(|input| input.family() == WorthUiPlanNodeInputFamily::DiagnosticsRef)
        .expect("fixture has diagnostics ref");
    node_inputs[diagnostics_index] = node_inputs[diagnostics_index]
        .clone()
        .with_identity_basis_for_test("canvas.fixture.diagnostics.allocation_drift");
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}
