use forge_store::{CursorContinuationPlan, ForgeStoreBuilder};

fn main() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let plan: CursorContinuationPlan = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = store.admit_continuation_advance(plan);
}
