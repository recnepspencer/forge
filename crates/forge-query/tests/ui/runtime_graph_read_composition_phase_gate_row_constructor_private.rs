use forge_query::facade::{
    ForgeQueryReadCompositionPhaseGateFamily, ForgeQueryReadCompositionPhaseGateRow,
    ForgeQueryReadCompositionPhaseGateStatus,
};

fn main() {
    let _ = ForgeQueryReadCompositionPhaseGateRow {
        family: ForgeQueryReadCompositionPhaseGateFamily::PhaseOneKernelComplete,
        status: ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
        reason: String::new(),
        row_digest: String::new(),
    };
}
