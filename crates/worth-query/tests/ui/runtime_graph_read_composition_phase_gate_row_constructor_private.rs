use worth_query::facade::runtime::{WorthQueryReadCompositionPhaseGateFamily, WorthQueryReadCompositionPhaseGateRow, WorthQueryReadCompositionPhaseGateStatus};

fn main() {
    let _ = WorthQueryReadCompositionPhaseGateRow {
        family: WorthQueryReadCompositionPhaseGateFamily::PhaseOneKernelComplete,
        status: WorthQueryReadCompositionPhaseGateStatus::Satisfied,
        reason: String::new(),
        row_digest: String::new(),
    };
}
