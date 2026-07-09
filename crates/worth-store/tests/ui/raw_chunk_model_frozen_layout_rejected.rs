use worth_store::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout, ChunkShapeVersion, PhysicalChunkId,
    SingleEntityAspectScope,
};
use worth_relational::facade::history::{BranchId, CommitId};

fn main() {
    let _ = ChunkModelFrozenPhysicalLayout::new(
        AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(BranchId("main".to_string()), CommitId(1)),
            AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ),
        1,
        ChunkDeterminismWitness::new(
            PhysicalChunkId::new("chunk-a"),
            ChunkShapeVersion::new(1),
            "digest-a".to_string(),
            vec![],
        ),
    );
}
