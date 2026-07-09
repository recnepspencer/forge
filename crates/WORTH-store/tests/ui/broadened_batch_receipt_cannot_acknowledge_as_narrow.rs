use worth_store::{BroadenedBatchReceipt, WORTHStoreBuilder};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let receipt: BroadenedBatchReceipt =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = store.admit_continuation_advance(receipt);
}
