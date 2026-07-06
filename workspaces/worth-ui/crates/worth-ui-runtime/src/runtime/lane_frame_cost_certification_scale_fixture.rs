use super::allocation_planning_test_support::allocation_planning;
use super::durable_state_inventory_test_support::platform_inventory;
use super::replacement_impact_test_support::{
    admitted_candidate, artifact_from_modules, impact_test_app, launch_runtime,
};
use super::virtualized_data_lane_test_support::virtualized_data_fixture;
use super::{
    WorthUiComponentLoweringHook, WorthUiExecutionLaneSupport, WorthUiExecutionPlanInput,
    WorthUiExtensionHookAdmission, WorthUiFrameExecutionReceipt, WorthUiHighFrequencyFramePolicy,
    WorthUiLaneAdapterHook, WorthUiPlanNodeInputFamily, WorthUiRealtimeFramePriority,
    WorthUiRealtimeFrameTarget, WorthUiRuntimeHandleAllocation, WorthUiRuntimeHost,
    WorthUiSteadyFrameCounterBoundary, WorthUiViewBindingHandle, WorthUiVirtualizedDataFrameTarget,
    WorthUiVisibleRange,
};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

pub(super) fn virtualized_data_scale_sample(
    active_plan_digest: u64,
    rows: u32,
    columns: u32,
) -> WorthUiFrameExecutionReceipt {
    let virtualized = virtualized_data_fixture();
    let data_handle = first_view_binding_handle(&virtualized.allocation);
    let range = WorthUiVisibleRange::grid(rows, rows, 0, columns).expect("range is valid");
    let receipt = virtualized
        .runtime
        .execute_virtualized_data_frame(
            &virtualized.data_plan,
            WorthUiVirtualizedDataFrameTarget::view_binding(data_handle, range),
        )
        .expect("virtualized scale sample executes");

    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_virtualized_data_frame(receipt)
        .seal()
        .expect("virtualized scale sample seals")
}

pub(super) fn realtime_overlay_scale_sample(
    active_plan_digest: u64,
    duplicate_render_resource: bool,
) -> WorthUiFrameExecutionReceipt {
    let context = realtime_scale_context(duplicate_render_resource);
    let surface = context.hud_plan.renderer_surfaces()[0].handle();
    let receipt = context
        .runtime
        .execute_realtime_frame(
            &context.hud_plan,
            WorthUiRealtimeFrameTarget::renderer_surface(surface),
        )
        .expect("realtime scale sample executes");

    WorthUiSteadyFrameCounterBoundary::for_active_plan(active_plan_digest)
        .record_realtime_overlay_frame(receipt)
        .seal()
        .expect("realtime scale sample seals")
}

struct RealtimeScaleContext {
    runtime: WorthUiRuntimeHost,
    hud_plan: super::WorthUiHudPlan,
}

fn realtime_scale_context(duplicate_render_resource: bool) -> RealtimeScaleContext {
    let app = impact_test_app();
    let active = realtime_scale_artifact(&app);
    let candidate = realtime_scale_artifact(&app);
    let runtime = launch_runtime(&app, active);
    let admitted = admitted_candidate(&app, &runtime, candidate);
    let pending = pending_plan_input(&runtime, admitted);
    let mut plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &realtime_component_hooks(),
        )
        .expect("execution plan input prepares");
    if duplicate_render_resource {
        plan_input = plan_input_with_duplicate_render_ref(&plan_input);
    }
    let planning = allocation_planning(&runtime, &plan_input, "lane-frame-cost.realtime");
    let allocation = runtime
        .allocate_runtime_handles(&planning)
        .expect("handle allocation succeeds");
    let lane_admission = runtime
        .admit_execution_lanes(&planning, &WorthUiExecutionLaneSupport::platform_default())
        .expect("lane admission succeeds");
    let execution_plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &planning,
            &allocation,
            &lane_admission,
        )
        .expect("execution plan topology assembles");
    let frame_policy = WorthUiHighFrequencyFramePolicy::frame_budgeted(
        16,
        WorthUiRealtimeFramePriority::HudOverlay,
    )
    .expect("frame policy is valid");
    let hud_plan = runtime
        .prepare_hud_plan(
            &execution_plan,
            &allocation,
            &lane_admission,
            &realtime_hook_admissions(&runtime, &lane_admission),
            frame_policy,
        )
        .expect("HUD plan prepares");

    RealtimeScaleContext { runtime, hud_plan }
}

fn pending_plan_input(
    runtime: &WorthUiRuntimeHost,
    admitted: super::WorthUiAdmittedReplacementCandidate,
) -> super::WorthUiPendingActivation {
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

fn realtime_scale_artifact(app: &crate::facade::WorthUiApp) -> crate::source::WorthUiArtifact {
    artifact_from_modules(
        app,
        [
            WorthUiRustAuthoredArtifactInputModule::new("app/realtime-scale.wui")
                .with_component("workspace.component.dashboard")
                .with_surface("workspace.surface.command_save")
                .with_token("theme.text.primary", "#202830"),
        ],
    )
}

fn realtime_component_hooks() -> [WorthUiComponentLoweringHook; 5] {
    [
        WorthUiComponentLoweringHook::registered(
            "realtime.scale.render_ref",
            WorthUiPlanNodeInputFamily::RenderResourceRef,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.scale.command",
            WorthUiPlanNodeInputFamily::Command,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.scale.accessibility",
            WorthUiPlanNodeInputFamily::Accessibility,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.scale.diagnostics",
            WorthUiPlanNodeInputFamily::DiagnosticsRef,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.scale.lane",
            WorthUiPlanNodeInputFamily::LanePartitionRef,
        ),
    ]
}

fn realtime_hook_admissions(
    runtime: &WorthUiRuntimeHost,
    lane_admission: &super::WorthUiLaneAdmission,
) -> Vec<WorthUiExtensionHookAdmission> {
    [WorthUiLaneAdapterHook::realtime_overlay_mechanics(
        "realtime.scale.hud",
    )]
    .into_iter()
    .map(|hook| {
        runtime
            .admit_extension_hook(lane_admission, hook)
            .expect("realtime hook admits against platform support")
    })
    .collect()
}

fn plan_input_with_duplicate_render_ref(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let duplicated = plan_input
        .node_inputs()
        .iter()
        .find(|input| input.family() == WorthUiPlanNodeInputFamily::RenderResourceRef)
        .expect("fixture has render resource ref")
        .clone()
        .with_identity_basis_for_test("realtime.scale.render_ref.extra");
    let mut node_inputs = plan_input.node_inputs().to_vec();
    node_inputs.push(duplicated);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn first_view_binding_handle(
    allocation: &WorthUiRuntimeHandleAllocation,
) -> WorthUiViewBindingHandle {
    allocation
        .view_binding_handles()
        .first()
        .copied()
        .expect("fixture has view binding handle")
}
