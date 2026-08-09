use worth_ui::facade::app::WorthUiVisibleRange;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_headless::WorthUiHeadlessHost;
use worth_ui_runtime::facade::application::{
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryPlanAvailability, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedPlanAvailability, WorthUiVirtualizedPlanSummaryRequest,
};
use worth_ui_runtime::facade::entry::{
    WorthUiActiveFrameworkTurnExecution, WorthUiActiveOrdinaryFrameCompletion,
};
use worth_ui_runtime::facade::execution::{
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialPlanAvailability,
    WorthUiFrameExecutionReceipt, WorthUiLaneHandle, WorthUiRealtimeFrameTarget,
    WorthUiRealtimePlanAvailability, WorthUiRendererSurfaceHandle,
};
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[derive(Clone, Copy)]
struct CrossLaneTargets {
    virtualized: WorthUiVirtualizedDataFrameTarget,
    canvas: WorthUiLaneHandle,
    realtime: WorthUiRendererSurfaceHandle,
}

#[test]
fn one_real_file_authored_bundle_executes_every_sealed_lane_posture() {
    let (mut scenario, workspace, mut session) = launch_cross_lane_world();
    let active_plan_digest = assert_cross_lane_bundle_contract(&session);
    admit_settled_query_projection(&mut scenario, &mut session);

    let targets = select_cross_lane_targets(&session);
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("cross-lane execution turn"));
    warm_cross_lane_execution(&execution, targets);
    let mut ordinary = None;
    let mut virtualized = None;
    let mut canvas = None;
    let mut realtime = None;
    let ordinary_allocations = allocation_counter::measure(|| {
        ordinary = Some(execution.execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell()));
    });
    let virtualized_allocations = allocation_counter::measure(|| {
        virtualized = Some(execution.execute_virtualized_data_frame(targets.virtualized));
    });
    let canvas_allocations =
        allocation_counter::measure(|| {
            canvas = Some(execution.execute_canvas_spatial_frame(
                WorthUiCanvasSpatialFrameTarget::draw(targets.canvas),
            ));
        });
    let realtime_allocations = allocation_counter::measure(|| {
        realtime = Some(execution.execute_realtime_frame(
            WorthUiRealtimeFrameTarget::renderer_surface(targets.realtime),
        ));
    });
    let ordinary = ordinary.unwrap().expect("ordinary lane executes");
    let virtualized = virtualized.unwrap().expect("virtualized lane executes");
    let canvas = canvas.unwrap().expect("canvas lane executes");
    let realtime = realtime.unwrap().expect("realtime lane executes");
    assert_ordinary_visual_lane_is_cold(&ordinary);
    assert_repeated_unchanged_ordinary_visual_lane_is_cold(&execution);
    let costs = [
        ordinary.cost_receipt().expect("ordinary cost certifies"),
        virtualized
            .cost_receipt()
            .expect("virtualized cost certifies"),
        canvas.cost_receipt().expect("canvas cost certifies"),
        realtime.cost_receipt().expect("realtime cost certifies"),
    ];
    let observed_public = [
        ordinary_allocations.count_total,
        virtualized_allocations.count_total,
        canvas_allocations.count_total,
        realtime_allocations.count_total,
    ];
    assert_lane_cost_contract(&costs, observed_public, active_plan_digest);

    drop((ordinary, virtualized, canvas, realtime));
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}

fn assert_ordinary_visual_lane_is_cold(ordinary: &WorthUiActiveOrdinaryFrameCompletion<'_>) {
    assert_eq!(
        ordinary.receipt().visual_inspection_cost().counters(),
        [0; 11],
        "an ordinary frame performs no snapshot, query, overlay, or retention work"
    );
}

fn assert_repeated_unchanged_ordinary_visual_lane_is_cold(
    execution: &WorthUiActiveFrameworkTurnExecution<'_>,
) {
    for frame in 0..3 {
        let ordinary = execution
            .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
            .expect("an unchanged ordinary frame executes");
        assert_eq!(
            ordinary.receipt().visual_inspection_cost().counters(),
            [0; 11],
            "unchanged ordinary frame {frame} performs no visual inspection work"
        );
    }
}

