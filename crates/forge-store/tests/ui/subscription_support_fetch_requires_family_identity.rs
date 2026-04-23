use forge_store::{ForgeStoreBuilder, SubscriptionSupportArtifactId};

fn main() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let artifact_id: SubscriptionSupportArtifactId = unsafe { std::mem::zeroed() };

    let _ = store.fetch_subscription_support(artifact_id);
}
