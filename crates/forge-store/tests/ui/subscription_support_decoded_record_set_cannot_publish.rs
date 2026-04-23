use forge_store::{ForgeStoreBuilder, SubscriptionSupportStoredRecordSet};

fn main() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let decoded_record_set: SubscriptionSupportStoredRecordSet = unsafe { std::mem::zeroed() };

    let _ = store.publish_subscription_support(decoded_record_set);
}
