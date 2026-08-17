use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_headless::WorthUiHeadlessHost;
use worth_ui_runtime::facade::application::{
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryPlanSummaryRequest,
};
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn receipt_only_executor_allocations_match_the_public_call() {
    let scenario = FilesystemApplicationLifecycleScenario::new("executor-allocation");
    let workspace = FilesystemContractWorkspace::new("executor-allocation");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
    );
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem acquisition should read real .wui bytes");
    let capabilities = scenario.capability_application();
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .expect("the filesystem-authored application should launch publicly");
    let targets = [
        WorthUiOrdinaryPlanSummaryRequest::Component,
        WorthUiOrdinaryPlanSummaryRequest::ChildRange,
        WorthUiOrdinaryPlanSummaryRequest::Command,
        WorthUiOrdinaryPlanSummaryRequest::Token,
        WorthUiOrdinaryPlanSummaryRequest::StateSlot,
    ]
    .map(|request| {
        session
            .inspect_ordinary_plan(request)
            .expect("the active family index should summarize")
            .target()
            .expect("the real source should contain every ordinary target")
            .frame_target()
    });
    let expected_plan = session.inspect_runtime().active_plan_digest();
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| {
            panic!("an empty public framework turn should lend ordinary execution")
        });

    let mut frame = None;
    let allocations = allocation_counter::measure(|| {
        frame = Some(execution.execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell()));
    });

    let frame = frame
        .expect("the allocator observer should run the public call")
        .expect("the armed public ordinary frame call should execute");
    let cost = frame
        .cost_receipt()
        .expect("the real completion should derive its generation-bound counter receipt");
    assert_eq!(
        cost.counters().executor_allocation_count(),
        allocations.count_total,
        "executor accounting must reconcile with the complete public lane call"
    );
    assert_eq!(allocations.count_total, 0);
    for target in targets {
        execution
            .execute_ordinary_frame(target)
            .expect("summary-discovered repeated execution should succeed");
    }
    assert_eq!(cost.basis().active_plan(), expected_plan);
    assert!(cost
        .lane_receipts()
        .iter()
        .all(|lane| lane.work_scope().is_within_request()));
    assert!(frame.receipt().touch().row_count() > 0);
    drop(frame);
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}
