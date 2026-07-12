use super::allocation_planning_test_support::allocation_planning;
use super::realtime_overlay_lane_pending_activation_fixture::pending_plan_input;
use super::realtime_overlay_plan_input_fixture::{
    plan_input_with_drifted_diagnostics_ref, plan_input_with_duplicate_render_ref,
    plan_input_without_family,
};
use super::replacement_impact_test_support::{
    admitted_candidate, artifact_from_modules, impact_test_app, launch_runtime,
};
use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiExecutionLane, WorthUiExecutionLaneSupport,
    WorthUiExecutionPlan, WorthUiExecutionPlanInput, WorthUiExtensionHookAdmission,
    WorthUiHighFrequencyFramePolicy, WorthUiHudPlan, WorthUiHudPlanDenial, WorthUiLaneAdapterHook,
    WorthUiLaneAdmission, WorthUiPlanNodeInputFamily, WorthUiRealtimeFramePriority, WorthUiRuntime,
    WorthUiRuntimeHandleAllocation,
};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

pub(super) fn realtime_overlay_fixture() -> RealtimeOverlayFixture {
    realtime_overlay_context().into_fixture()
}

pub(super) fn realtime_denial_for_missing_hook() -> WorthUiHudPlanDenial {
    let context = realtime_overlay_context();
    context
        .runtime
        .prepare_hud_plan(
            &context.execution_plan,
            &context.allocation,
            &context.lane_admission,
            &[],
            context.frame_policy,
        )
        .expect_err("HUD plan requires admitted realtime hook")
}

pub(super) fn realtime_denial_for_unsupported_hook() -> WorthUiHudPlanDenial {
    let context = realtime_overlay_context();
    let canvas_hook = context
        .runtime
        .admit_extension_hook(
            &context.lane_admission,
            WorthUiLaneAdapterHook::canvas_spatial_draw_and_hit_test("wrong.canvas.hook"),
        )
        .expect("canvas hook admits against platform support");
    context
        .runtime
        .prepare_hud_plan(
            &context.execution_plan,
            &context.allocation,
            &context.lane_admission,
            &[canvas_hook],
            context.frame_policy,
        )
        .expect_err("HUD plan rejects non-realtime hook")
}

pub(super) fn realtime_denial_for_missing_realtime_support() -> WorthUiHudPlanDenial {
    let context = realtime_overlay_context();
    let support = WorthUiExecutionLaneSupport::without_lane_for_test(
        WorthUiExecutionLane::RealtimeOverlayHud,
    );
    let planning = allocation_planning(
        &context.runtime,
        &context.plan_input,
        "realtime-overlay.missing-support",
    );
    let receipt = context
        .runtime
        .detached_allocation_receipt_for_test(&planning);
    let lane_admission = context
        .runtime
        .admit_execution_lanes(&receipt, &support)
        .expect("plan input can admit without realtime support row");
    context
        .runtime
        .prepare_hud_plan(
            &context.execution_plan,
            &context.allocation,
            &lane_admission,
            &context.hook_admissions,
            context.frame_policy,
        )
        .expect_err("HUD plan rejects missing realtime support")
}

