use worth_ui::facade::app::WorthUiPlanRegionStorageCounters;
use worth_ui::facade::host::WorthUiHeadlessHost;
use worth_ui::facade::runtime::{WorthUiCanvasSpatialFrameTarget, WorthUiHandleResolutionOutcome};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::filesystem_replacement_support::activate_current_filesystem_candidate;

const CANVAS_DECLARATION: &str = "component workspace.component.cross_lane_canvas {}\n";

#[test]
fn public_replacement_retires_and_remints_exact_canvas_resource_generation() {
    let scenario = FilesystemApplicationLifecycleScenario::new("canvas-resource-generation");
    let workspace = FilesystemContractWorkspace::new("canvas-resource-generation");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::cross_lane_source_text(),
    );
    let capabilities = scenario.cross_lane_capability_application(WorthUiHeadlessHost);
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("predecessor source settles from disk"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_cross_lane_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .expect("predecessor cross-lane application launches");
    let stale = session
        .first_canvas_spatial_handle()
        .expect("predecessor canvas resource handle");
    let predecessor = session
        .inspect_canvas_spatial_target(stale)
        .expect("predecessor canvas resource summary");

    let without_canvas = FilesystemApplicationLifecycleScenario::cross_lane_source_text()
        .replace(CANVAS_DECLARATION, "");
    workspace.write("app/main.wui", &without_canvas);
    let removal = activate_current_filesystem_candidate(&workspace, &mut session)
        .expect("canvas removal activates")
        .into_activation()
        .expect("canvas removal changes executable meaning");
    assert!(removal.query_retirement().is_empty());
    assert_eq!(
        session
            .inspect_canvas_spatial_target(stale)
            .expect_err("retired renderer resource cannot inspect successor truth")
            .outcome(),
        WorthUiHandleResolutionOutcome::TargetMissing
    );

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::cross_lane_source_text(),
    );
    let reinsertion = activate_current_filesystem_candidate(&workspace, &mut session)
        .expect("canvas reinsertion activates")
        .into_activation()
        .expect("canvas reinsertion changes executable meaning");
    assert!(reinsertion.query_retirement().is_empty());
    assert_eq!(
        session
            .inspect_canvas_spatial_target(stale)
            .expect_err("predecessor resource generation cannot resolve the reminted slot")
            .outcome(),
        WorthUiHandleResolutionOutcome::TargetMissing
    );
    let fresh = session
        .first_canvas_spatial_handle()
        .expect("successor canvas resource handle");
    assert_ne!(fresh, stale);
    let successor = session
        .inspect_canvas_spatial_target(fresh)
        .expect("successor resource summary");
    assert_eq!(
        successor.host_session_identity(),
        predecessor.host_session_identity()
    );
    assert_ne!(
        successor.plan_basis_digest(),
        predecessor.plan_basis_digest()
    );

    let execution = session
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("successor execution turn"));
    let denial = execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(stale))
        .expect_err("stale resource denies before successor draw work");
    assert_eq!(denial.counters().draw_pass_count(), 0);
    assert_eq!(denial.counters().spatial_hit_test_count(), 0);
    drop(execution);

    let _ = session.shutdown();
    workspace.close();
}

#[test]
fn public_large_canvas_replacement_rebuilds_only_the_removed_region() {
    let small = remove_first_canvas_region(4);
    let large = remove_first_canvas_region(128);

    assert!(large.successor_region_count > small.successor_region_count * 10);
    assert_eq!(small.affected_region_count, large.affected_region_count);
    assert_eq!(
        small.exact_region_comparison_count,
        large.exact_region_comparison_count
    );
    assert_eq!(small.regional_storage, large.regional_storage);
    assert!(small.successor_region_count > small.affected_region_count);
    assert!(large.successor_region_count > large.affected_region_count * 10);
}

#[derive(Clone, Copy)]
struct CanvasReplacementEvidence {
    successor_region_count: usize,
    affected_region_count: usize,
    exact_region_comparison_count: usize,
    regional_storage: WorthUiPlanRegionStorageCounters,
}

fn remove_first_canvas_region(canvas_count: usize) -> CanvasReplacementEvidence {
    let label = format!("public-large-canvas-{canvas_count}");
    let scenario = FilesystemApplicationLifecycleScenario::new(&label);
    let workspace = FilesystemContractWorkspace::new(&label);
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::scaled_canvas_source_text(canvas_count, false),
    );
    let capabilities =
        scenario.scaled_canvas_capability_application(WorthUiHeadlessHost, canvas_count);
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("scaled predecessor source settles from disk"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_scaled_canvas_application_with_host(submission, WorthUiHeadlessHost, canvas_count)
        .launch()
        .expect("scaled predecessor canvas application launches");

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::scaled_canvas_source_text(canvas_count, true),
    );
    let cutover = activate_current_filesystem_candidate(&workspace, &mut session)
        .expect("one-region canvas removal activates")
        .into_activation()
        .expect("one-region canvas removal changes executable meaning");
    assert!(cutover.query_retirement().is_empty());
    let summary = cutover
        .plan_decision()
        .summary()
        .expect("bounded canvas replacement carries equivalence evidence");
    let observation = session.inspect_runtime();
    let regional_storage = observation
        .cross_lane_bundle()
        .construction_counters()
        .regional_storage();
    let evidence = CanvasReplacementEvidence {
        successor_region_count: observation
            .cross_lane_bundle()
            .plan_digest()
            .basis()
            .plan_node_count(),
        affected_region_count: cutover.structural_reuse().affected_region_count(),
        exact_region_comparison_count: summary.exact_region_comparison_count(),
        regional_storage,
    };
    let _ = session.shutdown();
    workspace.close();
    evidence
}
