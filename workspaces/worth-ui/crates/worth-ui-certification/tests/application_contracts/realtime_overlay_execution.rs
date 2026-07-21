use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiBuilder};
use worth_ui::facade::host::WorthUiHeadlessHost;
use worth_ui::facade::registry::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentRealtimeOverlayContract, ComponentRealtimeOverlayPriority, ComponentStateOwnership,
};
use worth_ui::facade::runtime::{
    WorthUiHandleResolutionOutcome, WorthUiHudPlanDenialReason, WorthUiRealtimeFrameDenialReason,
    WorthUiRealtimeFrameTarget, WorthUiRealtimePlanAvailability, WorthUiRuntimeLaunchDenial,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementRequest, WorthUiHostCapability,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiHostOutputDisposition,
    WorthUiHostOutputEnvelope, WorthUiHostOutputLane, WorthUiHostOutputPayload,
    WorthUiMeasurementHostAdapter, WorthUiOperationalHostAdapter,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

const HUD: &str = "workspace.component.hud_0000";

#[test]
fn real_wui_realtime_overlay_executes_from_the_host_bound_active_plan() {
    let workspace = FilesystemContractWorkspace::new("realtime-overlay");
    let app = file_app(&workspace, &format!("component {HUD} {{}}\n"), hud_builder);
    let mut session = app.launch().expect("realtime active plan should seal");

    assert_eq!(
        session.realtime_plan_availability(),
        WorthUiRealtimePlanAvailability::Executable
    );
    let handle = session
        .first_realtime_renderer_surface()
        .expect("active realtime plan exposes one exact target");
    let summary = session
        .inspect_realtime_target(handle)
        .expect("compact target summary resolves");
    assert_eq!(summary.overlay_row_limit(), 8);
    assert_eq!(summary.policy().frame_budget_millis(), 16);
    assert_eq!(
        summary.host_session_identity(),
        session.host_session_identity().as_u64()
    );
    let host_session_identity = session.host_session_identity().as_u64();
    let execution = session
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("empty collection completes the framework turn"));
    let receipt = execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(handle))
        .expect("active realtime target executes directly");

    assert_eq!(receipt.touched_overlay_row_count(), 8);
    assert_eq!(receipt.touched_plan_indexes(), &[handle.plan_index()]);
    assert_eq!(receipt.counters().frame_synchronized_pass_count(), 1);
    assert_eq!(receipt.counters().targeted_overlay_row_count(), 8);
    assert_eq!(receipt.counters().ordinary_layout_pass_count(), 0);
    assert_eq!(receipt.counters().allocation_count(), 0);
    assert_eq!(
        receipt.certification().host_session_identity(),
        host_session_identity
    );
    assert_eq!(
        receipt.disposition(),
        WorthUiHostOutputDisposition::Consumed
    );
    assert_eq!(
        receipt.output().receipt_reference().lane(),
        WorthUiHostOutputLane::RealtimeOverlay
    );
    let output = match receipt.output().payload() {
        WorthUiHostOutputPayload::RealtimeOverlay(output) => output,
        _ => panic!("realtime execution must emit the realtime payload"),
    };
    assert_eq!(output.overlay_row_count(), 8);
    assert_ne!(output.meaning_digest(), 0);
    assert_ne!(receipt.output().receipt_reference().digest(), 0);
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}

#[test]
fn foreign_renderer_surface_denies_inspection_and_execution_before_work() {
    let first_workspace = FilesystemContractWorkspace::new("realtime-foreign-first");
    let second_workspace = FilesystemContractWorkspace::new("realtime-foreign-second");
    let source = format!("component {HUD} {{}}\n");
    let first = file_app(&first_workspace, &source, hud_builder)
        .launch()
        .expect("first realtime app launches");
    let foreign = first
        .first_realtime_renderer_surface()
        .expect("first renderer surface");
    let mut second = file_app(&second_workspace, &source, hud_builder)
        .launch()
        .expect("second realtime app launches");
    let inspection = second
        .inspect_realtime_target(foreign)
        .expect_err("foreign target cannot widen inspection authority");
    assert_eq!(
        inspection.outcome(),
        WorthUiHandleResolutionOutcome::ForeignSessionArena
    );
    let execution = second
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("framework turn execution"));
    let denial = execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(foreign))
        .expect_err("foreign surface denies before draw work");
    assert_eq!(
        denial.reason(),
        WorthUiRealtimeFrameDenialReason::TargetArenaMismatch
    );
    assert_eq!(denial.counters().frame_synchronized_pass_count(), 0);
    assert_eq!(denial.counters().renderer_surface_handoff_count(), 0);
    drop(execution);
    let _ = second.shutdown();
    let _ = first.shutdown();
    first_workspace.close();
    second_workspace.close();
}

