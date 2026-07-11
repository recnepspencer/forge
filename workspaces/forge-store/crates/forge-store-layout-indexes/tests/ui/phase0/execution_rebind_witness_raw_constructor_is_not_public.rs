use forge_store_layout_indexes::access_lowering::S8AccessLoweringBasis;
use forge_store_layout_indexes::layout_readmission::S8ExecutionRebindWitness;

fn main() {
    let basis = unsafe { core::mem::MaybeUninit::<S8AccessLoweringBasis>::zeroed().assume_init() };
    let _ = S8ExecutionRebindWitness::new(basis);
}
