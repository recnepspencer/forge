#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLayoutFamily {
    AppendLog,
    HeapFile,
    BTree,
    LsmLike,
    SparseIndex,
    ChunkTree,
}