#[test]
fn over_budget_or_unsupported_host_denies_before_active_publication() {
    let budget_workspace = FilesystemContractWorkspace::new("realtime-budget-denial");
    let budget_app = file_app(
        &budget_workspace,
        &format!("component {HUD} {{}}\n"),
        || hud_builder_with_policy(8, 17, 16, WorthUiHeadlessHost),
    );
    let budget_denial = match budget_app.launch() {
        Ok(_) => panic!("over-budget HUD cannot publish"),
        Err(denial) => denial,
    };
    assert!(matches!(
        budget_denial,
        WorthUiRuntimeLaunchDenial::RealtimeOverlayPlan(ref plan)
            if plan.reason() == WorthUiHudPlanDenialReason::FrameBudgetExhausted {
                budget_millis: 16,
                declared_cost_millis: 17,
            }
    ));

    let host_workspace = FilesystemContractWorkspace::new("realtime-host-denial");
    let host_app = file_app(&host_workspace, &format!("component {HUD} {{}}\n"), || {
        hud_builder_with_policy(8, 4, 16, MissingRealtimeHookHost)
    });
    let host_denial = match host_app.launch() {
        Ok(_) => panic!("host without exact realtime hook support cannot publish"),
        Err(denial) => denial,
    };
    assert!(matches!(
        host_denial,
        WorthUiRuntimeLaunchDenial::RealtimeOverlayPlan(ref plan)
            if plan.reason() == WorthUiHudPlanDenialReason::HostSupportMissing
    ));
    budget_workspace.close();
    host_workspace.close();
}

#[test]
fn one_target_work_is_independent_of_unrelated_and_other_hud_declarations() {
    let baseline = execute_scaled_target("realtime-scale-baseline", 1, 0);
    let unrelated_scale = execute_scaled_target("realtime-scale-ordinary", 1, 256);
    let hud_scale = execute_scaled_target("realtime-scale-hud", 128, 0);

    for scaled in [unrelated_scale, hud_scale] {
        assert_eq!(scaled, baseline);
    }
}

#[test]
fn real_wui_declaration_reordering_preserves_realtime_plan_and_frame_work() {
    const ORDINARY: &str = "workspace.component.ordinary_0000";
    let left_workspace = FilesystemContractWorkspace::new("realtime-order-left");
    let right_workspace = FilesystemContractWorkspace::new("realtime-order-right");
    let builder = || scaled_builder(1, 1);
    let mut left = file_app(
        &left_workspace,
        &format!("component {HUD} {{}}\ncomponent {ORDINARY} {{}}\n"),
        builder,
    )
    .launch()
    .expect("left order launches");
    let mut right = file_app(
        &right_workspace,
        &format!("component {ORDINARY} {{}}\ncomponent {HUD} {{}}\n"),
        builder,
    )
    .launch()
    .expect("right order launches");
    let left_handle = left.first_realtime_renderer_surface().expect("left target");
    let right_handle = right
        .first_realtime_renderer_surface()
        .expect("right target");
    let left_summary = left
        .inspect_realtime_target(left_handle)
        .expect("left summary");
    let right_summary = right
        .inspect_realtime_target(right_handle)
        .expect("right summary");
    assert_eq!(
        left_summary.plan_basis_digest(),
        right_summary.plan_basis_digest()
    );

    let left_execution = left
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("left framework turn"));
    let right_execution = right
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("right framework turn"));
    let left_receipt = left_execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(left_handle))
        .expect("left frame");
    let right_receipt = right_execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(right_handle))
        .expect("right frame");
    assert_eq!(
        left_receipt.certification().hud_plan_digest(),
        right_receipt.certification().hud_plan_digest()
    );
    assert_eq!(left_receipt.counters(), right_receipt.counters());
    assert_eq!(
        left_receipt.touched_overlay_row_count(),
        right_receipt.touched_overlay_row_count()
    );
    drop((left_execution, right_execution));
    let _ = left.shutdown();
    let _ = right.shutdown();
    left_workspace.close();
    right_workspace.close();
}

