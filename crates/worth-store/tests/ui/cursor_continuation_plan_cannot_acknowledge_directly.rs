use worth_store::{CursorContinuationPlan, WORTHStoreBuilder};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan: CursorContinuationPlan = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = store.admit_continuation_advance(plan);
}
