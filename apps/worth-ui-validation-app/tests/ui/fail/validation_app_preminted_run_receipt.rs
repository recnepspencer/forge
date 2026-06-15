use worth_ui_harness::facade::{
    HarnessEvidenceBundle, HarnessEvidenceLedger, HarnessReplayRecord, HarnessRunReceipt,
    HarnessScenarioId,
};

fn main() {
    let _receipt = HarnessRunReceipt {
        scenario_id: scenario_id(),
        evidence: evidence_bundle(),
        evidence_ledger: evidence_ledger(),
        replay_record: replay_record(),
        completed_steps: completed_steps(),
    };
}

fn scenario_id() -> HarnessScenarioId {
    panic!("compile-fail fixture")
}

fn evidence_bundle() -> HarnessEvidenceBundle {
    panic!("compile-fail fixture")
}

fn evidence_ledger() -> HarnessEvidenceLedger {
    panic!("compile-fail fixture")
}

fn replay_record() -> HarnessReplayRecord {
    panic!("compile-fail fixture")
}

fn completed_steps() -> usize {
    panic!("compile-fail fixture")
}
