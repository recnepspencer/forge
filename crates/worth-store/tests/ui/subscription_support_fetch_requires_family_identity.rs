use worth_store::{WORTHStoreBuilder, SubscriptionSupportArtifactId};

fn main() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let artifact_id: SubscriptionSupportArtifactId = unsafe { std::mem::zeroed() };

    let _ = store.fetch_subscription_support(artifact_id);
}
