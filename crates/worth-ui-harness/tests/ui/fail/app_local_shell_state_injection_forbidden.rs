use worth_ui_harness::facade::{
    HarnessEvidenceBundle, HarnessRunReceipt, HarnessScenarioId,
};

fn main() {
    let scenario_id = HarnessScenarioId::new("harness.fake").unwrap();
    let _ = HarnessRunReceipt {
        scenario_id,
        evidence: HarnessEvidenceBundle::empty(),
        completed_steps: 1,
    };
}
