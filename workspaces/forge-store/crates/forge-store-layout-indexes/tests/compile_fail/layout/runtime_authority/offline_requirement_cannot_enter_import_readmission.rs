use forge_store_layout_indexes::integrity::{
    layout_corruption, OfflineReadmissionRequirement,
};
use forge_store_recovery_physics::RecoveryLayoutReadmissionWitness;

fn misuse(required: OfflineReadmissionRequirement, witness: RecoveryLayoutReadmissionWitness) {
    let _ = layout_corruption().readmit_import(required, witness);
}

fn main() {}
