#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutStrategyFamily {
    AppendLog,
    HeapFile,
    PageTable,
    BaselineBTreeRange,
    BaselineLsmWriteOptimized,
    SparseIndex,
    ChunkTree,
    ManifestTable,
    BitmapAllocationMap,
    HashEqualityIndex,
    RangeMap,
    QuarantineMap,
    StreamingCursorIndex,
    ExactScan,
}

impl S8LayoutStrategyFamily {
    pub const fn is_baseline_family(self) -> bool {
        matches!(
            self,
            Self::BaselineBTreeRange | Self::BaselineLsmWriteOptimized
        )
    }

    pub const fn declaration_name(self) -> &'static str {
        match self {
            Self::AppendLog => "append-log",
            Self::HeapFile => "heap-file",
            Self::PageTable => "page-table",
            Self::BaselineBTreeRange => "baseline-btree-range",
            Self::BaselineLsmWriteOptimized => "baseline-lsm-write-optimized",
            Self::SparseIndex => "sparse-index",
            Self::ChunkTree => "chunk-tree",
            Self::ManifestTable => "manifest-table",
            Self::BitmapAllocationMap => "bitmap-allocation-map",
            Self::HashEqualityIndex => "hash-equality-index",
            Self::RangeMap => "range-map",
            Self::QuarantineMap => "quarantine-map",
            Self::StreamingCursorIndex => "streaming-cursor-index",
            Self::ExactScan => "exact-scan",
        }
    }
}
