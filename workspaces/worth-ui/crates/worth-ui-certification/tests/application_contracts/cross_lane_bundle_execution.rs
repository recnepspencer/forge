use worth_ui::facade::app::{
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryPlanAvailability,
    WorthUiVirtualizedPlanAvailability, WorthUiVirtualizedPlanSummaryRequest, WorthUiVisibleRange,
};
use worth_ui::facade::host::WorthUiHeadlessHost;
use worth_ui::facade::runtime::{
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialPlanAvailability,
    WorthUiRealtimeFrameTarget, WorthUiRealtimePlanAvailability,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn one_real_file_authored_bundle_executes_every_sealed_lane_posture() {
    let mut scenario = FilesystemApplicationLifecycleScenario::new("cross-lane-bundle");
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
    let mut session = scenario
        .prepare_cross_lane_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .expect("one cross-lane bundle seals");

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

    let projection = scenario.settled_query_projection();
    let fact_link = session
        .query_fact_link("inspector.measurements")
        .expect("active plan retains one compact Query fact link");
    let mut projection_admitted = false;
    let projection_turn = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|source| {
                projection_admitted = source.admit_settled(projection).is_ok()
                    && source.submit_settled(&fact_link).is_ok();
            });
        })
        .expect("no mounted presentation lease is active");
    assert!(projection_admitted);
    drop(projection_turn.into_completion());

    let virtualized_target = session
        .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
        .expect("installed Query view is in the same bundle")
        .target(WorthUiVisibleRange::rows(0, 1).expect("one visible row"));
    let canvas_handle = session
        .first_canvas_spatial_handle()
        .expect("same bundle has a canvas target");
    let realtime_handle = session
        .first_realtime_renderer_surface()
        .expect("same bundle has a realtime target");
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("cross-lane execution turn"));
    let _ = execution
        .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
        .expect("ordinary lane warm-up executes");
    let _ = execution
        .execute_virtualized_data_frame(virtualized_target)
        .expect("virtualized lane warm-up executes");
    let _ = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(canvas_handle))
        .expect("canvas lane warm-up executes");
    let _ = execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(
            realtime_handle,
        ))
        .expect("realtime lane warm-up executes");
    let mut ordinary = None;
    let mut virtualized = None;
    let mut canvas = None;
    let mut realtime = None;
    let ordinary_allocations = allocation_counter::measure(|| {
        ordinary = Some(execution.execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell()));
    });
    let virtualized_allocations = allocation_counter::measure(|| {
        virtualized = Some(execution.execute_virtualized_data_frame(virtualized_target));
    });
    let canvas_allocations = allocation_counter::measure(|| {
        canvas = Some(
            execution
                .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(canvas_handle)),
        );
    });
    let realtime_allocations = allocation_counter::measure(|| {
        realtime = Some(execution.execute_realtime_frame(
            WorthUiRealtimeFrameTarget::renderer_surface(realtime_handle),
        ));
    });
    let ordinary = ordinary.unwrap().expect("ordinary lane executes");
    let virtualized = virtualized.unwrap().expect("virtualized lane executes");
    let canvas = canvas.unwrap().expect("canvas lane executes");
    let realtime = realtime.unwrap().expect("realtime lane executes");
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
        .all(|cost| cost.active_plan_digest() == bundle.plan_digest().raw()));

    drop((ordinary, virtualized, canvas, realtime));
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}
