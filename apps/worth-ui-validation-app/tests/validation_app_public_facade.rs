use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessEvidenceFamily, HarnessEvidenceRequirement, HarnessScenario, HarnessScenarioOperation,
    HarnessScenarioStep,
};
use worth_ui_validation_app::honesty::ValidationAppHonestyBoundary;

#[test]
fn validation_app_launch_uses_only_public_worth_ui_facade() {
    let app = WorthUi::app().freeze();
    let launch = ValidationAppHonestyBoundary::prepare_public_facade_launch(&app)
        .expect("validation app launch should prepare through the public facade");
    let scenario = HarnessScenario::define("validation.honesty.public-launch")
        .expect("valid scenario id")
        .step(
            HarnessScenarioStep::new(
                "launch-runtime",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt())
            .requires(HarnessEvidenceRequirement::active_plan_observation()),
        );

    let receipt = ValidationAppHonestyBoundary::runner_for_public_app(app)
        .run(scenario)
        .expect("public facade launch should produce runtime evidence");

    assert!(receipt
        .evidence()
        .contains(HarnessEvidenceFamily::RuntimeReceipt));
    assert!(receipt
        .evidence()
        .contains(HarnessEvidenceFamily::ActivePlanObservation));
    ValidationAppHonestyBoundary::require_runtime_backed_receipt(&receipt)
        .expect("runtime-backed receipt should pass validation app evidence gate");
}
