use worth_store::{WORTHStoreBuilder, PhysicalChunkId};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let _ = store.admit_milestone_9_physical_chunk_reference(PhysicalChunkId::new("chunk-a"));
}
