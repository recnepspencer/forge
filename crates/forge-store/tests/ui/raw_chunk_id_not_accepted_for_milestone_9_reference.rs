use forge_store::{ForgeStoreBuilder, PhysicalChunkId};

fn main() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let _ = store.admit_milestone_9_physical_chunk_reference(PhysicalChunkId::new("chunk-a"));
}
