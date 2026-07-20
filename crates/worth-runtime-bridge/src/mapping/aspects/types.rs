#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthDeltaSurfaceKind {
    AuthoritativeAspect,
    EntityField,
    EntityRelationEndpoint,
    EntityRegion,
    EntityPartition,
    EntityFacet,
    LifecycleTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SliceWideningPolicy {
    Disallow,
    RegisteredEntityCoarseWidening,
    RegisteredAspectCoarseWidening,
    RegisteredSurfaceCoarseWidening,
    RegisteredPartitionWidening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeAuthoritativeSourcePrecisionPolicy {
    ExactOnly,
    AdmitDeclared(crate::input::envelope::BridgeAspectChangeWideningCause),
}