pub(super) fn realtime_denial_for_stale_lane_admission() -> WorthUiHudPlanDenial {
    let context = realtime_overlay_context();
    let drifted_plan_input = plan_input_with_duplicate_render_ref(&context.plan_input);
    let receipt_runtime = fresh_realtime_runtime();
    let drifted_planning = allocation_planning(
        &receipt_runtime,
        &drifted_plan_input,
        "realtime-overlay.stale-admission",
    );
    let stale_admission = receipt_runtime
        .admit_execution_lanes(
            &receipt_runtime.detached_allocation_receipt_for_test(&drifted_planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("drifted lane admission succeeds");
    context
        .runtime
        .prepare_hud_plan(
            &context.execution_plan,
            &context.allocation,
            &stale_admission,
            &context.hook_admissions,
            context.frame_policy,
        )
        .expect_err("HUD plan rejects stale lane admission")
}

pub(super) fn realtime_denial_for_stale_lane_admission_without_render_rows() -> WorthUiHudPlanDenial
{
    let context = realtime_overlay_context();
    let drifted_plan_input = plan_input_without_family(
        &context.plan_input,
        WorthUiPlanNodeInputFamily::RenderResourceRef,
    );
    let receipt_runtime = fresh_realtime_runtime();
    let drifted_planning = allocation_planning(
        &receipt_runtime,
        &drifted_plan_input,
        "realtime-overlay.stale-admission.no-render",
    );
    let stale_admission = receipt_runtime
        .admit_execution_lanes(
            &receipt_runtime.detached_allocation_receipt_for_test(&drifted_planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("drifted lane admission without render rows succeeds");
    context
        .runtime
        .prepare_hud_plan(
            &context.execution_plan,
            &context.allocation,
            &stale_admission,
            &context.hook_admissions,
            context.frame_policy,
        )
        .expect_err("HUD plan rejects stale lane admission before support interpretation")
}

pub(super) fn realtime_denial_for_mismatched_handle_allocation() -> WorthUiHudPlanDenial {
    let context = realtime_overlay_context();
    let drifted_plan_input = plan_input_with_drifted_diagnostics_ref(&context.plan_input);
    let receipt_runtime = fresh_realtime_runtime();
    let drifted_planning = allocation_planning(
        &receipt_runtime,
        &drifted_plan_input,
        "realtime-overlay.drifted-allocation",
    );
    let drifted_allocation = receipt_runtime
        .allocate_runtime_handles(
            &receipt_runtime.detached_allocation_receipt_for_test(&drifted_planning),
        )
        .expect("drifted handle allocation succeeds");
    context
        .runtime
        .prepare_hud_plan(
            &context.execution_plan,
            &drifted_allocation,
            &context.lane_admission,
            &context.hook_admissions,
            context.frame_policy,
        )
        .expect_err("HUD plan rejects mismatched handle allocation")
}

pub(super) fn realtime_denial_for_no_hud_rows() -> WorthUiHudPlanDenial {
    let context = realtime_overlay_context();
    let plan_input = plan_input_without_family(
        &context.plan_input,
        WorthUiPlanNodeInputFamily::RenderResourceRef,
    );
    let receipt_runtime = fresh_realtime_runtime();
    let planning = allocation_planning(
        &receipt_runtime,
        &plan_input,
        "realtime-overlay.no-hud-rows",
    );
    let allocation = receipt_runtime
        .allocate_runtime_handles(&receipt_runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handle allocation succeeds without render resource rows");
    let lane_admission = receipt_runtime
        .admit_execution_lanes(
            &receipt_runtime.detached_allocation_receipt_for_test(&planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("lane admission succeeds without render resource rows");
    let execution_plan = receipt_runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &receipt_runtime.detached_allocation_receipt_for_test(&planning),
            &allocation,
            &lane_admission,
        )
        .expect("execution plan topology assembles without render resource rows");
    let hook_admissions = realtime_hook_admissions(&receipt_runtime, &lane_admission);
    receipt_runtime
        .prepare_hud_plan(
            &execution_plan,
            &allocation,
            &lane_admission,
            &hook_admissions,
            context.frame_policy,
        )
        .expect_err("HUD plan rejects plan with no render resource rows")
}

fn fresh_realtime_runtime() -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    let app = impact_test_app();
    launch_runtime(&app, realtime_source_artifact(&app))
}

pub(super) struct RealtimeOverlayFixture {
    pub(super) runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pub(super) hud_plan: WorthUiHudPlan,
    pub(super) allocation: WorthUiRuntimeHandleAllocation,
}

struct RealtimeOverlayContext {
    runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    plan_input: WorthUiExecutionPlanInput,
    allocation: WorthUiRuntimeHandleAllocation,
    execution_plan: WorthUiExecutionPlan,
    lane_admission: WorthUiLaneAdmission,
    hook_admissions: Vec<WorthUiExtensionHookAdmission>,
    frame_policy: WorthUiHighFrequencyFramePolicy,
    hud_plan: WorthUiHudPlan,
}

impl RealtimeOverlayContext {
    fn into_fixture(self) -> RealtimeOverlayFixture {
        RealtimeOverlayFixture {
            runtime: self.runtime,
            hud_plan: self.hud_plan,
            allocation: self.allocation,
        }
    }
}

fn realtime_overlay_context() -> RealtimeOverlayContext {
    let app = impact_test_app();
    let active = realtime_source_artifact(&app);
    let candidate = realtime_source_artifact(&app);
    let runtime = launch_runtime(&app, active);
    let admitted = admitted_candidate(&app, &runtime, candidate);
    let pending = pending_plan_input(&runtime, admitted);
    let plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &realtime_component_hooks(),
        )
        .expect("execution plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "realtime-overlay.fixture");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handle allocation succeeds");
    let lane_admission = runtime
        .admit_execution_lanes(
            &runtime.detached_allocation_receipt_for_test(&planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("lane admission succeeds");
    let hook_admissions = realtime_hook_admissions(&runtime, &lane_admission);
    let execution_plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &runtime.detached_allocation_receipt_for_test(&planning),
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
            &hook_admissions,
            frame_policy,
        )
        .expect("HUD plan prepares");

    RealtimeOverlayContext {
        runtime,
        plan_input,
        allocation,
        execution_plan,
        lane_admission,
        hook_admissions,
        frame_policy,
        hud_plan,
    }
}

fn realtime_source_artifact(app: &crate::facade::WorthUiApp) -> crate::source::WorthUiArtifact {
    artifact_from_modules(
        app,
        [
            WorthUiRustAuthoredArtifactInputModule::new("app/realtime.wui")
                .with_component("workspace.component.dashboard")
                .with_surface("workspace.surface.command_save")
                .with_token("theme.text.primary", "#202830"),
        ],
    )
}

fn realtime_component_hooks() -> [WorthUiComponentLoweringHook; 5] {
    [
        WorthUiComponentLoweringHook::registered(
            "realtime.fixture.render_ref",
            WorthUiPlanNodeInputFamily::RenderResourceRef,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.fixture.command",
            WorthUiPlanNodeInputFamily::Command,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.fixture.accessibility",
            WorthUiPlanNodeInputFamily::Accessibility,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.fixture.diagnostics",
            WorthUiPlanNodeInputFamily::DiagnosticsRef,
        ),
        WorthUiComponentLoweringHook::registered(
            "realtime.fixture.lane",
            WorthUiPlanNodeInputFamily::LanePartitionRef,
        ),
    ]
}

fn realtime_hook_admissions(
    runtime: &WorthUiRuntime,
    lane_admission: &WorthUiLaneAdmission,
) -> Vec<WorthUiExtensionHookAdmission> {
    [WorthUiLaneAdapterHook::realtime_overlay_mechanics(
        "realtime.overlay.hud",
    )]
    .into_iter()
    .map(|hook| {
        runtime
            .admit_extension_hook(lane_admission, hook)
            .expect("realtime hook admits against platform support")
    })
    .collect()
}
