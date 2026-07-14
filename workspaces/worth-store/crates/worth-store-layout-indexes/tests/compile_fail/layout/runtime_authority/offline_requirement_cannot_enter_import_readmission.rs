use worth_store_layout_indexes::integrity::{
    import_readmission, OfflineReadmissionRequirement,
};
use worth_store_recovery_physics::RecoveryLayoutReadmissionWitness;

fn misuse(required: OfflineReadmissionRequirement, witness: RecoveryLayoutReadmissionWitness) {
    let _ = import_readmission().admit(required, witness);
}

fn main() {}
