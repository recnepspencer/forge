use forge_store::{BroadenedBatchReceipt, ForgeStoreBuilder};

fn main() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let receipt: BroadenedBatchReceipt =
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = store.admit_continuation_advance(receipt);
}
