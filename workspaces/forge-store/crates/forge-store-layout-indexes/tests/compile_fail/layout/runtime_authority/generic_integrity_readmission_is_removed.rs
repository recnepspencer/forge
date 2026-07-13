use forge_store_layout_indexes::integrity::{layout_corruption, S8LayoutCorruptionOutcome};
use forge_store_recovery_physics::RecoveryLayoutReadmissionWitness;

fn misuse(required: S8LayoutCorruptionOutcome, witness: RecoveryLayoutReadmissionWitness) {
    let _ = layout_corruption().readmit_with(required, witness);
}

fn main() {}
