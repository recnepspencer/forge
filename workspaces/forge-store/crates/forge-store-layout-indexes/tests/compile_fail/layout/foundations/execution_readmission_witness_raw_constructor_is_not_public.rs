use forge_store_layout_indexes::access_lowering::S8AccessLoweringBasis;
use forge_store_layout_indexes::layout_counters::S8AccessPathCounterSnapshot;
use forge_store_layout_indexes::layout_readmission::S8ExecutionReadmissionWitness;

fn main() {
    let basis = unsafe { core::mem::MaybeUninit::<S8AccessLoweringBasis>::zeroed().assume_init() };
    let planned = S8AccessPathCounterSnapshot::new(0, 0, 0, 0, 0);
    let _ = S8ExecutionReadmissionWitness::new(basis, planned);
}
