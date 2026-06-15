use worth_ui::facade::WorthUi;
use worth_ui_harness::facade::{
    HarnessEvidenceRequirement, HarnessRunner, HarnessScenario, HarnessScenarioOperation,
    HarnessScenarioStep,
};

fn main() {
    let app = WorthUi::app().freeze();
    let launch = worth_ui_harness::workbench::minimal_workbench_launch(&app).unwrap();
    let scenario = HarnessScenario::define("harness.compile.public-launch")
        .unwrap()
        .step(
            HarnessScenarioStep::new(
                "launch-runtime",
                HarnessScenarioOperation::launch_runtime(launch),
            )
            .requires(HarnessEvidenceRequirement::runtime_receipt()),
        );
    let _ = HarnessRunner::for_app(app).run(scenario).unwrap();
}
