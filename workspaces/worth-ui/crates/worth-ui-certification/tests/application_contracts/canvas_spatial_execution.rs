use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};

type BoundBuilder = worth_ui_certification::scenario::application_authority_closure::FixedCertificationApplicationBuilder;
use worth_ui::facade::declaration::{
    ComponentCanvasSpatialContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_host_headless::{WorthUiHeadlessCapabilityProfileHost, WorthUiHeadlessHost};
use worth_ui_runtime::facade::execution::{
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane,
    WorthUiCanvasSpatialPlanAvailability, WorthUiCanvasViewportRequest,
    WorthUiHandleResolutionOutcome, WorthUiSpatialHitTestRequest, WorthUiSpatialViewportPoint,
};
use worth_ui_runtime::facade::runtime_handoff::WorthUiRuntimeLaunchDenial;
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

const CANVAS: &str = "workspace.component.canvas_contract";

#[test]
fn real_wui_canvas_executes_only_from_the_host_bound_active_plan() {
    let workspace = FilesystemContractWorkspace::new("canvas-spatial");
    let app = file_app(
        &workspace,
        &format!("component {CANVAS} {{}}\n"),
        canvas_builder,
    );
    let mut session = app.launch().expect("canvas active plan should seal");

    assert_eq!(
        session.canvas_spatial_plan_availability(),
        WorthUiCanvasSpatialPlanAvailability::Executable
    );
    let handle = session
        .first_canvas_spatial_handle()
        .expect("active canvas plan exposes one exact target handle");
    let host_session_identity = session.host_session_identity().as_u64();
    let summary = session
        .inspect_canvas_spatial_target(handle)
        .expect("indexed summary should not materialize the spatial index");
    assert_eq!(summary.visible_primitive_limit(), 2_048);
    assert_eq!(summary.host_session_identity(), host_session_identity);
    assert_eq!(
        summary.strategy(),
        worth_ui_runtime::facade::execution::WorthUiSpatialIndexStrategy::Tiled
    );
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty collection closes a framework turn"));
    let first_receipt = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::hit_test(
            WorthUiSpatialHitTestRequest::for_viewport_point(
                handle,
                WorthUiSpatialViewportPoint::viewport(144, 96),
            ),
        ))
        .expect("active spatial hit test should execute directly");

    assert_eq!(first_receipt.lane(), WorthUiCanvasSpatialLane::HitTest);
    assert_eq!(first_receipt.queried_hit_test_region_count(), 1);
    assert_eq!(first_receipt.touched_plan_indexes(), &[handle.plan_index()]);
    assert_eq!(
        first_receipt.certification().host_session_identity(),
        host_session_identity
    );
    assert_eq!(
        first_receipt.counters().skipped_noncanvas_plan_row_count(),
        0
    );
    assert_ne!(first_receipt.touch_digest(), 0);
    let second_receipt = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::hit_test(
            WorthUiSpatialHitTestRequest::for_viewport_point(
                handle,
                WorthUiSpatialViewportPoint::viewport(-8, 512),
            ),
        ))
        .expect("a second exact hit-test point should execute");
    assert_ne!(first_receipt.touch_digest(), second_receipt.touch_digest());

    let viewport_receipt = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::viewport(
            WorthUiCanvasViewportRequest::pan_zoom(handle, 12, -4, 1_250)
                .expect("positive zoom factor"),
        ))
        .expect("exact viewport request should execute");
    assert_eq!(viewport_receipt.counters().viewport_transform_count(), 1);

    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}

#[test]
fn foreign_active_session_handle_denies_before_spatial_work() {
    let first_workspace = FilesystemContractWorkspace::new("canvas-foreign-first");
    let second_workspace = FilesystemContractWorkspace::new("canvas-foreign-second");
    let source = format!("component {CANVAS} {{}}\n");
    let first = file_app(&first_workspace, &source, canvas_builder)
        .launch()
        .expect("first canvas app launches");
    let foreign_handle = first.first_canvas_spatial_handle().expect("first handle");
    let mut second = file_app(&second_workspace, &source, canvas_builder)
        .launch()
        .expect("second canvas app launches");
    let inspection_denial = second
        .inspect_canvas_spatial_target(foreign_handle)
        .expect_err("inspection must not widen a foreign target into renderer authority");
    assert_eq!(
        inspection_denial.outcome(),
        WorthUiHandleResolutionOutcome::ForeignSessionArena
    );
    let execution = second
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("framework turn execution"));
    let denial = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(foreign_handle))
        .expect_err("foreign session handle must not enter frame work");

    assert_eq!(
        denial.reason(),
        worth_ui_runtime::facade::execution::WorthUiCanvasSpatialFrameDenialReason::TargetArenaMismatch
    );
    assert_eq!(denial.counters().draw_pass_count(), 0);
    assert_eq!(denial.counters().spatial_hit_test_count(), 0);
    drop(execution);
    let _ = second.shutdown();
    let _ = first.shutdown();
    first_workspace.close();
    second_workspace.close();
}

