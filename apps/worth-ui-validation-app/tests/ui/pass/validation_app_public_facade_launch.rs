use worth_ui::facade::{WorthUi, WorthUiRuntimeSourceModule};
use worth_ui_harness::facade::{
    HarnessEvidenceFamily, HarnessEvidenceRequirement, HarnessRunner, HarnessScenario,
    HarnessScenarioOperation, HarnessScenarioStep,
};

fn main() {
    let app = WorthUi::app().freeze();
    let launch = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new("validation/main.wui", ""))
        .prepare_for(&app)
        .expect("public launch should prepare");
    let scenario = HarnessScenario::define("validation.compile.public-launch")
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
        .expect("runtime launch should produce evidence");

    assert!(receipt.evidence().contains(HarnessEvidenceFamily::RuntimeReceipt));
}
