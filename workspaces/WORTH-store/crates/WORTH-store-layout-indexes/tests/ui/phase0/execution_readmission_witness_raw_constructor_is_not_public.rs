use worth_store_layout_indexes::{S8AccessLoweringBasis, S8AccessPathCounterSnapshot, S8ExecutionReadmissionWitness};

fn main() {
    let basis = unsafe { core::mem::MaybeUninit::<S8AccessLoweringBasis>::zeroed().assume_init() };
    let planned = S8AccessPathCounterSnapshot::new(0, 0, 0, 0, 0);
    let _ = S8ExecutionReadmissionWitness::new(basis, planned);
}
