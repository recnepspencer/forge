use forge_store::{
    ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout, ChunkShapeVersion, PhysicalChunkId,
};

fn main() {
    let _ = ChunkDeterminismWitness::new(
        PhysicalChunkId::new("chunk-a"),
        ChunkShapeVersion::new(1),
        "digest-a".to_string(),
        vec![],
    );
    let _ = ChunkModelFrozenPhysicalLayout;
}
