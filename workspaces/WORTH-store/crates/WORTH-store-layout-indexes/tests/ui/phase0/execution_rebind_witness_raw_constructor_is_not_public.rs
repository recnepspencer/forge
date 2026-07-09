use worth_store_layout_indexes::{S8AccessLoweringBasis, S8ExecutionRebindWitness};

fn main() {
    let basis = unsafe { core::mem::MaybeUninit::<S8AccessLoweringBasis>::zeroed().assume_init() };
    let _ = S8ExecutionRebindWitness::new(basis);
}
