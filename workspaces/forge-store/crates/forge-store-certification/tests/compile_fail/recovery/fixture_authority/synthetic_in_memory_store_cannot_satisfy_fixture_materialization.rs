use forge_store_physical_certification::PhysicalFixtureBuilder;

struct SyntheticInMemoryStore {
    pages: Vec<Vec<u8>>,
}

fn main() {
    let synthetic_store = SyntheticInMemoryStore {
        pages: vec![vec![0; 16]],
    };
    let _fixture = PhysicalFixtureBuilder::production_backed("synthetic-store")
        .materialize_with(synthetic_store);
}