fn select_cross_lane_targets(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> CrossLaneTargets {
    CrossLaneTargets {
        virtualized: session
            .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
            .expect("installed Query view is in the same bundle")
            .target(WorthUiVisibleRange::rows(0, 1).expect("one visible row")),
        canvas: session
            .first_canvas_spatial_handle()
            .expect("same bundle has a canvas target"),
        realtime: session
            .first_realtime_renderer_surface()
            .expect("same bundle has a realtime target"),
    }
}

fn warm_cross_lane_execution(
    execution: &WorthUiActiveFrameworkTurnExecution<'_>,
    targets: CrossLaneTargets,
) {
    let _ = execution
        .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
        .expect("ordinary lane warm-up executes");
    let _ = execution
        .execute_virtualized_data_frame(targets.virtualized)
        .expect("virtualized lane warm-up executes");
    let _ = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(targets.canvas))
        .expect("canvas lane warm-up executes");
    let _ = execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(
            targets.realtime,
        ))
        .expect("realtime lane warm-up executes");
}

fn launch_cross_lane_world() -> (
    FilesystemApplicationLifecycleScenario,
    FilesystemContractWorkspace,
    worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let scenario = FilesystemApplicationLifecycleScenario::new("cross-lane-bundle");
    let workspace = FilesystemContractWorkspace::new("cross-lane-bundle");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::cross_lane_source_text(),
    );
    let capabilities = scenario.cross_lane_capability_application(WorthUiHeadlessHost);
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("real cross-lane source settles"),
        capabilities.capabilities(),
    );
    let session = scenario
        .prepare_cross_lane_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .expect("one cross-lane bundle seals");
    (scenario, workspace, session)
}

fn assert_cross_lane_bundle_contract(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> u64 {
    let observation = session.inspect_runtime();
    let bundle = observation.cross_lane_bundle();
    assert_eq!(bundle.plan_digest().raw(), observation.active_plan_digest());
    assert_ne!(bundle.handle_allocation_basis_digest(), 0);
    assert_ne!(bundle.lane_support_digest(), 0);
    assert_ne!(bundle.lane_plan_input_basis_digest(), 0);
    assert_eq!(
        bundle.ordinary(),
        WorthUiOrdinaryPlanAvailability::Executable
    );
    assert_eq!(
        bundle.virtualized(),
        WorthUiVirtualizedPlanAvailability::Executable
    );
    assert_eq!(
        bundle.canvas_spatial(),
        WorthUiCanvasSpatialPlanAvailability::Executable
    );
    assert_eq!(
        bundle.realtime_overlay(),
        WorthUiRealtimePlanAvailability::Executable
    );
    let equivalence = bundle.plan_digest().basis();
    let construction = bundle.construction_counters();
    assert_eq!(
        equivalence.plan_node_count(),
        construction.topology().topology_node_count()
    );
    assert_eq!(
        equivalence.render_resource_ref_count(),
        construction.topology().render_resource_ref_count()
    );
    assert!(equivalence.lane_partition_count() >= 4);
    bundle.plan_digest().raw()
}

fn admit_settled_query_projection(
    scenario: &mut FilesystemApplicationLifecycleScenario,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let projection = scenario.settled_query_projection();
    let fact_link = session
        .query_fact_link("inspector.measurements")
        .expect("active plan retains one compact Query fact link");
    let mut projection_admitted = false;
    let turn = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|source| {
                projection_admitted = source.admit_settled(projection).is_ok()
                    && source.submit_settled(&fact_link).is_ok();
            });
        })
        .expect("no mounted presentation lease is active");
    assert!(projection_admitted);
    drop(turn.into_completion());
}

fn assert_lane_cost_contract(
    costs: &[WorthUiFrameExecutionReceipt; 4],
    observed_public: [u64; 4],
    active_plan_digest: u64,
) {
    for (lane, (cost, public)) in ["ordinary", "virtualized", "canvas", "realtime"]
        .into_iter()
        .zip(costs.iter().zip(observed_public))
    {
        assert_eq!(
            cost.counters().executor_allocation_count(),
            public,
            "independent {lane} executor observation must match receipt-only execution"
        );
    }
    assert!(costs.iter().all(|cost| {
        cost.lane_receipts()
            .iter()
            .all(|lane| lane.work_scope().is_within_request())
    }));
    assert!(costs
        .iter()
        .all(|cost| cost.active_plan_digest() == active_plan_digest));
}
