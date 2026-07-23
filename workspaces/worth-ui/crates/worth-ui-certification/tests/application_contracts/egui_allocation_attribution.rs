use worth_ui::facade::app::{WorthUiActiveOrdinaryFrameCompletion, WorthUiOrdinaryFrameTarget};
use worth_ui::facade::host::WorthUiHostOutputDisposition;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_egui::WorthUiHostEgui;

use super::allocation_observing_host::AllocationObservingHost;
use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn real_egui_frame_reconciles_non_overlapping_allocation_boundaries() {
    let scenario = FilesystemApplicationLifecycleScenario::new("egui-allocation-attribution");
    let workspace = FilesystemContractWorkspace::new("egui-allocation-attribution");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
    );
    let capabilities = scenario.capability_application();
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("real egui source settles"),
        capabilities.capabilities(),
    );
    let context = egui::Context::default();
    let (host, host_observation) =
        AllocationObservingHost::new(WorthUiHostEgui::new(context.clone()));
    let mut session = scenario
        .prepare_application_with_host(submission, host)
        .launch()
        .expect("the egui application launches");

    let _ = context.run(raw_input(), |_| {
        let execution = session
            .execute_framework_turn(|_| {})
            .into_execution()
            .unwrap_or_else(|_| panic!("egui warm-up framework turn"));
        execution
            .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
            .expect("egui warm-up frame executes");
    });

    let execution = session
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("measured egui framework turn"));
    let mut public_allocations = 0;
    let mut completion: Option<WorthUiActiveOrdinaryFrameCompletion> = None;
    let mut native_shape_count = 0;
    let native_allocations = allocation_counter::measure(|| {
        let native_output = context.run(raw_input(), |_| {
            let observed = allocation_counter::measure(|| {
                completion = Some(
                    execution
                        .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
                        .expect("measured egui frame executes"),
                );
            });
            public_allocations = observed.count_total;
        });
        native_shape_count = native_output.shapes.len();
    });

    let completion = completion.expect("the measured public frame returns a completion");
    assert_eq!(
        completion.disposition(),
        WorthUiHostOutputDisposition::Consumed
    );
    let host_allocations = host_observation.last_allocation_count();
    let executor_and_envelope_allocations = public_allocations
        .checked_sub(host_allocations)
        .expect("host translation is nested in the public frame call");
    let egui_native_allocations = native_allocations
        .count_total
        .checked_sub(public_allocations)
        .expect("the public frame call is nested in egui native execution");
    let production_count = completion
        .cost_receipt()
        .expect("the completion carries a cost receipt")
        .counters()
        .executor_allocation_count();

    assert_eq!(executor_and_envelope_allocations, production_count);
    assert!(
        host_allocations > 0,
        "real egui translation must be observed"
    );
    assert!(
        egui_native_allocations > 0,
        "real egui frame mechanics must be observed"
    );
    assert_eq!(
        native_allocations.count_total,
        executor_and_envelope_allocations + host_allocations + egui_native_allocations
    );
    assert_eq!(host_observation.call_count(), 2);
    assert!(native_shape_count > 0);

    drop(completion);
    drop(execution);
    let _ = session.shutdown();
    workspace.close();
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
        ..Default::default()
    }
}
