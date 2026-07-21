use worth_ui::facade::app::{WorthUiOrdinaryFrameTarget, WorthUiOrdinaryPlanSummaryRequest};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::allocation_observing_host::AllocationObservingHost;
use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::headless_output_observer::ObservingHeadlessHost;

#[test]
fn executor_and_envelope_allocations_exclude_host_translation() {
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
    let (observing_host, host_observation) = ObservingHeadlessHost::new();
    let (host, allocation_observation) = AllocationObservingHost::new(observing_host);
    let mut session = scenario
        .prepare_application_with_host(submission, host)
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
    let execution = session
        .execute_framework_turn(|_| {})
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
    let host_allocations = allocation_observation.last_allocation_count();
    let executor_and_envelope_allocations = allocations
        .count_total
        .checked_sub(host_allocations)
        .expect("nested host observation must be contained in the public call");
    let cost = frame
        .cost_receipt()
        .expect("the real completion should derive its generation-bound counter receipt");
    assert_eq!(
        cost.counters().executor_allocation_count(),
        executor_and_envelope_allocations,
        "executor accounting must reconcile after excluding host translation"
    );
    assert_eq!(executor_and_envelope_allocations, 0);
    assert_eq!(allocation_observation.call_count(), 1);
    for target in targets {
        execution
            .execute_ordinary_frame(target)
            .expect("summary-discovered repeated execution should succeed");
    }
    assert_eq!(cost.generation(), frame.output().generation());
    assert!(cost
        .lane_receipts()
        .iter()
        .all(|lane| lane.work_scope().is_within_request()));
    assert!(frame.receipt().touch().row_count() > 0);
    assert_eq!(
        host_observation.snapshot().call_count,
        1 + targets.len() as u64
    );
    assert_eq!(
        allocation_observation.call_count(),
        1 + targets.len() as u64
    );
    drop(frame);
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}