#[test]
fn missing_host_spatial_support_denies_before_publication() {
    let workspace = FilesystemContractWorkspace::new("canvas-host-denial");
    let app = file_app(
        &workspace,
        &format!("component {CANVAS} {{}}\n"),
        unsupported_canvas_builder,
    );
    let denial = match app.launch() {
        Ok(_) => panic!("unsupported host cannot publish canvas plan"),
        Err(denial) => denial,
    };

    assert!(matches!(
        denial,
        WorthUiRuntimeLaunchDenial::CanvasSpatialPlan(ref plan)
            if plan.reason() == worth_ui_runtime::facade::execution::WorthUiCanvasSpatialPlanDenialReason::HostSupportMissing
    ));
    workspace.close();
}

#[test]
fn unrelated_ordinary_source_does_not_expand_one_spatial_target() {
    const ORDINARY_COUNT: usize = 256;
    let workspace = FilesystemContractWorkspace::new("canvas-scale");
    let mut source = format!("component {CANVAS} {{}}\n");
    for index in 0..ORDINARY_COUNT {
        source.push_str(&format!(
            "component workspace.component.ordinary_{index} {{}}\n"
        ));
    }
    let app = file_app(&workspace, &source, || {
        canvas_builder_with_ordinary(ORDINARY_COUNT)
    });
    let mut session = app.launch().expect("mixed plan launches");
    let handle = session
        .first_canvas_spatial_handle()
        .expect("canvas handle");
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("framework turn execution"));
    let receipt = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(handle))
        .expect("indexed canvas draw executes");

    assert_eq!(receipt.visible_primitive_count(), 2_048);
    assert_eq!(receipt.touched_plan_indexes().len(), 1);
    assert_eq!(receipt.counters().skipped_noncanvas_plan_row_count(), 0);
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}

#[test]
fn lawful_source_reordering_preserves_spatial_plan_behavior() {
    let left_workspace = FilesystemContractWorkspace::new("canvas-order-left");
    let right_workspace = FilesystemContractWorkspace::new("canvas-order-right");
    let ordinary = "workspace.component.order_peer";
    let left_source = format!("component {CANVAS} {{}}\ncomponent {ordinary} {{}}\n");
    let right_source = format!("component {ordinary} {{}}\ncomponent {CANVAS} {{}}\n");
    let builder = || canvas_builder().register_component(ordinary_component(ordinary));
    let left = file_app(&left_workspace, &left_source, builder)
        .launch()
        .expect("left ordering launches");
    let right = file_app(&right_workspace, &right_source, builder)
        .launch()
        .expect("right ordering launches");
    let left_summary = left
        .inspect_canvas_spatial_target(left.first_canvas_spatial_handle().expect("left handle"))
        .expect("left summary");
    let right_summary = right
        .inspect_canvas_spatial_target(right.first_canvas_spatial_handle().expect("right handle"))
        .expect("right summary");

    assert_eq!(left_summary.strategy(), right_summary.strategy());
    assert_eq!(
        left_summary.visible_primitive_limit(),
        right_summary.visible_primitive_limit()
    );
    assert_eq!(
        left_summary.overlay_row_limit(),
        right_summary.overlay_row_limit()
    );
    assert_eq!(
        left_summary.tool_state_row_limit(),
        right_summary.tool_state_row_limit()
    );

    let _ = left.shutdown();
    let _ = right.shutdown();
    left_workspace.close();
    right_workspace.close();
}

fn canvas_builder() -> BoundBuilder {
    BoundBuilder::new(canvas_descriptor_builder(), WorthUiHeadlessHost)
}

fn canvas_builder_with_ordinary(count: usize) -> BoundBuilder {
    let mut builder = canvas_builder();
    for index in 0..count {
        builder = builder.register_component(ordinary_component(format!(
            "workspace.component.ordinary_{index}"
        )));
    }
    builder
}

fn unsupported_canvas_builder() -> BoundBuilder {
    BoundBuilder::new(
        canvas_descriptor_builder(),
        WorthUiHeadlessCapabilityProfileHost::missing_canvas_hit_test(),
    )
}

fn canvas_descriptor_builder() -> WorthUiApplicationBuilder {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_component(canvas_descriptor())
}

fn canvas_descriptor() -> ComponentDescriptor {
    let contract =
        ComponentCanvasSpatialContract::new(2_048, 8, 3).expect("positive primitive limit");
    ordinary_component(CANVAS).with_canvas_spatial_contract(contract)
}

fn ordinary_component(id: impl Into<String>) -> ComponentDescriptor {
    let id = id.into();
    ComponentDescriptor::new(
        ComponentId::new(id).expect("valid component id"),
        ComponentPropSchema::named("canvas.contract.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn file_app(
    workspace: &FilesystemContractWorkspace,
    source: &str,
    builder: impl Fn() -> BoundBuilder,
) -> WorthUiApp {
    workspace.write("app/main.wui", source);
    let capability_app = builder().freeze().expect("capabilities should freeze");
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem reader should acquire source");
    let submission = snapshot
        .attempt_candidate_for_certification(capability_app.capabilities())
        .expect("real source should lower through production semantics");
    builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("file-authored application should prepare")
}
