use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessEvidenceFamily, HarnessEvidenceRequirement, HarnessRunner, HarnessScenario,
    HarnessScenarioOperation, HarnessScenarioStep,
};

#[test]
fn harness_launch_uses_only_public_worth_ui_facade() {
    let app = WorthUi::app().freeze();
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(&app)
        .expect("minimal harness launch should prepare through public facade");
    let expected_snapshot_digest = app.capabilities().digest().as_u64();
    let scenario = HarnessScenario::define("harness.honesty.public-launch")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "launch-runtime",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt())
            .requires(HarnessEvidenceRequirement::active_plan_observation()),
        );

    let receipt = HarnessRunner::for_app(app)
        .run(scenario)
        .expect("public facade launch scenario should complete");

    receipt.assert_complete();
    assert_eq!(receipt.completed_steps(), 1);
    assert!(receipt
        .evidence()
        .contains(HarnessEvidenceFamily::RuntimeReceipt));
    assert!(receipt
        .evidence()
        .contains(HarnessEvidenceFamily::ActivePlanObservation));
    assert_eq!(
        receipt
            .evidence()
            .basis()
            .expect("runtime evidence should carry a basis")
            .snapshot_digest(),
        expected_snapshot_digest
    );
}
