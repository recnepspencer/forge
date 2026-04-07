#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthDeltaSurfaceKind {
    EntityField,
    EntityRelationEndpoint,
    EntityRegion,
    EntityPartition,
    EntityFacet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SliceFallbackPolicy {
    Disallow,
    RegisteredEntityCoarseFallback,
    RegisteredPartitionFallback,
}