fn execute_scaled_target(
    workspace_name: &str,
    hud_count: usize,
    ordinary_count: usize,
) -> (u16, worth_ui::facade::runtime::WorthUiRealtimeLaneCounters) {
    let workspace = FilesystemContractWorkspace::new(workspace_name);
    let mut source = String::new();
    for index in 0..hud_count {
        source.push_str(&format!(
            "component workspace.component.hud_{index:04} {{}}\n"
        ));
    }
    for index in 0..ordinary_count {
        source.push_str(&format!(
            "component workspace.component.ordinary_{index:04} {{}}\n"
        ));
    }
    let app = file_app(&workspace, &source, || {
        scaled_builder(hud_count, ordinary_count)
    });
    let mut session = app.launch().expect("scaled realtime plan launches");
    let handle = session
        .first_realtime_renderer_surface()
        .expect("scaled plan has a target");
    let execution = session
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("framework turn execution"));
    let receipt = execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(handle))
        .expect("one indexed realtime target executes");

    assert_eq!(receipt.touched_plan_indexes().len(), 1);
    let work = (receipt.touched_overlay_row_count(), receipt.counters());
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
    work
}

fn hud_builder() -> WorthUiBuilder {
    hud_builder_with_policy(8, 4, 16, WorthUiHeadlessHost)
}

fn hud_builder_with_policy(
    rows: u16,
    cost: u16,
    budget: u32,
    host: impl WorthUiOperationalHostAdapter + 'static,
) -> WorthUiBuilder {
    WorthUi::app()
        .register_component(realtime_component(HUD, rows, cost, budget))
        .with_host(host)
}

fn scaled_builder(hud_count: usize, ordinary_count: usize) -> WorthUiBuilder {
    let mut builder = WorthUi::app().with_host(WorthUiHeadlessHost);
    for index in 0..hud_count {
        builder = builder.register_component(realtime_component(
            format!("workspace.component.hud_{index:04}"),
            8,
            4,
            16,
        ));
    }
    for index in 0..ordinary_count {
        builder = builder.register_component(ordinary_component(format!(
            "workspace.component.ordinary_{index:04}"
        )));
    }
    builder
}

fn realtime_component(
    id: impl Into<String>,
    rows: u16,
    cost: u16,
    budget: u32,
) -> ComponentDescriptor {
    ordinary_component(id).with_realtime_overlay_contract(
        ComponentRealtimeOverlayContract::new(
            rows,
            cost,
            budget,
            ComponentRealtimeOverlayPriority::HudOverlay,
        )
        .expect("realtime contract is structurally valid"),
    )
}

fn ordinary_component(id: impl Into<String>) -> ComponentDescriptor {
    let id = id.into();
    ComponentDescriptor::new(
        ComponentId::new(&id).expect("component id is valid"),
        ComponentPropSchema::named("realtime.contract.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn file_app(
    workspace: &FilesystemContractWorkspace,
    source: &str,
    builder: impl Fn() -> WorthUiBuilder,
) -> WorthUiApp {
    workspace.write("app/main.wui", source);
    let capabilities = builder().freeze().expect("capabilities freeze");
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem reader acquires source");
    let submission = snapshot
        .lower_to_candidate_submission(capabilities.capabilities())
        .expect("real source lowers through production semantics");
    builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("file-authored application prepares")
}

#[derive(Clone, Copy)]
struct MissingRealtimeHookHost;

impl WorthUiMeasurementHostAdapter for MissingRealtimeHookHost {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        unreachable!("missing host capabilities deny before observation")
    }
}

impl WorthUiOperationalHostAdapter for MissingRealtimeHookHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::RealtimeOverlayDraw,
            WorthUiHostCapability::RealtimeOverlaySurface,
        ])
    }

    fn consume_output(&self, _output: &WorthUiHostOutputEnvelope) -> WorthUiHostOutputDisposition {
        WorthUiHostOutputDisposition::Consumed
    }
}
