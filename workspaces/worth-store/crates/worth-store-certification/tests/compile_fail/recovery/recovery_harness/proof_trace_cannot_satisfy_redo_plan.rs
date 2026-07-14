use worth_store_recovery_physics::{ProofProgressionRecoveryTrace, RecoveryRedoPlan};

fn requires_redo_plan(_: RecoveryRedoPlan) {}

fn main() {
    let trace: ProofProgressionRecoveryTrace = todo!();
    requires_redo_plan(trace);
}
