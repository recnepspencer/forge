use schema::facade::{CreateKey, MutationOrigin, RawTopologyIntent, TopologyMutationBatch};

fn main() {
    let _ = CreateKey::new("example");
    let _ = MutationOrigin::Seed;
    let _ = std::mem::size_of::<RawTopologyIntent>();
    let _ = std::mem::size_of::<TopologyMutationBatch>();
}
